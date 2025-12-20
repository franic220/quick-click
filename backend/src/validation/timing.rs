use std::time::Duration;

use crate::game::AttemptTime;

/// Validates client-reported times against server measurements
#[derive(Debug, Clone)]
pub struct TimingValidator {
    /// Acceptable difference between client and server times
    tolerance: Duration,
    /// Maximum network latency to account for
    max_latency: Duration,
}

impl TimingValidator {
    pub fn new(tolerance: Duration, max_latency: Duration) -> Self {
        Self {
            tolerance,
            max_latency,
        }
    }

    /// Validate that client time is within acceptable bounds of server time.
    /// Returns true if timing is considered valid.
    pub fn validate(&self, client_time: AttemptTime, server_elapsed: Duration) -> bool {
        let client_ms = client_time.as_millis();
        let server_ms = server_elapsed.as_millis() as u64;

        // Total allowed difference = tolerance + max_latency
        let allowed_diff =
            self.tolerance.as_millis() as u64 + self.max_latency.as_millis() as u64;

        let diff = client_ms.abs_diff(server_ms);

        diff <= allowed_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_timing_within_tolerance() {
        let validator = TimingValidator::new(
            Duration::from_millis(100),
            Duration::from_millis(50),
        );

        // Client says 5000ms, server measured 5050ms - should be valid
        assert!(validator.validate(
            AttemptTime(5000),
            Duration::from_millis(5050)
        ));
    }

    #[test]
    fn test_valid_timing_at_boundary() {
        let validator = TimingValidator::new(
            Duration::from_millis(100),
            Duration::from_millis(50),
        );

        // Exactly at the 150ms boundary
        assert!(validator.validate(
            AttemptTime(5000),
            Duration::from_millis(5150)
        ));
    }

    #[test]
    fn test_invalid_timing_exceeds_tolerance() {
        let validator = TimingValidator::new(
            Duration::from_millis(100),
            Duration::from_millis(50),
        );

        // Client says 5000ms, server measured 5200ms - exceeds 150ms tolerance
        assert!(!validator.validate(
            AttemptTime(5000),
            Duration::from_millis(5200)
        ));
    }
}
