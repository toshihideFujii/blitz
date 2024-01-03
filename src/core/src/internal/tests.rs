#![allow(dead_code)]

use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct Tests {
  test_use_compressed_oops_key: String,
  test_memory: ConfigEntry<u64>,
  test_dynamic_allocation_schedule_enabled: ConfigEntry<bool>,
  is_testing: ConfigEntry<bool>,
  test_no_stage_retry: ConfigEntry<bool>,
  test_reserved_memory: ConfigEntry<u64>,
  test_n_hosts: ConfigEntry<i64>,
  test_n_executors_host: ConfigEntry<i64>,
  test_n_cores_executor: ConfigEntry<i64>,
  resource_warning_testing: ConfigEntry<bool>,
  resource_profile_manager_testing: ConfigEntry<bool>,
  skip_validate_cores_testing: ConfigEntry<bool>,
  test_skip_ess_register: ConfigEntry<bool>,
}

impl Tests {
  pub fn new() -> Self {
    Tests {
      test_use_compressed_oops_key:
        "blitz.test.use_compressed_oops".to_string(),
      test_memory:
        ConfigBuilder::new("blitz.testing.memory")
          .version("1.6.0")
          .long_conf()
          .create_with_default(u64::MAX), // TODO
      test_dynamic_allocation_schedule_enabled:
        ConfigBuilder::new("blitz.testing.dynamic_allocation.schedule.enabled")
          .version("3.1.0")
          .boolean_conf()
          .create_with_default(true),
      is_testing:
        ConfigBuilder::new("blits.testing")
          .version("1.0.1")
          .boolean_conf()
          .create_with_default(false),
      test_no_stage_retry:
        ConfigBuilder::new("blitz.test.no_stage_retry")
          .version("1.2.0")
          .boolean_conf()
          .create_with_default(false),
      test_reserved_memory:
        ConfigBuilder::new("blitz.testing.reserved_memory")
          .version("1.6.0")
          .long_conf()
          .create_with_default(0), // TODO
      test_n_hosts:
        ConfigBuilder::new("blitz.testing.n_hosts")
          .version("3.0.0")
          .int_conf()
          .create_with_default(5),
      test_n_executors_host:
        ConfigBuilder::new("blitz.testing.n_executors_per_host")
          .version("3.0.0")
          .int_conf()
          .create_with_default(4),
      test_n_cores_executor:
        ConfigBuilder::new("blitz.testing.n_cores_per_executor")
          .version("3.0.0")
          .int_conf()
          .create_with_default(2),
      resource_warning_testing:
        ConfigBuilder::new("blitz.resources.warnings.testing")
          .version("3.1.0")
          .boolean_conf()
          .create_with_default(false),
      resource_profile_manager_testing:
        ConfigBuilder::new("blitz.testing.resource_profile_manager")
          .version("3.1.0")
          .boolean_conf()
          .create_with_default(false),
      skip_validate_cores_testing:
        ConfigBuilder::new("blitz.testing.skip_validate_cores")
          .version("3.1.0")
          .boolean_conf()
          .create_with_default(false),
      test_skip_ess_register:
        ConfigBuilder::new("blitz.testing.skip_ess_register")
          .version("4.0.0")
          .doc("None of Blitz testing modes (local, local-cluster) enables shuffle services.")
          .boolean_conf()
          .create_with_default(false)
    }
  }
}