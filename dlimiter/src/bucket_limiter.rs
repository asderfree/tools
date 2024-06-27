use crate::base_limiter::DLimiter;
use std::{sync::RwLock, time::{SystemTime, UNIX_EPOCH}};

pub struct TokenBucketLimiter {
    size: usize,    // the max number of tokens in the bucket.
    duration: u64,  // ms, a token will be ad to the bucket every duration ms.
    tokens: RwLock<usize>,  // current number of tokens in the bucket.
    last_update_time: RwLock<u64>,  // ms, last time the bucket was updated.
}

impl TokenBucketLimiter{
    pub fn new(duration: u64, size: usize) -> Self {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        TokenBucketLimiter {
            size,
            duration,
            tokens: RwLock::new(size),
            last_update_time: RwLock::new(current_time),
        }
    }
}

impl DLimiter for TokenBucketLimiter {
    fn get_max_rate(&self) -> usize {
        self.size
    }
    fn try_acquire(&mut self) -> bool {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let mut tokens = self.tokens.write().unwrap();
        let mut last_update_time = self.last_update_time.write().unwrap();
        let elapsed_time = current_time - *last_update_time;
        let tokens_to_add = (elapsed_time / self.duration) as usize;
        *tokens = (*tokens + tokens_to_add).min(self.size);
        if *tokens > 0 {
            *last_update_time = current_time;
            *tokens -= 1;
            true
        } else {
            false
        }
    }
}