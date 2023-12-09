#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
  Launching,
  Running,
  Finished,
  Failed,
  Killed,
  Lost,
}

pub fn is_failed(state: TaskState) -> bool {
  state == TaskState::Lost || state == TaskState::Failed
}

pub fn is_finished(state: TaskState) -> bool {
  state == TaskState::Finished || state == TaskState::Failed ||
  state == TaskState::Killed || state == TaskState::Lost
}