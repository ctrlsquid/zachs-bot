use axum::{
    routing::get,
    Router,
};
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK".to_string()).into_response()
}


/// A function that starts a health check server using axum
pub async fn start_server() {
    let app = Router::new()
        .route("/health", get(health_check));

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}