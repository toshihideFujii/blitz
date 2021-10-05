pub enum AddKeyConversion {
  DoNotConvert,
  ConvertToArrayIndex,
}

// This is a helper class for JSReceiver::GetKeys which collects and sort keys.
struct KeyAccumulator {}

impl KeyAccumulator {
  pub fn new() {}

  pub fn get_keys() {}

  pub fn get_own_enum_property_keys() {}

  pub fn add_key() {}

  pub fn isolate() {}

  pub fn filter() {}

  pub fn mode() {}

  pub fn set_skip_indices() {}

  pub fn add_shadowing_key() {}
}

struct FastKeyAccumulator {}

impl FastKeyAccumulator {
  pub fn new() {}

  pub fn is_receiver_simple_enum() {}

  pub fn has_empty_prototype() {}

  pub fn may_have_elements() {}

  pub fn get_keys() {}
}
