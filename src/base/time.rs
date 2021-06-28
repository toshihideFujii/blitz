use std::ops::{
  Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const HOURS_PER_DAY: i64 = 24;
const MILLISECONDS_PER_SECOND: i64 = 1000;
const MILLISECONDS_PER_DAY: i64 =
  MILLISECONDS_PER_SECOND * 60 * 60 * HOURS_PER_DAY;
const MICROSECONDS_PER_MILLISECOND: i64 = 1000;
const MICROSECONDS_PER_SECOND: i64 =
  MICROSECONDS_PER_MILLISECOND * MILLISECONDS_PER_SECOND;
const MICROSECONDS_PER_MINUTE: i64 = MICROSECONDS_PER_SECOND * 60;
const MICROSECONDS_PER_HOUR: i64 = MICROSECONDS_PER_MINUTE * 60;
const MICROSECONDS_PER_DAY: i64 = MICROSECONDS_PER_HOUR * HOURS_PER_DAY;
const MICROSECONDS_PER_WEEK: i64 = MICROSECONDS_PER_DAY * 7;
const NANOSECONDS_PER_MICROSECOND: i64 = 1000;
const NANOSECONDS_PER_SECOND: i64 =
  NANOSECONDS_PER_MICROSECOND * MICROSECONDS_PER_SECOND;

// TimeDelta
// This class reprsents a duration of time, internally represented in microseconds.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct TimeDelta {
  // Delta in microseconds.
  delta_: i64,
}

impl TimeDelta {
  fn new_default() -> TimeDelta {
    TimeDelta { delta_: 0 }
  }
  // Constructs a delta given the duration in microseconds.
  // This is private to avoid confusion by callers with an integer constructor.
  // Use FromSeconds, FromMilliseconds, etc. instead.
  fn new(delta: i64) -> TimeDelta {
    TimeDelta { delta_: delta }
  }

  // Converts units of time to TimeDeltas.
  pub fn from_days(days: i64) -> TimeDelta {
    TimeDelta::new(days * MICROSECONDS_PER_DAY)
  }

  pub fn from_hours(hours: i64) -> TimeDelta {
    TimeDelta::new(hours * MICROSECONDS_PER_HOUR)
  }

  pub fn from_minutes(minutes: i64) -> TimeDelta {
    TimeDelta::new(minutes * MICROSECONDS_PER_MINUTE)
  }

  pub fn from_seconds(seconds: i64) -> TimeDelta {
    TimeDelta::new(seconds * MICROSECONDS_PER_SECOND)
  }

  pub fn from_milliseconds(milliseconds: i64) -> TimeDelta {
    TimeDelta::new(milliseconds * MICROSECONDS_PER_MILLISECOND)
  }

  pub fn from_microseconds(microseconds: i64) -> TimeDelta {
    TimeDelta::new(microseconds)
  }

  pub fn from_nanoseconds(nanoseconds: i64) -> TimeDelta {
    TimeDelta::new(nanoseconds / NANOSECONDS_PER_MICROSECOND)
  }

  // Returns the maximum time delta, which should be greater than any reasonable
  // time delta we might compare it to.
  // Adding or subtracting the maximum time delta to a time or another time delt
  // has an undefined result.
  pub fn max() -> TimeDelta {
    TimeDelta::new(i64::MAX)
  }

  // Returns the minimum time delta, which should be less than any reasonable
  // time delta we might compare it to.
  // Adding or subtracting the minimum time delta to a time or another time delt
  // has an undefined result.
  pub fn min() -> TimeDelta {
    TimeDelta::new(i64::MIN)
  }

  // Returns true if the time delta is zero.
  pub fn is_zero(&self) -> bool {
    self.delta_ == 0
  }

  // Returns true if the time delta is the maximum time delta.
  pub fn is_max(&self) -> bool {
    self.delta_ == i64::MAX
  }

  // Returns true if the time delta is the minimum time delta.
  pub fn is_min(&self) -> bool {
    self.delta_ == i64::MIN
  }

