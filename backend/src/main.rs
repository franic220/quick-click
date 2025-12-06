use axum::{Router, routing::get, extract::Query};
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/hello", get(hello));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Server running on port {}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn hello(Query(params): Query<HelloParams>) -> String {
    let name = params.name.unwrap_or_else(|| "World".to_string());
    format!("Hello {name}")
}
