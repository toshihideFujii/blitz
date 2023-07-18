#![allow(dead_code)]

// This header defines the VitstreamReader class.
// This class can be used to read an arbitrary bitstream,
// regardless of its contents.

struct BitstreamBlockinfo {}
impl BitstreamBlockinfo {
  pub fn new() {}
  pub fn get_block_info() {}
  pub fn get_or_create_block_info() {}
}

struct SimpleBitstreamCursor {}
impl SimpleBitstreamCursor {
  pub fn new() {}
  pub fn can_skip_to_pos() {}
  pub fn at_end_of_stream() {}
  pub fn get_current_bit_no() {}
  pub fn get_bitcode_bytes() {}
  pub fn jump_to_bit() {}
  pub fn get_pointer_to_bit() {}
  pub fn fill_cur_word() {}
  pub fn read() {}
  pub fn read_vbr() {}
  pub fn read_vbr_64() {}
  pub fn skip_to_four_byte_boundary() {}
  pub fn size_in_bytes() {}
  pub fn skip_to_end() {}
  pub fn is_size_plausible() {}
}

struct BitstreamEntry {}

struct BitstreamCursor {}
impl BitstreamCursor {
  pub fn new() {}
  pub fn get_abbrev_id_with() {}
  pub fn advance() {}
  pub fn advance_skipping_sub_blocks() {}
  pub fn read_code() {}
  pub fn read_sub_block_id() {}
  pub fn skip_block() {}
  pub fn enter_sub_block() {}
  pub fn read_block_end() {}
  fn pop_block_scope() {}
  pub fn get_abbrev() {}
  pub fn skip_record() {}
  pub fn read_record() {}
  pub fn read_abbrev_record() {}
  pub fn read_block_info_block() {}
}