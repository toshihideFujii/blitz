// Smi represents integer Numbers that can be stored in 31 bits.
// Smis are immediate which means they are NOT allocated in the heap.
// For long smis it has the following format:
//   [32 bit signed int] [31 bits zero padding] 0
// Smi stands for small integer.
pub struct Smi {}

impl Smi {
  pub fn new() {}

  // Returns the integer value.
  pub fn value() {}

  pub fn to_uint32_ami() {}

  // Convert a Smi object to an int.
  pub fn to_int() {}

  // Convert a value to a Smi object.
  pub fn from_int() {}

  pub fn from_31_bit_pattern() {}

  // Returns whether value can be represented in a Smi.
  pub fn is_valid_smi() {}

  pub fn lexicographic_compare() {}

  // Dispatched behavior.
  pub fn smi_print() {}

  pub fn zero() {}
}
