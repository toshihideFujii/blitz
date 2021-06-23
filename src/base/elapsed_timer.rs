
use super::time::{/*TimeDelta,*/ TimeTicks};

struct ElapsedTimer {
    start_ticks_: TimeTicks
}

impl ElapsedTimer {
    // Returns the time elapsed since the previous start.
    // This method may only be called on a previously started timer.
    /*
    pub fn elapsed(&self) -> TimeDelta {
        let elapsed = ElapsedTimer::now() - self.start_ticks_;

    }
    */

    fn now() -> TimeTicks {
        TimeTicks::high_resolution_now()
    }
}