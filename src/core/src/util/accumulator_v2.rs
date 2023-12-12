#![allow(dead_code)]

use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy)]
pub struct LongAccumulator {
  sum: u64,
  count: u64,
}

impl LongAccumulator {
  pub fn new() -> Self {
    LongAccumulator {
      sum: 0,
      count: 0
    }
  }

  pub fn is_zero(&self) -> bool {
    self.sum == 0 && self.count == 0
  }

  pub fn reset(&mut self) {
    self.sum = 0;
    self.count = 0;
  }

  pub fn count(&self) -> u64 {
    self.count
  }

  pub fn sum(&self) -> u64 {
    self.sum
  }

  pub fn avg(&self) -> f64 {
    self.sum as f64 / self.count as f64
  }

  pub fn set_value(&mut self, val: u64) {
    self.sum = val;
  }
}

impl Add<u64> for LongAccumulator {
  type Output = LongAccumulator;
  fn add(self, rhs: u64) -> LongAccumulator {
    LongAccumulator {
      sum: self.sum + rhs,
      count: self.count + 1
    }
  }
}

impl AddAssign<u64> for LongAccumulator {
  fn add_assign(&mut self, rhs: u64) {
    *self = self.add(rhs);
  }
}

impl Add<LongAccumulator> for LongAccumulator {
  type Output = LongAccumulator;
  fn add(self, rhs: LongAccumulator) -> LongAccumulator {
    LongAccumulator {
      sum: self.sum + rhs.sum,
      count: self.count + rhs.count
    }
  }
}


pub struct DoubleAccumulator {
  sum: f64,
  count: u64,
}

impl DoubleAccumulator {
  pub fn new() -> Self {
    DoubleAccumulator {
      sum: 0.0,
      count: 0
    }
  }

  pub fn is_zero(&self) -> bool {
    self.sum == 0.0 && self.count == 0
  }

  pub fn reset(&mut self) {
    self.sum = 0.0;
    self.count = 0;
  }

  pub fn count(&self) -> u64 {
    self.count
  }

  pub fn sum(&self) -> f64 {
    self.sum
  }

  pub fn avg(&self) -> f64 {
    self.sum / self.count as f64
  }

  pub fn set_value(&mut self, val: f64) {
    self.sum = val;
  }
}

impl Clone for DoubleAccumulator {
  fn clone(&self) -> Self {
    DoubleAccumulator {
      sum: self.sum,
      count: self.count
    }
  }
}

impl Add<f64> for DoubleAccumulator{
  type Output = DoubleAccumulator;
  fn add(self, rhs: f64) -> DoubleAccumulator {
    DoubleAccumulator {
      sum: self.sum + rhs,
      count: self.count + 1
    }
  }
}

impl Add<DoubleAccumulator> for DoubleAccumulator{
  type Output = DoubleAccumulator;
  fn add(self, rhs: DoubleAccumulator) -> DoubleAccumulator {
    DoubleAccumulator {
      sum: self.sum + rhs.sum,
      count: self.count + rhs.count
    }
  }
}