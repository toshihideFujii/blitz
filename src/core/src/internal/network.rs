#![allow(dead_code)]

use std::time::Duration;
use super::{
  config_entry::ConfigEntry,
  config_builder::ConfigBuilder
};

pub struct Network {
  network_crypto_sasl_fallback: ConfigEntry<bool>,
  network_crypto_enabled: ConfigEntry<bool>,
  network_remote_read_nio_buffer_conversion: ConfigEntry<bool>,
  network_timeout: ConfigEntry<Duration>,
  network_timeout_interval: ConfigEntry<Duration>,
  rpc_ask_timeout: ConfigEntry<String>,
  rpc_connect_threads: ConfigEntry<i64>,
  rpc_io_num_connections_per_peer: ConfigEntry<i64>,
  rpc_io_threads: ConfigEntry<i64>,
  rpc_lookup_timeout: ConfigEntry<String>,
  rpc_message_max_size: ConfigEntry<i64>,
  rpc_netty_dispatcher_num_threads: ConfigEntry<i64>,
}

impl Network {
  pub fn new() -> Self {
    Network {
      network_crypto_sasl_fallback:
        ConfigBuilder::new("blitz.network.crypto.sasl_fallback")
          .version("2.2.0")
          .boolean_conf()
          .create_with_default(true),
      network_crypto_enabled:
        ConfigBuilder::new("blitz.network.crypto.enabled")
          .version("2.2.0")
          .boolean_conf()
          .create_with_default(false),
      network_remote_read_nio_buffer_conversion:
        ConfigBuilder::new("blitz.network.remote_read_nio_byffer_conversion")
          .version("2.4.0")
          .boolean_conf()
          .create_with_default(false),
      network_timeout:
        ConfigBuilder::new("blitz.network.timeout")
          .version("1.3.0")
          .time_conf()
          .create_with_default(Duration::from_secs(120)),
      network_timeout_interval:
        ConfigBuilder::new("blitz.network.timeout_interval")
          .version("1.3.2")
          .time_conf()
          .create_with_default(Duration::from_secs(60)),
      rpc_ask_timeout:
        ConfigBuilder::new("blitz.rpc.ask_timeout")
          .version("1.4.0")
          .string_conf()
          .create_with_default("".to_string()),
      rpc_connect_threads:
        ConfigBuilder::new("blitz.rpc.connect.threads")
          .version("1.6.0")
          .int_conf()
          .create_with_default(64),
      rpc_io_num_connections_per_peer:
        ConfigBuilder::new("blitz.rpc.io.num_connections_per_peer")
          .version("1.6.0")
          .int_conf()
          .create_with_default(1),
      rpc_io_threads:
        ConfigBuilder::new("blitz.rpc.io.threads")
          .version("1.6.0")
          .int_conf()
          .create_with_default(0),
      rpc_lookup_timeout:
        ConfigBuilder::new("blitz.rpc.lookup_timeout")
          .version("1.4.0")
          .string_conf()
          .create_with_default("".to_string()),
      rpc_message_max_size:
        ConfigBuilder::new("blitz.rpc.message.max_size")
          .version("2.0.0")
          .int_conf()
          .create_with_default(128),
      rpc_netty_dispatcher_num_threads:
        ConfigBuilder::new("blitz.rpc.netty.dispatcher.num_threads")
          .version("1.6.0")
          .int_conf()
          .create_with_default(0),
    }
  }
}