use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::game::{AttemptTime, GameSessionFactory, GameState, PlayerId};
use crate::messages::protocol::{ClientMessage, ErrorCode, ServerMessage};
use crate::storage::GameSessionRepository;

/// Instructions for the WebSocket handler about streaming
#[derive(Debug, Clone)]
pub enum StreamControl {
    /// No streaming action needed
    None,
    /// Start streaming timer ticks from this instant
    StartTimer { start_time: Instant },
    /// Stop streaming timer ticks
    StopTimer,
}

/// Response from handler including message and stream control
pub struct HandlerResponse {
    pub message: ServerMessage,
    pub stream_control: StreamControl,
}

impl HandlerResponse {
    fn message_only(message: ServerMessage) -> Self {
        Self {
            message,
            stream_control: StreamControl::None,
        }
    }

    fn with_start_timer(message: ServerMessage, start_time: Instant) -> Self {
        Self {
            message,
            stream_control: StreamControl::StartTimer { start_time },
        }
    }

    fn with_stop_timer(message: ServerMessage) -> Self {
        Self {
            message,
            stream_control: StreamControl::StopTimer,
        }
    }
}

/// Command handler - processes incoming messages and produces responses
pub struct MessageHandler {
    factory: Arc<GameSessionFactory>,
    repository: Arc<dyn GameSessionRepository>,
}

impl MessageHandler {
    pub fn new(
        factory: Arc<GameSessionFactory>,
        repository: Arc<dyn GameSessionRepository>,
    ) -> Self {
        Self {
            factory,
            repository,
        }
    }

    /// Handle an incoming client message
    pub async fn handle(
        &self,
        player_id: PlayerId,
        message: ClientMessage,
    ) -> HandlerResponse {
        match message {
            ClientMessage::StartGame => self.handle_start_game(player_id).await,
            ClientMessage::StopTimer { client_time_ms } => {
                self.handle_stop_timer(player_id, client_time_ms).await
            }
            ClientMessage::PlayAgain => self.handle_play_again(player_id).await,
            ClientMessage::Ping { timestamp } => self.handle_ping(timestamp),
        }
    }

    async fn handle_start_game(&self, player_id: PlayerId) -> HandlerResponse {
        // Create new game session
        let mut session = self.factory.create(player_id);

        match session.start() {
            Ok(target_time) => {
                let game_id = session.id;
                // Capture start time for streaming
                let start_time = Instant::now();

                // Save session
                if let Err(e) = self.repository.save(session).await {
                    return HandlerResponse::message_only(Self::error_response(
                        ErrorCode::InternalError,
                        &e.to_string(),
                    ));
                }

                HandlerResponse::with_start_timer(
                    ServerMessage::GameStarted {
                        game_id: game_id.0.to_string(),
                        target_time_ms: target_time.as_millis(),
                    },
                    start_time,
                )
            }
            Err(e) => {
                HandlerResponse::message_only(Self::error_response(ErrorCode::InvalidState, &e.to_string()))
            }
        }
    }

    async fn handle_stop_timer(&self, player_id: PlayerId, client_time_ms: u64) -> HandlerResponse {
        // Find player's active session (this takes ownership)
        let session = match self.repository.find_by_player(player_id).await {
            Ok(Some(s)) => s,
            Ok(None) => {
                return HandlerResponse::message_only(Self::error_response(
                    ErrorCode::GameNotFound,
                    "No active game",
                ));
            }
            Err(e) => {
                return HandlerResponse::message_only(Self::error_response(
                    ErrorCode::InternalError,
                    &e.to_string(),
                ));
            }
        };

        let mut session = session;

        // Verify game is in running state
        if !matches!(session.state(), GameState::Running(_)) {
            return HandlerResponse::message_only(Self::error_response(
                ErrorCode::InvalidState,
                "Game not running",
            ));
        }

        let client_time = AttemptTime(client_time_ms);

        match session.stop(client_time) {
            Ok(result) => {
                // Update session in storage
                if let Err(e) = self.repository.update(session).await {
                    return HandlerResponse::message_only(Self::error_response(
                        ErrorCode::InternalError,
                        &e.to_string(),
                    ));
                }

                HandlerResponse::with_stop_timer(ServerMessage::GameResult {
                    target_time_ms: result.target_time.as_millis(),
                    your_time_ms: result.attempt_time.as_millis(),
                    difference_ms: result.difference_ms,
                    difference_sec: result.difference_sec,
                    timing_valid: result.timing_valid,
                })
            }
            Err(e) => {
                HandlerResponse::message_only(Self::error_response(ErrorCode::InvalidState, &e.to_string()))
            }
        }
    }

    async fn handle_play_again(&self, player_id: PlayerId) -> HandlerResponse {
        // Find player's session
        let session = match self.repository.find_by_player(player_id).await {
            Ok(Some(s)) => s,
            Ok(None) => {
                return HandlerResponse::message_only(Self::error_response(
                    ErrorCode::GameNotFound,
                    "No game to reset",
                ));
            }
            Err(e) => {
                return HandlerResponse::message_only(Self::error_response(
                    ErrorCode::InternalError,
                    &e.to_string(),
                ));
            }
        };

        let mut session = session;

        match session.reset() {
            Ok(()) => {
                // Delete old session - a new one will be created on StartGame
                let _ = self.repository.delete(session.id).await;
                HandlerResponse::message_only(ServerMessage::GameReset)
            }
            Err(e) => {
                HandlerResponse::message_only(Self::error_response(ErrorCode::InvalidState, &e.to_string()))
            }
        }
    }

    fn handle_ping(&self, client_timestamp: u64) -> HandlerResponse {
        let server_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        HandlerResponse::message_only(ServerMessage::Pong {
            client_timestamp,
            server_timestamp,
        })
    }

    fn error_response(code: ErrorCode, message: &str) -> ServerMessage {
        ServerMessage::Error {
            code: code.as_str().to_string(),
            message: message.to_string(),
        }
    }
}
