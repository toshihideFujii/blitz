#![allow(dead_code)]

use super::fdr_records::Record;

struct RecordConsumer {}

// This consumer will collect all the records into a vector of records,
// in arrival order.
struct LogBuilderConsumer {
  records: Vec<Box<dyn Record>>
}

impl LogBuilderConsumer {
  pub fn new(records: Vec<Box<dyn Record>>) -> Self {
    LogBuilderConsumer { records: records }
  }

  pub fn consume(&mut self, record: Option<Box<dyn Record>>) -> Result<(), &str> {
    if record.is_none() {
      return Err("Must not call RecordConsumer::consume() with a null pointer.");
    }
    self.records.push(record.unwrap());
    Ok(())
  }
}

struct PipelineConsumer {}
impl PipelineConsumer {
  pub fn new() {}
  pub fn consume() {}
}