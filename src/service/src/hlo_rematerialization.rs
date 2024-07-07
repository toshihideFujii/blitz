#![allow(dead_code)]

struct RematerializationSizes {
  before_bytes: i64,
  after_bytes: i64,
}

struct RematerializationModeConfig {
  recompute: bool,
  compress: bool,
  host_offload: bool,
}

struct HostMemoryOffloadConfig {
  host_memory_space: i64,
  bandwidth_to_host_bytes_per_second: f64,
  bandwidth_from_host_bytes_per_second: f64,
}

struct Options {
  
}

pub struct HloRematerialization {}

impl HloRematerialization {
    
}