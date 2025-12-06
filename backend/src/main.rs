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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello(Query(params): Query<HelloParams>) -> String {
    let name = params.name.unwrap_or_else(|| "World".to_string());
    format!("Hello {name}")
}
