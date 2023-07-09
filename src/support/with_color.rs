#![allow(dead_code)]

fn get_color_category() {}

enum HighlightColor {
  Address,
  String,
  Tag,
  Attribute,
  Enumerator,
  Macro,
  Error,
  Warning,
  Note,
  Remark
}

enum ColorMode {
  Auto,
  Enable,
  Disable
}

struct WithColor {}
impl WithColor {
  pub fn new() {}
  pub fn get() {}
  pub fn error() {}
  pub fn warning() {}
  pub fn note() {}
  pub fn remark() {}
  pub fn colors_enabled() {}
  pub fn change_color() {}
  pub fn reset_color() {}
  pub fn default_error_handler() {}
  pub fn default_warning_handler() {}
  pub fn default_auto_detect_function() {}
  pub fn set_auto_detect_function() {}
}