use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{
    Router,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
};
use futures::{SinkExt, StreamExt};
use log::{error, info};
use tokio::sync::mpsc;

use crate::game::PlayerId;
use crate::messages::{ClientMessage, MessageHandler, ServerMessage, StreamControl};

/// Interval between timer ticks (10ms = 100 updates/sec)
const TICK_INTERVAL: Duration = Duration::from_millis(10);

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub message_handler: Arc<MessageHandler>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    // Generate player ID for this connection
    let player_id = PlayerId::new();
    info!("Client connected: {}", player_id.0);

    // Split socket into sender and receiver
    let (mut sender, mut receiver) = socket.split();

    // Send welcome message
    let welcome = ServerMessage::Connected {
        player_id: player_id.0.to_string(),
    };
    if let Err(e) = send_message_to_sink(&mut sender, &welcome).await {
        error!("Failed to send welcome: {}", e);
        return;
    }

    // Channel for timer ticks (sender held by timer task, receiver in main loop)
    let (tick_tx, mut tick_rx) = mpsc::channel::<ServerMessage>(100);

    // Handle to abort the timer task
    let mut timer_handle: Option<tokio::task::JoinHandle<()>> = None;

    loop {
        tokio::select! {
            // Handle incoming client messages
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(client_msg) => {
                                let response = state
                                    .message_handler
                                    .handle(player_id, client_msg)
                                    .await;

                                // Handle stream control
                                match response.stream_control {
                                    StreamControl::StartTimer { start_time } => {
                                        // Cancel any existing timer
                                        if let Some(handle) = timer_handle.take() {
                                            handle.abort();
                                        }
                                        // Start new timer task
                                        let tx = tick_tx.clone();
                                        timer_handle = Some(tokio::spawn(async move {
                                            timer_loop(tx, start_time).await;
                                        }));
                                    }
                                    StreamControl::StopTimer => {
                                        // Cancel the timer task
                                        if let Some(handle) = timer_handle.take() {
                                            handle.abort();
                                        }
                                    }
                                    StreamControl::None => {}
                                }

                                // Send the response message
                                if let Err(e) = send_message_to_sink(&mut sender, &response.message).await {
                                    error!("Failed to send response: {}", e);
                                    break;
                                }
                            }
                            Err(e) => {
                                let error = ServerMessage::Error {
                                    code: "INVALID_MESSAGE".to_string(),
                                    message: format!("Failed to parse message: {}", e),
                                };
                                let _ = send_message_to_sink(&mut sender, &error).await;
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        info!("Client {} disconnected gracefully", player_id.0);
                        break;
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error for {}: {}", player_id.0, e);
                        break;
                    }
                    None => {
                        // Stream ended
                        break;
                    }
                    _ => {} // Ignore binary, ping, pong
                }
            }

            // Handle timer ticks
            Some(tick_msg) = tick_rx.recv() => {
                if let Err(e) = send_message_to_sink(&mut sender, &tick_msg).await {
                    error!("Failed to send timer tick: {}", e);
                    break;
                }
            }
        }
    }

    // Cleanup: abort timer if still running
    if let Some(handle) = timer_handle.take() {
        handle.abort();
    }

    info!("Client disconnected: {}", player_id.0);
}

/// Timer loop that sends ticks every TICK_INTERVAL
async fn timer_loop(tx: mpsc::Sender<ServerMessage>, start_time: Instant) {
    let mut interval = tokio::time::interval(TICK_INTERVAL);

    loop {
        interval.tick().await;
        let elapsed_ms = start_time.elapsed().as_millis() as u64;
        let msg = ServerMessage::TimerTick { elapsed_ms };

        if tx.send(msg).await.is_err() {
            // Receiver dropped, stop the loop
            break;
        }
    }
}

async fn send_message_to_sink<S>(sender: &mut S, msg: &ServerMessage) -> Result<(), String>
where
    S: SinkExt<Message> + Unpin,
    S::Error: std::fmt::Display,
{
    let json = serde_json::to_string(msg).map_err(|e| e.to_string())?;
    sender
        .send(Message::Text(json.into()))
        .await
        .map_err(|e| e.to_string())
}
