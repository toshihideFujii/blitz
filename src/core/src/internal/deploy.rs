#![allow(dead_code)]

use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct Deploy {
  recovery_mode: ConfigEntry<String>,
  recovery_serializer: ConfigEntry<String>,
  recovery_compression_codec: ConfigEntry<String>,
  recovery_mode_factory: ConfigEntry<String>,
  recovery_directory: ConfigEntry<String>,
  zookeeper_url: ConfigEntry<String>,
  zookeeper_directory: ConfigEntry<String>,
  retained_applications: ConfigEntry<i64>,
  retained_drivers: ConfigEntry<i64>,
  reaper_iterations: ConfigEntry<i64>,
  max_executor_retries: ConfigEntry<i64>,
  spread_out_apps: ConfigEntry<bool>,
  default_cores: ConfigEntry<i64>,
  max_drivers: ConfigEntry<i64>,
  app_number_modulo: ConfigEntry<i64>,
  driver_id_pattern: ConfigEntry<String>,
  app_id_pattern: ConfigEntry<String>,
}

impl Deploy {
  pub fn new() -> Self {
    Deploy {
      recovery_mode:
        ConfigBuilder::new("blits.deploy.recovery_mode")
          .version("0.8.1")
          .string_conf()
          .create_with_default("None".to_string()),
      recovery_serializer:
        ConfigBuilder::new("blitz.deploy.recovery_serializer")
          .version("4.0.0")
          .string_conf() // TODO
          .create_with_default("recovery_serialization".to_string()),
      recovery_compression_codec:
        ConfigBuilder::new("blitz.deploy.recovery_compression_codec")
          .doc("A compression codec for persistence engines.")
          .version("4.0.0")
          .string_conf()
          .create_with_default("str".to_string()), // TODO
      recovery_mode_factory:
        ConfigBuilder::new("blitz.deploy.recovery_mode.factory")
          .version("1.2.0")
          .string_conf()
          .create_with_default("".to_string()),
      recovery_directory:
        ConfigBuilder::new("blitz.deploy.recovery_directory")
          .version("0.8.1")
          .string_conf()
          .create_with_default("".to_string()),
      zookeeper_url:
        ConfigBuilder::new("blitz.deploy.zookeeper.url")
          .version("0.8.1")
          .string_conf()
          .create_with_default("".to_string()), // TODO
      zookeeper_directory:
        ConfigBuilder::new("blitz.deploy.zookeeper.dir")
          .version("0.8.1")
          .string_conf()
          .create_with_default("".to_string()), // TODO
      retained_applications:
        ConfigBuilder::new("blitz.deploy.retained_applications")
          .version("0.8.0")
          .int_conf()
          .create_with_default(200),
      retained_drivers:
        ConfigBuilder::new("blitz.deploy.retained_drivers")
          .version("1.1.0")
          .int_conf()
          .create_with_default(200),
      reaper_iterations:
        ConfigBuilder::new("blitz.dead.worker.persistence")
          .version("0.8.0")
          .int_conf()
          .create_with_default(15),
      max_executor_retries:
        ConfigBuilder::new("blitz.deploy.max_executor_retries")
          .version("1.6.3")
          .int_conf()
          .create_with_default(10),
      spread_out_apps:
        ConfigBuilder::new("blitz.deploy.spread_out")
          .version("0.6.1")
          .boolean_conf()
          .create_with_default(true),
      default_cores:
        ConfigBuilder::new("blitz.deploy.default_cores")
          .version("0.9.0")
          .int_conf()
          .create_with_default(i64::MAX),
      max_drivers:
        ConfigBuilder::new("blitz.deploy.max_drivers")
          .doc("The maximum number of running drivers")
          .version("4.0.0")
          .int_conf()
          .create_with_default(i64::MAX),
      app_number_modulo:
        ConfigBuilder::new("blitz.deploy.app_number_modulo")
          .doc("The modulo for app number.")
          .version("4.0.0")
          .int_conf()
          .create_with_default(0), // TODO
      driver_id_pattern:
        ConfigBuilder::new("blitz.deploy.driver_id_pattern")
          .doc("The pattern for driver ID.")
          .version("4.0.0")
          .string_conf()
          .create_with_default("driver-%s-%04d".to_string()),
      app_id_pattern:
        ConfigBuilder::new("blitz.deploy.app_id_pattern")
          .doc("The pattern for app ID.")
          .version("4.0.0")
          .string_conf()
          .create_with_default("app-%s-%04d".to_string()),
    }
  }
}