#![allow(dead_code)]

// This object represents the result of hashing some entity.
// It is intended to be used to implement hashtables or other
// hashing-based data structures.
struct HashCode {
  value: usize
}

impl HashCode {
  pub fn new() {}
  pub fn hash_value() {}
}

pub fn hash_value() {}

// Override the execution seed with a fixed value.
pub fn set_fixed_execution_hash_seed() {}

pub fn fetch64() {}
pub fn fetch32() {}

// Some primes between 2^63 and 2^64 for various uses.
const K0: u64 = 0xc3a5c85c97cb3127;
const K1: u64 = 0xb492b66fbe98f273;
const K2: u64 = 0x9ae16a3b2f90404f;
const K3: u64 = 0xc949d7c7509e6557;

// Bitwise right rotate.
pub fn rotate() {}

pub fn shift_mix() {}

pub fn hash_16_bytes() {}
pub fn hash_1to3_bytes() {}
pub fn hash_4to8_bytes() {}
pub fn hash_9to16_bytes() {}
pub fn hash_17to32_bytes() {}
pub fn hash_33to64_bytes() {}

pub fn hash_short() {}

// The intermediate state used during hashing.
struct HashState {}

impl HashState {
  pub fn new() {}
  pub fn mix_32_bytes() {}
  pub fn mix() {}
  pub fn finalize() {}
}

pub fn get_execution_seed() {}
pub fn get_hashable_data() {}
pub fn store_and_advance() {}
pub fn hash_combine_range() {}

struct HashCombineRecursiveHelper {}
impl HashCombineRecursiveHelper {
  pub fn new() {}
  pub fn combine_data() {}
  pub fn combine() {}
}

pub fn hash_combine() {}
pub fn hash_integer_value() {}