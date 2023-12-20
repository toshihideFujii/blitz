#![allow(dead_code)]

use std::fmt::{Formatter, Display, Result};
use super::memory_mode::MemoryMode;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct StorageLevel {
  use_disk: bool,
  use_memory: bool,
  use_off_heap: bool,
  deserialized: bool,
  replication: u64
}

impl StorageLevel {
  pub fn new(
    use_disk: bool,
    use_memory: bool,
    use_off_heap: bool,
    deserialized: bool,
    replication: u64) -> Self
  {
    StorageLevel {
      use_disk: use_disk,
      use_memory: use_memory,
      use_off_heap: use_off_heap,
      deserialized: deserialized,
      replication: replication
    }
  }

  pub fn use_disk(&self) -> bool {
    self.use_disk
  }

  pub fn use_memory(&self) -> bool {
    self.use_memory
  }

  pub fn use_off_heap(&self) -> bool {
    self.use_off_heap
  }

  pub fn deserialized(&self) -> bool {
    self.deserialized
  }

  pub fn memory_mode(&self) -> MemoryMode {
    if self.use_off_heap {
      MemoryMode::OffHeap
    } else {
      MemoryMode::OnHeap
    }
  }

  pub fn is_valid(&self) -> bool {
    (self.use_memory || self.use_disk) && self.replication > 0
  }

  pub fn to_int(&self) -> u64 {
    let mut val: u64 = 0;
    if self.use_disk { val |= 8; }
    if self.use_memory { val |= 4; }
    if self.use_off_heap { val |= 2; }
    if self.deserialized { val |= 1; }
    val
  }

  pub fn write_external() {}
  pub fn read_external() {}
}

impl Display for StorageLevel {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "disk: {}, memory: {}, off_heap: {}, deserialized: {}, replicas: {}",
      self.use_disk, self.use_memory, self.use_off_heap, self.deserialized, self.replication)
  }
}

const NONE: StorageLevel = StorageLevel {
  use_disk: false,
  use_memory: false,
  use_off_heap: false,
  deserialized: false,
  replication: 1
};

const DISK_ONLY: StorageLevel = StorageLevel {
  use_disk: true,
  use_memory: false,
  use_off_heap: false,
  deserialized: false,
  replication: 1
};

const DISK_ONLY_2: StorageLevel = StorageLevel {
  use_disk: true,
  use_memory: false,
  use_off_heap: false,
  deserialized: false,
  replication: 2
};

const DISK_ONLY_3: StorageLevel = StorageLevel {
  use_disk: true,
  use_memory: false,
  use_off_heap: false,
  deserialized: false,
  replication: 3
};

const MEMORY_ONLY: StorageLevel = StorageLevel {
  use_disk: false,
  use_memory: true,
  use_off_heap: false,
  deserialized: true,
  replication: 1
};

const MEMORY_ONLY_2: StorageLevel = StorageLevel {
  use_disk: false,
  use_memory: true,
  use_off_heap: false,
  deserialized: true,
  replication: 2
};

const MEMORY_ONLY_SER: StorageLevel = StorageLevel {
  use_disk: false,
  use_memory: true,
  use_off_heap: false,
  deserialized: false,
  replication: 1
};

const MEMORY_ONLY_SER_2: StorageLevel = StorageLevel {
  use_disk: false,
  use_memory: true,
  use_off_heap: false,
  deserialized: false,
  replication: 2
};

const MEMORY_AND_DISK: StorageLevel = StorageLevel {
  use_disk: true,
  use_memory: true,
  use_off_heap: false,
  deserialized: true,
  replication: 1
};

const MEMORY_AND_DISK_2: StorageLevel = StorageLevel {
  use_disk: true,
  use_memory: true,
  use_off_heap: false,
  deserialized: true,
  replication: 2
};

const MEMORY_AND_DISK_SER: StorageLevel = StorageLevel {
  use_disk: true,
  use_memory: true,
  use_off_heap: false,
  deserialized: false,
  replication: 1
};

const MEMORY_AND_DISK_SER_2: StorageLevel = StorageLevel {
  use_disk: true,
  use_memory: true,
  use_off_heap: false,
  deserialized: false,
  replication: 2
};

const OFF_HEAP: StorageLevel = StorageLevel {
  use_disk: true,
  use_memory: true,
  use_off_heap: true,
  deserialized: false,
  replication: 1
};