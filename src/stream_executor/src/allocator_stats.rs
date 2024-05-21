#![allow(dead_code)]

pub struct AllocatorStats {
  num_allocs: i64,
  bytes_in_use: i64,
  peak_bytes_in_use: i64,
  largest_alloc_size: i64,
  bytes_limit: Option<i64>,
  bytes_reserved: i64,
  peak_bytes_reserved: i64,
  bytes_reservable_limit: Option<i64>,
  largest_free_block_bytes: i64
}

impl AllocatorStats {
  pub fn new() -> Self {
    AllocatorStats {
      num_allocs: 0,
      bytes_in_use: 0,
      peak_bytes_in_use: 0,
      largest_alloc_size: 0,
      bytes_limit: None,
      bytes_reserved: 0,
      peak_bytes_reserved: 0,
      bytes_reservable_limit: None,
      largest_free_block_bytes: 0
    }
  }

  pub fn debug_string(&self) -> String {
    let mut result = "".to_string();

    result.push_str("Limit: ");
    result.push_str(self.bytes_limit.unwrap_or_default().to_string().as_str());
    result.push('\n');
    result.push_str("InUse: ");
    result.push_str(self.bytes_in_use.to_string().as_str());
    result.push('\n');
    result.push_str("MaxInUse: ");
    result.push_str(self.peak_bytes_in_use.to_string().as_str());
    result.push('\n');
    result.push_str("NumAllocs: ");
    result.push_str(self.num_allocs.to_string().as_str());
    result.push('\n');
    result.push_str("MaxAllocSize: ");
    result.push_str(self.largest_alloc_size.to_string().as_str());
    result.push('\n');
    result.push_str("Reserved: ");
    result.push_str(self.bytes_reserved.to_string().as_str());
    result.push('\n');
    result.push_str("PeakReserved: ");
    result.push_str(self.peak_bytes_reserved.to_string().as_str());
    result.push('\n');
    result.push_str("LargestFreeBlock: ");
    result.push_str(self.largest_free_block_bytes.to_string().as_str());
    result.push('\n');

    result
  }
}