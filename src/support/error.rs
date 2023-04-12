#![allow(dead_code)]

// This file defines an API used to report recoverable errors.

struct ErrorInfoBase {}
impl ErrorInfoBase {
  pub fn new() {}
  pub fn log() {}
  pub fn message() {}
  pub fn convert_to_error_code() {}
  pub fn class_id() {}
  pub fn dynamic_class_id() {}
  pub fn is_a() {}
}

pub struct Error {}
impl Error {
  pub fn new() -> Self {
    Error {  }
  }
  pub fn handle_errors() {}
  pub fn wrap() {}
  pub fn success() {}
  pub fn is_a() {}
  pub fn dynamic_class_id() {}
}

struct ErrorSuccess {}
impl ErrorSuccess {
  pub fn new() {}
}

struct ErrorList {}
impl ErrorList {}

struct ECError {}

struct StringError {}

struct FileError {}

struct ExitOnError {}