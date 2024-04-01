use std::sync::Arc;

use axum::{
    routing::{get, post, put, delete},
    Router
};
use crate::{apis::v1::category::category_handler, AppState};

pub fn category_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(category_handler::post_category))
        .route("/:id", get(category_handler::get_category))
        .route("/:id", put(category_handler::update_category))
        .route("/:id", delete(category_handler::delete_category))
        .with_state(app_state)
}