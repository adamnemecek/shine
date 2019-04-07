use std::thread;
use std::time::{Duration, Instant};

pub enum FrameLimit {
    Sleep(Duration),
    Spin(Duration),
    SleepSpin(Duration),
}

pub struct FrameLimiter {
    sleep_limit: Duration,
    start: Option<Instant>,

    work_time: Duration,
    sleep_time: Duration,
    spin_time: Duration,
}

impl FrameLimiter {
    pub fn new() -> FrameLimiter {
        FrameLimiter {
            sleep_limit: Duration::from_millis(2),
            start: None,
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
        self.work_time = Duration::default();
        self.sleep_time = Duration::default();
        self.spin_time = Duration::default();
    }

    pub fn limit(&mut self, limit: FrameLimit) -> i128 {
        let start = self.start.take().unwrap();
        let elapsed = start.elapsed();
        self.work_time += elapsed;

        match limit {
            FrameLimit::Sleep(limit) => {
                self.do_sleep(start, limit);
                start.elapsed().as_micros() as i128 - limit.as_micros() as i128
            }
            FrameLimit::Spin(limit) => {
                self.do_spin(start, limit);
                start.elapsed().as_micros() as i128 - limit.as_micros() as i128
            }
            FrameLimit::SleepSpin(limit) => {
                self.do_sleep(start, limit);
                self.do_spin(start, limit);
                start.elapsed().as_micros() as i128 - limit.as_micros() as i128
            }
        }
        
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
