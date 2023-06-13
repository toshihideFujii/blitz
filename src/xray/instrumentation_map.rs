#![allow(dead_code)]

// Defines the interface for extracting the instrumentation map
// from an xray-instrumented binary.

enum FunctionKinds {
  Entry,
  Exit,
  Tail,
  LogArgsEnter,
  CustomEvent
}

// Represents an XRay instrumentation sled entry from an object file.
struct SledEntry {
  address: u64,
  function: u64,
  kind: FunctionKinds,
  always_instrument: bool,
  version: u32
}

struct YAMLXRaySledEntry {
  func_id: i32,
  address: u64,
  function: u64,
  kind: FunctionKinds,
  always_instrument: bool,
  function_name: String,
  version: u32
}

// The InstrumentationMap represents the computed function id's
// and indicated function addresses from an object file (or a YAML file).
// This provides an interface to just the mapping between the function
// id, and the function address.
struct InstrumentationMap {}
impl InstrumentationMap {
  pub fn new() {}
  pub fn get_function_addresses() {}
  pub fn get_function_id() {}
  pub fn get_function_addr() {}
  pub fn sleds() {}
}

pub fn enumeration() {}
pub fn mapping() {}