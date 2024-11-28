#![allow(dead_code)]

use crate::blitz_data::PaddingConfig;

pub type DimensionVector = Vec<i64>;

pub fn nan_with_sign_and_payload<T>(_sign: bool, _nan_payload: u64) -> T {
  unimplemented!()
}

pub fn append_status(_prior: i64, _context: String) -> Result<(), String> {
  unimplemented!()
}

pub fn make_no_padding_config(_rank: i64) -> PaddingConfig {
  unimplemented!()
}