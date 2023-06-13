#![allow(dead_code)]

// An implementation of the RecordVisitor which formats a block
// of records for easier human consumption.

use super::{fdr_records::{*}, record_printer::RecordPrinter};

#[derive(Debug, PartialEq)]
enum State {
  Start,
  Preamble,
  Metadata,
  Function,
  Arg,
  CustomEvent,
  End
}

struct BlockPrinter {
  rp: RecordPrinter,
  current_state: State
}

impl BlockPrinter {
  pub fn new(rp: RecordPrinter) -> Self {
    BlockPrinter { rp: rp, current_state: State::Start }
  }
}

impl RecordVisitor for BlockPrinter {
  fn visit_buffer_extents(&mut self, r: BufferExtents) -> Result<(), String> {
    self.current_state = State::Preamble;
    self.rp.visit_buffer_extents(r)
  }

  fn visit_wallclock_record(&mut self, r: WallClockRecord) -> Result<(), String> {
    self.current_state = State::Preamble;
    self.rp.visit_wallclock_record(r)
  }

  fn visit_new_cpu_id_record(&mut self, r: NewCpuIdRecord) -> Result<(), String> {
    if self.current_state == State::Preamble {
      // TBD
    }
    if self.current_state == State::Function {
      // TBD
    }
    self.current_state = State::Metadata;
    self.rp.visit_new_cpu_id_record(r)
  }

  fn visit_tsc_wrap_record(&mut self, r: TscWrapRecord) -> Result<(), String> {
    if self.current_state == State::Function {
      // TBD
    }
    self.current_state = State::Metadata;
    self.rp.visit_tsc_wrap_record(r)
  }

  fn visit_custom_event_record(&mut self, r: CustomEventRecord) -> Result<(), String> {
    if self.current_state == State::Metadata {
      // TBD
    }
    self.current_state = State::CustomEvent;
    self.rp.visit_custom_event_record(r)
  }

  fn visit_call_arg_record(&mut self, r: CallArgRecord) -> Result<(), String> {
    self.current_state = State::Arg;
    self.rp.visit_call_arg_record(r)
  }

  fn visit_pid_record(&mut self, r: PidRecord) -> Result<(), String> {
    self.current_state = State::Preamble;
    self.rp.visit_pid_record(r)
  }

  fn visit_new_buffer_record(&mut self, r: NewBufferRecord) -> Result<(), String> {
    if self.current_state == State::Start {
      // TBD
    }
    self.current_state = State::Preamble;
    self.rp.visit_new_buffer_record(r)
  }

  fn visit_end_buffer_record(&mut self, r: EndBufferRecord) -> Result<(), String> {
    self.current_state = State::End;
    self.rp.visit_end_buffer_record(r)
  }

  fn visit_function_record(&mut self, r: FunctionRecord) -> Result<(), String> {
    if self.current_state == State::Metadata {
      // TBD
    }
    self.current_state = State::Function;
    self.rp.visit_function_record(r)
  }

  fn visit_custom_event_record_v5(&mut self, r: CustomEventRecordV5) -> Result<(), String> {
    if self.current_state == State::Metadata {
      // TBD
    }
    self.current_state = State::CustomEvent;
    self.rp.visit_custom_event_record_v5(r)
  }

  fn visit_typed_event_record(&mut self, r: TypedEventRecord) -> Result<(), String> {
    if self.current_state == State::Metadata {
      // TBD
    }
    self.current_state = State::CustomEvent;
    self.rp.visit_typed_event_record(r)
  }
}