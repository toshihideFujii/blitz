#![allow(dead_code)]

use std::{
  ops::{Add, AddAssign, SubAssign, Sub},
  sync::{Mutex, MutexGuard},
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
      count: self.count + 1,
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

impl Sub<u64> for LongAccumulator {
  type Output = LongAccumulator;
  fn sub(self, rhs: u64) -> LongAccumulator {
    LongAccumulator {
      sum: self.sum - rhs,
      count: self.count - 1,
    }
  }
}

impl SubAssign<u64> for LongAccumulator {
  fn sub_assign(&mut self, rhs: u64) {
    *self = self.sub(rhs);
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionAccumulator<T> {
  list: Mutex<Vec<T>>,
}

impl<T: Clone> CollectionAccumulator<T> {
  pub fn is_zero(&self) -> bool {
    self.list.lock().unwrap().is_empty()
  }

  pub fn reset(&mut self) {
    self.list.lock().unwrap().clear()
  }

  pub fn add(&mut self, v: T) {
    self.list.lock().unwrap().push(v);
  }

  pub fn merge(&mut self, other: CollectionAccumulator<T>) {
    let mut other_list = Vec::new();
    other_list.clone_from(&other.list.lock().unwrap());
    self.list.lock().unwrap().append(&mut other_list);
  }

  pub fn value(&self) -> MutexGuard<'_, Vec<T>> {
    self.list.lock().unwrap()
  }

  pub fn set_value(&mut self, value: CollectionAccumulator<T>) {
    self.reset();
    self.list = value.list;
  }
}