use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use self::time::Time;
pub mod time;

const NANOS_PER_MS: u128 = 1_000_000;
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
    pub fn nanos_per_frame(&self) -> u128 {
        NANOS_PER_SECOND / self.fps()
    }

    pub fn fps(&self) -> u128 {
        match self {
            FrameRate::TwentyFour => 24,
            FrameRate::TwentFive => 25,
            FrameRate::Thirty => 30,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Copy, Clone)]
pub struct Source {
    start_time: Option<Instant>,
    pause_time: Option<Instant>,
    seek_time: Option<Time>,
    frame_rate: FrameRate,
    state: State,
}

impl Source {
    pub fn new(frame_rate: FrameRate) -> Self {
        Self {
            start_time: None,
            pause_time: None,
            seek_time: None,
            frame_rate,
            state: State::Stopped,
        }
    }

    // TODO: Milliseconds should be clamped to the frame boundaries. A real time
    //       code source will convert frame number to milliseconds.
    pub fn time(&self) -> Time {
        match self.state {
            State::Stopped => Time::new(0),
            State::Running => Time::from(&self.duration_from_seek()),
            State::Paused => {
                let time = self
                    .duration_from_seek()
                    .checked_sub(self.pause_time.unwrap().elapsed());
                match time {
                    Some(t) => Time::from(&t),
                    None => Time::at(0, 0, 0, 0),
                }
            }
        }
    }

    fn duration_from_seek(&self) -> Duration {
        if let Some(seek_time) = self.seek_time {
            let seek_duration: Duration = seek_time.into();

            seek_duration + self.start_time.unwrap().elapsed()
        } else {
            self.start_time.unwrap().elapsed()
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.state = State::Running;
    }

    pub fn start_at_time(&mut self, time: Instant) {
        self.start_time = Some(time);
        self.state = State::Running;
    }

    pub fn pause(&mut self) {
        self.pause_time = Some(Instant::now());
        self.state = State::Paused;
    }

    // NOTE: This way of resolving paused time destroys the original start time
    //       of a timecode. If in the future you need this, then you'd need to
    //       modify the struct to keep an absolute start, and a relative start.
    pub fn resume(&mut self) {
        if !(self.state == State::Paused) {
            return;
        }

        if let Some(pause_time) = self.pause_time {
            // Unwrap is safe because of gaurd condition.
            let mut start_time = self.start_time.take().unwrap();
            start_time += pause_time.elapsed();

            self.start_time = Some(start_time);
            self.pause_time = None;
            self.state = State::Running;
        }
    }

    pub fn stop(&mut self) {
        self.start_time = None;
        self.pause_time = None;
        self.state = State::Stopped;
    }

    pub fn seek(&mut self, time: Time) {
        self.seek_time = Some(time);

        match self.state {
            State::Paused => {
                let now = Instant::now();
                self.start_time = Some(now);
                self.pause_time = Some(now);
            }
            _ => {
                self.start_time = Some(Instant::now());
            }
        }
    }

    pub fn fps(&self) -> FrameRate {
        self.frame_rate
    }

    pub fn paused(&self) -> bool {
        self.state == State::Paused
    }
}

#[macro_export]
macro_rules! time {
    ($h:literal $m:literal $s:literal $f:literal $rate:ident) => {{
        let mut seconds = 0;
        seconds += $h * 60 * 60;
        seconds += $m * 60;
        seconds += $s;

        let nanos = $f * $crate::timecode::FrameRate::$rate.nanos_per_frame();

        Time::from(&Duration::new(seconds, nanos as u32))
    }};

    ($h:literal $m:literal $s:literal $ms:literal) => {{
        let mut seconds = 0;
        seconds += $h * 60 * 60;
        seconds += $m * 60;
        seconds += $s;

        let nanos = $ms * NANOS_PER_MS;

        Time::from(&Duration::new(seconds, nanos as u32))
    }};
}

#[cfg(test)]
mod tests {
    use crate::timecode::Time;
    use crate::timecode::NANOS_PER_MS;
    use std::{
        thread,
        time::{Duration, Instant},
    };

    use super::{FrameRate, Source};

    // TODO: These macros are shared between files, must be a way to make it common.
    macro_rules! assert_correct_time {
        ($time:ident, $h:literal $m:literal $s:literal $f:literal $rate:ident) => {
            assert_eq!($time.hours(), $h);
            assert_eq!($time.minutes(), $m);
            assert_eq!($time.seconds(), $s);
            assert_eq!($time.frames(FrameRate::$rate), $f);
        };
        ($time:ident, $h:literal $m:literal $s:literal $ms:literal) => {
            assert_eq!($time.hours(), $h);
            assert_eq!($time.minutes(), $m);
            assert_eq!($time.seconds(), $s);
            assert_eq!($time.milliseconds(), $ms);
        };
    }

    #[test]
    fn new_source_starts_at_zero() {
        let source = Source::new(FrameRate::Thirty);
        let time = source.time();
        assert_correct_time!(time, 0 0 0 0 Thirty);
    }

    #[test]
    fn returns_elapsed_time_once_started() {
        let mut source = Source::new(FrameRate::Thirty);
        let start = Instant::now() - Duration::new(3, 0);

        source.start_at_time(start);

        let time = source.time();
        assert_correct_time!(time, 0 0 3 0 Thirty);
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
        assert_correct_time!(time, 0 0 0 6 Thirty);
    }

    #[test]
    fn stop() {
        let mut source = Source::new(FrameRate::Thirty);
        source.start();
        thread::sleep(Duration::new(0, 50_000_000));
        source.stop();

        let time = source.time();
        assert_correct_time!(time, 0 0 0 0 Thirty);
    }

    #[test]
    fn seek_while_running() {
        let mut source = Source::new(FrameRate::Thirty);
        source.start();
        thread::sleep(Duration::new(0, 50_000_000));
        let seek_time = time!(0 0 3 0);
        source.seek(seek_time);
        let time = source.time();
        assert_correct_time!(time, 0 0 3 0 Thirty);
    }

    #[test]
    fn seek_while_paused() {
        let mut source = Source::new(FrameRate::Thirty);
        source.start();
        source.pause();
        let seek_time = time!(1 2 8 500);
        dbg!(&seek_time);
        dbg!(&seek_time.frames(FrameRate::Thirty));

        source.seek(seek_time);
        // + 0 frames (paused)
        thread::sleep(Duration::new(0, 100_000_000));
        let time = source.time();
        dbg!(&source);
        dbg!(&time);
        dbg!(&time.frames(FrameRate::Thirty));
        assert_correct_time!(time, 1 2 8 499);
    }
}
