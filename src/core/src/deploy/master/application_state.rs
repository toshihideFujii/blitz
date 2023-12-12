#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApplicationState {
  Waiting,
  Running,
  Finished,
  Failed,
  Killed,
  Unknown,
}