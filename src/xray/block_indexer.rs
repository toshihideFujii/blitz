#![allow(dead_code)]

// An implementation of the RecordVisitor which generates a mapping
// between a thread and a range of records representing a block.

use crate::adt::dense_map::DenseMap;
use super::fdr_records::{*};

struct Block {
  process_id: u64,
  thread_id: i32,
  wall_clock_time: Option<WallClockRecord>,
  records: Vec<Box<dyn Record>>
}

// The BlockIndexer will gather all related records associated with a
// process+thread and group them by 'Block'.
struct BlockIndexer {
  indices: DenseMap<(i32, i32), Vec<Block>>,
  current_block: Block
}

impl BlockIndexer {
  pub fn new(indices: DenseMap<(i32, i32), Vec<Block>>) -> Self {
    BlockIndexer {
      indices: indices,
      current_block: Block {
        process_id: 0,
        thread_id: 0,
        wall_clock_time: None,
        records: Vec::new()
      }
    }
  }

  // The flush function will clear out the current state of the visitor,
  // to allow for explicitly flushing a block's records to the currently
  // recognized thread and process combination.
  pub fn flush(&mut self) {
    // TBD
    self.current_block.process_id = 0;
    self.current_block.thread_id = 0;
    self.current_block.records = Vec::new();
    self.current_block.wall_clock_time = None;
  }
}

impl RecordVisitor for BlockIndexer {
  fn visit_buffer_extents(&mut self, _r: BufferExtents) -> Result<(), String> {
    Ok(())
  }

  fn visit_wallclock_record(&mut self, r: WallClockRecord) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    //self.current_block.wall_clock_time = Some(r);
    Ok(())
  }

  fn visit_new_cpu_id_record(&mut self, r: NewCpuIdRecord) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_tsc_wrap_record(&mut self, r: TscWrapRecord) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_custom_event_record(&mut self, r: CustomEventRecord) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_call_arg_record(&mut self, r: CallArgRecord) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_pid_record(&mut self, r: PidRecord) -> Result<(), String> {
    self.current_block.process_id = r.pid() as u64;
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_new_buffer_record(&mut self, r: NewBufferRecord) -> Result<(), String> {
    if !self.current_block.records.is_empty() {
      self.flush();
    }
    self.current_block.thread_id = r.tid();
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_end_buffer_record(&mut self, r: EndBufferRecord) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_function_record(&mut self, r: FunctionRecord) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_custom_event_record_v5(&mut self, r: CustomEventRecordV5) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    Ok(())
  }

  fn visit_typed_event_record(&mut self, r: TypedEventRecord) -> Result<(), String> {
    self.current_block.records.push(Box::new(r));
    Ok(())
  }
}