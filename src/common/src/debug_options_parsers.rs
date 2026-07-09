use std::{collections::HashMap, i64};

use crate::blitz_data::{DebugOptions, IntRangeInclusive};

pub fn parse_blitz_backend_extra_options(
  extra_options_map: &mut HashMap<String, String>,
  comma_separated_values: String)
{
  let extra_options_parts: Vec<&str> =
    comma_separated_values.split(',').collect();

  // The flag contains a comma-separated list of options; some options
  // have arguments following "=", some don't.
  for part in extra_options_parts {
    let eq_ops = part.find('=');
    if eq_ops.is_none() {
      extra_options_map.insert(part.to_string(), "".to_string());
    } else {
      let mut value = "".to_string();
      if eq_ops.unwrap() < part.len() {
        value = part.split_at(eq_ops.unwrap()+1).1.to_string();
      }
      extra_options_map.insert(
        part.split_at(eq_ops.unwrap()).0.to_string(), value);
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum RepeatedFlagModifierOp {
  Add,
  Remove,
  Clear,
}

#[derive(Debug, PartialEq)]
pub struct RepeatedFlagModifier {
  pub op: RepeatedFlagModifierOp,
  pub value: String,
}

impl RepeatedFlagModifier {
  pub fn default() -> Self {
    RepeatedFlagModifier {
      op: RepeatedFlagModifierOp::Add,
      value: String::new()
    }
  }

  pub fn new(op: RepeatedFlagModifierOp, value: String) -> Self {
    RepeatedFlagModifier { op: op, value: value }
  }
}

// Parses a comma-separated list of a repeated flag modifiers.
//
// The sequence should either be a list of values, that will replace the
// existing values, or a list of modifiers, that will be applied to the existing
// values.
//
// Uppercases the values and optionally prefixes them.
//
// For example:
// parseRepeatedEnumModifiers("a,pre_b", "pre_")
//   -> [(clear), (add "PRE_A"), (add "PRE_B")]
//
// parseRepeatedEnumModifiers("+a,-b,+c", "")
//   -> [(add, "A"), (remove, "B"), (add, "C")]
pub fn parse_repeated_enum_modifiers(
  flag_value: String,
  add_prefix: String) -> Result<Vec<RepeatedFlagModifier>, String>
{
  let values: Vec<&str> = flag_value.split(',').collect();
  let prefix = add_prefix.to_ascii_uppercase();
  let mut modifiers: Vec<RepeatedFlagModifier> = vec![];
  let mut incremental_modifiers_count = 0;
  let values_len = values.len();
  for value in values {
    let mut modifier = RepeatedFlagModifier::default();
    let mut value = value.trim();
    if value.is_empty() {
      continue;
    }
    if value.starts_with('+') || value.starts_with('-') {
      incremental_modifiers_count += 1;
      if value.starts_with('-') {
        modifier.op = RepeatedFlagModifierOp::Remove;
      }
      value = value.trim_start_matches(|c| { c == '+' || c == '-' });
    } else if modifiers.is_empty() {
      let mut mod_clear = RepeatedFlagModifier::default();
      mod_clear.op = RepeatedFlagModifierOp::Clear;
      modifiers.push(mod_clear);
    }
    modifier.value = value.to_ascii_uppercase().to_string();
    if !prefix.is_empty() && !modifier.value.starts_with(&prefix) {
      let mut new_value = prefix.clone();
      new_value.push_str(&modifier.value);
      modifier.value = new_value;
    }
    modifiers.push(modifier);
  }
  // If we have at least one incremental then all values are to be incremental.
  if incremental_modifiers_count > 0 && values_len != incremental_modifiers_count {
    return Err("All values must be incremental or none of htem".to_string());
  }
  Ok(modifiers)
}

// Parses a string representation of an inclusive range into `range`.
//
// The string representation can be either:
// - a single integer x,
// - a range min:max,
// - a half-open range min: or :max.
// The range is inclusive on both ends.
//
// Returns true if the string is a valid range representation.
pub fn parse_int_range_inclusive(
  string_value: String, range: &mut IntRangeInclusive) -> bool
{
  let parts: Vec<&str> = string_value.split(':').collect();
  if parts.len() == 1 {
    if parts[0].parse::<i64>().is_err() {
      return false;
    }
    let first: i64 = parts[0].parse::<i64>().unwrap();
    range.set_first(first);
    range.set_last(first);
    return true;
  }
  if parts.len() == 2 {
    if parts[0].is_empty() && parts[1].is_empty() {
      return false; // ":" is not a valid range.
    }
    // Allow semi-open ranges (e.g. "1:", ":100").
    #[allow(unused_assignments)]
    let mut first = i64::MIN;
    #[allow(unused_assignments)]
    let mut last = i64::MAX;
    let parts_0 = parts[0].parse::<i64>();
    if !parts[0].is_empty() {
      if parts_0.is_err() { return false; }
      first = parts_0.unwrap();
    }
    let parts_1 = parts[1].parse::<i64>();
    if !parts[1].is_empty() {
      if parts_1.is_err() { return false; }
      last = parts_1.unwrap();
    }
    //first = parts[0].parse::<i64>().unwrap();
    //last = parts[1].parse::<i64>().unwrap();
    if first > last {
      return false;
    }
    range.set_first(first);
    range.set_last(last);
    return true;
  }
  false
}

pub fn get_non_default_debug_options(debug_options: &DebugOptions) -> String {
  let mut non_default_options = String::new();

  // gpu_disable_async_collectives
  if debug_options.blitz_gpu_disable_async_collectives().is_empty() {
    non_default_options.push_str(&"blitz_gpu_disable_async_collectives: []\n".to_string());
  } else {
    non_default_options.push_str(&"blitz_gpu_disable_async_collectives: [".to_string());
    let collectives =
      debug_options.blitz_gpu_disable_async_collectives();
    let mut op_type_count = 1;
    for op_type in collectives {
      non_default_options.push_str(&op_type.to_string());
      if op_type_count != collectives.len() {
        non_default_options.push_str(&", ".to_string());
      }
      op_type_count += 1;
    }
    non_default_options.push_str(&"]\n".to_string());
  }
  // gpu_target_config_filename
  if !debug_options.blitz_gpu_target_config_filename().is_empty() {
    non_default_options.push_str(&"blitz_gpu_target_config_filename: ".to_string());
    non_default_options.push_str(&debug_options.blitz_gpu_target_config_filename());
    non_default_options.push('\n');
  }
  // hlo_pass_fix_detect_cycles
  if debug_options.blitz_hlo_pass_fix_detect_cycles() {
    non_default_options.push_str(&"blitz_hlo_pass_fix_detect_cycles: ".to_string());
    non_default_options.push_str(&debug_options.blitz_hlo_pass_fix_detect_cycles().to_string());
    non_default_options.push('\n');
  }
  // dump_hlo_as_long_text
  if !debug_options.blitz_dump_hlo_as_long_text() {
    non_default_options.push_str(&"blitz_dump_hlo_as_long_text: ".to_string());
    non_default_options.push_str(&debug_options.blitz_dump_hlo_as_long_text().to_string());
    non_default_options.push('\n');
  }
  // dump_to
  if !debug_options.blitz_dump_to().is_empty() {
    non_default_options.push_str(&"blitz_dump_to: ".to_string());
    non_default_options.push_str(&debug_options.blitz_dump_to());
    non_default_options.push('\n');
  }

  non_default_options
}

#[cfg(test)]
mod tests {
  use std::{env, fs::{self, File}, io::Write, str::FromStr};

  use crate::{
    blitz_data::{AutotuneBackend, CollectiveOpType, CommandBufferCmdType, DebugOptions, LibraryFusionType},
    debug_options_flags::{
      //Flag,
      FLAG_VALUES, consume_fuel, parse_debug_option_flags_from_env, parse_flags_from_debug_options_file, pass_fuel_is_set
    },
    parse_flags_from_env::parse_flags_from_env_and_die_if_unknown
  };

  use super::*;

  struct UppercaseStringSetterTestSpec {
    user_max_isa: String,
    expected_max_isa: String,
  }

  impl UppercaseStringSetterTestSpec {
    fn new() -> Self {
      UppercaseStringSetterTestSpec {
        user_max_isa: String::from_str("foo").unwrap(),
        expected_max_isa: String::from_str("foo").unwrap()
      }
    }
  }

  struct UppercaseStringSetterTest {
    flag_values: DebugOptions,
    //flag_objects: Vec<Flag>,
  }

  impl UppercaseStringSetterTest {
    fn new() -> Self {
      UppercaseStringSetterTest {
        flag_values: DebugOptions::default(),
        //flag_objects: Vec::new(),
      }
    }

    fn flag_values(&self) -> &DebugOptions {
      &self.flag_values
    }

    fn mutable_flag_values(&mut self) -> &mut DebugOptions {
      &mut self.flag_values
    }

    //fn flag_objects(&self) -> &Vec<Flag> {
      //&self.flag_objects
    //}
  }

  fn set_blitz_flags_env_var(value: &String) {
    env::set_var("BLITZ_FLAGS", value);
  }

  #[test]
  fn test_parse_blitz_backend_extra_options() {
    let mut test_map: HashMap<String, String> = HashMap::new();
    let test_string = "aa=bb,cc,dd=,ee=ff=gg".to_string();

    parse_blitz_backend_extra_options(
      &mut test_map, test_string);
    
    assert_eq!(test_map.len(), 4);
    assert_eq!(test_map.get("aa"), Some(&"bb".to_string()));
    assert_eq!(test_map.get("cc"), Some(&"".to_string()));
    assert_eq!(test_map.get("dd"), Some(&"".to_string()));
    assert_eq!(test_map.get("ee"), Some(&"ff=gg".to_string()));
  }

  #[test]
  fn test_blitz_cpu_max_isa() {
    let spec = UppercaseStringSetterTestSpec::new();
    let mut spec_test = UppercaseStringSetterTest::new();
    let mut value = "--blitz_cpu_max_isa=".to_string();
    value.push_str(&spec.user_max_isa);
    set_blitz_flags_env_var(&value);
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(),
      spec_test.mutable_flag_values(),
      false);
    assert_eq!(spec_test.flag_values().blitz_cpu_max_isa(), &spec.expected_max_isa);
  }

  #[test]
  fn test_fuel_pass_counts_are_separate() {
    env::set_var("BLITZ_FLAGS", "--blitz_fuel=ABC=1,PQR=2");
    parse_debug_option_flags_from_env(false);

    assert_eq!(consume_fuel(&"ABC".to_string(), &mut None), true);
    assert_eq!(consume_fuel(&"ABC".to_string(), &mut None), false);

    assert_eq!(consume_fuel(&"PQR".to_string(), &mut None), true);
    assert_eq!(consume_fuel(&"PQR".to_string(), &mut None), true);
    assert_eq!(consume_fuel(&"PQR".to_string(), &mut None), false);
  }

  #[test]
  fn test_pass_fuel_is_set_returns_true_on_explicitly_fueled_passes_and_false_othrewise() {
    env::set_var("BLITZ_FLAGS", "--blitz_fuel=MNO=1,XYZ=2");
    parse_debug_option_flags_from_env(true);

    assert_eq!(pass_fuel_is_set(&"ABC".to_string()), false);
    assert_eq!(pass_fuel_is_set(&"MNO".to_string()), true);
    assert_eq!(pass_fuel_is_set(&"PQR".to_string()), false);
    assert_eq!(pass_fuel_is_set(&"XYZ".to_string()), true);
  }

  fn write_debug_options_to_temp_file(
    debug_options: &mut DebugOptions, contents: &mut String) -> String
  {
    *contents = get_non_default_debug_options(debug_options);
    let fname = "test_dbg_opts.txt".to_string();
    let f = File::create(fname.clone());
    assert!(f.is_ok());
    let result = f.unwrap().write_all(contents.as_bytes());
    assert!(result.is_ok());
    fname
  }

  #[test]
  fn test_failed_parsing() {
    assert_eq!(parse_flags_from_debug_options_file(
      &"not_exist.txt".to_string()), false);
  }

  #[test]
  fn test_parsing_repeated_fields() {
    let mut debug_options = DebugOptions::default();
    assert!(debug_options.blitz_gpu_disable_async_collectives().is_empty());
    debug_options.add_blitz_gpu_disable_async_collectives(
      CollectiveOpType::AllGather);
    debug_options.add_blitz_gpu_disable_async_collectives(
      CollectiveOpType::ReduceScatter);
    
    let mut contents = String::new();
    let mut file_name =
      write_debug_options_to_temp_file(&mut debug_options, &mut contents);
    assert!(parse_flags_from_debug_options_file(&file_name));

    assert!(contents.contains(
      "blitz_gpu_disable_async_collectives: [AllGather, ReduceScatter]"));
    assert_eq!(debug_options.blitz_gpu_disable_async_collectives().len(), 2);
    assert_eq!(debug_options.blitz_gpu_disable_async_collectives()[0],
      CollectiveOpType::AllGather);
    assert_eq!(debug_options.blitz_gpu_disable_async_collectives()[1],
      CollectiveOpType::ReduceScatter);

    // We are not resetting the flags here, because we want to ensure that
    // [ALLGATHER, REDUCESCATTER] gets overwritted to [ALLTOALL] and not simply
    // appended.
    debug_options.clear_gpu_disable_async_collectives();
    debug_options.add_blitz_gpu_disable_async_collectives(
      CollectiveOpType::AllToAll);
    file_name =
      write_debug_options_to_temp_file(&mut debug_options, &mut contents);
    assert!(parse_flags_from_debug_options_file(&file_name));
    assert!(contents.contains(
      "blitz_gpu_disable_async_collectives: [AllToAll]"));
    assert!(!contents.contains(
      "blitz_gpu_disable_async_collectives: [AllGather, ReduceScatter]"));
    assert_eq!(debug_options.blitz_gpu_disable_async_collectives().len(), 1);
    assert_eq!(debug_options.blitz_gpu_disable_async_collectives()[0],
      CollectiveOpType::AllToAll);

    let result = fs::remove_file(file_name);
    assert!(result.is_ok());
  }

  #[test]
  fn test_parse_from_debug_options_file() {
    // Sanity checks: The test needs to use two flags that have false and true
    // default values.
    // Default value of blitz_hlo_pass_fix_detect_cycles is false.
    // Default value of blitz_dump_hlo_as_long_text is true.
    let mut debug_options = DebugOptions::default();
    assert!(debug_options.blitz_dump_hlo_as_long_text());
    assert!(!debug_options.blitz_hlo_pass_fix_detect_cycles());

    // default value (should not be in debug_options file).
    debug_options.set_blitz_dump_hlo_as_long_text(true);
    // non-default value (should be in debug_options file).
    debug_options.set_blitz_hlo_pass_fix_detect_cycles(true);

    let mut contents = String::new();
    let mut temp_file =
      write_debug_options_to_temp_file(&mut debug_options, &mut contents);
    assert!(parse_flags_from_debug_options_file(&temp_file));
    assert!(contents.contains("blitz_hlo_pass_fix_detect_cycles: true"));
    assert!(!contents.contains("blitz_dump_hlo_as_long_text"));
    assert!(debug_options.blitz_hlo_pass_fix_detect_cycles());
    assert!(debug_options.blitz_dump_hlo_as_long_text());

    // non-default value (should be in debug_options file).
    debug_options.set_blitz_dump_hlo_as_long_text(false);
    // default value (should not be in debug_options file).
    debug_options.set_blitz_hlo_pass_fix_detect_cycles(false);
    contents.clear();
    temp_file = write_debug_options_to_temp_file(&mut debug_options, &mut contents);
    assert!(parse_flags_from_debug_options_file(&temp_file));
    assert!(!contents.contains("blitz_hlo_pass_fix_detect_cycles"));
    assert!(contents.contains("blitz_dump_hlo_as_long_text: false"));
    assert!(!debug_options.blitz_hlo_pass_fix_detect_cycles());
    assert!(!debug_options.blitz_dump_hlo_as_long_text());

    // default value (should not be in debug_options file).
    debug_options.set_blitz_dump_hlo_as_long_text(true);
    // default value (should not be in debug_options file).
    debug_options.set_blitz_hlo_pass_fix_detect_cycles(false);
    contents.clear();
    temp_file = write_debug_options_to_temp_file(&mut debug_options, &mut contents);
    assert!(parse_flags_from_debug_options_file(&temp_file));
    assert!(!contents.contains("blitz_hlo_pass_fix_detect_cycles"));
    assert!(!contents.contains("blitz_dump_hlo_as_long_text"));
    assert!(!debug_options.blitz_hlo_pass_fix_detect_cycles());
    assert!(debug_options.blitz_dump_hlo_as_long_text());

    // non-default value. (should be in debug_options file).
    debug_options.set_blitz_dump_hlo_as_long_text(false);
    // non-default value. (should be in debug_options file).
    debug_options.set_blitz_hlo_pass_fix_detect_cycles(true);
    contents.clear();
    temp_file = write_debug_options_to_temp_file(&mut debug_options, &mut contents);
    assert!(parse_flags_from_debug_options_file(&temp_file));
    assert!(contents.contains("blitz_hlo_pass_fix_detect_cycles: true"));
    assert!(contents.contains("blitz_dump_hlo_as_long_text: false"));
    assert!(debug_options.blitz_hlo_pass_fix_detect_cycles());
    assert!(!debug_options.blitz_dump_hlo_as_long_text());

    let result = fs::remove_file(temp_file);
    assert!(result.is_ok());
  }

  #[test] // FAIL
  fn test_env_overwrites_debug_options_file() {
    let mut debug_options = DebugOptions::default();
    debug_options.set_blitz_dump_to("/path/from/debug/options/file".to_string());
    debug_options.set_blitz_gpu_target_config_filename(
      "/gpu/target/config/from/debug/options/file".to_string());
    
    let mut contents = String::new();
    let debug_options_file =
      write_debug_options_to_temp_file(&mut debug_options, &mut contents);
    assert!(contents.contains(
      "blitz_dump_to: /path/from/debug/options/file"));
    assert!(contents.contains(
      "blitz_gpu_target_config_filename: /gpu/target/config/from/debug/options/file"));

    env::set_var("BLITZ_FLAGS", "blitz_dump_to=/path/from/env/var
    blitz_gpu_per_fusion_autotune_cache_dir=/path/to/autotune/cache/dir/from/env");

    // This is a proxy for the allocate call in run_hlo_module, which parses the
    // options from env.
    parse_debug_option_flags_from_env(false);
    assert!(parse_flags_from_debug_options_file(&debug_options_file));

    unsafe {
      assert_eq!(FLAG_VALUES.get().unwrap().blitz_dump_to(),
        "/path/from/debug/options/file".to_string());
      assert_eq!(FLAG_VALUES.get().unwrap().blitz_gpu_target_config_filename(),
        "/gpu/target/config/from/debug/options/file".to_string());
    }

    // This is a proxy for the second parsing from env var after parsing from the
    // file.
    parse_debug_option_flags_from_env(true);
    unsafe {
      assert_eq!(FLAG_VALUES.get().unwrap().blitz_dump_to(),
        "/path/from/env/var".to_string());
      //assert_eq!(FLAG_VALUES.get().unwrap().blitz_gpu_target_config_filename(),
        //"/gpu/target/config/from/debug/options/file".to_string());
    }
  }

  #[test]
  fn test_replace() {
    let result =
      parse_repeated_enum_modifiers(
        "a,b".to_string(), "".to_string());
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap()[0],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Clear, "".to_string()));
    assert_eq!(result.as_ref().unwrap()[1],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "A".to_string()));
    assert_eq!(result.as_ref().unwrap()[2],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "B".to_string()));
  }

  #[test]
  fn test_empty() {
    let result =
      parse_repeated_enum_modifiers(
        " ".to_string(), "".to_string());
    assert!(result.is_ok());
    assert!(result.as_ref().unwrap().is_empty());
  }

  #[test]
  fn test_replace_with_prefix() {
    let result =
      parse_repeated_enum_modifiers(
        "a,b".to_string(), "PRE_".to_string());
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap()[0],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Clear, "".to_string()));
    assert_eq!(result.as_ref().unwrap()[1],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "PRE_A".to_string()));
    assert_eq!(result.as_ref().unwrap()[2],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "PRE_B".to_string()));
  }

  #[test]
  fn test_replace_with_prefix_already_present() {
    let result =
      parse_repeated_enum_modifiers(
        "PRE_A,b".to_string(), "pre_".to_string());
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap()[0],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Clear, "".to_string()));
    assert_eq!(result.as_ref().unwrap()[1],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "PRE_A".to_string()));
    assert_eq!(result.as_ref().unwrap()[2],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "PRE_B".to_string()));
  }

  #[test]
  fn test_add_remove() {
    let result =
      parse_repeated_enum_modifiers(
        "+a,-b,+c".to_string(), "".to_string());
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap()[0],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "A".to_string()));
    assert_eq!(result.as_ref().unwrap()[1],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Remove, "B".to_string()));
    assert_eq!(result.as_ref().unwrap()[2],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "C".to_string()));
  }

  #[test]
  fn test_add_remove_with_prefix() {
    let result =
      parse_repeated_enum_modifiers(
        "+a,-b,+c".to_string(), "pre_".to_string());
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap()[0],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "PRE_A".to_string()));
    assert_eq!(result.as_ref().unwrap()[1],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Remove, "PRE_B".to_string()));
    assert_eq!(result.as_ref().unwrap()[2],
      RepeatedFlagModifier::new(RepeatedFlagModifierOp::Add, "PRE_C".to_string()));
  }

  #[test]
  fn test_invalid() {
    let result =
      parse_repeated_enum_modifiers(
        "+a,b".to_string(), "".to_string());
    assert!(result.is_err());
  }

  #[test]
  fn test_command_buffer_cmd_type() {
    let mut debug_options = DebugOptions::default();

    // Check that the default setting has 6 types.
    let mut enabled_types =
      debug_options.blitz_gpu_enable_command_buffer();
    assert_eq!(enabled_types.len(), 6);
    assert_eq!(enabled_types,
      vec![CommandBufferCmdType::Fusion, CommandBufferCmdType::Cublas,
        CommandBufferCmdType::Cublaslt, CommandBufferCmdType::CustomCall,
        CommandBufferCmdType::Cudnn, CommandBufferCmdType::DynamicSliceFusion]);
    
    // Removing options from the existing setting.
    set_blitz_flags_env_var(&"--blitz_gpu_enable_command_buffer=-fusion,-cublas".to_string());
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    enabled_types = debug_options.blitz_gpu_enable_command_buffer();
    assert_eq!(enabled_types.len(), 4);
    assert_eq!(enabled_types,
      vec![CommandBufferCmdType::Cublaslt, CommandBufferCmdType::CustomCall,
        CommandBufferCmdType::Cudnn, CommandBufferCmdType::DynamicSliceFusion]);

    // Removing an option that isn't there and adding a duplicate.
    set_blitz_flags_env_var(&"--blitz_gpu_enable_command_buffer=+cublaslt,-fusion".to_string());
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    enabled_types = debug_options.blitz_gpu_enable_command_buffer();
    assert_eq!(enabled_types.len(), 4);
    assert_eq!(enabled_types,
      vec![CommandBufferCmdType::Cublaslt, CommandBufferCmdType::CustomCall,
        CommandBufferCmdType::Cudnn, CommandBufferCmdType::DynamicSliceFusion]);

    // Adding an option.
    set_blitz_flags_env_var(&"--blitz_gpu_enable_command_buffer=+cublas".to_string());
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    enabled_types = debug_options.blitz_gpu_enable_command_buffer();
    assert_eq!(enabled_types.len(), 5);
    assert_eq!(enabled_types,
      vec![CommandBufferCmdType::Cublaslt, CommandBufferCmdType::CustomCall,
        CommandBufferCmdType::Cudnn, CommandBufferCmdType::DynamicSliceFusion,
        CommandBufferCmdType::Cublas]);

    // Overwriting the default setting.
    set_blitz_flags_env_var(&"--blitz_gpu_enable_command_buffer=custom_call,fusion".to_string());
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    enabled_types = debug_options.blitz_gpu_enable_command_buffer();
    assert_eq!(enabled_types.len(), 2);
    assert_eq!(enabled_types,
      vec![CommandBufferCmdType::CustomCall, CommandBufferCmdType::Fusion]);

    // Unset everything.
    set_blitz_flags_env_var(&"--blitz_gpu_enable_command_buffer=".to_string());
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    enabled_types = debug_options.blitz_gpu_enable_command_buffer();
    assert_eq!(enabled_types.len(), 0);
  }

  // Common function to test oneDNN and XNN fusion type.
  fn test_library_fusion_type(lib: &str) {
    let mut debug_options = DebugOptions::default();
    let mut flag_name = "--blitz_cpu_experimental_".to_string();
    flag_name.push_str(lib);
    flag_name.push_str("_fusion_type");

    #[allow(unused_assignments)]
    let mut enabled_types = vec![];
    if lib == "onednn" {
      enabled_types = debug_options.blitz_cpu_experimental_onednn_fusion_type();
    } else {
      enabled_types = debug_options.blitz_cpu_experimental_xnn_fusion_type();
    }
    // Check that the default setting is empty.
    assert!(enabled_types.is_empty());

    // Overwriting the default setting.
    let mut flag_name_plus = flag_name.clone();
    flag_name_plus.push_str("=dot,eltwise");
    set_blitz_flags_env_var(&flag_name_plus);
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    if lib == "onednn" {
      enabled_types = debug_options.blitz_cpu_experimental_onednn_fusion_type();
    } else {
      enabled_types = debug_options.blitz_cpu_experimental_xnn_fusion_type();
    }
    assert_eq!(enabled_types.len(), 2);
    assert_eq!(enabled_types, vec![LibraryFusionType::Dot, LibraryFusionType::Eltwise]);

    // Adding / removing options from the existing setting.
    let mut flag_name_plus_plus = flag_name.clone();
    flag_name_plus_plus.push_str("=+reduce,-eltwise");
    set_blitz_flags_env_var(&flag_name_plus_plus);
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    if lib == "onednn" {
      enabled_types = debug_options.blitz_cpu_experimental_onednn_fusion_type();
    } else {
      enabled_types = debug_options.blitz_cpu_experimental_xnn_fusion_type();
    }
    assert_eq!(enabled_types.len(), 2);
    assert_eq!(enabled_types, vec![LibraryFusionType::Dot, LibraryFusionType::Reduce])
  }

  #[test]
  fn test_onednn_fusion_type() {
    test_library_fusion_type("onednn");
  }

  #[test]
  fn test_xnn_fusion_type() {
    test_library_fusion_type("xnn");
  }

  #[test]
  fn test_autotune_backend() {
    let mut debug_options = DebugOptions::default();
    let mut enabled_backends =
      debug_options.blitz_gpu_experimental_autotune_backends();
    
    // Check that the default setting is populated.
    assert!(enabled_backends.is_empty());

    // Overwriting the default setting.
    set_blitz_flags_env_var(
      &"--blitz_gpu_experimental_autotune_backends=cudnn,triton".to_string());
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    enabled_backends = debug_options.blitz_gpu_experimental_autotune_backends();
    assert_eq!(enabled_backends.len(), 2);
    assert_eq!(enabled_backends, vec![AutotuneBackend::Cudnn, AutotuneBackend::Triton]);

    // Adding / removing options from the existing setting.
    set_blitz_flags_env_var(
      &"--blitz_gpu_experimental_autotune_backends=+cublas,-triton".to_string());
    parse_flags_from_env_and_die_if_unknown(
      &"BLITZ_FLAGS".to_string(), &mut debug_options, false);
    enabled_backends = debug_options.blitz_gpu_experimental_autotune_backends();
    assert_eq!(enabled_backends.len(), 2);
    assert_eq!(enabled_backends, vec![AutotuneBackend::Cudnn, AutotuneBackend::Cublas]);
  }

  #[test]
  fn test_single_integer() {
    let mut range = IntRangeInclusive::new();
    assert_eq!(parse_int_range_inclusive("10".to_string(), &mut range), true);
    assert_eq!(range.first(), 10);
    assert_eq!(range.last(), 10);
  }

  #[test]
  fn test_range() {
    let mut range = IntRangeInclusive::new();
    assert_eq!(parse_int_range_inclusive("10:20".to_string(), &mut range), true);
    assert_eq!(range.first(), 10);
    assert_eq!(range.last(), 20);
  }

  #[test]
  fn test_half_open_range_with_min() {
    let mut range = IntRangeInclusive::new();
    assert_eq!(parse_int_range_inclusive("10:".to_string(), &mut range), true);
    assert_eq!(range.first(), 10);
    assert_eq!(range.last(), i64::MAX);
  }

  #[test]
  fn test_half_open_range_with_max() {
    let mut range = IntRangeInclusive::new();
    assert_eq!(parse_int_range_inclusive(":100".to_string(), &mut range), true);
    assert_eq!(range.first(), i64::MIN);
    assert_eq!(range.last(), 100);
  }

  #[test]
  fn test_invalid_range() {
    let mut range = IntRangeInclusive::new();
    assert_eq!(parse_int_range_inclusive("10:20:30".to_string(), &mut range), false);
  }

  #[test]
  fn test_invalid_half_open_range() {
    let mut range = IntRangeInclusive::new();
    assert_eq!(parse_int_range_inclusive(":".to_string(), &mut range), false);
  }

  #[test]
  fn test_reversed_range() {
    let mut range = IntRangeInclusive::new();
    assert_eq!(parse_int_range_inclusive("20:10".to_string(), &mut range), false);
  }
}