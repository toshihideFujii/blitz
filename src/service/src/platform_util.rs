#![allow(dead_code)]

use std::collections::HashSet;

use stream_executor::{platform::Platform, stream_executor::StreamExecutor};

// Utilities for querying platforms and devices used by Blitz.
pub struct PlatformUtil {}

impl PlatformUtil {
  // Returns the canonical name of the underlying platform.
  //
  // This is needed to differentiate if for given platform like GPU or CPU
  // there are multiple implementations. For example, GPU platform may be
  // cuda(Nvidia) or rocm(AMD)
  pub fn canonical_platform_name(name: String) -> Result<String, String> {
    let lowercase_platform_name = name.to_ascii_lowercase();

    // "cpu" and "host" mean the same thing.
    if &lowercase_platform_name == "cpu" {
      return Ok("host".to_string());
    }
    // When configured on CUDA, "gpu" and "cuda" mean the same thing.
    if &lowercase_platform_name == "gpu" {
      return Ok("cuda".to_string());
    }

    Err(lowercase_platform_name)
  }

  // Returns the platforms present on the system and supported by XLA.
  //
  // Note that, even if a platform is present with zero devices, if we *do* have
  // compilation support for it, it will be returned in this sequence.
  pub fn get_supported_platforms() -> Result<Vec<Box<dyn Platform>>, String> {
    unimplemented!()
  }

  // Convenience function which returns the default supported platform for
  // tests. If exactly one supported platform is present, then this platform is
  // the default platform. If exactly two platforms are present and one of them
  // is the interpreter platform, then the other platform is the default
  // platform. Otherwise returns an error.
  pub fn get_default_platform(&self) -> Result<Box<dyn Platform>, String> {
    unimplemented!()
  }

  // Returns the platform according to the given name. Returns error if there is
  // no such platform.
  pub fn get_platform(_platform_name: String) -> Result<Box<dyn Platform>, String> {
    unimplemented!()
  }

  // Returns a vector of StreamExecutors for the given platform.
  // If populated, only the devices in allowed_devices will have
  // their StreamExecutors initialized, otherwise all StreamExecutors will be
  // initialized and returned.
  //
  // If the platform has no visible devices, a not-found error is returned.
  pub fn get_stream_executors(
    _platform: &dyn Platform,
    _allowed_devices: Option<HashSet<i64>>) -> Result<Vec<Box<dyn StreamExecutor>>, String>
  {
    unimplemented!()    
  }
}