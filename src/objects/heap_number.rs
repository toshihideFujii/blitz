const MANTISSA_OFFSET: u64 = 0; // TODO kValueOffset
const EXPONENT_OFFSET: u64 = 4; // TODO kValueOffset + 4;

const SIGN_MASK: u64 = 0x80000000;
const EXPONENT_MASK: u64 = 0x7ff00000;
const MANTISSA_MASK: u64 = 0xfffff;
const MANTISSA_BITS: u64 = 52;
const EXPONENT_BITS: u64 = 11;
const EXPONENT_BIAS: u64 = 1023;
const EXPONENT_SHIFT: u64 = 20;
const INFINITY_OR_NAN_EXPONENT: u64 =
  (EXPONENT_MASK >> EXPONENT_SHIFT) - EXPONENT_BIAS;
const MANTISSA_BITS_IN_TOP_WORD: u64 = 20;
const NON_MANTISSA_BITS_IN_TOP_WORD: u64 = 12;

// The HeapNumber class describes heap allocated numbers that
// cannot be represented in a Smi (small integer).
struct HeapNumber {}

impl HeapNumber {
  pub fn value_as_bits() {}
  pub fn set_value_as_bits() {}

  pub fn get_exponent() {}
  pub fn get_sign() {}
}
