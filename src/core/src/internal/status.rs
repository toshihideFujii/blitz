#![allow(dead_code)]

use std::time::Duration;
use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct Status {
  async_tracking_enabled: ConfigEntry<bool>,
  live_entity_update_period: ConfigEntry<Duration>,
  live_entity_update_min_flush_period: ConfigEntry<Duration>,
  max_retained_jobs: ConfigEntry<i64>,
  max_retained_stages: ConfigEntry<i64>,
  max_retained_tasks_per_stage: ConfigEntry<i64>,
  max_retained_dead_executors: ConfigEntry<i64>,
  max_retained_root_nodes: ConfigEntry<i64>,
  metrics_app_status_source_enabled: ConfigEntry<bool>,
  live_ui_local_store_dir: ConfigEntry<String>,
}

impl Status {
  pub fn new() -> Self {
    Status {
      async_tracking_enabled:
        ConfigBuilder::new("blitz.app_state_store.async_tracking.enable")
          .version("2.3.0")
          .boolean_conf()
          .create_with_default(true),
      live_entity_update_period:
        ConfigBuilder::new("blitz.ui.live_update.period")
          .version("2.3.0")
          .time_conf()
          .create_with_default(Duration::from_millis(100)),
      live_entity_update_min_flush_period:
        ConfigBuilder::new("blitz.ui.live_update.min_flush_period")
          .doc("Minimum time elapsed before stale UI data is flushed.
            This avoids UI staleness when incoming task events are not fired frequentry.")
          .version("2.4.2")
          .time_conf()
          .create_with_default(Duration::from_secs(1)),
      max_retained_jobs:
        ConfigBuilder::new("blitz.ui.retained_jobs")
          .version("1.2.0")
          .int_conf()
          .create_with_default(1000),
      max_retained_stages:
        ConfigBuilder::new("blitz.ui.retained_stages")
          .version("0.9.0")
          .int_conf()
          .create_with_default(1000),
      max_retained_tasks_per_stage:
        ConfigBuilder::new("blitz.ui.retained_tasks")
          .version("2.0.1")
          .int_conf()
          .create_with_default(100000),
      max_retained_dead_executors:
        ConfigBuilder::new("blitz.ui.retained_dead_executors")
          .version("2.0.0")
          .int_conf()
          .create_with_default(100),
      max_retained_root_nodes:
        ConfigBuilder::new("blitz.ui.dag_graph.retained_root_rdds")
          .version("2.1.0")
          .int_conf()
          .create_with_default(i64::MAX),
      metrics_app_status_source_enabled:
        ConfigBuilder::new("blitz.metrics.app_status_source.enabled")
          .doc("Whether Dropwizard/Codahale metrics will be reported for the status
            of the running blitz app.")
          .version("3.0.0")
          .boolean_conf()
          .create_with_default(false),
      live_ui_local_store_dir:
        ConfigBuilder::new("key")
          .doc("Local directory where to cache application information for live UI.
            By default this is not set, meaning all application information will be
            kept in memory.")
          .version("3.4.0")
          .string_conf()
          .create_with_default("str".to_string()), // TODO
    }
  }
}