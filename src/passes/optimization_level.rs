#![allow(dead_code)]

// This header enumerates Blitz-provided high-level optimization levels.
// Each level has a specific goal and rationale.

#[derive(PartialEq, Eq)]
pub struct OptimizationLevel {
  speed_level: u32,
  size_level: u32
}

impl OptimizationLevel {
  pub fn new(speed_level: u32, size_level: u32) -> Self {
    OptimizationLevel { speed_level: speed_level, size_level: size_level }
  }

  pub fn is_optimizing_for_speed(&self) -> bool {
    self.size_level == 0 && self.speed_level > 0
  }

  pub fn is_optimizing_for_size(&self) -> bool {
    self.size_level > 0
  }

  pub fn get_speed_up_level(&self) -> u32 {
    self.speed_level
  }

  pub fn get_size_level(&self) -> u32 {
    self.size_level
  }
}

// Disable as many optimizations as possible.
// This doesn't completely disable the optimizer in all cases, for example
// always_inline functions can be required to be inlined for correctness.
pub fn create_optimization_level_o0() -> OptimizationLevel {
  OptimizationLevel::new(0, 0)
}

// Optimize quickly without destroying debuggability.
//
// This level is tuned to produce a result from the optimizer as auickly as
// possible and to avoid destroying debuggability.
// This tends to result in a very good development mode where the compiled code
// will be immediately executed as part of testing.
// As a consequence, where possible, we would like to produce efficient-to-execute
// code, but not if it significantly slows down compilation or would prevent
// even basic debugging of the resulting binary.
//
// As an example, complex loop transformation such as versioning, vectorization,
// or fusion don't make sense here due to the degree to which the executed code
// differs from the source code, and the compile time cost.
pub fn create_optimization_level_o1() -> OptimizationLevel {
  OptimizationLevel::new(1, 0)
}

// Optimize for fast execution as much as possible without triggering significant
// incremental compile time or code size grouth.
pub fn create_optimization_level_o2() -> OptimizationLevel {
  OptimizationLevel::new(2, 0)
}

// Optimize for fast execution as much as possible.
//
// This mode is significantly more aggressive in trading off compile time and
// code size to get execution time improvements.
// The core idea is that this mode should include any optimization that helps
// execution time on balance across a diverse collection of benchmarks, even if
// it increases code size ot compile time for some benchmarks without corresponding
// improvements to execution time.
pub fn create_optimization_level_o3() -> OptimizationLevel {
  OptimizationLevel::new(3, 0)
}

pub fn create_optimization_level_os() -> OptimizationLevel {
  OptimizationLevel::new(2, 1)
}

// A very specialized mode that will optimize for code size at any and all costs.
//
// This is useful primarily when there are absolute size limitations and
// any effort taken to reduce the size is worth it regardless of the execution
// time impact. You should expect this level to to produce rather slow, but
// very small, code.
pub fn create_optimization_level_oz() -> OptimizationLevel {
  OptimizationLevel::new(2, 2)
}