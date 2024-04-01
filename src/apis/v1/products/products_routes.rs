use std::sync::Arc;

use axum::{
    routing::{get, post, put, delete},
    Router
};
use crate::{apis::v1::products::products_handler, AppState};

pub fn products_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(products_handler::post_product))
        .route("/:id", get(products_handler::get_product))
        .route("/:id", put(products_handler::update_product))
        .route("/:id", delete(products_handler::delete_product))
        .with_state(app_state)
}