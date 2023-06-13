#![allow(dead_code)]

// Test a utility that can write out xray FDR mode formatted trace files.

use super::fdr_records::{*};

// The FDRTraceWriter allows us to hand-craft an xray flight data recorder
// mode log file.
struct FDRTraceWriter {}

impl FDRTraceWriter {
  pub fn new() {}
}

impl RecordVisitor for FDRTraceWriter {
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