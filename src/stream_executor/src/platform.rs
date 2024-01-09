#![allow(dead_code)]

pub trait Platform {
  fn name(&self) -> &str;
}