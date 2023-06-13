
#![allow(dead_code)]

use crate::support::data_extractor::DataExtractor;
use super::xray_record::XRayFileHeader;

struct RecordProducer {}

struct FileBasedRecordProducer {
  header: XRayFileHeader,
  de: DataExtractor,
  offset_ptr: u64,
  curr_buffer_bytes: u32
}

impl FileBasedRecordProducer {
  pub fn new(fh: XRayFileHeader, de: DataExtractor, op: u64) -> Self {
    FileBasedRecordProducer {
      header: fh,
      de: de,
      offset_ptr: op,
      curr_buffer_bytes: 0
    }
  }

  // This producer encapsulates the logic for loading a file-backed
  // RecordProducer hidden behind a DataExtractor.
  pub fn produce() {}
  pub fn find_next_buffer_extent() {}
}