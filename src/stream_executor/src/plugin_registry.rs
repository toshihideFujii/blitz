#![allow(dead_code)]

pub enum PluginKind {
  Invalid,
  Blas,
  Dnn,
  Fft,
}

pub struct PluginRegistry {}

impl PluginRegistry {
  pub fn register_factory() {}
  pub fn has_factory() {}
  pub fn get_factory() {}
}