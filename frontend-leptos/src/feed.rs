//! Freshness tracking for the polled API feeds (telemetry, network, Spotify).
//!
//! Each poll records a success or a failure. A feed only counts as stale or
//! unavailable after `STALE_AFTER_FAILURES` consecutive failures, so a single
//! slow request (for example a Cloud Run cold start) does not flicker the UI.

/// Consecutive failed polls before a feed is shown as stale or unavailable.
pub const STALE_AFTER_FAILURES: u32 = 2;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FeedHealth {
    /// `Date.now()` of the last successful poll, in milliseconds.
    pub last_success_ms: Option<f64>,
    pub consecutive_failures: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FeedState {
    /// No data yet and not enough failures to give up.
    Connecting,
    /// The latest data is current.
    Live,
    /// Data exists, but recent polls failed; `age_secs` since the last success.
    Stale { age_secs: u64 },
    /// No poll has ever succeeded.
    Unavailable,
}

impl FeedHealth {
    pub fn record_success(&mut self, now_ms: f64) {
        self.last_success_ms = Some(now_ms);
        self.consecutive_failures = 0;
    }

    pub fn record_failure(&mut self) {
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
    }

    pub fn state(&self, now_ms: f64) -> FeedState {
        let failing = self.consecutive_failures >= STALE_AFTER_FAILURES;
        match (self.last_success_ms, failing) {
            (None, false) => FeedState::Connecting,
            (None, true) => FeedState::Unavailable,
            (Some(_), false) => FeedState::Live,
            (Some(last), true) => FeedState::Stale {
                age_secs: ((now_ms - last).max(0.0) / 1000.0) as u64,
            },
        }
    }

    pub fn is_stale(&self) -> bool {
        self.last_success_ms.is_some() && self.consecutive_failures >= STALE_AFTER_FAILURES
    }
}

/// "45s ago", "3m ago", "2h ago".
pub fn format_age(secs: u64) -> String {
    if secs < 60 {
        format!("{}s ago", secs)
    } else if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else {
        format!("{}h ago", secs / 3600)
    }
}

/// Current time in milliseconds (browser clock; 0 outside the browser).
pub fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_connecting() {
        assert_eq!(FeedHealth::default().state(0.0), FeedState::Connecting);
    }

    #[test]
    fn single_failure_before_first_success_is_still_connecting() {
        let mut h = FeedHealth::default();
        h.record_failure();
        assert_eq!(h.state(0.0), FeedState::Connecting);
        h.record_failure();
        assert_eq!(h.state(0.0), FeedState::Unavailable);
    }

    #[test]
    fn success_is_live_and_one_miss_stays_live() {
        let mut h = FeedHealth::default();
        h.record_success(1_000.0);
        assert_eq!(h.state(1_000.0), FeedState::Live);
        h.record_failure();
        assert_eq!(h.state(6_000.0), FeedState::Live);
        assert!(!h.is_stale());
    }

    #[test]
    fn repeated_failures_after_success_are_stale_with_age() {
        let mut h = FeedHealth::default();
        h.record_success(10_000.0);
        h.record_failure();
        h.record_failure();
        assert!(h.is_stale());
        assert_eq!(h.state(40_500.0), FeedState::Stale { age_secs: 30 });
    }

    #[test]
    fn success_after_stale_recovers() {
        let mut h = FeedHealth::default();
        h.record_success(0.0);
        h.record_failure();
        h.record_failure();
        h.record_success(20_000.0);
        assert_eq!(h.state(20_000.0), FeedState::Live);
        assert_eq!(h.consecutive_failures, 0);
    }

    #[test]
    fn clock_skew_never_gives_negative_age() {
        let mut h = FeedHealth::default();
        h.record_success(50_000.0);
        h.record_failure();
        h.record_failure();
        assert_eq!(h.state(40_000.0), FeedState::Stale { age_secs: 0 });
    }

    #[test]
    fn age_formatting() {
        assert_eq!(format_age(0), "0s ago");
        assert_eq!(format_age(59), "59s ago");
        assert_eq!(format_age(60), "1m ago");
        assert_eq!(format_age(3599), "59m ago");
        assert_eq!(format_age(7200), "2h ago");
    }
}
