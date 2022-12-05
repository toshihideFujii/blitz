#![allow(dead_code)]

/*
This file contains some functions that are useful
when dealing with strings.
*/

pub fn hex_digit() {}

pub fn to_string_ref_array() {}

pub fn to_string_ref() {}

pub fn array_ref_from_string_ref() {}

pub fn hex_digit_value() {}

pub fn is_digit() {}

pub fn is_hex_digit() {}

pub fn is_alpha() {}

pub fn is_alnum() {}

pub fn is_ascii() {}

// Checks whether character c is printable.
pub fn is_print(c: char) -> bool {
  0x20 <= (c as u8) && (c as u8) <= 0x7E
}

// Checks whether character c is whitespace in the "C" locale.
pub fn is_space(c: char) -> bool {
  c.is_whitespace()
}

pub fn to_lower() {}

pub fn to_upper() {}

pub fn uto_hex_str() {}

pub fn to_hex() {}

pub fn try_get_hex_from_nibbles() {}

pub fn hex_from_nibbles() {}

pub fn try_get_from_hex() {}

pub fn from_hex() {}

pub fn to_integer() {}

pub fn to_float() {}

pub fn uto_str() {}

pub fn ito_str() {}

pub fn to_string() {}

pub fn str_in_str_no_case() {}

pub fn get_token() {}

pub fn split_string() {}

pub fn get_orfinal_suffix() {}

pub fn print_escaped_string() {}

pub fn print_html_escaped() {}

pub fn print_lower_case() {}

pub fn convert_to_snake_from_camel_case() {}

pub fn convert_to_camel_from_snake_case() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_print() {
    assert_eq!(is_print('\0'), false);
    assert_eq!(is_print('\t'), false);
    assert_eq!(is_print('0'), true);
    assert_eq!(is_print('a'), true);
    assert_eq!(is_print('A'), true);
    assert_eq!(is_print(' '), true);
    assert_eq!(is_print('~'), true);
    assert_eq!(is_print('?'), true);
  }

  fn test_is_space() {
    assert_eq!(is_space(' '), true);
    assert_eq!(is_space('\t'), true);
    assert_eq!(is_space('\n'), true);
    assert_eq!(is_space('\r'), true);
    assert_eq!(is_space('\0'), true);
    assert_eq!(is_space('_'), true);
    // TODO: \f, \v
  }
}