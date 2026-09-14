#![allow(dead_code)]

use std::{collections::HashMap, fs::File, io::Read, sync::OnceLock};

use crate::{blitz_data::DebugOptions, parse_flags_from_env::{
  parse_flags,
  //parse_flags_from_env_and_die_if_unknown
}};

pub static mut FLAG_VALUES: OnceLock<DebugOptions> = OnceLock::new();

static FLAG_OBJECTS: Vec<Flag> = Vec::new();

// Maps pass -> remaining fuel.
// All threads start off using this global fuel pool, but ResetThreadLocalFuel()
// switches them to a thread-local fuel pool.
static mut FUEL_EVER_CONSUMED: OnceLock<HashMap<String, bool>> = OnceLock::new();

// Maps pass -> remaining fuel.
// All threads start off using this global fuel pool, but ResetThreadLocalFuel()
// switches them to a thread-local fuel pool.
pub static mut GLOBAL_FUEL: OnceLock<HashMap<String, i64>> = OnceLock::new();

pub struct Flag {
  name: String,
  //bool_hook: Option<&'static dyn Fn(&mut DebugOptions, bool)>,
  default_value_for_display: bool,
  usage_text: String
}

impl Flag {
  pub fn new_from_bool_hook(
    name: String,
    //bool_hook: &'static dyn Fn(&mut DebugOptions, bool),
    default_value_for_display: bool,
    usage_text: String) -> Self
  {
    Flag {
      name: name,
      //bool_hook: Some(bool_hook),
      default_value_for_display: default_value_for_display,
      usage_text: usage_text
    }
  }
}

// Allocates flag_values and flag_objects; this function must not be called more
// than once - its call done via call_once.
fn allocate_flags(mut _defaults: Option<DebugOptions>) {
  /*
  if defaults.is_none() {
    defaults = Some(DebugOptions::default());
  }
  unsafe {
    FLAG_VALUES = defaults.unwrap().into();
    make_debug_options_flags(
      &FLAG_OBJECTS, &FLAG_VALUES.get().unwrap());
  
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(),
      FLAG_VALUES.get_mut().unwrap(),
      false);
  }
  */
}

// Construct flags which write to the debug_options proto when parsed. Existing
// contents of debug_options is used as the default. Can be called multiple
// times.
pub fn make_debug_options_flags(
  _flag_list: &Vec<Flag>, _debug_options: &DebugOptions)
{
  //unimplemented!()
}

// Appends flag definitions for debug options to flag_list. Existing
// contents of debug_options is used as the default. If debug_options is null,
// uses global defaults. Modifies global state on first call.
pub fn append_debug_options_flags(
  _flag_list: &Vec<i64>, _debug_options: &Option<DebugOptions>)
{
  unimplemented!()
}

// Parses the debug option flags from BLITZ_FLAGS environment variable. The
// global variable containing the debug options returned from
// 'GetDebugOptionsFromFlags' is mutated by setting fields explicitly specified
// in the environment variable. If `reset_envvar` is true, then the environment
// variable is read again, otherwise the previously read value is used.
pub fn parse_debug_option_flags_from_env(_reset_envvar: bool) {
  /*
  allocate_flags(None); // TODO: call once
  unsafe {
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(),
      FLAG_VALUES.get_mut().unwrap(),
      reset_envvar);
  }
  */
}

// Parse the debug options from debug_options file. Given a string containing
// the textual form of a DebugOptions protobuf, parses it. The global variable
// containing the debug options returned from 'GetDebugOptionsFromFlags' is
// mutated by setting fields explicitly specified in the file.
pub fn parse_flags_from_debug_options_file(filename: &String) -> bool {
  println!("Parsing flags from file: {:?}", filename);

  // Read the file content
  let file = File::open(filename);
  if file.is_err() {
    println!("Failed to open file {:?}", filename);
    return false;
  }
  let mut file_content = String::new();
  let result =
    file.unwrap().read_to_string(&mut file_content);
  if result.is_err() {
    return false;
  }
  let mut new_debug_options = DebugOptions::default();
  let flags: Vec<&str> = file_content.split('\n').collect();
  let mut key_values = vec![];
  for flag in flags {
    key_values.push(flag.to_string());
  }
  parse_flags(0, &key_values, &mut new_debug_options);

  unsafe {
    FLAG_VALUES = new_debug_options.into();
  }
  true
}

