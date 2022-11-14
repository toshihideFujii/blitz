#![allow(dead_code)]

/*
Utility for creating a in-memory buffer that will be
written to a file.
*/

enum FileType {
  Executable,
  NoMmap
}

struct FileOutputBuffer {
  final_path: String
}

impl FileOutputBuffer {
  pub fn create() {}

  pub fn get_buffer_start() {}

  pub fn get_buffer_end() {}

  pub fn get_buffer_size() {}

  pub fn get_path() {}

  pub fn commit() {}

  pub fn discaard() {}
}