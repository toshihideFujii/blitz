#![allow(dead_code)]

// This module exports ParseFlagsFromEnvAndDieIfUnknown(), which allows other
// modules to parse flags from an environment variable, or (if the first
// non-whitespace in the variable value is not '-'), a file named by that
// environment variable.
//
// The accepted syntax is that flags arguments are of the form --flag=value or
// (for boolean flags) --flag, and are whitespace separated.  The <value> may be
// one of:
//
//  - <non-whitespace, non-nul not starting with single-quote or double-quote>
//    in which case the effective value is the string itself
//  - <single-quote><characters string not containing nul or
//    single-quote><single_quote> in which case the effective value is the
//    string with the single-quotes removed
//  - <double-quote><character string not containing nul or unescaped
//    double-quote><double_quote> in which case the effective value if the
//    string with the double-quotes removed, and escaped sequences of
//    <backslash><char> replaced by <char>.
//
// Flags values inconsistent with the type of the flag will be rejected by the
// flag parser.
//
// Examples:
//
//  - BLITZ_FLAGS="--foo=bar  --wombat='value with a space'"
//  - BLITZ_FLAGS=/tmp/flagfile
//
// where /tmp/flagfile might contain
//
//  --some_flag="This is a string containing a \" and a '."
//  --another_flag=wombats

use std::{collections::HashMap, env, sync::{
  LazyLock, Mutex,
  //OnceLock
}};

use crate::{
  blitz_data::{
    AutotuneBackend, CommandBufferCmdType, DebugOptions, LibraryFusionType
  },
  debug_options_flags::GLOBAL_FUEL,
  //debug_options_flags::Flag
};


pub struct EnvArgv {
  initialized: bool, // whether the other fields have been set.
  pub argc: i64, // elements used in argv[]
  pub argv: Vec<String>  // flag arguments parsed from environment string.
}

impl EnvArgv {
  pub fn new() -> Self {
    EnvArgv { initialized: false, argc: 0, argv: Vec::new() }
  }

  pub fn append(&mut self, s: String) {
    self.argv.push(s);
    self.argc += 1;
  }

  // Given a string containing flags, parse them into the BLITZ command line flags.
  // The parse is best effort, and gives up on the first syntax error.
  pub fn parse_argv_from_string(&mut self, flag_str: &String) {
    let mut b = Self::find_first_not_of(
      flag_str, &vec!['\t', '\r', '\n'], 0);
    while b != flag_str.len() && flag_str.chars().nth(b) == Some('-') {
      // b is the index of the start of a flag.
      // Set e to the index just past the end of the flag.
      let mut e = b;
      while e != flag_str.len() &&
        flag_str.chars().nth(e).unwrap().is_ascii() &&
        (flag_str.chars().nth(e) == Some('-') ||
         flag_str.chars().nth(e) == Some('_') ||
         flag_str.chars().nth(e).unwrap().is_ascii_alphanumeric())
      {
        e += 1;
      }
      if e != flag_str.len() &&
        flag_str.chars().nth(e) == Some('=') &&
        e + 1 != flag_str.len() &&
        flag_str.chars().nth(e+1) == Some('\\')
      {
        // A flag of the form  --flag="something in double or single quotes"
        //let c = 0;
        e += 1; // point just past '='
        let quote = flag_str.chars().nth(e).unwrap();
        e += 1; // point just past quote
        // Put in value the string with quotes removed.
        let mut value = String::new();
        while flag_str.len() != e && (flag_str.chars().nth(e) != Some(quote)) {
          let mut c = flag_str.chars().nth(e).unwrap();
          if quote == '"' && c == '\\' && flag_str.len() != e + 1 {
            // Handle backslash in double quoted strings.  They are literal in
            // single-quoted strings.
            e += 1;
            c = flag_str.chars().nth(e).unwrap();
          }
          value.push(c);
          e += 1;
        }
        if e != flag_str.len() { // skip final " or '
          e += 1;
        }
        self.append(value);
      } else { // A flag without a quoted value.
        e = Self::find_first_of(
          flag_str, &vec!['\t', '\r', '\n'], e);
        //self.append(); // TODO
      }
      b = Self::find_first_not_of(
      flag_str, &vec!['\t', '\r', '\n'], e);
    }
  }

  // Like s.find_first_of(x, pos), but return s.size() when find_first_of() would
  // return std::string::npos.  This avoids if-statements elsewhere.
  fn find_first_of(s: &String, chars: &Vec<char>, _position: usize) -> usize {
    let mut result = s.len();
    for c in chars {
      let pos = s.find(*c);
      if pos.is_some() {
        if result == s.len() {
          result = pos.unwrap() + 1;
        } else if pos.unwrap() + 1 < result {
          result = pos.unwrap() + 1;
        }
      }
    }
    result
  }

