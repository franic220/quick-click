use async_trait::async_trait;

use crate::error::GameError;
use crate::game::{GameId, GameSession, PlayerId};

/// Result type for repository operations
pub type RepoResult<T> = Result<T, GameError>;

/// Repository trait for game session storage.
/// Designed as async to support future database backends.
#[async_trait]
pub trait GameSessionRepository: Send + Sync {
    /// Store a new game session
    async fn save(&self, session: GameSession) -> RepoResult<()>;

    /// Retrieve a game session by ID
    async fn find_by_id(&self, id: GameId) -> RepoResult<Option<GameSession>>;

    /// Retrieve a game session by player ID (active session)
    async fn find_by_player(&self, player_id: PlayerId) -> RepoResult<Option<GameSession>>;

    /// Update an existing game session
    async fn update(&self, session: GameSession) -> RepoResult<()>;

    /// Delete a game session
    async fn delete(&self, id: GameId) -> RepoResult<()>;
}
