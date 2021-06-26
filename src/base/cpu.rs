// CPU
// Query information about the processor.
// This class also has static methods for the architecture specific functions.
// Add methods here to cope with differences between the supported architectures.
// For each architecture the file cpu_<arch> contains the implementation of these functions.

// ARM-specific part codes
const ARM_CORTEX_A5: i32 = 0xc05;
const ARM_CORTEX_A7: i32 = 0xc07;
const ARM_CORTEX_A8: i32 = 0xc08;
const ARM_CORTEX_A9: i32 = 0xc09;
const ARM_CORTEX_A12: i32 = 0xc0c;
const ARM_CORTEX_A15: i32 = 0xc0f;

// Denver-specific part code
const NVIDIA_DENVER_V10: i32 = 0x002;

const UNKNOWN_CACHE_LINE_SIZE: i32 = 0;

struct Cpu {
  stepping_: i32,
  model_: i32,
  ext_model_: i32,
  family_: i32,
  ext_family_: i32,
  cpu_type_: i32,
  implementer_: i32,
  architecture_: i32,
  variant_: i32,
  part_: i32,
  icache_line_size_: i32,
  dcache_line_size_: i32,
  has_fpu_: bool,
  has_cmov_: bool,
  has_sahf_: bool,
  has_mmx_: bool,
  has_sse_: bool,
  has_sse2_: bool,
  has_sse3_: bool,
  has_ssse3_: bool,
  has_sse41_: bool,
  has_sse42_: bool,
  is_atom_: bool,
  has_osxsave_: bool,
  has_avx_: bool,
  has_avx2_: bool,
  has_fma3_: bool,
  has_bmi1_: bool,
  has_bmi2_: bool,
  has_lzcnt_: bool,
  has_popcnt_: bool,
  has_idiva_: bool,
  has_neon_: bool,
  has_thumb2_: bool,
  has_vfp_: bool,
  has_vfp3_: bool,
  has_vfp3_d32_: bool,
  has_jscvt_: bool,
  is_fp64_mode_: bool,
  has_non_stop_time_stamp_counter_: bool,
  is_running_in_vm_: bool,
  has_msa_: bool,
}

impl Cpu {
  // x86 CPUID information
  fn vendor() {}
  fn stepping(&self) -> i32 {
    self.stepping_
  }
  fn model(&self) -> i32 {
    self.model_
  }
  fn ext_model() {}
  fn family() {}
  fn ext_family() {}
  fn cpu_type() {}

  // arm implementer/part information
  fn implementer() {}
  fn architecture() {}
  fn variant() {}
  fn part() {}

  // General features
  fn has_fpu() {}
  fn icache_line_size() {}
  fn dcache_line_size() {}

  // x86 features
  fn has_cmov(&self) -> bool {
    self.has_cmov_
  }
  fn has_sahf(&self) -> bool {
    self.has_sahf_
  }
  fn has_mmx(&self) -> bool {
    self.has_mmx_
  }
  fn has_sse(&self) -> bool {
    self.has_sse_
  }
  fn has_sse2(&self) -> bool {
    self.has_sse2_
  }
  fn has_sse3(&self) -> bool {
    self.has_sse3_
  }
  fn has_ssse3(&self) -> bool {
    self.has_ssse3_
  }
  fn has_sse41(&self) -> bool {
    self.has_sse41_
  }
  fn has_sse42(&self) -> bool {
    self.has_sse42_
  }
  fn has_osxsave(&self) -> bool {
    self.has_osxsave_
  }
  fn has_avx(&self) -> bool {
    self.has_avx_
  }
  fn has_avx2(&self) -> bool {
    self.has_avx2_
  }
  fn has_fma3(&self) -> bool {
    self.has_fma3_
  }
  fn has_bmi1(&self) -> bool {
    self.has_bmi1_
  }
  fn has_bmi2(&self) -> bool {
    self.has_bmi2_
  }
  fn has_lzcnt(&self) -> bool {
    self.has_lzcnt_
  }
  fn has_popcnt(&self) -> bool {
    self.has_popcnt_
  }
  fn is_atom(&self) -> bool {
    self.is_atom_
  }
  fn has_non_stop_time_stamp_counter(&self) -> bool {
    self.has_non_stop_time_stamp_counter_
  }

  // arm features
  fn has_idiva() {}
  fn has_neon() {}
  fn has_thumb2() {}
  fn has_vfp() {}
  fn has_vfp3() {}
  fn has_vfp3_d32() {}
  fn hasjscvt() {}

  // mips features
  fn is_fp64_mode() {}
  fn has_msa() {}
}
