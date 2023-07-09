#![allow(dead_code)]

// This header defines interfaces to write Blitz bitcode
// files/streams.

struct BitcodeWriter {}
impl BitcodeWriter {
  pub fn new() {}
  pub fn write_symtab() {}
  pub fn write_strtab() {}
  pub fn copy_strtab() {}
  pub fn write_module() {}
  pub fn write_thin_link_bitcode() {}
  pub fn write_index() {}
}

pub fn write_thin_link_bitcode_to_file() {}

pub fn write_index_to_file() {}

pub fn embed_bitcode_in_module() {}