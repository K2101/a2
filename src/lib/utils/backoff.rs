use crate::domain::SEVEN_DAYS;
use exponential_backoff::Backoff;
use std::time::Duration;

#[derive(Debug)]
pub struct BackoffSetting {
    backoff: Backoff,
}

impl BackoffSetting {
    pub fn new(retries: u32, min_mill: u64, max_sec: u64) -> Self {
        let min = Duration::from_millis(min_mill);
        let max = Duration::from_secs(max_sec);
        Self {
            backoff: Backoff::new(retries, min, max),
        }
    }
    pub fn get_backoff(self) -> Backoff {
        self.backoff
    }
}

impl Default for BackoffSetting {
    // retry for seven day
    fn default() -> Self {
        let backoff = BackoffSetting::new(3000, 100, SEVEN_DAYS as u64).get_backoff();
        Self { backoff }
    }
}
