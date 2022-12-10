use super::{FrameRate, NANOS_PER_HOUR, NANOS_PER_MINUTE, NANOS_PER_SECOND};
use std::fmt::Debug;
use std::time::Duration;

#[derive(PartialOrd, Ord)]
pub struct Time {
    nanoseconds: u128,
    frame_rate: FrameRate,
}

impl Time {
    pub fn new(nanoseconds: u128, frame_rate: FrameRate) -> Self {
        Self {
            nanoseconds,
            frame_rate,
        }
    }

    pub fn from(duration: &Duration, frame_rate: FrameRate) -> Self {
        Self {
            nanoseconds: duration.as_nanos(),
            frame_rate,
        }
    }

    // NOTE: It should always be safe to unwarp the conversions here, as they
    //       are all modulus of a value samller than u8, so never can exceed
    //       the u8 maximum.
    pub fn frames(&self) -> u8 {
        (self.nanoseconds / self.frame_rate.nanos_per_frame() % self.frame_rate.fps())
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
}

impl From<Time> for Duration {
    fn from(time: Time) -> Self {
        let mut seconds = time.seconds() as u64;
        seconds += time.minutes() as u64 * 60;
        seconds += time.hours() as u64 * 60 * 60;

        let nanos = time.frames() as u128 * time.frame_rate.nanos_per_frame();

        Duration::new(seconds, nanos as u32)
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hours() == other.hours()
            && self.minutes() == other.minutes()
            && self.seconds() == other.seconds()
            && self.frames() == other.frames()
    }
}
impl Eq for Time {}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{} @{:?}",
            self.hours(),
            self.minutes(),
            self.seconds(),
            self.frames(),
            self.frame_rate
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::timecode::time::Time;
    use crate::timecode::FrameRate;
    use std::time::Duration;

    macro_rules! assert_correct_time {
        ($time:ident, $h:literal $m:literal $s:literal $f:literal) => {
            assert_eq!($time.hours(), $h);
            assert_eq!($time.minutes(), $m);
            assert_eq!($time.seconds(), $s);
            assert_eq!($time.frames(), $f);
        };
    }

    macro_rules! time {
        ($h:literal $m:literal $s:literal $f: literal, $rate:ident) => {{
            let mut seconds = 0;
            seconds += $h * 60 * 60;
            seconds += $m * 60;
            seconds += $s;

            let nanos = FrameRate::$rate.nanos_per_frame() * $f;

            Time::from(&Duration::new(seconds, nanos as u32), FrameRate::$rate)
        }};
    }

    #[test]
    fn half_a_second() {
        let time = time!(0 0 0 15, Thirty);
        assert_correct_time!(time, 0 0 0 15);
    }

    #[test]
    fn ten_and_a_third_seconds() {
        let time = time!(0 0 10 10, Thirty);
        assert_correct_time!(time, 0 0 10 10);
    }

    #[test]
    fn three_minutes_eight_seconds_six_frames() {
        let time = time!(0 3 8 6, Thirty);
        assert_correct_time!(time, 0 3 8 6);
    }

    #[test]
    fn nine_hours_twenty_minutes_eighteen_seconds_twentythree_frames() {
        let time = time!(9 20 18 23, TwentyFour);
        assert_correct_time!(time, 9 20 18 23);
    }
}
