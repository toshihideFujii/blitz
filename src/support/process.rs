
#![allow(dead_code)]

/*
Prpvodes a library for accessing information about this
process and other processes on the operating systems.
Also provides means of spawning subprocess for commands.
*/

use std::process;
use std::env;
use page_size;
use rand;
use crate::adt::string_ref::StringRef;

// A collection of legacy interfaces for querying information
// about the current executing process.
struct Process {
  pid_: u32
}

impl Process {
  // Get the process's identifier.
  pub fn get_process_id() -> u32 {
    process::id()
  }

  // Get the process's page size.
  pub fn get_page_size() -> usize {
    page_size::get()
  }

  // Get the process's estimated page size.
  pub fn get_page_size_estimate() -> usize {
    let page_size = Process::get_page_size();
    if page_size != 0 {
      return page_size;
    } else {
      return 4096;
    }
  }

  // Return process memory usage.
  // This static function will return the total amount of memory
  // allocated by the proess.
  pub fn get_malloc_usage() {}

  pub fn get_time_usage() {}

  pub fn prevent_core_files() {}

  pub fn are_core_files_prevented() {}

  // This function returns the environment variable name's
  // value as a UTF-8 string.
  pub fn get_env(name: StringRef) -> String {
    match env::var(name.data().as_str()) {
      Ok(val) => return val,
      Err(e) => return e.to_string(),
    }
  }

  pub fn find_in_env_path() {}

  pub fn fixup_standard_file_descriptors() {}

  pub fn safely_close_file_descriptor() {}

  pub fn standard_in_is_user_unput() {}

  pub fn standard_out_is_displayed() {}

  pub fn standard_err_is_displayed() {}

  pub fn file_descriptor_is_displayed() {}

  pub fn file_descriptor_has_colors() {}

  pub fn standard_out_columns() {}

  pub fn standard_err_columns() {}

  pub fn standard_out_has_colors() {}

  pub fn standard_err_has_colors() {}

  pub fn use_ansi_escape_cpdes() {}

  pub fn color_needs_flush() {}

  pub fn output_color() {}

  pub fn output_bold() {}

  pub fn output_reverse() {}

  pub fn reset_color() {}

  // Get the result of a process wide random number generator.
  pub fn get_random_number() -> u32 {
    rand::random::<u32>()
  }

  pub fn exit(code: i32) {
    process::exit(code)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_process_id() {
    let pid = Process::get_process_id();
    assert_eq!(pid, process::id())
  }

  #[test]
  fn test_get_random_number() {
    let r1 = Process::get_random_number();
    let r2 = Process::get_random_number();
    assert_ne!((r1 | r2), 0)
  }
}

