#![allow(dead_code)]

use std::time::Duration;
use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct UI {
  ui_show_console_progress: ConfigEntry<bool>,
  ui_console_progress_update_interval: ConfigEntry<Duration>,
  ui_enabled: ConfigEntry<bool>,
  ui_port: ConfigEntry<i64>,
}

impl UI {
  pub fn new() -> Self {
    UI {
      ui_show_console_progress:
        ConfigBuilder::new("blitz.ui.show_console_progress")
          .version("1.2.1")
          .boolean_conf()
          .create_with_default(false),
      ui_console_progress_update_interval:
        ConfigBuilder::new("blitz.ui.console_progress.update.interval")
          .version("2.1.0")
          .time_conf()
          .create_with_default(Duration::from_millis(200)),
      ui_enabled:
        ConfigBuilder::new("blitz.ui.enabled")
          .doc("Whether to run the web UI for the Blitz application.")
          .version("1.1.1")
          .boolean_conf()
          .create_with_default(true),
      ui_port:
        ConfigBuilder::new("blitz.ui.port")
          .doc("Port for your app's dashboard, which shows memory and workload data.")
          .version("0.7.0")
          .int_conf()
          .create_with_default(4040)
    }
  }
}