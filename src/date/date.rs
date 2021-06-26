use crate::common::globals;

pub const MS_PER_MIN: i64 = 60 * 1000;
pub const SEC_PER_DAY: i64 = 24 * 60 * 60;
pub const MS_PER_DAY: i64 = SEC_PER_DAY * 1000;
pub const MS_PER_MONTH: i64 = MS_PER_DAY * 30;

// The largest time that can be passed to OS date-time library functions.
pub const MAX_EPOCH_TIME_IN_SEC: i64 = globals::MAX_INT;
pub const MAX_EPOCH_TIME_IN_MS: i64 = globals::MAX_INT * 1000;

// The largest time that can be stored in JSDate.
pub const MAX_TIME_IN_MS: i64 = 864000000 * 10000000;

// Conservative upper bound on time that can be stored in JSDate before UTC conversion.
pub const MAX_TIME_BEFORE_UTC_IN_MS: i64 = MAX_TIME_IN_MS + MS_PER_MONTH;

// Sentinel that denotes an invalid local offset.
pub const INVALID_LOCAL_OFFSET_IN_MS: i64 = globals::MAX_INT;

// Sentinel that denotes an invalid cache stamp.
// It is an invariant of DateCache that cache stamp is non-negative.
pub const INVALID_STAMP: i64 = -1;

struct DateCache {}

// Computes modulo(time_ms, MS_PER_DAY) given that
// days = floor(time_ms / MS_PER_DAY)
pub fn time_in_day(time_ms: i64, days: i64) -> i64 {
  return time_ms - days * MS_PER_DAY;
}

// Given the number of days since the epoch, computes the weekday.
pub fn week_day(days: i64) -> i64 {
  let result: i64 = (days + 4) % 7;
  if result > 0 {
    return result;
  } else {
    return result + 7;
  }
}

pub fn is_leap(year: i64) -> bool {
  return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
}
