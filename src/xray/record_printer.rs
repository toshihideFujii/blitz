#![allow(dead_code)]

// An implementation of the RecordVisitor which prints an individual
// record's data in an adhoc format, suitable for human inspection.

use std::io::{BufWriter, Stdout, stdout};

use super::fdr_records::{*};

pub struct RecordPrinter {
  writer: BufWriter<Stdout>,
  delim: String
}

impl RecordPrinter {
  pub fn new() -> Self {
    RecordPrinter { writer: BufWriter::new(stdout()), delim: String::new() }
  }
}

impl RecordVisitor for RecordPrinter {
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