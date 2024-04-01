use std::sync::Arc;

use axum::{
    routing::get,
    Router, middleware
};
use crate::apis::{v1::{products::{products_routes, products_handler}, category::{category_routes, category_handler}}, jwt_auth::auth};
use crate::AppState;

pub fn v1_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/products", get(products_handler::get_products))
        .route("/categories", get(category_handler::get_categories))
        .with_state(app_state.clone())
        .nest("/products", products_routes::products_router(app_state.clone()))
        .nest("/categories", category_routes::category_router(app_state.clone()))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
}
