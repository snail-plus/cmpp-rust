use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};

/// Convert Duration to milliseconds.
#[inline]
pub fn duration_to_ms(d: Duration) -> u64 {
    let nanos = d.subsec_nanos() as u64;
    // Most of case, we can't have so large Duration, so here just panic if overflow now.
    d.as_secs() * 1_000 + (nanos / 1_000_000)
}

/// Convert Duration to seconds.
#[inline]
pub fn duration_to_sec(d: Duration) -> f64 {
    let nanos = d.subsec_nanos() as f64;
    // Most of case, we can't have so large Duration, so here just panic if overflow now.
    d.as_secs() as f64 + (nanos / 1_000_000_000.0)
}

/// Convert Duration to nanoseconds.
#[inline]
pub fn duration_to_nanos(d: Duration) -> u64 {
    let nanos = d.subsec_nanos() as u64;
    // Most of case, we can't have so large Duration, so here just panic if overflow now.
    d.as_secs() * 1_000_000_000 + nanos
}

#[inline]
pub fn format_date(date: DateTime<Local>, format: &str) -> String {
    // 格式化本地时间为字符串
    let formatted_local = date.format(format).to_string();
    return formatted_local;
}

fn get_timestamp_in_seconds() -> u64 {
    // 获取当前的系统时间
    let now = SystemTime::now();

    // 计算从UNIX纪元到现在的时间差
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

    // 将时间差转换为秒
    duration_since_epoch.as_secs()
}

pub struct SlowTimer {
    slow_time: Duration,
    t: Instant,
}

impl SlowTimer {
    pub fn new() -> SlowTimer {
        SlowTimer::default()
    }

    pub fn from(slow_time: Duration) -> SlowTimer {
        SlowTimer {
            slow_time,
            t: Instant::now(),
        }
    }

    pub fn from_secs(secs: u64) -> SlowTimer {
        SlowTimer::from(Duration::from_secs(secs))
    }

    pub fn from_millis(millis: u64) -> SlowTimer {
        SlowTimer::from(Duration::from_millis(millis))
    }

    pub fn elapsed(&self) -> Duration {
        self.t.elapsed()
    }

    pub fn is_slow(&self) -> bool {
        self.elapsed() >= self.slow_time
    }
}

#[allow(dead_code)]
const DEFAULT_SLOW_SECS: u64 = 1;

impl Default for SlowTimer {
    fn default() -> SlowTimer {
        SlowTimer::from_secs(DEFAULT_SLOW_SECS)
    }
}

#[allow(dead_code)]
const DEFAULT_WAIT_MS: u64 = 100;
#[allow(dead_code)]
const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;
#[allow(dead_code)]
const MILLISECOND_PER_SECOND: i64 = 1_000;
#[allow(dead_code)]
const NANOSECONDS_PER_MILLISECOND: i64 = 1_000_000;


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    use chrono::Local;

    use crate::util::time::{duration_to_ms, duration_to_nanos, duration_to_sec, format_date};

    #[test]
    fn test_duration_to() {
        let tbl = vec![0, 100, 1_000, 5_000, 9999, 1_000_000, 1_000_000_000];
        for ms in tbl {
            let d = Duration::from_millis(ms);
            assert_eq!(ms, duration_to_ms(d));
            let exp_sec = ms as f64 / 1000.0;
            let act_sec = duration_to_sec(d);
            assert!((act_sec - exp_sec).abs() < f64::EPSILON);
            assert_eq!(ms * 1_000_000, duration_to_nanos(d));
        }
    }

    #[test]
    fn test_format_date() {
        let now = Local::now();
        let f = format_date(now, "%Y-%m-%d %H:%M:%S");
        println!("{}", f);

        let counter = Arc::new(Mutex::new(0));

        // 创建多个线程，每个线程都会增加计数器的值
        let num_threads = 10;
        let mut handles = Vec::new();
        for _ in 0..num_threads {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                // 模拟一些工作，然后增加计数器的值
                thread::sleep(Duration::from_millis(10)); // 休眠一段时间以模拟工作
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        // 打印最终的计数器值
        println!("Final counter value: {}", *counter.lock().unwrap());

    }
}