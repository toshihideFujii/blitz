use std::{ops::Div, u64};

pub trait BitOp<T> {
  // Returns the number of bits set in |value|.
  fn count_population(value: T) -> u32;

  // Returns the number of zero bits following the most significant
  // 1 bit in |value| if |value| is non-zero, otherwise it returns
  // { sizeof(T) * 8 }.
  fn count_leading_zeros(value: T) -> u32;

  // Returns the number of zero bits preceding the least significant 1 bit
  // in |value| if |value| is non-zero, otherwise it returns { sizeof(T) * 8 }.
  fn count_trailing_zeros(value: T) -> u32;

  // Returns true if |value| is a power of 2.
  fn is_power_of_two(value: T) -> bool;

  // Identical to {count_trailing_zeros}, but only works for power of 2.
  fn which_power_of_two(value: T) -> u32;

  // Returns the smallest power of two which is greater than or equal to |value|.
  fn round_up_to_power_of_two(value: T) -> T;

  // Returns the greatest power of two which is less than or
  // equal to |value|. If you pass in a |value| that is already
  // a power of two, it is returned as is.
  fn round_down_to_power_of_two(value: T) -> T;

  fn rotate_left(value: T, shift: u32) -> T;

  fn rotate_right(value: T, shift: u32) -> T;

  // Divides |lhs| by |rhs| and returns the quotient truncated to u32.
  // If |rhs| is zero, then zero is returned.
  fn unsigned_div(lhs: T, rhs: T) -> T;

  // Divides |lhs| by |rhs| and returns the remainder truncated to u32.
  // If |rhs| is zero, then zero is returned.
  fn unsigned_mod(lhs: T, rhs: T) -> T;
}

pub struct Bits {}

impl BitOp<u8> for Bits {
  fn count_population(value: u8) -> u32 {
    value.count_ones()
  }
  fn count_leading_zeros(value: u8) -> u32 {
    value.leading_zeros()
  }
  fn count_trailing_zeros(value: u8) -> u32 {
    value.trailing_zeros()
  }
  fn is_power_of_two(value: u8) -> bool {
    value > 0 && (value & (value - 1)) == 0
  }
  fn which_power_of_two(value: u8) -> u32 {
    if Bits::is_power_of_two(value) {
      Bits::count_trailing_zeros(value)
    } else {
      panic!("is_power_of_2({}) is false.", value);
    }
  }
  fn round_up_to_power_of_two(value: u8) -> u8 {
    value.next_power_of_two()
  }
  fn round_down_to_power_of_two(_value: u8) -> u8 {
    panic!("Not supported.")
  }
  fn rotate_left(value: u8, shift: u32) -> u8 {
    value.rotate_left(shift)
  }
  fn rotate_right(value: u8, shift: u32) -> u8 {
    value.rotate_right(shift)
  }
  fn unsigned_div(lhs: u8, rhs: u8) -> u8 {
    if rhs == 0 {
      return 0 as u8;
    }
    lhs.div(rhs)
  }
  fn unsigned_mod(lhs: u8, rhs: u8) -> u8 {
    if rhs == 0 {
      return 0 as u8;
    }
    lhs % rhs
  }
}

impl BitOp<u16> for Bits {
  fn count_population(value: u16) -> u32 {
    value.count_ones()
  }
  fn count_leading_zeros(value: u16) -> u32 {
    value.leading_zeros()
  }
  fn count_trailing_zeros(value: u16) -> u32 {
    value.trailing_zeros()
  }
  fn is_power_of_two(value: u16) -> bool {
    value > 0 && (value & (value - 1)) == 0
  }
  fn which_power_of_two(value: u16) -> u32 {
    if Bits::is_power_of_two(value) {
      Bits::count_trailing_zeros(value)
    } else {
      panic!("is_power_of_2({}) is false.", value);
    }
  }
  fn round_up_to_power_of_two(value: u16) -> u16 {
    value.next_power_of_two()
  }
  fn round_down_to_power_of_two(_value: u16) -> u16 {
    panic!("Not supported.")
  }
  fn rotate_left(value: u16, shift: u32) -> u16 {
    value.rotate_left(shift)
  }
  fn rotate_right(value: u16, shift: u32) -> u16 {
    value.rotate_right(shift)
  }
  fn unsigned_div(lhs: u16, rhs: u16) -> u16 {
    if rhs == 0 {
      return 0 as u16;
    }
    lhs.div(rhs)
  }
  fn unsigned_mod(lhs: u16, rhs: u16) -> u16 {
    if rhs == 0 {
      return 0 as u16;
    }
    lhs % rhs
  }
}

