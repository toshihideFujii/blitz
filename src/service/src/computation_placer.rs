#![allow(dead_code)]

pub struct DeviceAssignment {}

impl DeviceAssignment {
  pub fn new() {}
  pub fn replica_count() {}
  pub fn computation_count() {}
  pub fn logical_id_for_device() {}
  pub fn replica_id_for_device() {}
  pub fn get_device_to_logical_id_map() {}
  pub fn serialize() {}
  pub fn deserialize() {}
  pub fn to_string() {}
}

// A generic implementation of the Blitz computation placer, which assigns device
// ids to a set of replicated computations.
pub struct ComputationPlacer {}

impl ComputationPlacer {
  pub fn new() {}
  pub fn device_id() {}
  pub fn assign_devices() {}
  pub fn register_computation_placer() {}
  pub fn get_for_platform() {}
}