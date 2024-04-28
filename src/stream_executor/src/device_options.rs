#![allow(dead_code)]

use std::collections::HashMap;

pub struct DeviceOptions {
  non_portable_tags: HashMap<String, String>,
  flags: u64
}

impl DeviceOptions {
  const FO_NOT_RECLAIM_STACK_ALLOCATION: u64 = 0x1;
  const SCHEDULE_SPIN: u64 = 0x02;
  const SCHEDULE_YIELD: u64 = 0x04;
  const SCHEDULE_BLOCKING_SYNC: u64 = 0x08;
  const MASK: u64 = 0xf;

  pub fn new(flags: u64) -> Self {
    debug_assert!(flags & DeviceOptions::MASK == flags);
    DeviceOptions { non_portable_tags: HashMap::new(), flags: flags }
  }

  // Factory for the default set of device options.
  pub fn default() -> Self {
    DeviceOptions::new(0)
  }

  pub fn flags(&self) -> u64 {
    self.flags
  }

  pub fn to_string(&self) -> String {
    let mut flags_on = Vec::new();
    if self.flags & DeviceOptions::FO_NOT_RECLAIM_STACK_ALLOCATION != 0 {
      flags_on.push("DO_NOT_RECLAIM_STACK_ALLOCATION".to_string());
    }
    if self.flags & DeviceOptions::SCHEDULE_SPIN != 0 {
      flags_on.push("SCHEDULE_SPIN".to_string());
    }
    if self.flags & DeviceOptions::SCHEDULE_YIELD != 0 {
      flags_on.push("SCHEDULE_YIELD".to_string());
    }
    if self.flags & DeviceOptions::SCHEDULE_BLOCKING_SYNC != 0 {
      flags_on.push("SCHEDULE_BLOKING_SYNC".to_string())
    }

    if flags_on.is_empty() {
      return "none".to_string();
    } else {
      flags_on.join("|").as_str().to_string()
    }
  }
}