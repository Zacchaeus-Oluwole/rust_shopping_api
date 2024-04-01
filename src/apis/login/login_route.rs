use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::apis::{
    login::handler::{
        get_me_handler, health_checker_handler, login_user_handler, logout_handler,
        register_user_handler, verify_email_handler,
    },
    jwt_auth::auth,
};

use crate::AppState;

pub fn login_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/apichecker", get(health_checker_handler))
        .route("/auth/register", post(register_user_handler))
        .route("/auth/login", post(login_user_handler))
        .route(
            "/auth/logout",
            get(logout_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/auth/verifyemail/:verification_code",
            get(verify_email_handler),
        )
        .route(
            "/me",
            get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}
