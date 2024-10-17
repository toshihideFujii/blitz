#![allow(dead_code)]

use std::{
  //cell::OnceCell,
  collections::HashMap, sync::LazyLock
};

use crate::platform::Platform;

// Manages multiple platforms that may be present on the current machine.
pub struct PlatformManagerImpl {
  id_map: HashMap<i64, Box<dyn Platform>>,
  name_map: HashMap<String, Box<dyn Platform>>
}

impl PlatformManagerImpl {
  pub fn new() -> Self {
    PlatformManagerImpl {
      id_map: HashMap::new(),
      name_map: HashMap::new()
    }
  }

  // Registers a platform object, returns an error status if the platform is
  // already registered. The associated listener, if not null, will be used to
  // trace events for ALL executors for that platform.
  // Takes ownership of platform.
  pub fn register_platform(&self, _platform: Box<dyn Platform>) -> Result<(), String> {
    unimplemented!()
  }

  // Retrieves the platform registered with the given platform name (e.g.
  // "CUDA", "OpenCL", ...) or id (an opaque, comparable value provided by the
  // Platform's Id() method).
  //
  // If the platform has not already been initialized, it will be initialized
  // with a default set of parameters.
  //
  // If the requested platform is not registered, an error status is returned.
  // Ownership of the platform is NOT transferred to the caller --
  // the PlatformManager owns the platforms in a singleton-like fashion.
  pub fn platform_with_name(&self, target: &String) -> Result<&Box<dyn Platform>, String> {
    self.platform_with_name_without_initialized(target, true)
  }

  pub fn platform_with_id(&self, id: &i64) -> Result<&Box<dyn Platform>, String> {
    self.platform_with_id_without_initialized(id, true)
  }

  // Same functions as above, but allows platforms to be returned without
  // initialization if initialize_platform == false.
  pub fn platform_with_name_without_initialized(
    &self,
    target: &String,
    initialized_platform: bool) -> Result<&Box<dyn Platform>, String>
  {
    let platform = self.lookup_by_name_locked(target);
    if platform.is_err() {
      return Err(platform.err().unwrap());
    }
    if initialized_platform && !platform.as_ref().unwrap().initialized() {
      let result = platform.as_ref().unwrap().initialize();
      if result.is_err() {
        return Err(result.err().unwrap());
      }
    }
    platform
  }

  pub fn platform_with_id_without_initialized(
    &self,
    target: &i64,
    initialized_platform: bool) -> Result<&Box<dyn Platform>, String>
  {
    let platform = self.lookup_by_id_locked(*target);
    if platform.is_err() {
      return Err(platform.err().unwrap());
    }
    if initialized_platform && !platform.as_ref().unwrap().initialized() {
      let result = platform.as_ref().unwrap().initialize();
      if result.is_err() {
        return Err(result.err().unwrap());
      }
    }
    platform
  }

  // Retrieves the platform registered with the given platform id (an opaque,
  // comparable value provided by the Platform's Id() method).
  //
  // If the requested platform is not registered, an error status is returned.
  // Ownership of the platform is NOT transferred to the caller --
  // the PlatformManager owns the platforms in a singleton-like fashion.
  pub fn initialize_platform_with_id(&self, id: i64) -> Result<&Box<dyn Platform>, String> {
    let platform = self.lookup_by_id_locked(id);
    if platform.is_err() {
      return Err(platform.err().unwrap());
    }
    if platform.as_ref().unwrap().initialized() {
      let mut err_msg = "platform with id ".to_string();
      err_msg.push_str(&id.to_string());
      err_msg.push_str(" is already initialized.");
      return Err(err_msg);
    }
    let result = platform.as_ref().unwrap().initialize();
    if result.is_err() {
      return Err(result.err().unwrap());
    }
    platform
  }

  // Retrieves the platforms satisfying the given filter, i.e. returns true.
  // Returned Platforms are always initialized.
  pub fn platform_with_filter(
    &self,
    filter: &Box<dyn Fn(&dyn Platform) -> bool>,
    initialize_platform: bool) -> Result<Vec<&Box<dyn Platform>>, String>
  {
    assert!(self.name_map.len() == self.id_map.len());
    let mut platforms = vec![];
    platforms.reserve(self.id_map.len());
    for entry in &self.id_map {
      let platform = entry.1;
      if filter(platform.as_ref()) {
        if initialize_platform && !platform.as_ref().initialized() {
          let result = platform.as_ref().initialize();
          if result.is_err() {
            return Err(result.err().unwrap());
          }
        }
        platforms.push(platform);
      }
    }
    Ok(platforms)
  }

  fn lookup_by_name_locked(&self, target: &String) -> Result<&Box<dyn Platform>, String> {
    let result =
      self.name_map.get(&target.to_ascii_lowercase());
    if result.is_none() {
      let mut err_msg = "Could not find registered platform with name: ".to_string();
      err_msg.push_str(&target);
      return Err(err_msg);
    }
    Ok(result.unwrap())
  }

  fn lookup_by_id_locked(&self, id: i64) -> Result<&Box<dyn Platform>, String> {
    let result = self.id_map.get(&id);
    if result.is_none() {
      let mut err_msg = "Could not find registered platform with id: ".to_string();
      err_msg.push_str(&id.to_string());
      return Err(err_msg);
    }
    Ok(result.unwrap())
  }
}

unsafe impl Sync for PlatformManagerImpl {}
unsafe impl Send for PlatformManagerImpl {}

// Singleton
static PLATFORM_MGR: LazyLock<PlatformManagerImpl> = LazyLock::new(PlatformManagerImpl::new);

pub fn register_platform(platform: Box<dyn Platform>) -> Result<(), String> {
  PLATFORM_MGR.register_platform(platform)
}

pub fn platform_with_name(target: &String) -> Result<&Box<dyn Platform>, String> {
  PLATFORM_MGR.platform_with_name(target)
}

pub fn platform_with_id(id: &i64) -> Result<&Box<dyn Platform>, String> {
  PLATFORM_MGR.platform_with_id(id)
} 

pub fn platform_with_filter(
  filter: &Box<dyn Fn(&dyn Platform) -> bool>) -> Result<Vec<&Box<dyn Platform>>, String>
{
  PLATFORM_MGR.platform_with_filter(filter, true)    
}


