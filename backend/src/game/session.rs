use crate::error::GameError;
use crate::game::{AttemptTime, GameConfig, GameId, GameState, PlayerId, TargetTime};
use crate::validation::TimingValidator;

/// Represents a single game session for one player
#[derive(Debug)]
pub struct GameSession {
    pub id: GameId,
    pub player_id: PlayerId,
    state: GameState,
    config: GameConfig,
    timing_validator: TimingValidator,
}

/// Result of stopping the game
#[derive(Debug, Clone)]
pub struct GameResult {
    pub target_time: TargetTime,
    pub attempt_time: AttemptTime,
    pub difference_ms: i64,
    pub difference_sec: f64,
    pub timing_valid: bool,
}

impl GameSession {
    pub fn new(id: GameId, player_id: PlayerId, config: GameConfig) -> Self {
        let timing_validator = TimingValidator::new(
            config.timing_tolerance,
            config.max_network_latency,
        );
        Self {
            id,
            player_id,
            state: GameState::new(),
            config,
            timing_validator,
        }
    }

    /// Generate a random target time within configured bounds
    fn generate_target_time(&self) -> TargetTime {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let min_ms = self.config.min_target_time.as_millis() as u64;
        let max_ms = self.config.max_target_time.as_millis() as u64;
        TargetTime(rng.gen_range(min_ms..=max_ms))
    }

    /// Start the game with a randomly generated target time
    pub fn start(&mut self) -> Result<TargetTime, GameError> {
        let target = self.generate_target_time();
        self.state = std::mem::take(&mut self.state).start(target)?;
        Ok(target)
    }

    /// Stop the game and calculate results
    pub fn stop(&mut self, client_time: AttemptTime) -> Result<GameResult, GameError> {
        // Get server elapsed time and target before state transition
        let (target_time, server_elapsed) = match &self.state {
            GameState::Running(running) => {
                (running.target_time, running.server_start_time.elapsed())
            }
            _ => {
                return Err(GameError::InvalidStateTransition {
                    from: self.state.name(),
                    action: "stop",
                })
            }
        };

        // Validate client time against server time
        let timing_valid = self.timing_validator.validate(client_time, server_elapsed);

        // Use client time if valid, server time if not
        let effective_time = if timing_valid {
            client_time
        } else {
            AttemptTime(server_elapsed.as_millis() as u64)
        };

        // Calculate difference
        let difference_ms = effective_time.as_millis() as i64 - target_time.as_millis() as i64;
        let difference_sec = difference_ms as f64 / 1000.0;

        // Transition state
        self.state = std::mem::take(&mut self.state).stop(effective_time)?;

        Ok(GameResult {
            target_time,
            attempt_time: effective_time,
            difference_ms,
            difference_sec,
            timing_valid,
        })
    }

    /// Reset for another round
    pub fn reset(&mut self) -> Result<(), GameError> {
        self.state = std::mem::take(&mut self.state).reset()?;
        Ok(())
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }
}