impl BitOp<u32> for Bits {
  fn count_population(value: u32) -> u32 {
    value.count_ones()
  }
  fn count_leading_zeros(value: u32) -> u32 {
    value.leading_zeros()
  }
  fn count_trailing_zeros(value: u32) -> u32 {
    value.trailing_zeros()
  }
  fn is_power_of_two(value: u32) -> bool {
    value > 0 && (value & (value - 1)) == 0
  }
  fn which_power_of_two(value: u32) -> u32 {
    if Bits::is_power_of_two(value) {
      Bits::count_trailing_zeros(value)
    } else {
      panic!("is_power_of_2({}) is false.", value);
    }
  }
  fn round_up_to_power_of_two(value: u32) -> u32 {
    value.next_power_of_two()
  }
  fn round_down_to_power_of_two(value: u32) -> u32 {
    if value > 0x80000000 {
      return 0x80000000;
    }
    let mut result = Bits::round_up_to_power_of_two(value);
    if result > value {
      result >>= 1;
    }
    return result;
  }
  fn rotate_left(value: u32, shift: u32) -> u32 {
    value.rotate_left(shift)
  }
  fn rotate_right(value: u32, shift: u32) -> u32 {
    value.rotate_right(shift)
  }
  fn unsigned_div(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
      return 0 as u32;
    }
    lhs.div(rhs)
  }
  fn unsigned_mod(lhs: u32, rhs: u32) -> u32 {
    if rhs == 0 {
      return 0 as u32;
    }
    lhs % rhs
  }
}

impl BitOp<u64> for Bits {
  fn count_population(value: u64) -> u32 {
    value.count_ones()
  }
  fn count_leading_zeros(value: u64) -> u32 {
    value.leading_zeros()
  }
  fn count_trailing_zeros(value: u64) -> u32 {
    value.trailing_zeros()
  }
  fn is_power_of_two(value: u64) -> bool {
    value > 0 && (value & (value - 1)) == 0
  }
  fn which_power_of_two(value: u64) -> u32 {
    if Bits::is_power_of_two(value) {
      Bits::count_trailing_zeros(value)
    } else {
      panic!("is_power_of_2({}) is false.", value);
    }
  }
  fn round_up_to_power_of_two(value: u64) -> u64 {
    value.next_power_of_two()
  }
  fn round_down_to_power_of_two(_value: u64) -> u64 {
    panic!("Not supported.")
  }
  fn rotate_left(value: u64, shift: u32) -> u64 {
    value.rotate_left(shift)
  }
  fn rotate_right(value: u64, shift: u32) -> u64 {
    value.rotate_right(shift)
  }
  fn unsigned_div(lhs: u64, rhs: u64) -> u64 {
    if rhs == 0 {
      return 0 as u64;
    }
    lhs.div(rhs)
  }
  fn unsigned_mod(lhs: u64, rhs: u64) -> u64 {
    if rhs == 0 {
      return 0 as u64;
    }
    lhs % rhs
  }
}

impl Bits {
  // Returns |value| in reverse bit order.
  pub fn reverse_bits() {}

  // Multiplies two signed 32-bit values |lhs| and |rhs|,
  // extracts the most significant 32 bits of the result, and returns those.
  #[allow(unused_variables)]
  pub fn signed_mul_high_32(lhs: i32, rhs: i32) -> i32 {
    0
  }

  // Multiplies two signed 32-bit values |lhs| and |rhs|,
  // extracts the most significant 32 bits of the result, and adds the accumulate value |acc|.
  #[allow(unused_variables)]
  pub fn signed_mul_high_and_add_32(lhs: i32, rhs: i32, acc: i32) -> i32 {
    0
  }

  // Divides |lhs| by |rhs| and returns the quotient truncated i32.
  // If |rhs| is zero, then zero is returned.
  // If |lhs| is minint and |rhs| is -1, it returns minint.
  pub fn signed_div_32(lhs: i32, rhs: i32) -> i32 {
    if rhs == 0 {
      return 0;
    }
    if rhs == -1 {
      if lhs == i32::MIN {
        return lhs;
      } else {
        return -lhs;
      }
    }
    return lhs / rhs;
  }

