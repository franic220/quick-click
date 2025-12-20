use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Unique identifier for a game session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameId(pub uuid::Uuid);

impl GameId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for GameId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a player
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub uuid::Uuid);

impl PlayerId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

/// Target time the player must hit (in milliseconds)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TargetTime(pub u64);

impl TargetTime {
    pub fn as_millis(&self) -> u64 {
        self.0
    }

    pub fn as_secs_f64(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    pub fn as_duration(&self) -> Duration {
        Duration::from_millis(self.0)
    }
}

/// Player's recorded attempt time (in milliseconds)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttemptTime(pub u64);

impl AttemptTime {
    pub fn as_millis(&self) -> u64 {
        self.0
    }

    pub fn as_secs_f64(&self) -> f64 {
        self.0 as f64 / 1000.0
    }
}

/// Server-side timestamp for validation
#[derive(Debug, Clone, Copy)]
pub struct ServerTimestamp(pub Instant);

impl ServerTimestamp {
    pub fn now() -> Self {
        Self(Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}
