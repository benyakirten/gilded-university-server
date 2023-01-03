use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::errors::TimeError;

pub struct Time {}

impl Time {
    pub fn now() -> Result<Duration, TimeError> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TimeError::NowError)
    }

    pub fn now_plus_duration(duration: Duration) -> Result<Duration, TimeError> {
        let now = Time::now()?;
        let time = now.checked_add(duration);
        match time {
            Some(dur) => Ok(dur),
            None => Err(TimeError::CalculationError(duration.as_secs())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use super::Time;
    #[test]
    fn get_now_time() {
        let want = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let got = Time::now().unwrap();
        assert_eq!(want.as_secs(), got.as_secs());
    }

    #[test]
    fn now_plus_duration_is_error_if_too_much_time() {
        let got = Time::now_plus_duration(Duration::from_secs(u64::MAX));
        assert!(got.is_err());
    }

    #[test]
    fn now_plus_duration_is_ok_if_time_reasonable() {
        let want = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60;
        let got = Time::now_plus_duration(Duration::from_secs(60))
            .unwrap()
            .as_secs();
        assert_eq!(got, want);
    }
}
