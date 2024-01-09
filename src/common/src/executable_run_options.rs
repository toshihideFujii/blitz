#![allow(dead_code)]

#[derive(Clone)]
pub struct RunId {
  data: i64,
}

impl RunId {
  pub fn new(value: i64) -> Self {
    RunId {
      data: value
    }
  }

  pub fn to_string(&self) -> String {
    let mut result = String::from("RunId: ");
    result.push_str(self.data.to_string().as_str());
    result
  }

  pub fn to_int(&self) -> i64 {
    self.data
  }

  pub fn absl_hash_value() {}
}

pub struct ExecutableRunOptions {
  device_ordinal: i64,
  rng_seed: i64,
  launch_id: i32,
  run_id: RunId,
}

impl ExecutableRunOptions {
  pub fn new() {}

  pub fn set_allocator() {}
  pub fn allocator() {}

  pub fn set_device_ordinal(&mut self, device_ordinal: i64) -> &mut ExecutableRunOptions {
    self.device_ordinal = device_ordinal;
    self
  }

  pub fn device_ordinal(&self) -> i64 {
    self.device_ordinal
  }

  pub fn set_stream() {}
  pub fn stream() {}

  pub fn set_host_to_device_stream() {}
  pub fn host_to_device_stream() {}

  pub fn set_device_to_host_stream() {}
  pub fn device_to_host_stream() {}

  pub fn set_intra_op_thread_pool() {}
  pub fn intra_op_thread_pool() {}

  pub fn set_execution_profile() {}
  pub fn execution_profile() {}

  pub fn set_device_assignment() {}
  pub fn device_assignment() {}

  pub fn set_rng_seed(&mut self, rng_seed: i64) -> &mut ExecutableRunOptions {
    self.rng_seed = rng_seed;
    self
  }

  pub fn rng_seed(&self) -> i64 {
    self.rng_seed
  }

  pub fn set_launch_id(&mut self, launch_id: i32) -> &mut ExecutableRunOptions {
    self.launch_id = launch_id;
    self
  }

  pub fn launch_id(&self) -> i32 {
    self.launch_id
  }

  pub fn set_run_id(&mut self, run_id: RunId) -> &mut ExecutableRunOptions {
    self.run_id = run_id;
    self
  }

  pub fn run_id(&self) -> RunId {
    self.run_id.clone()
  }

  pub fn set_then_executable_function() {}
  pub fn then_executable_function() {}

  pub fn set_senf_device_memory_function() {}
  pub fn send_device_memory_function() {}

  pub fn set_recv_device_memory_function() {}
  pub fn recv_device_memory_function() {}

  pub fn set_cpu_executable_run_options() {}
  pub fn cpu_executable_run_options() {}

  pub fn set_gpu_executable_run_options() {}
  pub fn gpu_executable_run_options() {}
}