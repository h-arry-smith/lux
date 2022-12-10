use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use self::time::Time;
pub mod time;

const NANOS_PER_SECOND: u128 = 1_000_000_000;
const NANOS_PER_MINUTE: u128 = NANOS_PER_SECOND * 60;
const NANOS_PER_HOUR: u128 = NANOS_PER_MINUTE * 60;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum FrameRate {
    TwentyFour,
    TwentFive,
    Thirty,
}

impl Debug for FrameRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}fps", self.fps())
    }
}

impl FrameRate {
    fn nanos_per_frame(&self) -> u128 {
        NANOS_PER_SECOND / self.fps()
    }

    fn fps(&self) -> u128 {
        match self {
            FrameRate::TwentyFour => 24,
            FrameRate::TwentFive => 25,
            FrameRate::Thirty => 30,
        }
    }
}

#[derive(Debug)]
pub struct Source {
    start_time: Option<Instant>,
    pause_time: Option<Instant>,
    frame_rate: FrameRate,
}

impl Source {
    pub fn new(frame_rate: FrameRate) -> Self {
        Self {
            start_time: None,
            pause_time: None,
            frame_rate,
        }
    }

    pub fn time(&self) -> Time {
        if let Some(pause_time) = self.pause_time {
            if let Some(start_time) = self.start_time {
                if pause_time == start_time {
                    return Time::from(&start_time.elapsed(), self.frame_rate);
                } else {
                    let duration_paused_at = start_time.elapsed() - pause_time.elapsed();
                    return Time::from(&duration_paused_at, self.frame_rate);
                }
            }
        }

        match self.start_time {
            Some(start_time) => Time::from(&start_time.elapsed(), self.frame_rate),
            None => Time::new(0, self.frame_rate),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn start_at_time(&mut self, time: Instant) {
        self.start_time = Some(time);
    }

    pub fn pause(&mut self) {
        if self.start_time.is_some() {
            self.pause_time = Some(Instant::now())
        }
    }

    // NOTE: This way of resolving paused time destroys the original start time
    //       of a timecode. If in the future you need this, then you'd need to
    //       modify the struct to keep an absolute start, and a relative start.
    pub fn resume(&mut self) {
        if self.start_time.is_none() {
            return;
        }

        if let Some(pause_time) = self.pause_time {
            // Unwrap is safe because of gaurd condition.
            let mut start_time = self.start_time.take().unwrap();
            start_time += pause_time.elapsed();

            self.start_time = Some(start_time);
            self.pause_time = None;
        }
    }

    pub fn stop(&mut self) {
        self.start_time = None;
        self.pause_time = None;
    }

    pub fn seek(&mut self, time: Time) {
        match self.pause_time {
            Some(_) => {
                // When paused, must update the pause point
                let duration: Duration = time.into();
                let new_time = Instant::now() - duration;
                self.start_time = Some(new_time);

                // NOTE: This small addition of time solves a rounding issue in
                //       the seeking behaviour, but feels a bit kludgy.
                self.pause_time = Some(Instant::now() + Duration::new(0, 500));
            }
            None => {
                // We aren't paused, so we can just seek to the new position
                let duration: Duration = time.into();
                self.start_time = Some(Instant::now() - duration);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::timecode::Time;
    use std::{
        thread,
        time::{Duration, Instant},
    };

    use super::{FrameRate, Source};

    // TODO: These macros are shared between files, must be a way to make it common.
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
    fn new_source_starts_at_zero() {
        let source = Source::new(FrameRate::Thirty);
        let time = source.time();
        assert_correct_time!(time, 0 0 0 0);
    }

    #[test]
    fn returns_elapsed_time_once_started() {
        let mut source = Source::new(FrameRate::Thirty);
        let start = Instant::now() - Duration::new(3, 0);

        source.start_at_time(start);

        let time = source.time();
        assert_correct_time!(time, 0 0 3 0);
    }

    #[test]
    fn pausing_a_source() {
        let mut source = Source::new(FrameRate::Thirty);
        source.start();
        let time_before_pause = source.time();
        source.pause();
        thread::sleep(Duration::new(0, 100_000_000));
        let time_during_pause = source.time();
        source.resume();
        thread::sleep(Duration::new(0, 100_000_000));
        let time_after_pause = source.time();

        assert_eq!(time_before_pause, time_during_pause);
        assert_ne!(time_during_pause, time_after_pause);
    }

    #[test]
    fn pausing_multiple_times() {
        let mut source = Source::new(FrameRate::Thirty);
        source.start();
        // + 3 frames
        thread::sleep(Duration::new(0, 100_000_000));
        source.pause();
        // + 0 frames (paused)
        thread::sleep(Duration::new(0, 100_000_000));
        source.resume();
        // + 3 frames
        thread::sleep(Duration::new(0, 100_000_000));
        source.pause();
        // + 0 frames (paused)
        thread::sleep(Duration::new(0, 100_000_000));

        let time = source.time();
        assert_correct_time!(time, 0 0 0 6);
    }

    #[test]
    fn stop() {
        let mut source = Source::new(FrameRate::Thirty);
        source.start();
        thread::sleep(Duration::new(0, 50_000_000));
        source.stop();

        let time = source.time();
        assert_correct_time!(time, 0 0 0 0);
    }

    #[test]
    fn seek_while_running() {
        let mut source = Source::new(FrameRate::Thirty);
        source.start();
        thread::sleep(Duration::new(0, 50_000_000));
        let seek_time = time!(0 0 3 0, Thirty);
        source.seek(seek_time);
        let time = source.time();
        assert_correct_time!(time, 0 0 3 0);
    }

    #[test]
    fn seek_while_paused() {
        let mut source = Source::new(FrameRate::Thirty);
        source.start();
        source.pause();
        let seek_time = time!(1 2 8 4, Thirty);

        source.seek(seek_time);
        // + 0 frames (paused)
        thread::sleep(Duration::new(0, 100_000_000));
        let time = source.time();
        dbg!(&time);
        dbg!(&source);
        assert_correct_time!(time, 1 2 8 4);
    }
}
