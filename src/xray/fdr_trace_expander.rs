#![allow(dead_code)]

// We define an FDR record visitor which can re-constitute XRayRecord
// instances from a sequence of FDR mode records in arrival order into
// a collection.

use super::{fdr_records::{*}, xray_record::{XRayRecord, RecordTypes}};

struct TraceExpander {
  c: fn(&XRayRecord),
  pid: i32,
  tid: i32,
  base_tsc: u64,
  cpu_id: u16,
  log_version: u16,
  building_record: bool,
  ignoring_records: bool,
  current_record: XRayRecord
}

impl TraceExpander {
  pub fn new(c: fn(&XRayRecord)) -> Self {
    TraceExpander {
      c: c,
      pid: 0, tid: 0, base_tsc: 0, cpu_id: 0, log_version: 0,
      building_record: false, ignoring_records: false,
      current_record: XRayRecord {
        record_type: 0, cpu: 0,
        type_: RecordTypes::Enter,
        func_id: 0, tsc: 0, tid: 0, pid: 0,
        call_args: Vec::new(),
        data: String::new()
      }
    }
  }

  pub fn flush(&mut self) -> Result<(), String> {
    self.reset_current_record();
    Ok(())
  }

  fn reset_current_record(&mut self) {
    if self.building_record {
      (self.c)(&self.current_record);
    }
    self.building_record = false;
    self.current_record.call_args.clear();
    self.current_record.data.clear();
  }
}

impl RecordVisitor for TraceExpander {
  fn visit_buffer_extents(&mut self, _r: BufferExtents) -> Result<(), String> {
    self.reset_current_record();
    Ok(())
  }

  fn visit_wallclock_record(&mut self, _r: WallClockRecord) -> Result<(), String> {
    Ok(())
  }

  fn visit_new_cpu_id_record(&mut self, r: NewCpuIdRecord) -> Result<(), String> {
    self.cpu_id = r.cpu_id();
    self.base_tsc = r.tsc();
    Ok(())
  }

  fn visit_tsc_wrap_record(&mut self, r: TscWrapRecord) -> Result<(), String> {
    self.base_tsc = r.tsc();
    Ok(())
  }

  fn visit_custom_event_record(&mut self, r: CustomEventRecord) -> Result<(), String> {
    self.reset_current_record();
    if !self.ignoring_records {
      self.current_record.tsc = r.tsc();
      self.current_record.cpu = r.cpu();
      self.current_record.pid = self.pid as u32;
      self.current_record.tid = self.tid as u32;
      self.current_record.type_ = RecordTypes::CustomEvent;
      self.current_record.data = r.data().clone();
      self.building_record = true;
    }
    Ok(())
  }

  fn visit_call_arg_record(&mut self, r: CallArgRecord) -> Result<(), String> {
    self.current_record.call_args.push(r.arg());
    self.current_record.type_ = RecordTypes::EnterArg;
    Ok(())
  }

  fn visit_pid_record(&mut self, r: PidRecord) -> Result<(), String> {
    self.pid = r.pid();
    Ok(())
  }

  fn visit_new_buffer_record(&mut self, r: NewBufferRecord) -> Result<(), String> {
    if self.ignoring_records {
      self.ignoring_records = false;
    }
    self.tid = r.tid();
    if self.log_version == 2 {
      self.pid = r.tid();
    }
    Ok(())
  }

  fn visit_end_buffer_record(&mut self, _r: EndBufferRecord) -> Result<(), String> {
    self.ignoring_records = true;
    self.reset_current_record();
    Ok(())
  }

  fn visit_function_record(&mut self, r: FunctionRecord) -> Result<(), String> {
    self.reset_current_record();
    if !self.ignoring_records {
      self.base_tsc += r.delta() as u64;
      self.current_record.type_ = r.record_type().clone();
      self.current_record.func_id = r.function_id();
      self.current_record.tsc = self.base_tsc;
      self.current_record.pid = self.pid as u32;
      self.current_record.tid = self.tid as u32;
      self.current_record.cpu = self.cpu_id;
      self.building_record = true;
    }
    Ok(())
  }

  fn visit_custom_event_record_v5(&mut self, r: CustomEventRecordV5) -> Result<(), String> {
    self.reset_current_record();
    if !self.ignoring_records {
      self.base_tsc += r.delta() as u64;
      self.current_record.tsc = self.base_tsc;
      self.current_record.cpu = self.cpu_id;
      self.current_record.pid = self.pid as u32;
      self.current_record.tid = self.tid as u32;
      self.current_record.type_ = RecordTypes::CustomEvent;
      self.current_record.data = r.data().clone();
      self.building_record = true;
    }
    Ok(())
  }

  fn visit_typed_event_record(&mut self, r: TypedEventRecord) -> Result<(), String> {
    self.reset_current_record();
    if !self.ignoring_records {
      self.base_tsc += r.delta() as u64;
      self.current_record.tsc = self.base_tsc;
      self.current_record.cpu = self.cpu_id;
      self.current_record.pid = self.pid as u32;
      self.current_record.tid = self.tid as u32;
      self.current_record.record_type = r.event_type();
      self.current_record.type_ = RecordTypes::TypedEvent;
      self.current_record.data = r.data().clone();
      self.building_record = true;
    }
    Ok(())
  }
}