  // Returns the time delta in some unit.
  pub fn in_days(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ / MICROSECONDS_PER_DAY
  }

  pub fn in_hours(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ / MICROSECONDS_PER_HOUR
  }

  pub fn in_minutes(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ / MICROSECONDS_PER_MINUTE
  }

  pub fn in_seconds(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ / MICROSECONDS_PER_SECOND
  }

  pub fn in_milliseconds(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ / MICROSECONDS_PER_MILLISECOND
  }

  pub fn in_milliseconds_round_up(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    (self.delta_ + MICROSECONDS_PER_MILLISECOND - 1)
      / MICROSECONDS_PER_MILLISECOND
  }

  pub fn in_microseconds(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_
  }

  pub fn in_nanoseconds(&self) -> i64 {
    if self.is_max() {
      return i64::MAX;
    }
    self.delta_ * NANOSECONDS_PER_MICROSECOND
  }

  // Converts to/from Mach time specs.
  pub fn from_mach_timespec() {}
  pub fn to_mach_timespec() {}

  // Converts to/from POSIX time specs.
  pub fn from_timespec() {}
  pub fn to_timespec() {}

  pub fn times_of(self, other: TimeDelta) -> i64 {
    return self.delta_ / other.delta_;
  }
  pub fn percent_of() {}
}

impl Add for TimeDelta {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      delta_: self.delta_ + other.delta_,
    }
  }
}

impl Sub for TimeDelta {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      delta_: self.delta_ - other.delta_,
    }
  }
}

impl AddAssign for TimeDelta {
  fn add_assign(&mut self, other: Self) {
    *self = Self {
      delta_: self.delta_ + other.delta_,
    }
  }
}

impl SubAssign for TimeDelta {
  fn sub_assign(&mut self, other: Self) {
    *self = Self {
      delta_: self.delta_ - other.delta_,
    }
  }
}

impl Neg for TimeDelta {
  type Output = TimeDelta;
  fn neg(self) -> Self {
    Self {
      delta_: self.delta_ * -1,
    }
  }
}

impl Mul<i64> for TimeDelta {
  type Output = TimeDelta;
  fn mul(self, rhs: i64) -> Self {
    Self {
      delta_: self.delta_ * rhs,
    }
  }
}

impl Div<i64> for TimeDelta {
  type Output = TimeDelta;
  fn div(self, rhs: i64) -> Self {
    Self {
      delta_: self.delta_ / rhs,
    }
  }
}

impl Div for TimeDelta {
  type Output = TimeDelta;
  fn div(self, rhs: Self) -> Self {
    Self {
      delta_: self.delta_ / rhs.delta_,
    }
  }
}

impl MulAssign<i64> for TimeDelta {
  fn mul_assign(&mut self, rhs: i64) {
    self.delta_ *= rhs
  }
}

impl DivAssign<i64> for TimeDelta {
  fn div_assign(&mut self, rhs: i64) {
    self.delta_ /= rhs
  }
}

// Provides value storage and comparison /math operations to all time classes.
// Each subclass provides for strong type-checking to ensure semantically meaningful
// comparison/math of time values from the same clock source or timeline.

// This class represents an absolute point in time, internally represented as
// microseconds (s/1000000) since 00:00:00 UTC, January 1, 1970.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Time {
  us_: i64,
}

impl Time {
  pub fn new(us: i64) -> Time {
    Time { us_: us }
  }

  // Returns true if this object has not been initialized.
  pub fn is_null(self) -> bool {
    self.us_ == 0
  }

  pub fn max() -> Time {
    Time::new(i64::MAX)
  }

  pub fn min() -> Time {
    Time::new(i64::MIN)
  }

  // Returns true if this object represents the maximum/minimum time.
  pub fn is_max(&self) -> bool {
    self.us_ == i64::MAX
  }

  pub fn is_min(&self) -> bool {
    self.us_ == i64::MIN
  }

  // For serializing only.
  pub fn to_internal_value(self) -> i64 {
    self.us_
  }

