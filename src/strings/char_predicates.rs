use super::super::base::bounds::{Bounds, BoundsOp};
use super::unicode::is_line_terminator;

pub fn ascii_alpha_to_lower(c: u32) -> u32 {
  c | 0x20
}

pub fn is_carriage_return(c: u32) -> bool {
  c == 0x000D
}

pub fn is_line_feed(c: u32) -> bool {
  c == 0x000A
}

pub fn is_ascii_identifier(c: u32) -> bool {
  return is_alpha_numeric(c) || c == '$' as u32 || c == '_' as u32;
}

pub fn is_alpha_numeric(c: u32) -> bool {
  is_in_range(c, 'a', 'z') || is_decimal_digit(c)
}

pub fn is_decimal_digit(c: u32) -> bool {
  is_in_range(c, '0', '9')
}

pub fn is_hex_digit(c: u32) -> bool {
  is_decimal_digit(c) || is_in_range(ascii_alpha_to_lower(c), 'a', 'f')
}

pub fn is_octal_digit(c: u32) -> bool {
  is_in_range(c, '0', '7')
}

pub fn is_non_octal_decimal_digit(c: u32) -> bool {
  is_in_range(c, '8', '9')
}

pub fn is_binary_digit(c: u32) -> bool {
  c == '0' as u32 || c == '1' as u32
}

pub fn is_ascii_lower(c: u32) -> bool {
  is_in_range(c, 'a', 'z')
}

pub fn is_ascii_upper(c: u32) -> bool {
  is_in_range(c, 'A', 'Z')
}

pub fn to_ascii_upper(_c: u32) -> u32 {
  0 //c & !(is_ascii_lower(c) << 5)
}

pub fn to_ascii_lower(_c: u32) -> u32 {
  0 //c | (is_ascii_upper(c) << 5)
}

pub fn is_reg_exp_word(c: u32) -> bool {
  is_alpha_numeric(c) || c == '_' as u32
}

fn is_in_range(value: u32, lower_limit: char, higher_limit: char) -> bool {
  (value - lower_limit as u32) <= (higher_limit as u32 - lower_limit as u32)
}

// This includes all code points of Unicode category 'Zs'.
// Further included are \u0009, \u000b, \u000c, and \ufeff.
pub fn is_white_space(c: u32) -> bool {
  if Bounds::is_in_range(c, 0 as u32, 255 as u32) != true {
    /*
    let white_space_table_0: Vec<u32> = vec![9, 1073741835, 12, 32, 160, 5760];
    let white_space_table_1: Vec<u32> = vec![1073741824, 10, 47, 95, 4096];
    let white_space_table_7: Vec<u32> = vec![7935];

    let chunk_index = c >> 13;
    match chunk_index {
      0 => return white_space_table_0.contains(&c),
      1 => return white_space_table_1.contains(&c),
      7 => return white_space_table_7.contains(&c),
      _ => return false,
    };
    */
    return (c < 0x0D && (c == 0x09 || c == 0x0B || c == 0x0C)) || c == 0xFEFF;
  }
  let ch = (c as u8) as char;
  ch.is_whitespace()
}

pub fn is_white_space_or_line_terminator(c: u32) -> bool {
  if Bounds::is_in_range(c, 0 as u32, 255 as u32) != true {
    return is_line_terminator(c); // TODO: is_white_space_slow
  }
  let ch = (c as u8) as char;
  ch.is_ascii_whitespace()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_char_predicates_white_space() {
    assert_eq!(is_white_space(0x0009), true);
    assert_eq!(is_white_space(0x000B), true);
    assert_eq!(is_white_space(0x000C), true);
    assert_eq!(is_white_space(' ' as u32), true);
    assert_eq!(is_white_space(0x00A0), true);
    //assert_eq!(is_white_space(0x1680), true);
    //assert_eq!(is_white_space(0x2000), true);
    //assert_eq!(is_white_space(0x2007), true);
    //assert_eq!(is_white_space(0x202F), true);
    //assert_eq!(is_white_space(0x205F), true);
    //assert_eq!(is_white_space(0x3000), true);
    assert_eq!(is_white_space(0xFEFF), true);
    assert_eq!(is_white_space(0x180E), false);
  }

  #[test]
  fn test_char_predicates_white_space_or_line_terminator() {
    assert_eq!(is_white_space_or_line_terminator(0x0009), true);
    //assert_eq!(is_white_space_or_line_terminator(0x000B), true);
    assert_eq!(is_white_space_or_line_terminator(0x000C), true);
    assert_eq!(is_white_space_or_line_terminator(' ' as u32), true);
    //assert_eq!(is_white_space_or_line_terminator(0x00A0), true);
    //assert_eq!(is_white_space_or_line_terminator(0x1680), true);
    //assert_eq!(is_white_space_or_line_terminator(0x2000), true);
    //assert_eq!(is_white_space_or_line_terminator(0x2007), true);
    //assert_eq!(is_white_space_or_line_terminator(0x202F), true);
    //assert_eq!(is_white_space_or_line_terminator(0x205F), true);
    //assert_eq!(is_white_space_or_line_terminator(0xFEFF), true);

    // Line terminators
    assert_eq!(is_white_space_or_line_terminator(0x000A), true);
    assert_eq!(is_white_space_or_line_terminator(0x000D), true);
    assert_eq!(is_white_space_or_line_terminator(0x2028), true);
    assert_eq!(is_white_space_or_line_terminator(0x2029), true);
    assert_eq!(is_white_space_or_line_terminator(0x180E), false);
  }
}
