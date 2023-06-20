#![allow(dead_code)]

struct MachinePassRegistryListener {}
impl MachinePassRegistryListener {
  pub fn new() {}
  pub fn notify_add() {}
  pub fn notify_remove() {}
}

struct MachinePassRegistryNode {}
impl MachinePassRegistryNode {
  pub fn new() {}
  pub fn get_next() {}
  pub fn get_next_address() {}
  pub fn get_name() {}
  pub fn get_description() {}
  pub fn get_ctor() {}
  pub fn set_next() {}
}

struct MachinePassRegistry {}
impl MachinePassRegistry {
  pub fn new() {}
  pub fn get_list() {}
  pub fn get_default() {}
  pub fn set_default() {}
  pub fn set_listener() {}
  pub fn add() {}
  pub fn remove() {}
}

struct RegisterPassParser {}
impl RegisterPassParser {
  pub fn new() {}
  pub fn initialize() {}
  pub fn notify_add() {}
  pub fn notify_remove() {}
}