use axum::{Router, routing::get};

pub(crate) fn router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "OK"
}