  // Reuturns the current time.
  pub fn now() -> Time {
    let us;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
      Ok(n) => {
        us = n.as_secs() as i64 * MICROSECONDS_PER_SECOND
          + (n.subsec_micros() as i64)
      }
      Err(_) => panic!("Systemtime before UNIX EPOCH!"),
    }
    Time { us_: us }
  }

  // Returns the current time.
  // Same as now() except that this function always uses system time so that
  // there are no discrepancies between the returned time and system time even
  // on virtual environments including our test bot.
  // For timing sensitive unittests, this function should be used.
  pub fn now_from_system_time() -> Time {
    Time::now()
  }

  // Returns the time for epoch in Unix-like system (Jan 1, 1970).
  pub fn unix_epoch() -> Time {
    Time::new(0)
  }

  // Converts from the JavaScript convention for times, a number of milliseconds
  // since the epoch.
  pub fn from_js_time(ms_since_epoch: f64) -> Time {
    if ms_since_epoch == f64::MAX {
      return Time::max();
    }
    let us_f64 = ms_since_epoch * (MICROSECONDS_PER_MILLISECOND as f64);
    Time::new(us_f64 as i64)
  }

  // Converts to the JavaScript convention for times, a number of milliseconds
  // since the epoch.
  pub fn to_js_time(&self) -> f64 {
    if self.is_null() {
      return 0 as f64;
    }
    if self.is_max() {
      return f64::MAX;
    }
    (self.us_ as f64) / (MICROSECONDS_PER_MILLISECOND as f64)
  }
}

impl Add for Time {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      us_: self.us_ + other.us_,
    }
  }
}

impl Add<TimeDelta> for Time {
  type Output = Time;
  fn add(self, delta: TimeDelta) -> Time {
    Self {
      us_: self.us_ + delta.delta_,
    }
  }
}

impl Sub for Time {
  type Output = TimeDelta;
  fn sub(self, other: Time) -> TimeDelta {
    TimeDelta {
      delta_: self.us_ - other.us_,
    }
  }
}

impl Sub<TimeDelta> for Time {
  type Output = Time;
  fn sub(self, delta: TimeDelta) -> Time {
    Self {
      us_: self.us_ - delta.delta_,
    }
  }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct TimeTicks {
  us_: i64,
}

impl TimeTicks {
  pub fn new(us: i64) -> TimeTicks {
    TimeTicks { us_: us }
  }

  // Returns true if this object has not been initialized.
  pub fn is_null(self) -> bool {
    self.us_ == 0
  }

  pub fn max() -> TimeTicks {
    TimeTicks::new(i64::MAX)
  }

  pub fn min() -> TimeTicks {
    TimeTicks::new(i64::MIN)
  }

  // Returns true if this object represents the maximum/minimum time.
  pub fn is_max(&self) -> bool {
    self.us_ == i64::MAX
  }

  pub fn is_min(&self) -> bool {
    self.us_ == i64::MIN
  }

  // For serializing only.
  pub fn to_internal_value(self) -> i64 {
    self.us_
  }

  pub fn now() -> TimeTicks {
    let us;
    match SystemTime::now().duration_since(UNIX_EPOCH) {
      Ok(n) => {
        us = n.as_secs() as i64 * MICROSECONDS_PER_SECOND
          + (n.subsec_micros() as i64)
      }
      Err(_) => panic!("Systemtime before UNIX EPOCH!"),
    }
    TimeTicks { us_: us }
  }

  pub fn high_resolution_now() -> TimeTicks {
    TimeTicks::now()
  }

  // Returns true if the high-resolution clock is working on this system.
  pub fn is_high_resolution() -> bool {
    true
  }
}

impl Add for TimeTicks {
  type Output = TimeDelta;
  fn add(self, other: TimeTicks) -> TimeDelta {
    TimeDelta {
      delta_: self.us_ + other.us_,
    }
  }
}

impl Sub for TimeTicks {
  type Output = TimeDelta;
  fn sub(self, other: TimeTicks) -> TimeDelta {
    TimeDelta {
      delta_: self.us_ - other.us_,
    }
  }
}

// ThreadTicks
// Represents a clock, specific to a particular thread, than runs only while the thread is running.
struct ThreadTicks {
  us_: i64,
}

impl ThreadTicks {
  // Returns true if ThreadTicks::now() is supported on this system.
  pub fn is_supported() -> bool {
    return true;
  }

