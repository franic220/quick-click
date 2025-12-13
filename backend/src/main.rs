mod http;
mod ws;

use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = http::router().merge(ws::router());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Server running on port {}", port);
    axum::serve(listener, app).await.unwrap();
}
