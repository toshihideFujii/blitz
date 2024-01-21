
#![allow(dead_code)]

use crate::hlo_module::HloModule;

pub struct HloModuleGroup {
  name: String,
  modules: Vec<HloModule>,
  cache_key: String,
}

impl HloModuleGroup {
  pub fn new() {}

  pub fn modules(&self) -> &Vec<HloModule> {
    &self.modules
  }

  pub fn module(&self, index: usize) -> &HloModule {
    &self.modules[index]
  }

  pub fn push_back(&mut self, module: HloModule) {
    self.modules.push(module);
  }
  
  pub fn replace_module() {}
  pub fn consume_modules() {}

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn to_string() {}

  pub fn cleanup(&mut self) {
    for module in &mut self.modules {
      module.cleanup()
    }
  }

  pub fn size(&self) -> usize {
    self.modules.len()
  }

  pub fn empty(&self) -> bool {
    self.modules.is_empty()
  }

  pub fn cache_key(&self) -> String {
    self.cache_key.clone()
  }

  pub fn set_cache_key(&mut self, cache_key: String) {
    self.cache_key = cache_key;
  }
}