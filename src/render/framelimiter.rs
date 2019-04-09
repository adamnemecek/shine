use std::fmt;
use std::thread;
use std::time::{Duration, Instant};

pub enum FrameLimit {
    Sleep(Duration),
    Spin(Duration),
    SleepSpin(Duration),
}

pub struct FrameLimiter {
    epoch: Instant,
    start: Option<Instant>,
    sleep_limit: Duration,
    global_off_time_us: i64,
    max_compensation_us: i64,

    frame_count: u32,
    work_time: Duration,
    sleep_time: Duration,
    spin_time: Duration,
}

impl FrameLimiter {
    pub fn new() -> FrameLimiter {
        FrameLimiter {
            epoch: Instant::now(),
            start: None,
            sleep_limit: Duration::from_millis(2),
            max_compensation_us: 2000,
            global_off_time_us: 0,
            frame_count: 0,
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

    pub fn global_off_time_us(&self) -> i64 {
        self.global_off_time_us
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
        self.frame_count += 1;
    }

    pub fn limit(&mut self, limit: FrameLimit) -> i64 {
        let start = self.start.take().unwrap();
        let elapsed = start.elapsed();
        self.work_time += elapsed;

        match limit {
            FrameLimit::Sleep(limit) => {
                let fixed_limit = self.get_compensation(limit);
                self.do_sleep(start, fixed_limit);
                self.update_compensation(start, limit)
            }
            FrameLimit::Spin(limit) => {
                let fixed_limit = self.get_compensation(limit);
                self.do_spin(start, fixed_limit);
                self.update_compensation(start, limit)
            }
            FrameLimit::SleepSpin(limit) => {
                let fixed_limit = self.get_compensation(limit);
                log::info!("fixed: {:?},{:?}", limit, fixed_limit);
                self.do_sleep(start, fixed_limit);
                self.do_spin(start, fixed_limit);
                self.update_compensation(start, limit)
            }
        }
    }

    fn get_compensation(&self, limit: Duration) -> Duration {
        if self.global_off_time_us > self.max_compensation_us {
            limit - Duration::from_micros(self.max_compensation_us as u64)
        } else if self.global_off_time_us > 0 {
            limit - Duration::from_micros(self.global_off_time_us as u64)
        } else if self.global_off_time_us < -self.max_compensation_us {
            limit + Duration::from_micros(self.max_compensation_us as u64)
        } else if self.global_off_time_us < 0 {
            limit + Duration::from_micros(-self.global_off_time_us as u64)
        } else {
            limit
        }
    }

    fn update_compensation(&mut self, start: Instant, limit: Duration) -> i64 {
        let elapsed_us = self.epoch.elapsed().as_micros() as i64;
        let limit_us = limit.as_micros() as i64 * self.frame_count as i64;
        //let elapsed_us = start.elapsed().as_micros() as i64;
        //let limit_us = limit.as_micros() as i64;
        let drift = elapsed_us - limit_us;
        self.global_off_time_us += drift;
        drift
    }

    fn do_sleep(&mut self, start: Instant, limit: Duration) {
        let elapsed = start.elapsed();
        if limit <= self.sleep_limit + elapsed {
            return;
        }

        //log::info!("sleep: {:?}, {:?}, {:?}", limit, elapsed, self.sleep_limit);
        let wait = limit - elapsed - self.sleep_limit;
        let sleep_start = Instant::now();
        log::info!("sleep: {:?}", wait);
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
            "FrameLimit(Work({:?}), Sleep({:?}), Spin({:?}), OffTime({:?}µs), AvgTime({:?})",
            self.work_time(),
            self.sleep_time(),
            self.spin_time(),
            self.global_off_time_us(),
            (self.work_time() + self.sleep_time() + self.spin_time()) / self.frame_count,
        )
    }
}
