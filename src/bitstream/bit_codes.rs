#![allow(dead_code)]

// This header defines bitstream enum values.

enum Encoding {
  Fixed,
  VBR,
  Array,
  Char6,
  Blob
}

struct BitCodeAbbrevOp {}
impl BitCodeAbbrevOp {
  pub fn new() {}
  pub fn is_valid_encoding() {}
  pub fn is_literal() {}
  pub fn is_encoding() {}
  pub fn get_literal_value() {}
  pub fn get_encoding() {}
  pub fn get_encoding_data() {}
  pub fn has_encoding_data() {}
  pub fn is_char6() {}
  pub fn decode_char6() {}
}

struct BitCodeAbbrev {}
impl BitCodeAbbrev {
  pub fn new() {}
  pub fn get_num_operand_infos() {}
  pub fn get_operand_info() {}
  pub fn add() {}
}