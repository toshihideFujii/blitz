const MULTIPLIER: i64 = 0x5deece66d;
const ADD_END: i64 = 0xb;
const MASK: i64 = 0xffffffffffff; // TODO

pub struct RandomNumberGenerator {
  initial_seed_: i64,
  state0_: u64,
  state1_: u64,
}

impl RandomNumberGenerator {
  pub fn new(_seeds: i64) {}

  // EntropySource is used as a callback function when blits needs a
  // source of entropy.
  pub fn set_entropy_source() {}

  // Returns the next pseudorandom, uniformly distributed int value
  // from this random number generator's sequence.
  // The general contract of |next_int()| is that one int value is
  // pseudorandomly generated and returned.
  // All 2^32 possible integer values are produced with (approximately)
  // equal probability.
  pub fn next_int(&mut self) -> i32 {
    self.next(32)
  }

  // Returns the next pseudorandom, uniformly distributed boolean value
  // from this random number generator's sequence.
  // The general contract of |next_boolean()| is that one boolean value is
  // pseudorandomly generated and returned.
  // The values true and false are produced with (approximately)
  // equal probability.
  pub fn next_bool(&mut self) -> bool {
    self.next(1) != 0
  }

  // Returns the next pseudorandom, uniformly distributed double value
  // between 0.0 and 1.0 from this random number generator's sequence.
  // The general contract of |next_double()| is that one double value, choosen
  // (approximatly) uniformly from the range 0.0 (inclusive) to 1.0 (exclusive),
  // is pseudorandomly generated and returned.
  pub fn next_double(&mut self) -> f64 {
    RandomNumberGenerator::xor_shift_128(&mut self.state0_, &mut self.state1_);
    RandomNumberGenerator::to_double(self.state0_)
  }

  pub fn next_i64(&mut self) -> i64 {
    RandomNumberGenerator::xor_shift_128(&mut self.state0_, &mut self.state1_);
    return (self.state0_ + self.state1_) as i64;
  }

  // Fills the elements of a specified array of bytes with random numbers.
  pub fn next_bytes(&mut self, length: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    for n in 0..length as usize {
      buffer[n] = *&self.next(8) as u8;
    }
    return buffer;
  }

  // Returns the next pseudorandom set of n unique u64 values smaller than max.
  // n must be less or equal to max.
  pub fn next_sample() {}

  pub fn next_sample_slow() {}

  // Override the current seed.
  pub fn set_seed(&mut self, seed: i64) {
    self.initial_seed_ = seed;
    self.state0_ = RandomNumberGenerator::murmur_hash_3(seed as u64);
    self.state1_ = RandomNumberGenerator::murmur_hash_3(!self.state0_);
  }

  pub fn initial_seed(&self) -> i64 {
    self.initial_seed_
  }

  pub fn to_double(state0: u64) -> f64 {
    // Exponent for double values for [1.0 .. 2.0)
    let exponent_bits: u64 = 0x3FF0000000000000;
    let random: u64 = (state0 >> 12) | exponent_bits;
    return (random as f64) - 1.0;
  }

  pub fn xor_shift_128(state0: &mut u64, state1: &mut u64) {
    let mut s1: u64 = *state0;
    let s0: u64 = *state1;
    *state0 = s0;
    s1 ^= s1 << 23;
    s1 ^= s1 >> 17;
    s1 ^= s0;
    s1 ^= s0 >> 26;
    *state1 = s1;
  }

  pub fn murmur_hash_3(mut h: u64) -> u64 {
    h ^= h >> 33;
    h *= 0xFF51AFD7ED558CCD;
    h ^= h >> 33;
    h *= 0xC4CEB9FE1A85EC53;
    h ^= h >> 33;
    h
  }

  pub fn next(&mut self, bits: i32) -> i32 {
    if bits < 0 {
      panic!("bits < 0");
    }
    if bits >= 32 {
      panic!("bits >= 32");
    }
    RandomNumberGenerator::xor_shift_128(&mut self.state0_, &mut self.state1_);
    return ((self.state0_ + self.state1_) >> (64 - bits)) as i32;
  }
}
