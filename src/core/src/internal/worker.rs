#![allow(dead_code)]

use std::time::Duration;
use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct Worker {
  blitz_worker_prefix: String,
  blitz_worker_resource_file: ConfigEntry<String>,
  worker_timeout: ConfigEntry<u64>,
  worker_initial_registration_retries: ConfigEntry<i64>,
  worker_max_registration_retries: ConfigEntry<i64>,
  worker_driver_terminate_timeout: ConfigEntry<Duration>,
  worker_cleanup_enabled: ConfigEntry<bool>,
  worker_cleanup_interval: ConfigEntry<u64>,
  app_data_retention: ConfigEntry<u64>,
  prefer_configured_master_address: ConfigEntry<bool>,
  worker_ui_port: ConfigEntry<i64>,
  worker_ui_retained_executors: ConfigEntry<i64>,
  worker_ui_retained_drivers: ConfigEntry<i64>,
  uncompressed_log_file_length_cache_size_conf: ConfigEntry<i64>,
  worker_decommission_sginal: ConfigEntry<String>,
  worker_id_pattern: ConfigEntry<String>,
}

impl Worker {
  pub fn new() -> Self {
    Worker {
      blitz_worker_prefix:
        "blitz.worker".to_string(),
      blitz_worker_resource_file:
        ConfigBuilder::new("blitz.worker.resource_file")
        .internal()
        .doc("Path to a file containing the resources allocated to the worker.
          The file should be formatted as a JSON array of ResourceAllocation objects.
          Only used internally in standalone mode.")
        .version("3.0.0")
        .string_conf()
        .create_with_default("str".to_string()), // TODO
      worker_timeout:
        ConfigBuilder::new("blitz.worker.timeout")
          .version("0.6.2")
          .long_conf()
          .create_with_default(60),
      worker_initial_registration_retries:
        ConfigBuilder::new("blitz.worker.initial_registration_retries")
          .version("4.0.0")
          .internal()
          .doc("The number of retries to reconnect in short intervals (between 5 and 16 seconds).")
          .int_conf()
          .create_with_default(6),
      worker_max_registration_retries:
        ConfigBuilder::new("blitz.worker.max_registration_retries")
          .version("4.0.0")
          .internal()
          .doc("The max number of retries to reconnect.
            After blitz.worker.initial_registration_retries attempts, the interval is between 30 and 90 seconds.")
          .int_conf()
          .create_with_default(16),
      worker_driver_terminate_timeout:
        ConfigBuilder::new("blitz.worker.driver_terminate_timeout")
          .version("2.1.2")
          .time_conf() // TODO
          .create_with_default(Duration::from_secs(10)),
      worker_cleanup_enabled:
        ConfigBuilder::new("blitz.worker.cleanup.enabled")
          .version("1.0.0")
          .boolean_conf()
          .create_with_default(false),
      worker_cleanup_interval:
        ConfigBuilder::new("blitz.worker.cleanup.interval")
          .version("1.0.0")
          .long_conf()
          .create_with_default(60 * 30),
      app_data_retention:
        ConfigBuilder::new("blitz.worker.cleanup.app_data_ttl")
          .version("1.0.0")
          .long_conf()
          .create_with_default(7 * 24 * 3600),
      prefer_configured_master_address:
        ConfigBuilder::new("blitz.worker.prefer_configured_master_address")
          .version("2.2.1")
          .boolean_conf()
          .create_with_default(false),
      worker_ui_port:
        ConfigBuilder::new("blitz.worker.ui.port")
          .version("1.1.0")
          .int_conf()
          .create_with_default(0), // TODO
      worker_ui_retained_executors:
        ConfigBuilder::new("blitz.worker.ui.retained_executors")
          .version("1.5.0")
          .int_conf()
          .create_with_default(1000),
      worker_ui_retained_drivers:
        ConfigBuilder::new("blitz.worker.ui.retained_drivers")
          .version("1.5.0")
          .int_conf()
          .create_with_default(1000),
      uncompressed_log_file_length_cache_size_conf:
        ConfigBuilder::new("blitz.worker.ui.compressed_log_file_length_cache_size")
          .version("2.0.2")
          .int_conf()
          .create_with_default(100),
      worker_decommission_sginal:
        ConfigBuilder::new("blitz.worker.decommission.signla")
          .doc("The signal that used to trigger the worker to start decommission.")
          .version("3.2.0")
          .string_conf()
          .create_with_default("PWR".to_string()),
      worker_id_pattern:
        ConfigBuilder::new("blitz.worker.id_pattern")
          .internal()
          .doc("The pattern for worker ID.")
          .version("4.0.0")
          .string_conf()
          .create_with_default("worker-%s-%s-%d".to_string()),
    }
  }
}