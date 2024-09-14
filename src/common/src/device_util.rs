use stream_executor::stream_executor::StreamExecutor;

// Returns a string that represents the device in terms of platform and ordinal;
// e.g. the first CUDA device will be "cuda:0"
pub fn device_identifier(se: &dyn StreamExecutor) -> String {
  let mut result = String::new();
  result.push_str(se.get_platform().name().as_str());
  result.push(':');
  result.push_str(se.device_ordinal().to_string().as_str());
  result
}