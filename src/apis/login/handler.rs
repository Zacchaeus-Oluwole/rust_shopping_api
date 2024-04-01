use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Path, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{rngs::OsRng, Rng};
use serde_json::json;

use crate::apis::login::{
    model::{LoginUserSchema, RegisterUserSchema, TokenClaims, User},
    response::{ErrorResponse, FilteredUser},
};

use crate::AppState;

use super::email::send_email;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str =
        "Rust - User Registration and Email Verification using Axum, Postgres, and SQLX";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(body.email.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = ErrorResponse {
                    status: "fail",
                    message: format!("Database error: {}", e),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = ErrorResponse {
                status: "fail",
                message: "User with that email already exists".to_string(),
            };
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = ErrorResponse {
                status: "fail",
                message: format!("Error while hashing password: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let verification_code = gen_rand_num();
    let verification_code_string = &verification_code.to_string();
    let email = body.email.to_owned().to_ascii_lowercase();

    let email_body = format!("Just checking to be sure you are the one. Please copy and paste the following code into the verification Code field. {}", verification_code);

    let _user: User = sqlx::query_as(
        "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(body.name.to_owned())
    .bind(email.clone())
    .bind(hashed_password)
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = ErrorResponse {
            status: "fail",
            message: format!("Database error: {}", e),
        };
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_response.into()),
        )
    })?;

    //  Create an Email instance
    if let Err(_) = send_email(email.clone(), email_body) {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Something bad happened while sending the verification code".to_string(),
        };
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_error)));
    }

    sqlx::query("UPDATE users SET verification_code = $1")
        .bind(verification_code_string)
        .execute(&data.db)
        .await
        .map_err(|e| {
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error updating user: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?;

    let user_response = serde_json::json!({"status": "success","message": format!("We sent an email with a verification code to {}", email)});

    Ok(Json(user_response))
}

pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let email = body.email.to_ascii_lowercase();
    let user: User = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let error_response = ErrorResponse {
                status: "error",
                message: format!("Database error: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?
        .ok_or_else(|| {
            let error_response = ErrorResponse {
                status: "fail",
                message: "Invalid email or password".to_string(),
            };
            (StatusCode::BAD_REQUEST, Json(error_response))
        })?;

    if !user.verified {
        let error_response = ErrorResponse {
            status: "fail",
            message: "Please verify your email before you can log in".to_string(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = ErrorResponse {
            status: "fail",
            message: "Invalid email or password".to_string(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.config.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn verify_email_handler(
    State(data): State<Arc<AppState>>,
    Path(verification_code): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user: User = sqlx::query_as("SELECT * FROM users WHERE verification_code = $1")
        .bind(&verification_code)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            let error_response = ErrorResponse {
                status: "error",
                message: format!("Database error: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?
        .ok_or_else(|| {
            let error_response = ErrorResponse {
                status: "fail",
                message: "Invalid verification code or user doesn't exist".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;

    if user.verified {
        let error_response = ErrorResponse {
            status: "fail",
            message: "User already verified".to_string(),
        };
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    sqlx::query(
        "UPDATE users SET verification_code = $1, verified = $2 WHERE verification_code = $3",
    )
    .bind("")
    .bind(true)
    .bind(&verification_code)
    .execute(&data.db)
    .await
    .map_err(|e| {
        let json_error = ErrorResponse {
            status: "fail",
            message: format!("Error updating user: {}", e),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
    })?;

    let response = serde_json::json!({
            "status": "success",
            "message": "Email verified successfully"
        }
    );

    Ok(Json(response))
}

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn get_me_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&user)
        })
    });

    Ok(Json(json_response))
}

fn filter_user_record(user: &User) -> FilteredUser {
    let created_at_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(user.created_at.unwrap(), Utc);
    let updated_at_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(user.updated_at.unwrap(), Utc);
    FilteredUser {
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        verified: user.verified,
        createdAt: created_at_utc,
        updatedAt: updated_at_utc,
    }
}

fn gen_rand_num() -> u32 {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(100_000..1_000_000);
    random_number
}
