#![allow(dead_code)]

struct MemoryBlock {}

impl MemoryBlock {
  pub fn base() {}

  pub fn allocated_size() {}
}

struct Memory {}

impl Memory {}

pub fn allocate_mapped_memory() {}

pub fn release_mapped_memory() {}

pub fn protect_mapped_memory() {}

pub fn invalidate_instruction_cache() {}

struct OwningMemoryBlock {}

impl OwningMemoryBlock {
  pub fn base() {}

  pub fn allocated_size() {}

  pub fn get_memory_block() {}

  pub fn release() {}
}