use core::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SysTime(SystemTime);

impl SysTime {
    pub fn new() -> Self {
        Self(SystemTime::now())
    }
    #[inline]
    pub fn since_unix_epoch(&self) -> crate::Result<Duration> {
        self.0
            .duration_since(UNIX_EPOCH)
            .map_err(crate::Error::SysTime)
    }
    pub fn since_unix_epoch_time(&self) -> crate::Result<Time> {
        let ue = self.since_unix_epoch()?;
        Ok(Time::from_duration(ue))
    }
    #[inline]
    pub fn elapsed(&self) -> crate::Result<Duration> {
        self.0.elapsed().map_err(crate::Error::SysTime)
    }
    #[inline]
    pub fn elapsed_time(&self) -> crate::Result<Time> {
        let dur = self.elapsed()?;
        Ok(Time::from_duration(dur))
    }
}
impl Default for SysTime {
    fn default() -> Self {
        Self(UNIX_EPOCH)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
    millis: u8,
}

impl Time {
    pub fn from_duration(dur: Duration) -> Self {
        // Extract hours, minutes, seconds, and milliseconds from the duration
        let dur_second = dur.as_secs();
        Self {
            hours: (dur_second / 3600) as u8,
            minutes: ((dur_second % 3600) / 60) as u8,
            seconds: (dur_second % 60) as u8,
            millis: (dur.subsec_millis() / 10) as u8,
        }
    }
}
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            hours,
            minutes,
            seconds,
            millis,
        } = self;
        write!(f, "{hours:02}:{minutes:02}:{seconds:02}.{millis:02}")
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::Time;
    #[test]
    fn test_time_from_duration() {
        let times = Time::from_duration(Duration::from_secs(3696));
        assert_eq!(
            times,
            Time {
                hours: 1,
                minutes: 1,
                seconds: 36,
                millis: 0
            }
        );
    }
}
