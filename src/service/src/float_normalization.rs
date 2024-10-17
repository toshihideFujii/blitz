#![allow(dead_code)]

use std::collections::HashSet;

use hlo::hlo_module::HloModule;

use crate::float_support::FloatSupport;

// A pass which adds type conversions (e.g. F32 <-> BF16) for HLO instructions
// that do not support low-precision input/output or mixed precision, according
// to the passed-in backend-specific FloatSupport instance.
pub struct FloatNormalization {
  float_support: FloatSupport
}

impl FloatNormalization {
  pub fn new(float_support: FloatSupport) -> Self {
    FloatNormalization { float_support: float_support }
  }

  pub fn name(&self) -> String {
    "float-normalization".to_string()
  }

  // Run float normalization on the given computation. Returns whether the
  // computation was changed.
  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }
}

// A pass that unconditionally removes the mixed F32/BF16 uses in HLO
// instructions (excluding convert) by adding F32 <-> BF16 conversions. Unlike
// FloatNormalization, this pass does not use a backend-specific
// FloatSupport, and does not change HLOs that have BF16 data if they do not
// use mixed precision; it removes mixed precision even if the backend supports
// it. This pass is used to make the HLO module valid for other HLO passes which
// do not support mixed precision. Currently, this pass is only used by the
// Despecializer, not by our normal compilation flow on TPU.
pub struct BFloat16MixedPrecisionRemoval {}

impl BFloat16MixedPrecisionRemoval {
  pub fn new() -> Self {
    BFloat16MixedPrecisionRemoval {  }
  }

  pub fn name() -> String {
    "bf16-mixed-precision-removal".to_string()
  }

  // Run mixed precision removal on the given computation. Returns whether the
  // computation was changed.
  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<(), String>
  {
    unimplemented!()
  }
}