  pub fn wait_until_initialized() {}

  pub fn now() {}
}

#[cfg(test)]
mod tests {

  use super::super::elapsed_timer::ElapsedTimer;
  use super::*;

  // TimeDelta
  #[test]
  fn test_timedelta_is_zero() {
    let zero = TimeDelta::new(0);
    assert_eq!(zero.is_zero(), true);
    let max = TimeDelta::max();
    assert_eq!(max.is_max(), true);
    assert!(max > TimeDelta::from_days(100 * 365));
    assert!(max > zero);
    let min = TimeDelta::min();
    assert_eq!(min.is_min(), true);
    assert!(min < TimeDelta::from_days(-100 * 365));
    assert!(min < zero);
  }

  #[test]
  fn test_timedelta_max_conversions() {
    let max = TimeDelta::max();
    assert_eq!(max.in_days(), i64::MAX);
    assert_eq!(max.in_hours(), i64::MAX);
    assert_eq!(max.in_minutes(), i64::MAX);
    assert_eq!(max.in_seconds(), i64::MAX);
    assert_eq!(max.in_milliseconds(), i64::MAX);
    assert_eq!(max.in_milliseconds_round_up(), i64::MAX);
  }

  #[test]
  fn test_timedelta_numeric_operators() {
    let i = 2 as i64;
    assert_eq!(
      TimeDelta::from_milliseconds(2000),
      TimeDelta::from_milliseconds(1000) * i
    );
    assert_eq!(
      TimeDelta::from_milliseconds(500),
      TimeDelta::from_milliseconds(1000) / i
    );
    let mut mulassign = TimeDelta::from_milliseconds(1000);
    mulassign *= i;
    assert_eq!(TimeDelta::from_milliseconds(2000), mulassign);
    let mut divassign = TimeDelta::from_milliseconds(1000);
    divassign /= i;
    assert_eq!(TimeDelta::from_milliseconds(500), divassign);

    assert_eq!(
      TimeDelta::from_milliseconds(2000),
      TimeDelta::from_milliseconds(1000) * 2
    );
    assert_eq!(
      TimeDelta::from_milliseconds(500),
      TimeDelta::from_milliseconds(1000) / 2
    );
    mulassign = TimeDelta::from_milliseconds(1000);
    mulassign *= 2;
    assert_eq!(TimeDelta::from_milliseconds(2000), mulassign);
    divassign = TimeDelta::from_milliseconds(1000);
    divassign /= 2;
    assert_eq!(TimeDelta::from_milliseconds(500), divassign);
  }

  #[test]
  //#[should_panic]
  fn test_timedelta_disabled_overflows() {
    let large_delta = TimeDelta::max() - TimeDelta::from_milliseconds(1);
    let large_negative = -large_delta;
    assert!(TimeDelta::new(0) > large_negative);
    assert_eq!(large_delta.is_max(), false);
    assert_eq!((-large_negative).is_min(), false);

    //let one_sec = TimeDelta::from_seconds(1);
    /*
    assert_eq!((large_delta + one_sec).is_max(), true);
    assert_eq!((large_negative + (-one_sec)).is_min(), true);
    assert_eq!((large_negative - one_sec).is_min(), true);
    assert_eq!((large_delta - (-one_sec)).is_max(), true);
    assert_eq!((large_delta * 2).is_max(), true);
    assert_eq!((large_delta * -2).is_max(), true);
    */
    //let time_now = Time::now();
    //let delta1 = time_now + one_sec;
    //let delta2 = delta1 - time_now;
    //assert_eq!(one_sec, delta);
  }

