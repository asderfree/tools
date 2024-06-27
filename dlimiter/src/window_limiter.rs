use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::base_limiter::DLimiter;

pub struct WindowLimiter {
    size: usize,
    duration: u64,  // ms
    requests: RwLock<Vec<u64>>,
}

impl WindowLimiter {
    pub fn new(duration: u64, size: usize) -> WindowLimiter {
        WindowLimiter {
            size,
            duration,
            requests: RwLock::new(Vec::new()),
        }
    }
}

impl DLimiter for WindowLimiter {
    fn get_max_rate(&self) -> usize {
        self.size
    }

    fn try_acquire(&mut self) -> bool {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let mut requests = self.requests.write().unwrap();
        // maybe we can faster?
        requests.retain(|&x| x > current_time - self.duration);
        if requests.len() < self.size {
            requests.push(current_time);
            return true;
        }
        false
    }
    
}