#![allow(dead_code)]

use common::array2d::Array2D;

// Class that represents the device assignment for a set of Blitz replicated
// computations. For R replicas and C computations, R * C devices are required
// execute the computation in parallel. The assigned device ids can be accessed
// by assignment(replica, computation).
pub struct DeviceAssignment {
  array: Array2D<i64>
}

impl DeviceAssignment {
  pub fn new(replica_count: usize, computation_count: usize) -> Self {
    DeviceAssignment {
      array: Array2D::new(replica_count, computation_count)
    }
  }

  pub fn replica_count(&self) -> usize {
    self.array.height()
  }

  pub fn computation_count(&self) -> usize {
    self.array.width()
  }

  pub fn logical_id_for_device() {}
  pub fn replica_id_for_device() {}
  pub fn get_device_to_logical_id_map() {}
  pub fn serialize() {}
  pub fn deserialize() {}
  pub fn to_string() {}

  pub fn set_data(
    &mut self, replica_count: usize, computation_count: usize, data: i64)
  {
    self.array.set_data(replica_count, computation_count, data);    
  }
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