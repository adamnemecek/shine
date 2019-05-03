use std::fmt;
use std::thread;
use std::time::{Duration, Instant};

/// Limit policy used by the FrameLimiter
pub enum FrameLimit {
    Sleep(Duration),
    Spin(Duration),
    SleepSpin(Duration),
}

/// Limit frame rate by sleeping and spinning.
pub struct FrameLimiter {
    start: Option<Instant>,
    sleep_limit: Duration,
    work_time: Duration,
    sleep_time: Duration,
    spin_time: Duration,
}

impl FrameLimiter {
    pub fn new() -> FrameLimiter {
        FrameLimiter {
            start: None,
            sleep_limit: Duration::from_millis(2),
            work_time: Duration::default(),
            sleep_time: Duration::default(),
            spin_time: Duration::default(),
        }
    }

    pub fn work_time(&self) -> Duration {
        self.work_time
    }

    pub fn sleep_time(&self) -> Duration {
        self.sleep_time
    }

    pub fn spin_time(&self) -> Duration {
        self.spin_time
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn limit(&mut self, limit: FrameLimit) -> i64 {
        let start = self.start.take().unwrap();
        let elapsed = start.elapsed();
        self.work_time += elapsed;

        match limit {
            FrameLimit::Sleep(limit) => {
                self.do_sleep(start, limit);
                Self::get_off_time(start, limit)
            }
            FrameLimit::Spin(limit) => {
                self.do_spin(start, limit);
                Self::get_off_time(start, limit)
            }
            FrameLimit::SleepSpin(limit) => {
                self.do_sleep(start, limit);
                self.do_spin(start, limit);
                Self::get_off_time(start, limit)
            }
        }
    }

    fn get_off_time(start: Instant, limit: Duration) -> i64 {
        let elapsed_us = start.elapsed().as_micros() as i64;
        let limit_us = limit.as_micros() as i64;
        elapsed_us - limit_us
    }

    fn do_sleep(&mut self, start: Instant, limit: Duration) {
        let elapsed = start.elapsed();
        if limit <= self.sleep_limit + elapsed {
            return;
        }

        let wait = limit - elapsed - self.sleep_limit;
        let sleep_start = Instant::now();
        thread::sleep(wait);
        self.sleep_time += sleep_start.elapsed();
    }

    fn do_spin(&mut self, start: Instant, limit: Duration) {
        let spin_start = Instant::now();
        loop {
            let elapsed = start.elapsed();
            if limit <= elapsed {
                break;
            }
            thread::yield_now();
        }
        self.spin_time += spin_start.elapsed();
    }
}

impl fmt::Debug for FrameLimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FrameLimit(Work({:?}), Sleep({:?}), Spin({:?}))",
            self.work_time(),
            self.sleep_time(),
            self.spin_time(),
        )
    }
}
