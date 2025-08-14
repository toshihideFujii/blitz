#![allow(dead_code)]

use crate::{
  device_description::DeviceDescription
};

use super::host_platform::HostPlatform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostExecutor {
  device_ordinal: i64
}

impl HostExecutor {
  pub fn default(device_ordinal: i64) -> Self {
    HostExecutor { device_ordinal: device_ordinal }
  }

  pub fn new(_platform: &HostPlatform, _device_ordinal: i64) -> Self {
    unimplemented!()
  }

  pub fn init(&self) -> Result<(), String> {
    unimplemented!()
  }

  pub fn create_device_description(&self) -> Result<DeviceDescription, String> {
    HostExecutor::create_device_description_by_ordinal(0)
  }

  pub fn create_device_description_by_ordinal(
    _device_ordinal: i64) -> Result<DeviceDescription, String>
  {
    unimplemented!()
  }
}