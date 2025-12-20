use std::time::Duration;

use crate::error::GameError;
use crate::game::{AttemptTime, ServerTimestamp, TargetTime};

/// Represents all possible game states
#[derive(Debug, Clone)]
pub enum GameState {
    /// Waiting for player to start
    Idle,
    /// Game is running, timer active
    Running(RunningState),
    /// Game completed, showing results
    Completed(CompletedState),
}

/// State when game timer is running
#[derive(Debug, Clone)]
pub struct RunningState {
    pub target_time: TargetTime,
    pub server_start_time: ServerTimestamp,
}

/// State when game is completed
#[derive(Debug, Clone)]
pub struct CompletedState {
    pub target_time: TargetTime,
    pub attempt_time: AttemptTime,
    pub server_elapsed: Duration,
}

impl GameState {
    pub fn new() -> Self {
        GameState::Idle
    }

    /// Check if game can be started from this state
    pub fn can_start(&self) -> bool {
        matches!(self, GameState::Idle)
    }

    /// Check if game can be stopped from this state
    pub fn can_stop(&self) -> bool {
        matches!(self, GameState::Running(_))
    }

    /// Check if game can be reset from this state
    pub fn can_reset(&self) -> bool {
        matches!(self, GameState::Completed(_))
    }

    /// Transition: Idle -> Running
    pub fn start(self, target_time: TargetTime) -> Result<Self, GameError> {
        match self {
            GameState::Idle => Ok(GameState::Running(RunningState {
                target_time,
                server_start_time: ServerTimestamp::now(),
            })),
            _ => Err(GameError::InvalidStateTransition {
                from: self.name(),
                action: "start",
            }),
        }
    }

    /// Transition: Running -> Completed
    pub fn stop(self, attempt_time: AttemptTime) -> Result<Self, GameError> {
        match self {
            GameState::Running(running) => {
                let server_elapsed = running.server_start_time.elapsed();
                Ok(GameState::Completed(CompletedState {
                    target_time: running.target_time,
                    attempt_time,
                    server_elapsed,
                }))
            }
            _ => Err(GameError::InvalidStateTransition {
                from: self.name(),
                action: "stop",
            }),
        }
    }

    /// Transition: Completed -> Idle
    pub fn reset(self) -> Result<Self, GameError> {
        match self {
            GameState::Completed(_) => Ok(GameState::Idle),
            _ => Err(GameError::InvalidStateTransition {
                from: self.name(),
                action: "reset",
            }),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            GameState::Idle => "Idle",
            GameState::Running(_) => "Running",
            GameState::Completed(_) => "Completed",
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
