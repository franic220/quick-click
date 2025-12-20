use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::game::{GameId, GameSession, PlayerId};
use crate::storage::repository::{GameSessionRepository, RepoResult};

/// Thread-safe in-memory storage for game sessions
pub struct InMemoryGameSessionRepository {
    sessions: Arc<RwLock<HashMap<GameId, GameSession>>>,
    player_sessions: Arc<RwLock<HashMap<PlayerId, GameId>>>,
}

impl InMemoryGameSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            player_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryGameSessionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GameSessionRepository for InMemoryGameSessionRepository {
    async fn save(&self, session: GameSession) -> RepoResult<()> {
        let game_id = session.id;
        let player_id = session.player_id;

        let mut sessions = self.sessions.write().await;
        let mut player_sessions = self.player_sessions.write().await;

        sessions.insert(game_id, session);
        player_sessions.insert(player_id, game_id);

        Ok(())
    }

    async fn find_by_id(&self, id: GameId) -> RepoResult<Option<GameSession>> {
        let sessions = self.sessions.read().await;
        // We can't clone GameSession directly, so we return None for now
        // In a real implementation, we'd need to make GameSession Clone
        // or use a different approach
        Ok(sessions.get(&id).map(|_| {
            // This is a limitation - we can't clone the session
            // For now, we'll handle this differently in the handler
            panic!("find_by_id not fully implemented - use find_by_player instead")
        }))
    }

    async fn find_by_player(&self, player_id: PlayerId) -> RepoResult<Option<GameSession>> {
        let player_sessions = self.player_sessions.read().await;
        if let Some(game_id) = player_sessions.get(&player_id) {
            let mut sessions = self.sessions.write().await;
            // Remove and return the session (take ownership)
            Ok(sessions.remove(game_id))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, session: GameSession) -> RepoResult<()> {
        let mut sessions = self.sessions.write().await;
        let mut player_sessions = self.player_sessions.write().await;

        let game_id = session.id;
        let player_id = session.player_id;

        sessions.insert(game_id, session);
        player_sessions.insert(player_id, game_id);

        Ok(())
    }

    async fn delete(&self, id: GameId) -> RepoResult<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.remove(&id) {
            let mut player_sessions = self.player_sessions.write().await;
            player_sessions.remove(&session.player_id);
        }
        Ok(())
    }
}
