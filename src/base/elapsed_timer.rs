use super::time::{TimeDelta, TimeTicks};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ElapsedTimer {
  start_ticks_: TimeTicks,
}

impl ElapsedTimer {
  pub fn new() -> ElapsedTimer {
    ElapsedTimer {
      start_ticks_: TimeTicks::new(0),
    }
  }

  // Starts this timer.
  // Once started a timer can be checked with |Elapsed()| or |HasExpired()|,
  // and may be restarted using |Restart()|.
  // This method must not be called on an already started timer.
  pub fn start(&mut self) {
    if self.is_started() {
      panic!("Not started.");
    }
    self.start_ticks_ = ElapsedTimer::now()
  }

  // Stops this timer.
  // Must not be called on a timer that was not started before.
  pub fn stop(&mut self) {
    if !self.is_started() {
      panic!("Not started.");
    }
    self.start_ticks_ = TimeTicks::new(0);
  }

  // Reuturns true is this timer was started previously.
  pub fn is_started(&self) -> bool {
    !self.start_ticks_.is_null()
  }

  // Restarts the timer and returns the time elapsed since the previous start.
  // This method is equivalent the elapsed time with |Elapsed()| and then starting
  // the timer again, but does so in one single operation, avoiding the need to
  // obtain the clock value twice. It may only be called on a previously started timer.
  pub fn restart(&mut self) -> TimeDelta {
    if !self.is_started() {
      panic!("Not started.");
    }
    let ticks = ElapsedTimer::now();
    let elapsed = ticks - self.start_ticks_;
    self.start_ticks_ = ticks;
    elapsed
  }

  // Returns the time elapsed since the previously start.
  // This method may only be called on a previously started timer.
  pub fn elapsed(&self) -> TimeDelta {
    if !self.is_started() {
      panic!("Not started.")
    }
    ElapsedTimer::now() - self.start_ticks_
  }

  // Returns true if the specified time_delta has elapsed since the previous start,
  // or false if not. This methid only be called on a previously started timer.
  pub fn has_expired(&self, time_delta: TimeDelta) -> bool {
    self.elapsed() >= time_delta
  }

  fn now() -> TimeTicks {
    TimeTicks::high_resolution_now()
  }
}
