
// MetricsHookInterface is an abstract interface for compiler backends to record
// stages of their compilation process.
pub struct MetricsHookInterface {}

impl MetricsHookInterface {
  pub fn default() -> Self {
    MetricsHookInterface {  }
  }
}