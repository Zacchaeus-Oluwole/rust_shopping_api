// use axum::handler::get;
use axum::Router;
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

mod routes;
mod errors;
mod apis;

use apis::config::Config;

use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use tower_http::cors::CorsLayer;

pub struct AppState {
    db: PgPool,
    config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenv().ok();
    let config = Config::init();

    // let env = fs::read_to_string(".env").unwrap();
    // let (_, database_url) = env.split_once("=").unwrap();

    let pool = PgPoolOptions::new()
    .max_connections(50)
    .connect(&config.database_url)
    .await
    .context("Could not connect to the database_url")?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new().nest("/api", routes::create_router(
        Arc::new(AppState {
            db: pool.clone(),
            config: config.clone()
        })
    ))
    .layer(cors);

    let addr = "127.0.0.1:8000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}