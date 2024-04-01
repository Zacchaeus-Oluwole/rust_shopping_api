use std::sync::Arc;
use crate::AppState;
use crate::errors::CustomError;
use crate::apis::v1::category::category_model::{Category, NewCategory};

// Implement similar functions for other CRUD operations

use axum::{response::IntoResponse, extract::{Path, State}, http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn get_categories(State(pool): State<Arc<AppState>>) -> impl IntoResponse {
    let sql = "SELECT * FROM categories".to_string();
    let category = sqlx::query_as::<_, Category>(&sql).fetch_all(&pool.db).await.unwrap();

    (StatusCode::OK, Json(category))
}

pub async fn get_category(Path(id): Path<i32>, State(pool): State<Arc<AppState>>) -> Result<Json<Category>, CustomError> {
    let sql = "SELECT * FROM categories where id=$1".to_string();
    let category : Category = sqlx::query_as(&sql).bind(id).fetch_one(&pool.db).await.map_err(|_| {
        CustomError::TaskNotFound
    })?;

    Ok(Json(category))
}

#[axum_macros::debug_handler]
pub async fn post_category(State(pool): State<Arc<AppState>>, Json(data): Json<NewCategory>) -> Result<(StatusCode, Json<NewCategory>), CustomError> {
    let sql = "INSERT INTO categories (name) values ($1)".to_string();
    let _  = sqlx::query(&sql)
    .bind(&data.name)
    .execute(&pool.db)
    .await.map_err(|_| {
        CustomError::InternalServerError
    })?;

    Ok((StatusCode::CREATED, Json(data)))
}

pub async fn update_category(Path(id): Path<i32>, State(pool): State<Arc<AppState>>, Json(data): Json<NewCategory>) -> Result<(StatusCode, Json<NewCategory>), CustomError> {
    let sql = "SELECT * FROM categories where id=$1".to_string();
    let _ :Category = sqlx::query_as(&sql).bind(id).fetch_one(&pool.db).await.map_err(|_| {
        CustomError::TaskNotFound
    })?;

    sqlx::query("UPDATE categories SET name = $1 WHERE id=$2")
    .bind(&data.id)
    .bind(&data.name)
    .execute(&pool.db)
    .await.map_err(|_| {
        CustomError::InternalServerError
    })?;

    Ok((StatusCode::OK, Json(data)))
}

pub async fn delete_category(Path(id): Path<i32>, State(pool): State<Arc<AppState>>) -> Result<(StatusCode, Json<Value>), CustomError> {
    let sql = "SELECT * FROM categories where id=$1".to_string();
    let _ : Category = sqlx::query_as(&sql)
    .bind(id)
    .fetch_one(&pool.db)
    .await
    .map_err(|_| {
        CustomError::TaskNotFound
    })?;

    sqlx::query("DELETE FROM categories WHERE id=$1")
    .bind(id)
    .execute(&pool.db)
    .await
    .map_err(|_| {
        CustomError::TaskNotFound
    })?;

    Ok((StatusCode::OK ,Json(json!({"msg": "Category Deleted"}))))
}
