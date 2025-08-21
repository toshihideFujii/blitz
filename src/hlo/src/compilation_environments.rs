#![allow(dead_code)]

// A class for holding CompilationEnvironments, i.e., protos holding the values
// of command line flags and environment variables that affect compilation.
//
// CompilationEnvironments uses lazy initialization, (see GetEnv() for more
// details). Lazy initialization is used so we can avoid:
// A) Requiring every code path to explitily construct all needed compilation
//    environments, particularly when the default constructed environment is
//    all we need AND
// B) Requiring CompilationEnvironments to implicitly construct all needed
//    environments, thereby requiring it to statically know the types of all
//    such environments
//
// CompilationEnvironments is not thread-safe.
#[derive(Debug, Clone)]
pub struct CompilationEnvironments {}

impl CompilationEnvironments {
  pub fn new() {}
  pub fn register_process_new_env_fn() {}
  pub fn add_env() {}
  pub fn get_mutable_env() {}
  pub fn get_env() {}
  pub fn has_env() {}
  pub fn clear() {}
}