

pub trait DLimiter {
    fn get_max_rate(&self) -> usize;
    fn try_acquire(&mut self) -> bool;
}