// Fetches a DebugOptions proto message from flags provided to the program.
// Flags must be registered with the flags parser using AppendDebugOptionsFlags
// first.
pub fn get_debug_options_from_flags() -> &'static OnceLock<DebugOptions> {
  //unsafe { addr_of!(FLAG_VALUES) }
  unimplemented!()
}

// Gets a DebugOptions proto that reflects the defaults as if no flags were set.
pub fn default_debug_options_ignoring_flags(
  flag_list: &mut Vec<Flag>, debug_options: &DebugOptions)
{
  flag_list.push(Flag::new_from_bool_hook(
    "blitz_hlo_evaluator_use_fast_path".to_string(),
    //&DebugOptions::set_blitz_hlo_evaluator_use_fast_path,
    debug_options.blitz_hlo_evaluator_use_fasst_path(),
    "Enable fast evaluation of dots in the HloEvaluator".to_string()));

  flag_list.push(Flag::new_from_bool_hook(
    "blitz_cpu_enable_fast_math".to_string(),
    //&DebugOptions::set_blitz_cpu_enable_fast_math,
    debug_options.blitz_cpu_enable_fast_math(),
    "Enable unsafe fast-math optimizations in the CPU compiler; this may 
    produce faster code at the expense of some accuracy.".to_string()));

  flag_list.push(Flag::new_from_bool_hook(
    "blitz_cpu_enable_platform_dependent_math".to_string(),
    //&DebugOptions::set_blitz_cpu_enable_platform_dependent_math,
    debug_options.blitz_cpu_enable_platform_dependent_math(),
    "Enable platform dependent math in the CPU compiler; this may 
    produce faster code at the expense of consistent results across CPUs.".to_string()));
}

// Checks whether the pass fuel was explicitly set.
pub fn pass_fuel_is_set(_pass: &String) -> bool {
  //unsafe {
    //debug_assert!(GLOBAL_FUEL.get().is_some());
    //GLOBAL_FUEL.get().unwrap().get(pass).is_some()
  //}
  unimplemented!()
}

// Consumes a unit of "compiler fuel" for the given pass, and returns false if
// we're out of fuel for that pass.
//
// Compiler fuel is a debugging tool useful for bisecting compiler passes.  Each
// time a pass "does something", it consumes a unit of fuel, and once it's out
// of fuel, it stops doing any transformations.  This way if you suspect a pass
// has a bug, you can bisect the amount of fuel it gets and find exactly which
// change causes the problem.
//
// The very first time a pass runs out of fuel, `just_ran_out` is set to true.
// This lets you take action (e.g. log a message).  But see also the convenience
// overload below.
//
// By default all passes have infinite fuel.  You can restrict how much fuel a
// pass has by specifying XLA_FLAGS=--xla_fuel=PASS1=NUM1,PASS2=NUM2,...
//
// If a user specifies --xla_fuel=PASS=NUM but ConsumeFuel(PASS) is not called
// before the program exits, we'll print a warning.
//
// We recommend as a convention you use a pass's name for the `pass` argument,
// but any value is accepted.
pub fn consume_fuel(_pass: &String, _just_ran_out: &mut Option<bool>) -> bool {
  /*
  //allocate_flags(None); // TODO call_once
  if just_ran_out.is_some() {
    *just_ran_out.as_mut().unwrap() = false;
  }

  #[allow(unused_assignments)]
  let mut remaining_fuel: Option<&mut i64> = None;
  #[allow(unused_assignments)]
  let mut remaining = 0;
  unsafe {
    debug_assert!(GLOBAL_FUEL.get().is_some());
    if GLOBAL_FUEL.get().unwrap().is_empty() {
      return true;
    }
    remaining_fuel = GLOBAL_FUEL.get_mut().unwrap().get_mut(pass);
    if remaining_fuel.is_none() {
      return true;
    }
    if FUEL_EVER_CONSUMED.get().is_none() {
      FUEL_EVER_CONSUMED = HashMap::new().into(); // TODO
    }
    FUEL_EVER_CONSUMED.get_mut().unwrap().insert(pass.clone(), true);
    remaining = **remaining_fuel.as_ref().unwrap();
    *remaining_fuel.unwrap() -= 1;
  }

  if just_ran_out.is_some() {
    *just_ran_out.as_mut().unwrap() = remaining == 0;
  }
  remaining > 0
  */
  unimplemented!()
}