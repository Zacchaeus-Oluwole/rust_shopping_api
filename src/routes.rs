use std::sync::Arc;

use axum::Router;

use crate::apis::{
    login::login_route,
    v1::v_route
};

use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("", login_route::login_router(app_state.clone()))
        .nest("", v_route::v1_routes(app_state.clone()))
}