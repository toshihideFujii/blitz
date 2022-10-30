
#![allow(dead_code)]

fn combine_hash_value() {}

struct DenseMapInfo<T>(T);

const LOG_2_MAX_ALIGN: u64 = 12;

impl DenseMapInfo<char> {
  pub fn get_empty_key() -> char {
    0xFF as char
  }

  pub fn get_tombstone_key() -> char {
    let val: u8 = 0xFF - 1;
    return  val as char;
  }

  pub fn get_hash_value(val: char) -> u32 {
    (val as u32) * 37
  }

  pub fn is_equal(lhs: char, rhs: char) -> bool {
    lhs == rhs
  }
}

impl DenseMapInfo<u32> {
  pub fn get_empty_key() -> u32 {
    0xFFFFFFFF
  }

  pub fn get_tombstone_key() -> u32 {
    0xFFFFFFFF - 1
  }

  pub fn get_hash_value(val: u32) -> u32 {
    val * 37
  }

  pub fn is_equal(lhs: u32, rhs: u32) -> bool {
    lhs == rhs
  }
}

impl DenseMapInfo<u64> {
  pub fn get_empty_key() -> u64 {
    0xFFFFFFFFFFFFFFFF
  }

  pub fn get_tombstone_key() -> u64 {
    0xFFFFFFFFFFFFFFFF - 1
  }

  pub fn get_hash_value(val: u64) -> u32 {
    (val * 37) as u32
  }

  pub fn is_equal(lhs: u64, rhs: u64) -> bool {
    lhs == rhs
  }
}

impl DenseMapInfo<i16> {
  pub fn get_empty_key() -> i16 {
    0x7FFF
  }

  pub fn get_tombstone_key() -> i16 {
    -0x7FFF - 1
  }

  pub fn get_hash_value(val: i16) -> u32 {
    (val * 37) as u32
  }

  pub fn is_equal(lhs: i16, rhs: i16) -> bool {
    lhs == rhs
  }
}

impl DenseMapInfo<i32> {
  pub fn get_empty_key() -> i32 {
    0x7fffffff
  }

  pub fn get_tombstone_key() -> i32 {
    -0x7fffffff - 1
  }

  pub fn get_hash_value(val: i32) -> u32 {
    (val * 37) as u32
  }

  pub fn is_equal(lhs: i32, rhs: i32) -> bool {
    lhs == rhs
  }
}