use std::time::Duration;

/// Configuration for game sessions
#[derive(Debug, Clone)]
pub struct GameConfig {
    /// Minimum target time
    pub min_target_time: Duration,
    /// Maximum target time
    pub max_target_time: Duration,
    /// Tolerance for client/server time difference (hybrid validation)
    pub timing_tolerance: Duration,
    /// Maximum allowed network latency for validation
    pub max_network_latency: Duration,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            min_target_time: Duration::from_secs(1),
            max_target_time: Duration::from_secs(10),
            timing_tolerance: Duration::from_millis(100),
            max_network_latency: Duration::from_millis(500),
        }
    }
}

impl GameConfig {
    pub fn builder() -> GameConfigBuilder {
        GameConfigBuilder::default()
    }
}

/// Builder for GameConfig
#[derive(Default)]
pub struct GameConfigBuilder {
    min_target_time: Option<Duration>,
    max_target_time: Option<Duration>,
    timing_tolerance: Option<Duration>,
    max_network_latency: Option<Duration>,
}

impl GameConfigBuilder {
    pub fn min_target_time(mut self, duration: Duration) -> Self {
        self.min_target_time = Some(duration);
        self
    }

    pub fn max_target_time(mut self, duration: Duration) -> Self {
        self.max_target_time = Some(duration);
        self
    }

    pub fn timing_tolerance(mut self, duration: Duration) -> Self {
        self.timing_tolerance = Some(duration);
        self
    }

    pub fn max_network_latency(mut self, duration: Duration) -> Self {
        self.max_network_latency = Some(duration);
        self
    }

    pub fn build(self) -> GameConfig {
        let default = GameConfig::default();
        GameConfig {
            min_target_time: self.min_target_time.unwrap_or(default.min_target_time),
            max_target_time: self.max_target_time.unwrap_or(default.max_target_time),
            timing_tolerance: self.timing_tolerance.unwrap_or(default.timing_tolerance),
            max_network_latency: self.max_network_latency.unwrap_or(default.max_network_latency),
        }
    }
}
