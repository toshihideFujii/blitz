#![allow(dead_code)]

use stream_executor::stream_executor::StreamExecutor;

pub fn device_identifier(se: &StreamExecutor) -> String {
  let mut result = String::new();
  result.push_str(se.platform().name());
  result.push(':');
  result.push_str(se.device_ordinal().to_string().as_str());
  result
}