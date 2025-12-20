use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Invalid state transition: cannot {action} from {from} state")]
    InvalidStateTransition {
        from: &'static str,
        action: &'static str,
    },

    #[error("Game not found: {0}")]
    GameNotFound(String),

    #[error("Player not found: {0}")]
    PlayerNotFound(String),

    #[error("Storage error: {0}")]
    StorageError(String),
}
