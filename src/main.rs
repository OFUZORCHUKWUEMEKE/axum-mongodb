mod db;
mod error;
mod handler;
mod models;
mod response;
mod route;
mod schema;
use axum::{response::IntoResponse, routing::get, Json, Router};
use db::DB;
use dotenv::dotenv;
use error::MyError;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};

pub struct AppState {
    db: DB,
}

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "RESTful API in rust using axum framework and mongodb";

    let json_response = serde_json::json!({
        "status":"success",
        "message":MESSAGE
    });
    Json(json_response)
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    dotenv().ok();

    let db = DB::init().await?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(Arc::new(AppState { db: db.clone() }));
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
