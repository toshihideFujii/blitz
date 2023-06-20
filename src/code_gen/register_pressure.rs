#![allow(dead_code)]

struct RegisterMaskPair {}

struct RegisterPressure {}

struct IntervalPressure {}
impl IntervalPressure {
  pub fn new() {}
  pub fn reset() {}
  pub fn open_top() {}
  pub fn open_bottom() {}
}

struct RegionPressure {}
impl RegionPressure {
  pub fn new() {}
  pub fn reset() {}
  pub fn open_top() {}
  pub fn open_bottom() {}
}

struct PressureChange {}
impl PressureChange {
  pub fn new() {}
  pub fn is_valid() {}
  pub fn get_p_set() {}
  pub fn get_p_set_or_max() {}
  pub fn get_unit_inc() {}
  pub fn set_unit_inc() {}
  pub fn dump() {}
}

struct PressureDiff {}
impl PressureDiff {
  pub fn new() {}
  pub fn add_pressure_change() {}
  pub fn dump() {}
}

struct RegisterOperands {}
impl RegisterOperands {
  pub fn new() {}
  pub fn collect() {}
  pub fn detect_dead_defs() {}
  pub fn adjust_lane_liveness() {}
}