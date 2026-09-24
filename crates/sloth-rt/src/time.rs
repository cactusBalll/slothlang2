//! Monotonic clock + sleeping for the IO layer (event timeouts, HTTP
//! keep-alive deadlines). Times are milliseconds since an unspecified epoch;
//! only differences are meaningful.

#[no_mangle]
pub extern "C" fn __sloth_now_ms() -> i64 {
    unsafe {
        let mut ts: libc::timespec = std::mem::zeroed();
        libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
        ts.tv_sec as i64 * 1000 + ts.tv_nsec as i64 / 1_000_000
    }
}

/// sleep `ms` milliseconds (interruptible by signals)
#[no_mangle]
pub extern "C" fn __sloth_sleep_ms(ms: i64) -> i64 {
    if ms <= 0 {
        return 0;
    }
    unsafe {
        let req = libc::timespec {
            tv_sec: ms / 1000,
            tv_nsec: (ms % 1000) * 1_000_000,
        };
        let mut rem: libc::timespec = std::mem::zeroed();
        libc::nanosleep(&req, &mut rem);
    }
    0
}
