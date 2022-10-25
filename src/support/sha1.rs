
#![allow(dead_code)]

const BLOCK_LENGTH: u64 = 64;
const HASH_LENGTH: u64 = 20;

struct SHA1 {}

impl SHA1 {
  pub fn init() {}

  // Digest more data.
  pub fn update() {}

  //pub fn final() {}

  pub fn result() {}

  // Returns a raw 160-bit SHA1 hash for the given data.
  pub fn hash() {}

  fn write_byte() {}

  fn hash_block() {}

  fn add_uncounted() {}

  fn pad() {}
}