#![allow(dead_code)]

use std::collections::HashMap;

pub struct HloProfilePrinterData {}

impl HloProfilePrinterData {
  pub fn extra_metrics(&self) -> &HashMap<String, i64> {
    unimplemented!()
  }
}