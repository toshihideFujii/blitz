#![allow(dead_code)]

// An implementation of the RecordVisitor which verifies a
// sequence of records associated with a block, following the 
// FDR mode log format's specifications.

use super::fdr_records::{*};

enum State {
  Unknown,
  BufferExtents,
  NewBuffer,
  WallClockTime,
  PidEntry,
  NewCpuId,
  TscWrap,
  CustomEvent,
  TypedEvent,
  Function,
  CallArg,
  EndOfBuffer,
  StateMax
}

struct BlockVerifier {
  current_record: State
}

impl BlockVerifier {
  pub fn new() -> Self {
    BlockVerifier { current_record: State::Unknown }
  }
  
  pub fn verify() {}
  pub fn reset() {}
}

impl RecordVisitor for BlockVerifier {
  fn visit_buffer_extents(&mut self, _r: BufferExtents) -> Result<(), String> { Ok(()) }
  fn visit_wallclock_record(&mut self, _r: WallClockRecord) -> Result<(), String> { Ok(()) }
  fn visit_new_cpu_id_record(&mut self, _r: NewCpuIdRecord) -> Result<(), String> { Ok(()) }
  fn visit_tsc_wrap_record(&mut self, _r: TscWrapRecord) -> Result<(), String> { Ok(()) }
  fn visit_custom_event_record(&mut self, _r: CustomEventRecord) -> Result<(), String> { Ok(()) }
  fn visit_call_arg_record(&mut self, _r: CallArgRecord) -> Result<(), String> { Ok(()) }
  fn visit_pid_record(&mut self, _r: PidRecord) -> Result<(), String> { Ok(()) }
  fn visit_new_buffer_record(&mut self, _r: NewBufferRecord) -> Result<(), String> { Ok(()) }
  fn visit_end_buffer_record(&mut self, _r: EndBufferRecord) -> Result<(), String> { Ok(()) }
  fn visit_function_record(&mut self, _r: FunctionRecord) -> Result<(), String> { Ok(()) }
  fn visit_custom_event_record_v5(&mut self, _r: CustomEventRecordV5) -> Result<(), String> { Ok(()) }
  fn visit_typed_event_record(&mut self, _r: TypedEventRecord) -> Result<(), String> { Ok(()) }
}