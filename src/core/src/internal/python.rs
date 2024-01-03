#![allow(dead_code)]

use std::time::Duration;
use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct Python {
  python_worker_reuse: ConfigEntry<bool>,
  python_task_kill_timeout: ConfigEntry<Duration>,
  python_use_daemon: ConfigEntry<bool>,
  python_log_info: ConfigEntry<bool>,
  python_daemon_module: ConfigEntry<String>,
  python_worker_module: ConfigEntry<String>,
  python_auth_socket_timeout: ConfigEntry<Duration>,
  python_worker_faulthandler_enabled: ConfigEntry<bool>,
}

impl Python {
  pub fn new() -> Self {
    Python {
      python_worker_reuse:
        ConfigBuilder::new("blitz.python.worker.reuse")
          .version("1.2.0")
          .boolean_conf()
          .create_with_default(true),
      python_task_kill_timeout:
        ConfigBuilder::new("blitz.python.task.kill_timeout")
          .version("2.2.2")
          .time_conf() // TODO
          .create_with_default(Duration::from_secs(2)),
      python_use_daemon:
        ConfigBuilder::new("blitz.python.use.daemon")
          .version("2.3.0")
          .boolean_conf()
          .create_with_default(true),
      python_log_info:
        ConfigBuilder::new("blitz.executor.python.worker.log.details")
          .version("3.5.0")
          .boolean_conf()
          .create_with_default(false),
      python_daemon_module:
        ConfigBuilder::new("blitz.python.daemon.module")
          .version("2.4.0")
          .string_conf()
          .create_with_default("str".to_string()), // TODO
      python_worker_module:
        ConfigBuilder::new("blitz.python.worker.module")
          .version("2.4..0")
          .string_conf()
          .create_with_default("str".to_string()), // TODO
      python_auth_socket_timeout:
        ConfigBuilder::new("blitz.python.authenticate.socket_timeout")
          .version("3.1.0")
          .time_conf()
          .create_with_default(Duration::from_secs(15)), // TODO
      python_worker_faulthandler_enabled:
        ConfigBuilder::new("blitz.python.worker.faulthandler.enabled")
          .doc("When true, Python workers set up the faulthandler for the case when the Python
            worker exits unexpectedly (crashed), and shows the stack trace of the moment the
            Python worker crashes in the error message if captured successfully.")
          .version("3.2.0")
          .boolean_conf()
          .create_with_default(false),
    }
  }
}