// The max length of the converting the case of a single character.
const MAX_MAPPING_SIZE: u32 = 4;

struct Predicate {}

impl Predicate {
  pub fn predicate() {}
  pub fn get() {}
  fn calculate_value() {}
}

struct CacheEntry {}

impl CacheEntry {
  pub fn code_point() {}
  pub fn value() {}
}

const MAX_NON_SURROGATE_CHAR_CODE: u32 = 0xffff;

struct Utf16 {}

impl Utf16 {
  pub fn is_surrogate_pair(lead: i32, trail: i32) -> bool {
    Utf16::is_lead_surrogate(lead) && Utf16::is_trail_surrogate(trail)
  }

  pub fn is_lead_surrogate(code: i32) -> bool {
    (code & 0xfc00) == 0xd800
  }

  pub fn is_trail_surrogate(code: i32) -> bool {
    (code & 0xfc00) == 0xdc00
  }

  pub fn combine_surrogate_pair(lead: u32, trail: u32) -> i32 {
    (0x10000 + ((lead & 0x3ff) << 10) + (trail & 0x3ff)) as i32
  }

  pub fn lead_surrogate(char_code: u32) -> u16 {
    (0xd800 + (((char_code - 0x10000) >> 10) & 0x3ff)) as u16
  }

  pub fn trail_surrogate(char_code: u32) -> u16 {
    (0xdc00 + (char_code & 0x3ff)) as u16
  }

  pub fn has_unpaired_surrogate() {}
}

struct Latin1 {}

impl Latin1 {
  // Convert the charcter to Latin-1 case equivalent if possioble.
  pub fn try_convert_to_latin1(c: u16) -> u16 {
    match c {
      0x39c => return 0xb5, // This are equiavalent characters in unicode.
      0x3bc => return 0xb5,
      0x178 => return 0xff, // This is an uppercase of a Latin-1 character outside of Latin-1.
      _ => c,
    }
  }
}

// The unicode replacement character, used to signal invalid unicode
// sequences (e.g. an orphan surrogate) when converting to a UTF-8 encoding.
const BAD_CHAR: u32 = 0xFFFD;
const BUFFER_EMPTY: u32 = 0x0;
const INCOMPLETE: u32 = 0xFFFFFFFC;
const MAX_ENCODED_SIZE: u32 = 4;
const MAX_ONE_BYTE_CHAR: u32 = 0x7f;
const MAX_TWO_BYTE_CHAR: u32 = 0x7ff;
const MAX_THREE_BYTE_CHAR: u32 = 0xffff;
const MAX_FOUR_BYTE_CHAR: u32 = 0x1fffff;

// A single surrogate is coded as a 3 byte UTF-8 sequence, but two together
// that match are coded as a 4 byte UTF-8 sequence.
const BYTES_SAVED_BY_COMBINING_SURROGATES: u32 = 2;
const SIZE_OF_UNMATCHED_SURROGATE: u32 = 3;

struct Utf8 {}

impl Utf8 {
  pub fn length(c: u32, previous: i32) -> u32 {
    if c <= MAX_ONE_BYTE_CHAR {
      1
    } else if c <= MAX_TWO_BYTE_CHAR {
      2
    } else if c <= MAX_THREE_BYTE_CHAR {
      if Utf16::is_surrogate_pair(previous, c as i32) {
        return SIZE_OF_UNMATCHED_SURROGATE
          - BYTES_SAVED_BY_COMBINING_SURROGATES;
      }
      3
    } else {
      4
    }
  }

  pub fn encode_one_byte(out: &mut str, c: u8) -> u32 {
    let mask: u8 = !(1 << 6);
    if c as u32 <= MAX_ONE_BYTE_CHAR {
      unsafe {
        let bytes = out.as_bytes_mut();
        bytes[0] = c;
      }
      return 1;
    }
    unsafe {
      let bytes = out.as_bytes_mut();
      bytes[0] = 0xc0 | (c >> 6);
      bytes[1] = 0x80 | (c & mask);
    }
    return 2;
  }

  pub fn encode() {}
  pub fn calculate_value() {}
  pub fn value_of() {}
  pub fn value_of_incremental() {}
  pub fn value_of_incremental_finish() {}

  pub fn is_valid_character(c: u32) -> bool {
    c < 0xD800
      || (c >= 0xE000 && c < 0xFDD0)
      || (c > 0xFDEF
        && c <= 0x10FFFF
        && (c & 0xFFFE) != 0xFFFE
        && c != BAD_CHAR)
  }

  pub fn validate_encoding() {}
}

pub fn is_line_terminator(c: u32) -> bool {
  c == 0x000A || c == 0x000D || c == 0x2028 || c == 0x2029
}

pub fn is_string_literal_line_terminator(c: u32) -> bool {
  c == 0x000A || c == 0x000D
}
