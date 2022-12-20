use super::{FrameRate, NANOS_PER_HOUR, NANOS_PER_MINUTE, NANOS_PER_MS, NANOS_PER_SECOND};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Sub;
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct Time {
    // TODO: If we always operate at ms resolution, maybe we should just convert
    //       to ms at the earliest possible oppurtunity.
    nanoseconds: u128,
}

impl Time {
    pub fn new(nanoseconds: u128) -> Self {
        Self { nanoseconds }
    }

    pub fn from(duration: &Duration) -> Self {
        Self {
            nanoseconds: duration.as_nanos(),
        }
    }

    pub fn at(hours: u128, minutes: u128, seconds: u128, milliseconds: u128) -> Self {
        let mut nanoseconds = 0;
        nanoseconds += hours * NANOS_PER_HOUR;
        nanoseconds += minutes * NANOS_PER_MINUTE;
        nanoseconds += seconds * NANOS_PER_SECOND;
        nanoseconds += milliseconds * NANOS_PER_MS;

        Self { nanoseconds }
    }

    // NOTE: It should always be safe to unwrap the conversions here, as they
    //       are all modulus of a value samller than u8, so never can exceed
    //       the u8 maximum.
    pub fn frames(&self, frame_rate: FrameRate) -> u8 {
        (self.nanoseconds / frame_rate.nanos_per_frame() % frame_rate.fps())
            .try_into()
            .unwrap()
    }

    pub fn milliseconds(&self) -> u16 {
        ((self.nanoseconds / NANOS_PER_MS) % 1000)
            .try_into()
            .unwrap()
    }

    pub fn seconds(&self) -> u8 {
        ((self.nanoseconds / NANOS_PER_SECOND) % 60)
            .try_into()
            .unwrap()
    }

    pub fn minutes(&self) -> u8 {
        ((self.nanoseconds / NANOS_PER_MINUTE) % 60)
            .try_into()
            .unwrap()
    }

    // TODO: Theoretically, a long running timecode could exceed the maximum of
    //       u64. But either a real TC source can't run this high, or the maximum
    //       of u64 hour exceeds any realistic time frame. Either way, this unwrap
    //       may or may not be safe, but we should look into this.
    pub fn hours(&self) -> u64 {
        (self.nanoseconds / NANOS_PER_HOUR).try_into().unwrap()
    }

    fn total_milliseconds(&self) -> u64 {
        let mut milliseconds = self.milliseconds() as u64;
        milliseconds += self.seconds() as u64 * 1000;
        milliseconds += self.minutes() as u64 * 60 * 1000;
        milliseconds += self.hours() as u64 * 60 * 60 * 1000;

        milliseconds
    }

    pub fn tc_string(&self, frame_rate: FrameRate) -> String {
        format!(
            "{}:{}:{}:{} @{:?}",
            self.hours(),
            self.minutes(),
            self.seconds(),
            self.frames(frame_rate),
            frame_rate
        )
    }
}

impl From<Time> for Duration {
    fn from(time: Time) -> Self {
        let mut seconds = time.seconds() as u64;
        seconds += time.minutes() as u64 * 60;
        seconds += time.hours() as u64 * 60 * 60;

        let nanos = time.milliseconds() as u128 * NANOS_PER_MS;

        Duration::new(seconds, nanos as u32)
    }
}

// NOTE: Our time resoloution is equivelant if they match to the millisecond,
//       not the nanosecond. So we have to implement these traits ourselves.

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hours() == other.hours()
            && self.minutes() == other.minutes()
            && self.seconds() == other.seconds()
            && self.milliseconds() == other.milliseconds()
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_milliseconds().cmp(&other.total_milliseconds())
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{} // {}ns",
            self.hours(),
            self.minutes(),
            self.seconds(),
            self.milliseconds(),
            self.nanoseconds
        )
    }
}

impl Hash for Time {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.total_milliseconds().hash(state)
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Time::new(self.nanoseconds - rhs.nanoseconds)
    }
}

#[cfg(test)]
mod tests {
    use super::Time;

    macro_rules! assert_correct_time {
        ($time:ident, $h:literal $m:literal $s:literal $ms:literal) => {
            assert_eq!($time.hours(), $h);
            assert_eq!($time.minutes(), $m);
            assert_eq!($time.seconds(), $s);
            assert_eq!($time.milliseconds(), $ms);
        };
    }

    macro_rules! time {
        ($h:literal $m:literal $s:literal $ms: literal) => {{
            Time::at($h, $m, $s, $ms)
        }};
    }

    #[test]
    fn half_a_second() {
        let time = time!(0 0 0 500);
        assert_correct_time!(time, 0 0 0 500);
    }

    #[test]
    fn ten_and_a_third_seconds() {
        let time = time!(0 0 10 10);
        assert_correct_time!(time, 0 0 10 10);
    }

    #[test]
    fn three_minutes_eight_seconds_six_milliseconds() {
        let time = time!(0 3 8 6);
        assert_correct_time!(time, 0 3 8 6);
    }

    #[test]
    fn nine_hours_twenty_minutes_eighteen_seconds_twentythree_milliseconds() {
        let time = time!(9 20 18 23);
        assert_correct_time!(time, 9 20 18 23);
    }
}
