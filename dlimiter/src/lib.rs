mod counter_limiter;
mod base_limiter;
mod window_limiter;
mod bucket_limiter;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use base_limiter::DLimiter;
    use std::time::Duration;
    use std::thread::sleep;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn get_rate(d:  & impl DLimiter) -> usize {
        d.get_max_rate()
    }
    fn try_acquire(d: &mut impl DLimiter) -> bool {
        d.try_acquire()
    }

    #[test]
    fn counter_limiter_tester() {
        let mut limiter = counter_limiter::CounterLimiter::new(1000, 10);
        assert_eq!(get_rate(&limiter), 10);
        for i in 0..20 {
            if i < 10 {
                assert_eq!(try_acquire(&mut limiter), true);
            } else {
                assert_eq!(try_acquire(&mut limiter), false);
            }
        }
        sleep(Duration::from_secs(1));
        assert_eq!(try_acquire(&mut limiter), true);
    }

    #[test]
    fn window_limiter_tester() {
        let mut limiter = window_limiter::WindowLimiter::new(10, 1);
        assert_eq!(get_rate(&limiter), 1);
        for i in 0..3 {
            if i  < 1 {
                assert_eq!(try_acquire(&mut limiter), true);
            } else {
                assert_eq!(try_acquire(&mut limiter), false);
            }
        }
        sleep(Duration::from_millis(10));
        assert_eq!(try_acquire(&mut limiter), true);
    }

    #[test]
    fn token_bucket_limiter_tester() {
        let mut limiter = bucket_limiter::TokenBucketLimiter::new(10, 1);
        assert_eq!(get_rate(&limiter), 1);
        for i in 0..3 {
            if i < 1 {
                assert_eq!(try_acquire(&mut limiter), true);
            } else {
                assert_eq!(try_acquire(&mut limiter), false);
            }
        }
        sleep(Duration::from_millis(5));
        assert_eq!(try_acquire(&mut limiter), false);
        sleep(Duration::from_millis(5));
        assert_eq!(try_acquire(&mut limiter), true);
    }
}