  // Like s.find_first_not_of(x, pos), but return s.size() when
  // find_first_not_of() would return std::string::npos.  This avoids
  // if-statements elsewhere.
  fn find_first_not_of(s: &String, chars: &Vec<char>, _position: usize) -> usize {
    let mut result = s.len();
    for c in chars {
      let pos = s.find(*c);
      if pos.is_some() {
        if result == s.len() {
          result = pos.unwrap() + 1;
        } else if pos.unwrap() + 1 > result {
          result = pos.unwrap() + 1;
        }
      }
    }
    result
  }

  // Call ParseArgvFromString(..., a) on a string derived from the setting of the
  // environment variable `envvar`, or a file it points to.
  pub fn set_argv_from_env(&mut self, envvar: &String) {
    if !self.initialized {
      let dummy_argv = "<argv[0]>".to_string();
      self.append(dummy_argv);

      let env = env::var(envvar);
      if env.is_err() || env.as_ref().unwrap().is_empty() {
        // nothing
      } else if env.as_ref().unwrap().chars().nth(0) == Some('-') {
        //self.parse_argv_from_string(env.as_ref().unwrap());  
        self.append(env.unwrap().clone());
      } else { // assume it's a file name
        // TODO
      }
      self.initialized = true;
    }
  }
}

// The simulated argv[] parsed from the environment, one for each different
// environment variable we've seen.
static ENV_ARGVS: LazyLock<Mutex<HashMap<String, EnvArgv>>> =
  LazyLock::new(|| { Mutex::new(HashMap::new()) });

// Calls tsl::Flags::Parse(argc, argv, flag_list) against any as yet
// unrecognized flags passed in the environment variable `envvar`.
//
// Raises a fatal error if any flags in `envvar` were not recognized, or if flag
// parsing failed.
pub fn parse_flags_from_env_and_die_if_unknown(
  envvar: &String,
  dbg_opts: &mut DebugOptions,
  reset_env_var: bool)
{
  parse_flags_from_env_and_ignore_unknown(envvar, dbg_opts, reset_env_var);
  die_if_env_has_unknown_flags_left(envvar);
}

// Calls tsl::Flags::Parse(argc, argv, flag_list) against any as yet
// unrecognized flags passed in the environment variable `envvar`, and returns
// its return value.
//
// Ignores unknown flags, raises a fatal if flag parsing failed.
pub fn parse_flags_from_env_and_ignore_unknown(
  envvar: &String,
  dbg_opts: &mut DebugOptions,
  reset_env_var: bool)
{
  let mut env_argvs = ENV_ARGVS.lock().unwrap();
  if reset_env_var { env_argvs.remove(envvar); }

  let mut env_argv = EnvArgv::new();
  env_argv.set_argv_from_env(envvar);
  let mut argv = vec![];
  argv.clone_from(&env_argv.argv);
  parse_flags(0, &argv, dbg_opts);
  env_argvs.insert(envvar.clone(), env_argv);
}

// Used only for testing.  Not to be used by clients.
pub fn reset_flags_from_env_for_testing(envvar: &String) {
  env::remove_var(envvar);
  debug_assert!(env::var(envvar).is_err());
}

fn die_if_env_has_unknown_flags_left(_envvar: &String) {
    
}

