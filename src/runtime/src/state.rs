#![allow(dead_code)]

use std::sync::Mutex;

pub struct Snapshot<T> {
  owning_state: StateVector<T>
}

impl<T> Snapshot<T> {
  pub fn new(state: StateVector<T>) -> Self {
    Snapshot {
      owning_state: state
    }
  }

  pub fn get(_id: usize) {}
  pub fn erase(_id: usize) {}
  
  pub fn state() {}
}

pub struct StateVector<T> {
  vector: Mutex<Vec<T>>,
}

impl<T> StateVector<T> where T: Clone {
  pub fn new(reserve: usize) -> Self {
   let state_vec =  StateVector {
      vector: Mutex::new(Vec::new()),
    };
    state_vec.vector.lock().unwrap().reserve(reserve);
    state_vec
  }

  pub fn snapshot(&self) -> Snapshot<T> {
    Snapshot::new(self.clone())
  }
}

impl<T> Clone for StateVector<T>
  where T: Clone
{
  fn clone(&self) -> Self {
    let vec: Mutex<Vec<T>> = Mutex::new(Vec::new());
    vec.lock().unwrap().clone_from_slice(self.vector.lock().unwrap().as_slice());
    StateVector { 
      vector: vec,
    }
  }
}

pub struct State<T> {
  id: usize,
  snapshot: Snapshot<T>,
}

impl<T> State<T> {
  pub fn new(id: usize, snapshot: Snapshot<T>) -> Self {
    State {
      id: id,
      snapshot: snapshot
    }
  }

  pub fn get_or_create() {}
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_get_or_create() {
    let _state: StateVector<i32> = StateVector::new(0);
  }
}