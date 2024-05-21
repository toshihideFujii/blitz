#![allow(dead_code)]

pub enum CommandBufferState {
  Create,
  Update,
  Finalized,
}

pub enum CommandBufferMode {
  Primary,
  Nested,
}

pub struct CommandBuffer {}

impl CommandBuffer {
  pub fn new() {}
  pub fn trace() {}
  pub fn supports_conditional_commands() {}
  pub fn barrier() {}
  pub fn launch() {}
  pub fn add_nested_command_buffer() {}
  pub fn memcpy_device_to_device() {}
  pub fn memset() {}
  pub fn allocate() {}
  pub fn free() {}
  pub fn if_() {}
  pub fn if_else() {}
  pub fn case() {}
  pub fn for_() {}
  pub fn while_() {}
  pub fn finalize() {}
  pub fn update() {}
  pub fn mode() {}
  pub fn state() {}
}