#![allow(dead_code)]

// This header defines interfaces to read Blitz bitcode
// file/streams.

struct ParserCallbacks {}

struct BitcodeLTOInfo {}

struct BitcodeModule {}
impl BitcodeModule {
  pub fn new() {}
  pub fn get_bitcode_file_contents() {}
  pub fn get_module_impl() {}
  pub fn get_buffer() {}
  pub fn get_strtab() {}
  pub fn get_module_identifier() {}
  pub fn get_lazy_module() {}
  pub fn parse_module() {}
  pub fn get_lto_info() {}
  pub fn get_summary() {}
  pub fn read_summary() {}
}

pub fn get_bitcode_file_contents() {}

pub fn get_bitcode_module_list() {}

pub fn get_lazy_bitcode_module() {}

pub fn get_owning_lazy_bitcode_module() {}

pub fn get_bitcode_target_triple() {}

pub fn is_bitcode_containing_objc_category() {}

pub fn get_bitcode_producer_string() {}

pub fn parse_bitcode_file() {}

pub fn get_bitcode_lto_info() {}

pub fn get_module_summary_index() {}

pub fn read_module_summary_index() {}

pub fn get_module_summary_index_for_file() {}

pub fn is_bitcode_wrapper() {}

pub fn is_raw_bitcode() {}

pub fn is_bitcode() {}

pub fn skip_bitcode_wrapper_header() {}

pub fn read_wide_apint() {}