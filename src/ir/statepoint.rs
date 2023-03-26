#![allow(dead_code)]

enum StatepointFlags {
  None,
  GCTransition,
  DeoptLiveIn,
  MaskAll
}

struct GCStatepointInst {}

impl GCStatepointInst {
  pub fn get_id() {}
  pub fn get_num_patch_bytes() {}
  pub fn get_num_call_args() {}
  pub fn get_flags() {}
}