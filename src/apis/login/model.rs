use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    pub verified: bool,
    pub verification_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}