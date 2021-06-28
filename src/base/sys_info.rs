use num_cpus;
use rsys;

pub struct SysInfo {}

impl SysInfo {
  // Returns the number of logical processors/core on the current machine.
  pub fn number_of_processors() -> usize {
    println!("SysInfo::number_of_processors: {}", num_cpus::get());
    num_cpus::get()
  }

  // Returns the number of bytes of phisical memory on the current machine.
  pub fn amount_of_physical_memory() -> usize {
    let result = rsys::memory();
    if result.is_ok() {
      let amount = result.unwrap_or(0);
      println!("SysInfo::amount_of_physical_memory: {}", amount);
      amount
    } else {
      0
    }
  }

  // Returns the number of bytes of virtual memory of this process.
  // A return value of zero means there is no limit on the available virtual memory.
  //pub fn amount_of_virtual_memory() -> std::result::Result<(rlimit::Rlim, rlimit::Rlim), std::io::Error> {}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sysinfo_number_of_processors() {
    assert!(0 < SysInfo::number_of_processors());
  }

  #[test]
  fn test_sysinfo_amount_of_physical_memory() {
    assert!(0 < SysInfo::amount_of_physical_memory());
  }
}
