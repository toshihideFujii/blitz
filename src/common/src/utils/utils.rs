#![allow(dead_code)]

use super::byte_unit::{ByteUnit, BYTE, KB, MB, GB, TB, PB};

pub fn byte_string_as(_str: String, _unit: &ByteUnit) -> u64 { 0 }

pub fn byte_string_as_bytes(str: String) -> u64 {
  byte_string_as(str, &BYTE)
}

pub fn byte_string_as_kb(str: String) -> u64 {
  byte_string_as(str, &KB)
}

pub fn byte_string_as_mb(str: String) -> u64 {
  byte_string_as(str, &MB)
}

pub fn byte_string_as_gb(str: String) -> u64 {
  byte_string_as(str, &GB)
}

pub fn byte_string_as_tb(str: String) -> u64 {
  byte_string_as(str, &TB)
}

pub fn byte_string_as_pb(str: String) -> u64 {
  byte_string_as(str, &PB)
}
