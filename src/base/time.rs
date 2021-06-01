

const HOURS_PER_DAY: i64 = 24;
const MILLISECONDS_PER_SECOND: i64 = 1000;
const MILLISECONDS_PER_DAY: i64 = MILLISECONDS_PER_SECOND * 60 * 60  * HOURS_PER_DAY;
const MICROSECONDS_PER_MILLISECONDS: i64 = 1000;
const MICROSECONDS_PER_SECOND: i64 = MILLISECONDS_PER_SECOND * MILLISECONDS_PER_SECOND;


// TimeDelta
// This class rprsents a duration of time, internally represented in microseconds.
struct TimeDelta {
    // Delta in microseconds.
    delta: i64
}

impl TimeDelta {
    fn new_default() -> TimeDelta {
        TimeDelta{ delta: 0 }
    }
    // Constructs a delta given the duration in microseconds.
    // This is private to avoid confusion by callers with an integer constructor.
    // Use FromSeconds, FromMilliseconds, etc. instead.
    fn new(delta: i64) -> TimeDelta {
        TimeDelta{ delta: delta }
    }

    fn from_days(days: i64) -> TimeDelta {
        TimeDelta::new(days * MILLISECONDS_PER_DAY)
    }
    // Returns true if the time delta is zero.
    fn is_zero(&self) -> bool {
        self.delta == 0
    }
    // Returns true if the time delta is the maximum time delta.
    fn is_max(&self) -> bool {
        self.delta == std::i64::MAX
    }
    // Returns true if the time delta is the minimum time delta.
    fn is_min(&self) -> bool {
        self.delta == std::i64::MIN
    }
}