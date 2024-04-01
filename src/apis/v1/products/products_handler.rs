use crate::errors::CustomError;
use crate::apis::v1::products::products_model::{Product, NewProduct};

// Implement similar functions for other CRUD operations

use axum::{response::IntoResponse, extract::{Path,State}, http::StatusCode, Json};
use serde_json::{json, Value};
use crate::AppState;
use std::sync::Arc;

pub async fn get_products(State(pool): State<Arc<AppState>>) -> impl IntoResponse {
    let sql = "SELECT * FROM products".to_string();
    let product = sqlx::query_as::<_, Product>(&sql).fetch_all(&pool.db).await.unwrap();

    (StatusCode::OK, Json(product))
}

pub async fn get_product(Path(id): Path<i32>, State(pool): State<Arc<AppState>>) -> Result<Json<Product>, CustomError> {
    let sql = "SELECT * FROM products where id=$1".to_string();
    let product : Product = sqlx::query_as(&sql).bind(id).fetch_one(&pool.db).await.map_err(|_| {
        CustomError::TaskNotFound
    })?;

    Ok(Json(product))
}

#[axum_macros::debug_handler]
pub async fn post_product(State(pool): State<Arc<AppState>>, Json(data): Json<NewProduct>) -> Result<(StatusCode, Json<NewProduct>), CustomError> {
    let sql = "INSERT INTO product (id, name, category_name, description, available, price) values ($1, $2, $3, $4, $5, $6)".to_string();
    let _  = sqlx::query(&sql)
    .bind(&data.id)
    .bind(&data.name)
    .bind(&data.category_name)
    .bind(&data.description)
    .bind(&data.price)
    .execute(&pool.db)
    .await.map_err(|_| {
        CustomError::InternalServerError
    })?;

    Ok((StatusCode::CREATED, Json(data)))
}

pub async fn update_product(Path(id): Path<i32>, State(pool): State<Arc<AppState>>, Json(data): Json<NewProduct>) -> Result<(StatusCode, Json<NewProduct>), CustomError> {
    let sql = "SELECT * FROM product where id=$1".to_string();
    let _ :Product = sqlx::query_as(&sql).bind(id).fetch_one(&pool.db).await.map_err(|_| {
        CustomError::TaskNotFound
    })?;

    sqlx::query("UPDATE product SET name = $1, category_name = $2, description = $, available = $4, price = $5 WHERE id=$6")
    .bind(&data.id)
    .bind(&data.name)
    .bind(&data.category_name)
    .bind(&data.description)
    .bind(&data.price)
    .execute(&pool.db)
    .await.map_err(|_| {
        CustomError::InternalServerError
    })?;

    Ok((StatusCode::OK, Json(data)))
}

pub async fn delete_product(Path(id): Path<i32>, State(pool): State<Arc<AppState>>) -> Result<(StatusCode, Json<Value>), CustomError> {
    let sql = "SELECT * FROM product where id=$1".to_string();
    let _ : Product = sqlx::query_as(&sql)
    .bind(id)
    .fetch_one(&pool.db)
    .await
    .map_err(|_| {
        CustomError::TaskNotFound
    })?;

    sqlx::query("DELETE FROM employee WHERE id=$1")
    .bind(id)
    .execute(&pool.db)
    .await
    .map_err(|_| {
        CustomError::TaskNotFound
    })?;

    Ok((StatusCode::OK ,Json(json!({"msg": "Profile Deleted"}))))
}
