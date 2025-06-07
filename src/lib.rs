use axum::{routing::get, Router};

async fn health_check() -> &'static str {
    "OK"
}

pub fn build_router() -> Router {
    Router::new().route("/health", get(health_check))
}
