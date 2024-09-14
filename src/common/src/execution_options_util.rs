#![allow(dead_code)]

use crate::{
  blitz_data::ExecutionOptions,
  debug_options_flags::get_debug_options_from_flags
};

// Create a default ExecutionOptions proto; this proto has its debug options
// populated to the default values taken from flags.
pub fn create_default_execution_options() -> ExecutionOptions {
  let mut execution_options = ExecutionOptions::new();
  execution_options.set_debug_options(get_debug_options_from_flags());
  execution_options
}