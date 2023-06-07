#![allow(dead_code)]

use crate::adt::string_ref::StringRef;
use super::{opt_specifier::OptSpecifier, option::{OptionClass, Option_}};

pub struct OptTableInfo {
  pub prefixes: Vec<String>,
  pub name: StringRef,
  pub help_text: String,
  pub meta_var: String,
  pub id:u32,
  pub kind: OptionClass,
  pub param: u32,
  pub flags: u32,
  pub group_id: OptSpecifier,
  pub alias_id: OptSpecifier,
  pub alias_args: String,
  pub values: String
}

// Provide access to the Option into table.
pub struct OptTable {
  option_infos: Vec<OptTableInfo>,
  ignore_case: bool,
  grouped_short_options: bool,
  env_var: String,
  input_option_id: u32,
  unknown_option_id: u32,
  first_searchable_index: u32,
  prefix_chars: String
}

impl OptTable {
  pub fn new() {}
  pub fn get_prefixs_union() {}
  pub fn parse_one_arg_grouped() {}
  pub fn build_prefix_chars() {}

  // Return the total number of option classes.
  pub fn get_num_options(&self) -> usize {
    self.option_infos.len()
  }

  // Get the given Opt's Option_ instance, lazily creating it if necessary.
  pub fn get_option(&self, opt: OptSpecifier) -> Option_ {
    let id = opt.get_id();
    if id == 0 {
      return Option_::new(None, None);
    }
    debug_assert!(id - 1 < self.get_num_options() as u32, "Invalid id.");
    Option_::new(/*self.get_info(opt)*/ None, /*Some(*self)*/ None)
  }

  // Lookup the name of the given option.
  pub fn get_option_name(&self, id: OptSpecifier) -> &StringRef {
    &self.get_info(id).unwrap().name
  }

  // Get the kind of the given option.
  pub fn get_option_kind(&self, id: OptSpecifier) -> OptionClass {
    self.get_info(id).unwrap().kind.clone()
  }

  // Get the group id for the given option.
  pub fn get_option_group_id(&self, id: OptSpecifier) -> &OptSpecifier {
    &self.get_info(id).unwrap().group_id
  }

  // Get the help text to use to describe this option.
  pub fn get_option_help_text(&self, id:OptSpecifier) -> String {
    self.get_info(id).unwrap().help_text.clone()
  }

  // Get the meta-variable name to use when describing this options values
  // in the help text.
  pub fn get_option_meta_var(&self, id: OptSpecifier) -> String {
    self.get_info(id).unwrap().meta_var.clone()
  }

  // Specify the environment variable where initial options should be read.
  pub fn set_initial_options_from_environment(&mut self, env_var: String) {
    self.env_var = env_var;
  }

  // Support grouped short options.
  pub fn set_grouped_short_options(&mut self, value: bool) {
    self.grouped_short_options = value;
  }

  pub fn suggest_value_completions() {}
  pub fn find_by_prefix() {}
  pub fn find_nearest() {}
  pub fn find_exact() {}
  pub fn parse_one_arg() {}
  pub fn parse_args() {}
  pub fn print_help() {}

  fn get_info(&self, opt: OptSpecifier) -> Option<&OptTableInfo> {
    let id = opt.get_id() as usize;
    debug_assert!(id > 0 && id - 1 < self.get_num_options(), "Invalid option id.");
    self.option_infos.get(id - 1)
  }
}

struct GenericOptTable {}

struct PrecomputedOptTable {}