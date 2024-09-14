#![allow(dead_code)]

use crate::blitz_data::DebugOptions;

// Construct flags which write to the debug_options proto when parsed. Existing
// contents of debug_options is used as the default. Can be called multiple
// times.
pub fn make_debug_options_flags(
  _flag_list: &Vec<i64>, _debug_options: &DebugOptions)
{
  unimplemented!()
}

// Appends flag definitions for debug options to flag_list. Existing
// contents of debug_options is used as the default. If debug_options is null,
// uses global defaults. Modifies global state on first call.
pub fn append_debug_options_flags(
  _flag_list: &Vec<i64>, _debug_options: &Option<DebugOptions>)
{
  unimplemented!()
}

// Fetches a DebugOptions proto message from flags provided to the program.
// Flags must be registered with the flags parser using AppendDebugOptionsFlags
// first.
pub fn get_debug_options_from_flags() -> DebugOptions {
  unimplemented!()
}

// Gets a DebugOptions proto that reflects the defaults as if no flags were set.
pub fn default_debug_options_ignoring_flags() -> DebugOptions {
  let mut opts = DebugOptions::new();

  opts.set_blitz_cpu_use_thunk_runtime(true);
  opts.set_blitz_cpu_parallel_codegen_split_count(32);
  opts.set_blitz_cpu_enable_concurrency_optimized_scheduler(false);
  opts.set_blitz_cpu_prefer_vector_width(256);

  opts.set_blitz_cpu_enable_fast_math(false);
  opts.set_blitz_cpu_fast_math_honor_nans(true);
  opts.set_blitz_cpu_fast_maath_honor_infs(true);
  opts.set_blitz_cpu_fast_math_honor_functions(true);
  opts.set_blitz_cpu_fast_math_honor_division(true);

  opts
}