#![allow(dead_code)]

enum BufferKind {
  Malloc,
  MMap
}

// This interface provides simple read-only access to a block of
// memory, and provides simple methods for reading files and standard
// input into a memory buffer.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryBuffer {}

impl MemoryBuffer {
  pub fn new() {}

  pub fn get_buffer_start() {}
  pub fn get_buffer_end() {}
  pub fn get_buffer_size() {}
  pub fn get_buffer() {}

  pub fn get_buffer_identifier() {}
  pub fn dont_need_if_map() {}

  pub fn get_file() {}
  pub fn get_file_as_stream() {}
  pub fn get_open_file_slice() {}

  pub fn get_open_file() {}
  pub fn get_mem_buffer() {}

  pub fn get_file_or_stdin() {}
  pub fn get_file_slice() {}

  pub fn get_buffer_kind() {}
  pub fn get_mem_buffer_ref() {}
}

struct WritableMemoryBuffer {}
impl WritableMemoryBuffer {}

struct WriteThroughMemoryBuffer {}
impl WriteThroughMemoryBuffer {}