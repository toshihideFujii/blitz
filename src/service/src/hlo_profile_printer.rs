#![allow(dead_code)]

use crate::hlo_profile_printer_data::HloProfilePrinterData;

// Pretty-print an array of profie counters using hlo_profile_printer_data.
pub fn print_hlo_profile(
  _hlo_profile_printer_data: &HloProfilePrinterData,
  _counters: i64,
  _clock_rate_ghz: f64) -> String
{
  let result = "".to_string();
  result
}