#![allow(dead_code)]

pub struct GpuGraphSupport {}

impl GpuGraphSupport {
  pub fn new() {}
  pub fn notify_graph_exec_created() {}
  pub fn notify_graph_exec_destroyed() {}

  pub fn allocated_gpu_graph_execs() {}
  pub fn alive_gpu_graph_execs() {}
}

pub struct OwnedGpuGraph {}

pub struct OwnedGpuGraphExec {
  id: u64,
  num_updates: u64,
  num_launches: u64
}

impl OwnedGpuGraphExec {
  pub fn new() {}

  pub fn update() {}
  pub fn launch() {}

  pub fn id(&self) -> u64 {
    self.id
  }
}

pub fn create_gpu_graph() {}

pub fn add_kernel_mode() {}

pub fn add_memcpy_d2d_node() {}

pub fn capture_gpu_graph() {}

pub fn instantiate_gpu_graph() {}

pub fn is_stream_capturing() {}