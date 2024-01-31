#![allow(dead_code)]

pub struct LocalClientOptions {}

impl LocalClientOptions {
  pub fn new() {}
  pub fn set_platform() {}
  pub fn platform() {}
  pub fn set_number_of_replicas() {}
  pub fn number_of_replicas() {}
  pub fn set_intra_op_parallelism_threads() {}
  pub fn intra_op_parallelism_threads() {}
  pub fn set_allowed_devices() {}
}

pub struct ClientLibrary {}

impl ClientLibrary {
  pub fn new() {}
  pub fn get_or_create_local_client() {}
  pub fn local_client_or_die() {}
  pub fn get_blitz_service() {}
  pub fn get_or_create_compile_only_client() {}
  pub fn destroy_local_instances() {}
}