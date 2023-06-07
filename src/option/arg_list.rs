#![allow(dead_code)]

use super::{arg::Arg, opt_specifier::OptSpecifier};

// Ordered collection of driver arguments.
// The ArgList class manages a list of Arg instances as well as
// auxiliary data and convenience methods to allow tools to quickly
// check for the presence of Arg instances for a particular Option
// and to iterate over groups of arguments.
struct ArgList {
  args: Vec<Arg>
}

impl ArgList {
  pub fn new() {}
  pub fn to_opt_specifier(s: OptSpecifier) -> OptSpecifier {
    return s;
  }

  pub fn append() {}

  pub fn get_args(&self) -> &Vec<Arg> {
    &self.args
  }

  pub fn size(&self) -> usize {
    self.args.len()
  }

  pub fn filtered() {}
  pub fn filtered_reverse() {}
  pub fn erase_arg() {}
  pub fn has_arg_no_claim() {}
  pub fn has_arg() {}
  pub fn has_multiple_args() {}

  // Return the last argumentmatching id, or null.
  pub fn get_last_arg(&self) -> Option<Arg> {
    None
  }

  pub fn get_last_arg_no_claim() {}
  pub fn get_arg_string() {}
  pub fn get_num_imput_arg_strings() {}
  pub fn get_last_arg_value() {}
  pub fn get_all_arg_values() {}

  // Given an option pos and its negative form neg, return true if the
  // option is present, false it the negation is present, and default if
  // neither option is given.
  pub fn has_flag(&self, _pos: OptSpecifier, _neg: OptSpecifier, default: bool) -> bool {
    default
  }

  pub fn add_opt_in_flag() {}
  pub fn add_opt_aut_flag() {}
  pub fn add_last_arg() {}
  pub fn add_all_args_except() {}
  pub fn add_all_args() {}
  pub fn add_all_arg_values() {}
  pub fn add_all_args_translated() {}
  pub fn claim_all_args() {}
  pub fn make_arg_string_ref() {}
  pub fn make_arg_string() {}
  pub fn get_or_make_joined_arg_string() {}
  pub fn print() {}
  pub fn dump() {}
}