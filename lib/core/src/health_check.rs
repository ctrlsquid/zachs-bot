use axum::{Error, Router, routing::get};
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string()).into_response()
}

const PORT: &str = "3000";
const HOST: &str = "0.0.0.0";

/// A function that starts a health check server using axum
pub async fn start_server() -> hyper::Result<()> {
    // Initialize router
    let app = Router::new()
        .route("/health", get(health_check));
    // Parse intended address
    let addr = format!("{HOST}{PORT}");
    // Start server
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
}