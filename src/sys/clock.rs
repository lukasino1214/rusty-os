use crate::sys;
use crate::sys::cmos::CMOS;

use time::{OffsetDateTime, Duration};
use crate::println;

const DAYS_BEFORE_MONTH: [u64; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

// NOTE: This clock is monotonic
pub fn uptime() -> f64 {
    sys::time::time_between_ticks() * sys::time::ticks() as f64
}

// NOTE: This clock is not monotonic
pub fn realtime() -> f64 {
    let rtc = CMOS::new().rtc(); // Assuming GMT

    let timestamp = 86400 * days_before_year(rtc.year as u64)
        + 86400 * days_before_month(rtc.year as u64, rtc.month as u64)
        + 86400 * (rtc.day - 1) as u64
        +  3600 * rtc.hour as u64
        +    60 * rtc.minute as u64
        +         rtc.second as u64;

    let fract = sys::time::time_between_ticks()
        * (sys::time::ticks() - sys::time::last_rtc_update()) as f64;

    (timestamp as f64) + fract
}

fn days_before_year(year: u64) -> u64 {
    (1970..year).fold(0, |days, y| {
        days + if is_leap_year(y) { 366 } else { 365 }
    })
}

fn days_before_month(year: u64, month: u64) -> u64 {
    let leap_day = is_leap_year(year) && month > 2;
    DAYS_BEFORE_MONTH[(month as usize) - 1] + if leap_day { 1 } else { 0 }
}

fn is_leap_year(year: u64) -> bool {
    if year % 4 != 0 {
        false
    } else if year % 100 != 0 {
        true
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}

pub fn init() {
    let s = realtime();
    let ns = Duration::nanoseconds(libm::floor(1e9 * (s - libm::floor(s))) as i64);
    let dt = OffsetDateTime::from_unix_timestamp(s as i64) + ns;
    let rtc = dt.format("%F %H:%M:%S UTC");
    println!("RTC {}", rtc);
}

#[test_case]
fn test_uptime() {
    assert!(uptime() > 0.0);
}

#[test_case]
fn test_realtime() {
    assert!(realtime() > 1234567890.0);
}
