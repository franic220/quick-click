mod error;
mod game;
mod http;
mod messages;
mod storage;
mod validation;
mod ws;

use std::sync::Arc;

use log::info;

use crate::game::{GameConfig, GameSessionFactory};
use crate::messages::MessageHandler;
use crate::storage::InMemoryGameSessionRepository;
use crate::ws::AppState;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Initialize configuration
    let config = GameConfig::default();

    // Initialize dependencies
    let factory = Arc::new(GameSessionFactory::new(config));
    let repository = Arc::new(InMemoryGameSessionRepository::new());
    let message_handler = Arc::new(MessageHandler::new(factory, repository));

    // Create application state
    let app_state = AppState { message_handler };

    // Build router
    let app = http::router().merge(ws::router(app_state));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("Server running on port {}", port);
    axum::serve(listener, app).await.unwrap();
}