  // Divides |lhs| by |rhs| and returns the remainder truncated i32.
  // If either |rhs| is zero or |lhs| is minint and |rhs| is -1, it returns zero.
  pub fn signed_mod_32(lhs: i32, rhs: i32) -> i32 {
    if rhs == 0 || rhs == -1 {
      return 0;
    }
    return lhs % rhs;
  }

  // Adds |lhs| and |rhs|, checks and returns the result.
  pub fn signed_saturated_add_64(lhs: i64, rhs: i64) -> i64 {
    if rhs < 0 && lhs < i64::MIN - rhs {
      return i64::MIN;
    }
    if rhs >= 0 && lhs > i64::MAX - rhs {
      return i64::MAX;
    }
    return lhs + rhs;
  }

  // Subtracts |lhs| and |rhs|, checks and returns the result.
  pub fn signed_saturated_sub_64(lhs: i64, rhs: i64) -> i64 {
    if rhs > 0 && lhs < i64::MIN + rhs {
      return i64::MIN;
    }
    if rhs <= 0 && lhs > i64::MAX + rhs {
      return i64::MAX;
    }
    return lhs - rhs;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bits_count_population_u8() {
    assert_eq!(Bits::count_population(0 as u8), 0);
    assert_eq!(Bits::count_population(1 as u8), 1);
    assert_eq!(Bits::count_population(0x11 as u8), 2);
    assert_eq!(Bits::count_population(0x0F as u8), 4);
    assert_eq!(Bits::count_population(0x3F as u8), 6);
    assert_eq!(Bits::count_population(0xFF as u8), 8);
  }

  #[test]
  fn test_bits_count_population_u16() {
    assert_eq!(Bits::count_population(0 as u16), 0);
    assert_eq!(Bits::count_population(1 as u16), 1);
    assert_eq!(Bits::count_population(0x1111 as u16), 4);
    assert_eq!(Bits::count_population(0xF0F0 as u16), 8);
    assert_eq!(Bits::count_population(0xF0FF as u16), 12);
    assert_eq!(Bits::count_population(0xFFFF as u16), 16);
  }

  #[test]
  fn test_bits_count_population_u32() {
    assert_eq!(Bits::count_population(0 as u32), 0);
    assert_eq!(Bits::count_population(1 as u32), 1);
    assert_eq!(Bits::count_population(0x11111111 as u32), 8);
    assert_eq!(Bits::count_population(0xF0F0F0F0 as u32), 16);
    assert_eq!(Bits::count_population(0xFFF0F0FF as u32), 24);
    assert_eq!(Bits::count_population(0xFFFFFFFF as u32), 32);
  }

  #[test]
  fn test_bits_count_population_u64() {
    assert_eq!(Bits::count_population(0 as u64), 0);
    assert_eq!(Bits::count_population(1 as u64), 1);
    assert_eq!(Bits::count_population(0x8000000000000001 as u64), 2);
    assert_eq!(Bits::count_population(0x11111111 as u64), 8);
    assert_eq!(Bits::count_population(0xF0F0F0F0 as u64), 16);
    assert_eq!(Bits::count_population(0xFFF0F0FF as u64), 24);
    assert_eq!(Bits::count_population(0xFFFFFFFF as u64), 32);
    assert_eq!(Bits::count_population(0x1111111111111111 as u64), 16);
    assert_eq!(Bits::count_population(0xF0F0F0F0F0F0F0F0 as u64), 32);
    assert_eq!(Bits::count_population(0xFFF0F0FFFFF0F0FF as u64), 48);
    assert_eq!(Bits::count_population(0xFFFFFFFFFFFFFFFF as u64), 64);
  }

  #[test]
  fn test_bits_count_leading_zeros_u16() {
    assert_eq!(Bits::count_leading_zeros(0 as u16), 16);
    assert_eq!(Bits::count_leading_zeros(1 as u16), 15);
    let mut shift: u16;
    for i in 0..16 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::count_leading_zeros((1 << shift) as u16),
        (15 - shift) as u32
      );
    }
    assert_eq!(Bits::count_leading_zeros(0x0F0F as u16), 4);
  }

  #[test]
  fn test_bits_count_leading_zeros_u32() {
    assert_eq!(Bits::count_leading_zeros(0 as u32), 32);
    assert_eq!(Bits::count_leading_zeros(1 as u32), 31);
    let mut shift: u32;
    for i in 0..32 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::count_leading_zeros((1 << shift) as u32),
        (31 - shift) as u32
      );
    }
    assert_eq!(Bits::count_leading_zeros(0x0F0F0F0F as u32), 4);
  }

  #[test]
  fn test_bits_count_leading_zeros_u64() {
    assert_eq!(Bits::count_leading_zeros(0 as u64), 64);
    assert_eq!(Bits::count_leading_zeros(1 as u64), 63);
    let mut shift: u64;
    for i in 0..64 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::count_leading_zeros(((1 as u64) << shift) as u64),
        (63 - shift) as u32
      );
    }
    assert_eq!(Bits::count_leading_zeros(0x0F0F0F0F00000000 as u64), 4);
  }

  #[test]
  fn test_bits_count_trailing_zeros_u16() {
    assert_eq!(Bits::count_trailing_zeros(0 as u16), 16);
    assert_eq!(Bits::count_trailing_zeros(0x8000 as u16), 15);
    let mut shift: u16;
    for i in 0..16 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::count_trailing_zeros((1 << shift) as u16),
        shift as u32
      );
    }
    assert_eq!(Bits::count_trailing_zeros(0xF0F0 as u16), 4);
  }

  #[test]
  fn test_bits_count_trailing_zeros_u32() {
    assert_eq!(Bits::count_trailing_zeros(0 as u32), 32);
    assert_eq!(Bits::count_trailing_zeros(0x80000000 as u32), 31);
    let mut shift: u32;
    for i in 0..31 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::count_trailing_zeros((1 << shift) as u32),
        shift as u32
      );
    }
    assert_eq!(Bits::count_trailing_zeros(0xF0F0F0F0 as u32), 4);
  }

  #[test]
  fn test_bits_count_trailing_zeros_u64() {
    assert_eq!(Bits::count_trailing_zeros(0 as u64), 64);
    assert_eq!(Bits::count_trailing_zeros(0x8000000000000000 as u64), 63);
    let mut shift: u64;
    for i in 0..63 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::count_trailing_zeros(((1 as u64) << shift) as u64),
        shift as u32
      );
    }
    assert_eq!(Bits::count_trailing_zeros(0xF0F0F0F0 as u64), 4);
    assert_eq!(Bits::count_trailing_zeros(0xF0F0F0F000000000 as u64), 36);
  }

  #[test]
  fn test_bits_is_power_of_two_u32() {
    assert_eq!(Bits::is_power_of_two(0 as u32), false);
    let mut shift: u32;
    for i in 0..32 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(Bits::is_power_of_two((1 << shift) as u32), true);
      assert_eq!(Bits::is_power_of_two((1 << shift) as u32 + 5), false);
      assert_eq!(Bits::is_power_of_two(!(1 << shift) as u32), false);
    }
    for i in 2..32 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(Bits::is_power_of_two((1 << shift) as u32 - 1), false);
    }
    assert_eq!(Bits::is_power_of_two(0xFFFFFFFF as u32), false);
  }

  #[test]
  fn test_bits_is_power_of_two_u64() {
    assert_eq!(Bits::is_power_of_two(0 as u64), false);
    let mut shift: u64;
    for i in 0..64 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(Bits::is_power_of_two(((1 as u64) << shift) as u64), true);
      assert_eq!(
        Bits::is_power_of_two(((1 as u64) << shift) as u64 + 5),
        false
      );
      assert_eq!(Bits::is_power_of_two(!((1 as u64) << shift) as u64), false);
    }
    for i in 2..64 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::is_power_of_two(((1 as u64) << shift) as u64 - 1),
        false
      );
    }
    assert_eq!(Bits::is_power_of_two(0xFFFFFFFFFFFFFFFF as u64), false);
  }

  #[test]
  fn test_bits_which_power_of_two_u32() {
    let mut shift: u32;
    for i in 0..32 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(Bits::which_power_of_two((1 << shift) as u32), shift);
    }
  }

  #[test]
  fn test_bits_which_power_of_two_u64() {
    let mut shift: u64;
    for i in 0..64 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::which_power_of_two(((1 as u64) << shift) as u64),
        shift as u32
      );
    }
  }

  #[test]
  fn test_bits_round_up_to_power_of_two_u32() {
    let mut shift: u32;
    for i in 0..32 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::round_up_to_power_of_two((1 << shift) as u32),
        1 << shift
      )
    }
    assert_eq!(Bits::round_up_to_power_of_two(0 as u32), 1);
    assert_eq!(Bits::round_up_to_power_of_two(1 as u32), 1);
    assert_eq!(Bits::round_up_to_power_of_two(3 as u32), 4);
    assert_eq!(
      Bits::round_up_to_power_of_two(0x7FFFFFFF as u32),
      0x80000000
    );
  }

  #[test]
  fn test_bits_round_up_to_power_of_two_u64() {
    let mut shift: u64;
    for i in 0..64 {
      shift = i;
      println!("shift = {}", i);
      let value: u64 = (1 as u64) << shift;
      assert_eq!(Bits::round_up_to_power_of_two(value), value)
    }
    assert_eq!(Bits::round_up_to_power_of_two(0 as u64), 1);
    assert_eq!(Bits::round_up_to_power_of_two(1 as u64), 1);
    assert_eq!(Bits::round_up_to_power_of_two(3 as u64), 4);
    assert_eq!(
      Bits::round_up_to_power_of_two(((1 as u64) << 63) - 1),
      ((1 as u64) << 63)
    );
    assert_eq!(
      Bits::round_up_to_power_of_two((1 as u64) << 63),
      ((1 as u64) << 63)
    );
  }

  #[test]
  fn test_bits_round_down_to_power_of_two_u32() {
    let mut shift: u32;
    for i in 0..32 {
      shift = i;
      println!("shift = {}", i);
      assert_eq!(
        Bits::round_down_to_power_of_two(((1 as u32) << shift) as u32),
        ((1 as u32) << shift)
      )
    }
    assert_eq!(Bits::round_down_to_power_of_two(0 as u32), 0);
    assert_eq!(Bits::round_down_to_power_of_two(5 as u32), 4);
    assert_eq!(
      Bits::round_down_to_power_of_two(0x80000001 as u32),
      0x80000000
    );
  }

  #[test]
  fn test_bits_rotate_right_u32() {
    let mut shift: u32;
    for i in 0..32 {
      shift = i;
      println!("shift = {}", shift);
      assert_eq!(Bits::rotate_right(0 as u32, shift), 0)
    }
    assert_eq!(Bits::rotate_right(1 as u32, 0 as u32), 1);
    assert_eq!(Bits::rotate_right(2 as u32, 1 as u32), 1);
    assert_eq!(Bits::rotate_right(1 as u32, 1 as u32), 0x80000000);
  }

  #[test]
  fn test_bits_rotate_right_u64() {
    let mut shift: u64;
    for i in 0..64 {
      shift = i;
      println!("shift = {}", shift);
      assert_eq!(Bits::rotate_right(0 as u64, shift as u32), 0)
    }
    assert_eq!(Bits::rotate_right(1 as u64, 0 as u32), 1);
    assert_eq!(Bits::rotate_right(2 as u64, 1 as u32), 1);
    assert_eq!(Bits::rotate_right(1 as u64, 1 as u32), 0x8000000000000000);
  }

  #[test]
  fn test_bits_unsigned_div_u32() {
    let mut shift: u32;
    for i in 0..51 {
      shift = i;
      println!("shift = {}", shift);
      assert_eq!(Bits::unsigned_div(i, 0), 0);
      for j in i + 1..101 {
        assert_eq!(Bits::unsigned_div(j, j), 1);
        assert_eq!(Bits::unsigned_div(i, j), i / j);
      }
    }
  }

  #[test]
  fn test_bits_unsigned_mod_u32() {
    let mut shift: u32;
    for i in 0..51 {
      shift = i;
      println!("shift = {}", shift);
      assert_eq!(Bits::unsigned_mod(i, 0), 0);
      for j in i + 1..101 {
        assert_eq!(Bits::unsigned_mod(j, j), 0);
        assert_eq!(Bits::unsigned_mod(i, j), i % j);
      }
    }
  }
}
