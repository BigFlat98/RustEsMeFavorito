use axum::{
    Router,
    routing::{get,post,put,delete},
    extract::{Json,Path,State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::mysql::MySqlPool;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;



#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    println!("port: {}", port);

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",port)).await.expect("Failed to bind to port");
    println!("Server is running on http://127.0.0.1:{}",port);
    axum::serve(listener, app.into_make_service()).await.expect("Failed to serve");
}