pub fn parse_flags(_argc: i64, argv: &Vec<String>, dbg_opts: &mut DebugOptions) {
  for key_value in argv {
    if key_value.contains("blitz_cpu_max_isa") {
      let key_value_vec: Vec<&str> = key_value.split('=').collect();
      let value = key_value_vec[1];
      dbg_opts.set_blitz_cpu_max_isa(value.to_string());
    }
    // command line
    if key_value.contains("--blitz_fuel") {
      unsafe {
        if GLOBAL_FUEL.get().is_none() { GLOBAL_FUEL = HashMap::new().into(); }
      }
      let key_values_str = key_value.strip_prefix("--blitz_fuel=").unwrap();
      let key_values: Vec<&str> = key_values_str.split(",").collect();
      
      for k_v_str in key_values {
        let k_v: Vec<&str> = k_v_str.split('=').collect();
        let key = k_v[0].to_string();
        let value = k_v[1].to_string().parse::<i64>().unwrap();
        unsafe {
          GLOBAL_FUEL.get_mut().unwrap().insert(key, value);
        }
      }
    }
    // command line
    if key_value.contains("--blitz_dump_to") {
      let values: Vec<&str> =
        key_value.strip_prefix("--blitz_dump_to=").unwrap().split('\n').collect();
      dbg_opts.set_blitz_dump_to(values[0].to_string());
    }
    // config file
    if key_value.contains("blitz_dump_to: ") {
      let values: Vec<&str> =
        key_value.strip_prefix("blitz_dump_to: ").unwrap().split('\n').collect();
      dbg_opts.set_blitz_dump_to(values[0].to_string());
    }
    // config file
    if key_value.contains("blitz_gpu_target_config_filename: ") {
      let values: Vec<&str> = key_value
        .strip_prefix("blitz_gpu_target_config_filename: ").unwrap().split('\n').collect();
      dbg_opts.set_blitz_gpu_target_config_filename(values[0].to_string());
    }
    // command line
    if key_value.contains("--blitz_gpu_enable_command_buffer") {
      let values: Vec<&str> = key_value
        .strip_prefix("--blitz_gpu_enable_command_buffer=").unwrap().split(",").collect();
      // One value only (or no value)
      if values.is_empty() {
        let value = key_value.strip_prefix("--blitz_gpu_enable_command_buffer=").unwrap();
        if value == "+fusion" {
          dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Fusion);
        } else if value == "-fusion" {
          dbg_opts.remove_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Fusion);
        } else if value == "+cublas" {
          dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublas);  
        } else if value == "-cublas" {
          dbg_opts.remove_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublas);
        } else if value == "+cublaslt" {
          dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublaslt);
        } else if value == "-cublaslt" {
          dbg_opts.remove_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublaslt);
        } else if value == "+customcall" {
          dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::CustomCall);
        } else if value == "-customcall" {
          dbg_opts.remove_blitz_gpu_enable_command_buffer(CommandBufferCmdType::CustomCall);
        } else { // no value
          dbg_opts.clear_blitz_gpu_enable_command_buffer();
        }
      }
      // Some values
      // Check reset default values or not.
      let sample = values[0];
      if !sample.starts_with("-") && !sample.starts_with("+") {
        dbg_opts.clear_blitz_gpu_enable_command_buffer();
        for value in &values {
          if *value == "fusion" {
            dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Fusion);
          } else if *value == "cublas" {
            dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublas);  
          } else if *value == "cublaslt" {
            dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublaslt);
          } else if *value == "custom_call" {
            dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::CustomCall);
          }
        }
      } else {
        for value in &values {
          if *value == "-fusion" {
            dbg_opts.remove_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Fusion);
          } else if *value == "+cublas" {
            dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublas);  
          } else if *value == "-cublas" {
            dbg_opts.remove_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublas);
          } else if *value == "+cublaslt" {
            dbg_opts.add_blitz_gpu_enable_command_buffer(CommandBufferCmdType::Cublaslt);
          }
        }
      }
      // TODO
    }
    // command line
    if key_value.contains("--blitz_cpu_experimental_onednn_fusion_type") {
      let values: Vec<&str> = key_value
        .strip_prefix("--blitz_cpu_experimental_onednn_fusion_type=").unwrap().split(",").collect();
      // One value only (or no value)
      if values.is_empty() {
        let value = key_value.strip_prefix("--blitz_cpu_experimental_onednn_dusion_type=").unwrap();
        if value == "+dot" {
          dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Dot);
        } else if value == "-dot" {
          dbg_opts.remove_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Dot);
        } else if value == "+eltwise" {
          dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Eltwise);
        } else if value == "-eltwise" {
          dbg_opts.remove_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Eltwise);
        } else if value == "+reduce" {
          dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Reduce);
        } else if value == "-reduce" {
          dbg_opts.remove_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Reduce);
        } else if value == "+individual_dot" {
          dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::IndividualDot);
        } else if value == "-individual_dot" {
          dbg_opts.remove_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::IndividualDot);
        } else { // no value
          //dbg_opts.clear_blitz_gpu_enable_command_buffer();
        }
      }
      // Some values
      // Check reset default values or not.
      let sample = values[0];
      if !sample.starts_with("-") && !sample.starts_with("+") {
        dbg_opts.clear_blitz_cpu_experimental_onednn_fusion_type();
        for value in &values {
          if *value == "dot" {
            dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Dot);
          } else if *value == "eltwise" {
            dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Eltwise);
          } else if *value == "reduce" {
            dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Reduce);
          } else if *value == "individual_dot" {
            dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::IndividualDot);
          }
        }
      } else {
        for value in &values {
          if *value == "+dot" {
            dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Dot);
          } else if *value == "-dot" {
            dbg_opts.remove_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Dot);
          } else if *value == "+eltwise" {
            dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Eltwise);
          } else if *value == "-eltwise" {
            dbg_opts.remove_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Eltwise);
          } else if *value == "+reduce" {
            dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Reduce);
          } else if *value == "-reduce" {
            dbg_opts.remove_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::Reduce);
          } else if *value == "+individual_dot" {
            dbg_opts.add_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::IndividualDot);
          } else if *value == "-individual_dot" {
            dbg_opts.remove_blitz_cpu_experimental_onednn_fusion_type(LibraryFusionType::IndividualDot);
          }
        }
      }
    }
    // command line
    if key_value.contains("--blitz_cpu_experimental_xnn_fusion_type") {
      let values: Vec<&str> = key_value
        .strip_prefix("--blitz_cpu_experimental_xnn_fusion_type=").unwrap().split(",").collect();
      // One value only (or no value)
      if values.is_empty() {
        let value = key_value.strip_prefix("--blitz_cpu_experimental_xnn_dusion_type=").unwrap();
        if value == "+dot" {
          dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Dot);
        } else if value == "-dot" {
          dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Dot);
        } else if value == "+eltwise" {
          dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Eltwise);
        } else if value == "-eltwise" {
          dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Eltwise);
        } else if value == "+reduce" {
          dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Reduce);
        } else if value == "-reduce" {
          dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Reduce);
        } else if value == "+individual_dot" {
          dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::IndividualDot);
        } else if value == "-individual_dot" {
          dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::IndividualDot);
        } else { // no value
          //dbg_opts.clear_blitz_gpu_enable_command_buffer();
        }
      }
      // Some values
      // Check reset default values or not.
      let sample = values[0];
      if !sample.starts_with("-") && !sample.starts_with("+") {
        dbg_opts.clear_blitz_cpu_experimental_xnn_fusion_type();
        for value in &values {
          if *value == "dot" {
            dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Dot);
          } else if *value == "eltwise" {
            dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Eltwise);
          } else if *value == "reduce" {
            dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Reduce);
          } else if *value == "individual_dot" {
            dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::IndividualDot);
          }
        }
      } else {
        for value in &values {
          if *value == "+dot" {
            dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Dot);
          } else if *value == "-dot" {
            dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Dot);
          } else if *value == "+eltwise" {
            dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Eltwise);
          } else if *value == "-eltwise" {
            dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Eltwise);
          } else if *value == "+reduce" {
            dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Reduce);
          } else if *value == "-reduce" {
            dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Reduce);
          } else if *value == "+individual_dot" {
            dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::IndividualDot);
          } else if *value == "-individual_dot" {
            dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::IndividualDot);
          }
        }
      }
    }
    // command line
    if key_value.contains("--blitz_gpu_experimental_autotune_backends") {
      let values: Vec<&str> = key_value
        .strip_prefix("--blitz_gpu_experimental_autotune_backends=").unwrap().split(",").collect();
      // One value only (or no value)
      if values.is_empty() {
        let value = key_value.strip_prefix("--blitz_gpu_experimental_autotune_backends=").unwrap();
        if value == "+dot" {
          //dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Dot);
        } else if value == "-dot" {
          //dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Dot);
        } else if value == "+eltwise" {
          //dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Eltwise);
        } else if value == "-eltwise" {
          //dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Eltwise);
        } else if value == "+reduce" {
          //dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Reduce);
        } else if value == "-reduce" {
          //dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Reduce);
        } else if value == "+individual_dot" {
          //dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::IndividualDot);
        } else if value == "-individual_dot" {
          //dbg_opts.remove_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::IndividualDot);
        } else { // no value
          //dbg_opts.clear_blitz_gpu_enable_command_buffer();
        }
      }
      // Some values
      // Check reset default values or not.
      let sample = values[0];
      if !sample.starts_with("-") && !sample.starts_with("+") {
        dbg_opts.clear_blitz_gpu_experimental_autotune_backends();
        for value in &values {
          if *value == "cudnn" {
            dbg_opts.add_blitz_gpu_experimental_autotune_backends(AutotuneBackend::Cudnn);
          } else if *value == "triton" {
            dbg_opts.add_blitz_gpu_experimental_autotune_backends(AutotuneBackend::Triton);
          } else if *value == "" {
            //dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::Reduce);
          } else if *value == "" {
            //dbg_opts.add_blitz_cpu_experimental_xnn_fusion_type(LibraryFusionType::IndividualDot);
          }
        }
      } else {
        for value in &values {
          if *value == "+cublas" {
            dbg_opts.add_blitz_gpu_experimental_autotune_backends(AutotuneBackend::Cublas);
          } else if *value == "-cublas" {
            dbg_opts.remove_blitz_gpu_experimental_autotune_backends(AutotuneBackend::Cublas);
          } else if *value == "+triton" {
            dbg_opts.add_blitz_gpu_experimental_autotune_backends(AutotuneBackend::Triton);
          } else if *value == "-triton" {
            dbg_opts.remove_blitz_gpu_experimental_autotune_backends(AutotuneBackend::Triton);
          } else if *value == "" {
          } else if *value == "" {
          } else if *value == "" {
          } else if *value == "" {
          }
        }
      }
    }
  }
}