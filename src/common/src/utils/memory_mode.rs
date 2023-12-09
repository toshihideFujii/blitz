#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryMode {
  OnHeap,
  OffHeap,
}