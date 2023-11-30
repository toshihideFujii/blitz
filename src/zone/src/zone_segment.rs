#![allow(dead_code)]
use crate::zone::Zone;

pub struct Segment {
  zone: Option<Zone>,
  //next: Option<Segment>,
  size: usize,
}

impl Segment {
  pub fn new() {}
}
