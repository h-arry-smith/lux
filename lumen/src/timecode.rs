use std::time::Duration;

const NANOS_PER_SECOND: u128 = 1_000_000_000;

pub trait FrameRate {
    fn frame_number_at_time(time: u128) -> u8 {
        ((time / Self::nanos_per_frame()) % Self::fps()) as u8
    }

    fn nanos_per_frame() -> u128 {
        NANOS_PER_SECOND / Self::fps()
    }

    fn fps() -> u128;

    fn state() -> Self;
}

// NOTE: Though SMPTE allows for 29.97fps (drop frame) for now keeping things
//       simple by just copying what GrandMA do.
// TODO: Move to seperate file?
pub mod fps {
    use super::FrameRate;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TwentyFour {}
    impl FrameRate for TwentyFour {
        fn fps() -> u128 {
            24
        }

        fn state() -> Self {
            Self {}
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct TwentyFive {}
    impl FrameRate for TwentyFive {
        fn fps() -> u128 {
            25
        }

        fn state() -> Self {
            Self {}
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Thirty {}
    impl FrameRate for Thirty {
        fn fps() -> u128 {
            30
        }

        fn state() -> Self {
            Self {}
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Time<R: FrameRate> {
    hours: u8,
    minutes: u8,
    seconds: u8,
    frames: u8,
    frame_rate: R,
}

impl<R: FrameRate> Time<R> {
    fn new(hours: u8, minutes: u8, seconds: u8, frames: u8) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            frames,
            frame_rate: R::state(),
        }
    }
}

impl<R: FrameRate> From<Duration> for Time<R> {
    fn from(duration: Duration) -> Self {
        let nano = duration.as_nanos();
        Self {
            hours: 0,
            minutes: 0,
            seconds: (nano / NANOS_PER_SECOND).try_into().unwrap(),
            frames: R::frame_number_at_time(nano),
            frame_rate: R::state(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod frame_rate {
        use crate::timecode::{
            fps::{Thirty, TwentyFour},
            FrameRate, NANOS_PER_SECOND,
        };

        #[test]
        fn frame_number_24() {
            let nanos = NANOS_PER_SECOND / 2;

            assert_eq!(TwentyFour::frame_number_at_time(nanos), 12);
        }

        #[test]
        fn frame_number_30() {
            let nanos = NANOS_PER_SECOND / 4;

            assert_eq!(Thirty::frame_number_at_time(nanos), 7);
        }
    }

    mod from_duration {
        use std::time::Duration;

        use crate::timecode::{
            fps::{Thirty, TwentyFour},
            Time, NANOS_PER_SECOND,
        };

        #[test]
        fn half_a_second() {
            let time: Time<TwentyFour> =
                Time::from(Duration::new(0, (NANOS_PER_SECOND / 2).try_into().unwrap()));

            assert_eq!(time, Time::<TwentyFour>::new(0, 0, 0, 12))
        }

        #[test]
        fn ten_and_a_third_seconds() {
            let time: Time<Thirty> = Time::from(Duration::new(
                10,
                (NANOS_PER_SECOND / 3).try_into().unwrap(),
            ));

            assert_eq!(time, Time::<Thirty>::new(0, 0, 10, 10))
        }
    }
}
