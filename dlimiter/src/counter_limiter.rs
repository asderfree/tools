use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::RwLock;
use crate::base_limiter::DLimiter; // Add this line to import the missing trait.

pub struct CounterLimiter{
    duration: usize,  // time window size in ms.
    size : usize,
    limit: RwLock<usize>,
    last_reset_time: RwLock<u64>, // ms
}

impl CounterLimiter {
    pub fn new(duration: usize, size: usize) -> CounterLimiter {
        // get current time ms.
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        CounterLimiter{
            duration,
            size,
            limit: RwLock::new(0),
            last_reset_time: RwLock::new(current_time),
        }
    }
}

impl DLimiter for CounterLimiter {
    fn get_max_rate(&self) -> usize {
        self.size
    }

    fn try_acquire(&mut self) -> bool {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let mut limit = self.limit.write().unwrap();
        let mut last_reset_time = self.last_reset_time.write().unwrap();
        if current_time - *last_reset_time > self.duration as u64 {
            *last_reset_time = current_time;
            *limit = 1;
            return true;
        }
        else if *limit < self.size {
            *limit += 1;
            return true;
        }
        false
    }
}