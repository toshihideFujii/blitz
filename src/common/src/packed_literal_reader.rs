#![allow(dead_code)]

use crate::{shape::Shape, layout::Layout};

pub struct PackedLiteralReader {}

impl PackedLiteralReader {
  pub fn new() {}
  pub fn read(_shape: &Shape, _layout: &Option<Layout>) {}
  pub fn is_exhausted() {}
}