  #[test]
  fn test_timedelta_from_and_in() {
    assert_eq!(TimeDelta::from_days(2), TimeDelta::from_hours(48));
    assert_eq!(TimeDelta::from_hours(3), TimeDelta::from_minutes(180));
    assert_eq!(TimeDelta::from_minutes(2), TimeDelta::from_seconds(120));
    assert_eq!(
      TimeDelta::from_seconds(2),
      TimeDelta::from_milliseconds(2000)
    );
    assert_eq!(
      TimeDelta::from_milliseconds(2),
      TimeDelta::from_microseconds(2000)
    );
    assert_eq!(TimeDelta::from_days(13).in_days(), 13);
    assert_eq!(TimeDelta::from_hours(13).in_hours(), 13);
    assert_eq!(TimeDelta::from_minutes(13).in_minutes(), 13);
    assert_eq!(TimeDelta::from_seconds(13).in_seconds(), 13);
    assert_eq!(TimeDelta::from_milliseconds(13).in_milliseconds(), 13);
    assert_eq!(TimeDelta::from_microseconds(13).in_microseconds(), 13);
  }

  // Time
  #[test]
  fn test_time_max() {
    let max = Time::max();
    assert_eq!(max.is_max(), true);
    assert_eq!(max, Time::max());
    assert!(max > Time::now());
    assert!(max > Time::new(0));
  }

  #[test]
  fn test_time_max_conversions() {
    let max = Time::max();
    assert_eq!(max.to_internal_value(), i64::MAX);
  }

  #[test]
  fn test_time_jstime() {
    let t = Time::from_js_time(70000.3);
    assert_eq!(t.to_js_time(), 70000.3);
  }

  #[test]
  fn test_time_now_resolution() {
    let target_granularity = TimeDelta::from_milliseconds(16);

    let expiration_timeout = TimeDelta::from_seconds(1);
    let mut timer = ElapsedTimer::new();
    timer.start();
    let mut delta;
    loop {
      let start = Time::now();
      // Loop until we can detect that the clock has changed.
      // Non highres timers will increment in chunks, i.e. 15ms.
      // By spinning until we see a clock change, we detect the minimum time between measurements.
      loop {
        let now = Time::now();
        delta = now - start;
        if now > start {
          break;
        }
      }
      assert_ne!(delta.in_microseconds(), 0);
      if delta <= target_granularity || timer.has_expired(expiration_timeout) {
        break;
      }
    }
    assert!(delta <= target_granularity);
  }

  // TimeTicks
  #[test]
  fn test_timeticks_now_resolution() {
    let target_granularity = match TimeTicks::is_high_resolution() {
      true => TimeDelta::from_microseconds(1),
      false => TimeDelta::from_milliseconds(16),
    };
    let expiration_timeout = TimeDelta::from_seconds(1);
    let mut timer = ElapsedTimer::new();
    timer.start();
    let mut delta;
    loop {
      let start = TimeTicks::now();
      // Loop until we can detect that the clock has changed.
      // Non highres timers will increment in chunks, i.e. 15ms.
      // By spinning until we see a clock change, we detect the minimum time between measurements.
      loop {
        let now = TimeTicks::now();
        delta = now - start;
        if now > start {
          break;
        }
      }
      assert_ne!(delta.in_microseconds(), 0);
      if delta <= target_granularity || timer.has_expired(expiration_timeout) {
        break;
      }
    }
    assert!(delta <= target_granularity);
  }

  #[test]
  fn test_timeticks_is_monotonic() {
    let mut prev_normal_ticks = TimeTicks::new(0);
    let mut prev_highres_ticks = TimeTicks::new(0);
    let mut timer = ElapsedTimer::new();
    timer.start();
    while !timer.has_expired(TimeDelta::from_milliseconds(100)) {
      let normal_ticks = TimeTicks::now();
      let highres_ticks = TimeTicks::high_resolution_now();
      assert!(normal_ticks >= prev_normal_ticks);
      assert!((normal_ticks - prev_normal_ticks).in_microseconds() >= 0);
      assert!(highres_ticks >= prev_highres_ticks);
      assert!((highres_ticks - prev_highres_ticks).in_microseconds() >= 0);
      prev_normal_ticks = normal_ticks;
      prev_highres_ticks = highres_ticks;
    }
  }
}
