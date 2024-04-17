#![allow(dead_code)]

use common::blitz_data::OpMetadata;

pub fn op_metadata_to_string(metadata: &OpMetadata, only_op_name: bool) -> String {
  let mut result = String::new();
  if only_op_name {
    if !metadata.op_name().is_empty() {
      result.push_str("op_name=");
      result.push_str(metadata.op_name().as_str());
      result.push('\n');
      return result;
    } else {
      return "".to_string();
    }
  }
  if !metadata.op_type().is_empty() {
    result.push_str("op_type=");
    result.push_str(metadata.op_type().as_str());
    result.push('\n'); 
  }
  if !metadata.op_name().is_empty() {
    result.push_str("op_name=");
    result.push_str(metadata.op_name().as_str());
    result.push('\n');
  }
  if !metadata.source_file().is_empty() {
    result.push_str("source_file=");
    result.push_str(metadata.source_file().as_str());
    result.push('\n');
  }
  if metadata.source_line() != 0 {
    result.push_str("source_line=");
    result.push_str(metadata.source_line().to_string().as_str());
    result.push('\n');
  }
  if !metadata.profile_type().is_empty() {
    result.push_str("profile_type=");
    result.push_str(metadata.profile_type().as_str());
    result.push('\n');
  }
  if !metadata.deduplicated_name().is_empty() {
    result.push_str("deduplicated_name=");
    result.push_str(metadata.deduplicated_name().as_str());
    result.push('\n');
  }
  if metadata.preserve_layout() {
    result.push_str("preserve_layout=true");
  }
  result
}