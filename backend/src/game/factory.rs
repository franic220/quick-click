use crate::game::{GameConfig, GameId, GameSession, PlayerId};

/// Factory for creating game sessions with consistent configuration
pub struct GameSessionFactory {
    config: GameConfig,
}

impl GameSessionFactory {
    pub fn new(config: GameConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(GameConfig::default())
    }

    /// Create a new game session for a player
    pub fn create(&self, player_id: PlayerId) -> GameSession {
        GameSession::new(GameId::new(), player_id, self.config.clone())
    }

    /// Create with a specific game ID (useful for testing)
    pub fn create_with_id(&self, game_id: GameId, player_id: PlayerId) -> GameSession {
        GameSession::new(game_id, player_id, self.config.clone())
    }
}

impl Default for GameSessionFactory {
    fn default() -> Self {
        Self::with_default_config()
    }
}
