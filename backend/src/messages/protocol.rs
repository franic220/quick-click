use serde::{Deserialize, Serialize};

/// Messages sent from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClientMessage {
    /// Request to start a new game
    StartGame,

    /// Player clicked stop - includes client-measured time in milliseconds
    StopTimer { client_time_ms: u64 },

    /// Request to play again
    PlayAgain,

    /// Ping for latency measurement
    Ping { timestamp: u64 },
}

/// Messages sent from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    /// Welcome message with player ID
    Connected { player_id: String },

    /// Game started - client should start timer
    GameStarted {
        game_id: String,
        target_time_ms: u64,
    },

    /// Real-time timer tick (streamed while game is running)
    TimerTick { elapsed_ms: u64 },

    /// Game result after stop
    GameResult {
        target_time_ms: u64,
        your_time_ms: u64,
        difference_ms: i64,
        difference_sec: f64,
        timing_valid: bool,
    },

    /// Ready for new game
    GameReset,

    /// Error occurred
    Error { code: String, message: String },

    /// Pong response for latency measurement
    Pong {
        client_timestamp: u64,
        server_timestamp: u64,
    },
}

/// Error codes for client
#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    InvalidState,
    InvalidMessage,
    GameNotFound,
    InternalError,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::InvalidState => "INVALID_STATE",
            ErrorCode::InvalidMessage => "INVALID_MESSAGE",
            ErrorCode::GameNotFound => "GAME_NOT_FOUND",
            ErrorCode::InternalError => "INTERNAL_ERROR",
        }
    }
}
