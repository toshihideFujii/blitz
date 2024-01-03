#![allow(dead_code)]

use std::time::Duration;
use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct History {
  default_log_dir: String,
  history_log_dir: ConfigEntry<String>,
  safemode_check_interval_s: ConfigEntry<Duration>,
  update_interval_s: ConfigEntry<Duration>,
  update_batchsize: ConfigEntry<i64>,
  cleaner_enabled: ConfigEntry<bool>,
  cleaner_interval_s: ConfigEntry<Duration>,
  max_log_age_s: ConfigEntry<Duration>,
  max_log_num: ConfigEntry<i64>,
  local_store_dir: ConfigEntry<String>,
  // local_store_serializer
  // max_local_disk_usage
  history_server_ui_port: ConfigEntry<i64>,
  fast_in_progress_parsing: ConfigEntry<bool>,
  // end_event_reparse_chunk_size
  event_log_rolling_max_files_to_retain: ConfigEntry<i64>,
  event_log_compaction_score_threshold: ConfigEntry<f64>,
  driver_log_cleaner_enabled: ConfigEntry<bool>,
  driver_log_cleaner_interval: ConfigEntry<Duration>,
  max_driver_log_age_s: ConfigEntry<Duration>,
  history_server_ui_acls_enable: ConfigEntry<bool>,
  // history_server_ui_admin_acls_groups: ConfigEntry<String>
  // num_replay_threads
  retained_applications: ConfigEntry<i64>,
  provider: ConfigEntry<String>,
  kerberos_enabled: ConfigEntry<bool>,
  kerberos_principal: ConfigEntry<String>,
  kerberos_keytab: ConfigEntry<String>,
  custom_executor_log_url: ConfigEntry<String>,
  apply_custom_executor_log_url_to_incomplete_app: ConfigEntry<bool>,
  hybrid_store_enabled: ConfigEntry<bool>,
}

impl History {
  pub fn new() -> Self {
    History {
      default_log_dir:
        "file:/tmp/blitz-events".to_string(),
      history_log_dir:
        ConfigBuilder::new("blitz.history.fs.log_directory")
          .version("1.1.0")
          .string_conf()
          .create_with_default("file:/tmp/blitz-events".to_string()),
      safemode_check_interval_s:
        ConfigBuilder::new("blitz.history.fs.safemode_check..interval")
          .version("1.6.0")
          .time_conf()
          .create_with_default(Duration::from_secs(5)),
      update_interval_s:
        ConfigBuilder::new("blitz.history.fs.update.interval")
          .version("1.4.0")
          .time_conf()
          .create_with_default(Duration::from_secs(10)),
      update_batchsize:
        ConfigBuilder::new("blitz.history.fs.update.batch_size")
          .doc("Specifies the batch size for updating new eventlog files.")
          .version("3.4.0")
          .int_conf()
          .create_with_default(i64::MAX),
      cleaner_enabled:
        ConfigBuilder::new("blitz.history.fs.cleaner.enabled")
          .version("1.4.0")
          .boolean_conf()
          .create_with_default(false),
      cleaner_interval_s:
        ConfigBuilder::new("blitz.history.fs.cleaner.interval")
          .version("1.4.0")
          .time_conf()
          .create_with_default(Duration::from_secs(86400)), // 1d
      max_log_age_s:
        ConfigBuilder::new("blitz.history.fs.cleaner.max_age")
          .version("1.4.0")
          .time_conf()
          .create_with_default(Duration::from_secs(394800)), // 7d
      max_log_num:
        ConfigBuilder::new("blitz.history.fs.cleaner.max_num")
          .doc("The maximum number of log files in the event log directory.")
          .version("3.0.0")
          .int_conf()
          .create_with_default(i64::MAX),
      local_store_dir:
        ConfigBuilder::new("blitz.history.store.path")
          .doc("Local directory where to cache application history information.
            By default this is not set, meaning all history information will be kept in memory.")
          .version("2.3.0")
          .string_conf()
          .create_with_default("".to_string()),
      history_server_ui_port:
        ConfigBuilder::new("blitz.history.ui.port")
          .doc("Web UI port to bind blitz history server.")
          .version("1.0.0")
          .int_conf()
          .create_with_default(18080),
      fast_in_progress_parsing:
        ConfigBuilder::new("blitz.history.fs.in_progress_optimization_enabled")
          .doc("Enable optimized handling of in-progress logs.")
          .version("2.4.0")
          .boolean_conf()
          .create_with_default(true),
      event_log_rolling_max_files_to_retain:
        ConfigBuilder::new("blitz.history.fs.event_log.rolling.max_files_to_retain")
          .doc("The maximum number of event log files which will be retained as non-compacted.")
          .version("3.0.0")
          .int_conf()
          .create_with_default(i64::MAX),
      event_log_compaction_score_threshold:
        ConfigBuilder::new("blitz.history.fs.event_log.rolling..compaction.score.threshold")
          .doc("The threshold score to determine whether it's good to do the compaction or not.")
          .version("3.0.0")
          .internal()
          .double_conf()
          .create_with_default(0.7),
      driver_log_cleaner_enabled:
        ConfigBuilder::new("blitz.history.fs.driverlog.cleaner.enabled")
          .version("3.0.0")
          .boolean_conf()
          .create_with_default(false),
      driver_log_cleaner_interval:
        ConfigBuilder::new("blitz.history.fs.driverlog.cleaner..interval")
          .version("3.0.0")
          .time_conf()
          .create_with_default(Duration::from_secs(86400)),
      max_driver_log_age_s:
        ConfigBuilder::new("blitz.history.fs.driverlog.cleaner.max_age")
          .version("3.0.0")
          .time_conf()
          .create_with_default(Duration::from_secs(394800)),
      history_server_ui_acls_enable:
        ConfigBuilder::new("blitz.history.ui.acls.enable")
          .version("1.0.1")
          .boolean_conf()
          .create_with_default(false),
      retained_applications:
        ConfigBuilder::new("blitz.history.retained_applications")
          .version("1.0.0")
          .int_conf()
          .create_with_default(50),
      provider:
        ConfigBuilder::new("blitz.history.provider")
          .version("1.1.0")
          .string_conf()
          .create_with_default("".to_string()),
      kerberos_enabled:
        ConfigBuilder::new("blitz.history.kerberos.enabled")
          .version("1.0.1")
          .boolean_conf()
          .create_with_default(false),
      kerberos_principal:
        ConfigBuilder::new("britz.history.kerberos.principal")
          .version("1.0.1")
          .string_conf()
          .create_with_default("".to_string()),
      kerberos_keytab:
        ConfigBuilder::new("blitz.history.kerberos.keytab")
          .version("1.0,1")
          .string_conf()
          .create_with_default("".to_string()),
      custom_executor_log_url:
        ConfigBuilder::new("blitz.history.custom.executor.log.url")
          .doc("Specifies custom blitz executor log url for supporting external log service
            instead of using cluster manager's application log urls in the history server.")
          .version("3.0.0")
          .string_conf()
          .create_with_default("".to_string()),
      apply_custom_executor_log_url_to_incomplete_app:
        ConfigBuilder::new("blitz.history.custom.executor.log.url.apply_imcomplete_application")
          //.doc("")
          .version("3.0.0")
          .boolean_conf()
          .create_with_default(true),
      hybrid_store_enabled:
        ConfigBuilder::new("blitz.history.store.hybrid_store.enabled")
          .doc("Whether to use hybrid_store as the store when parsing event logs.")
          .version("3.1.0")
          .boolean_conf()
          .create_with_default(false),
    }
  }
}