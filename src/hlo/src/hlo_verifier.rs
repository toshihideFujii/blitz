#![allow(dead_code)]

pub struct HloVerifierOpts {
  layout_sensitive: bool,
  allow_mixed_precision: bool,
  verify_broadcast_dimensions_order: bool,
  verify_reshape_is_bitcast: bool,
  verify_custom_call_nested_computation_thread_name: bool,
  verify_sharding_device_numbers: bool,
  allow_bitcast_to_have_different_size: bool,
  allow_unbounded_dynamism: bool,
}

impl HloVerifierOpts {
  pub fn make_layout_sensitive(&mut self) -> &mut Self {
    self.layout_sensitive = true;
    self
  }

  pub fn with_layout_sensitive(&mut self, layout_sensitive: bool) -> &mut Self {
    self.layout_sensitive = layout_sensitive;
    self
  }

  pub fn with_allow_mixed_precision(&mut self, allow_mixed_precision: bool) -> &mut Self{
    self.allow_mixed_precision = allow_mixed_precision;
    self
  }

  pub fn verify_allow_mixed_precision(&mut self) -> &mut Self {
    self.allow_mixed_precision = true;
    self
  }

  pub fn verify_broadcast_dimensions_order(&mut self) -> &mut Self {
    self.verify_broadcast_dimensions_order = true;
    self
  }

  pub fn verify_reshape_is_bitcast(&mut self) -> &mut Self {
    self.verify_reshape_is_bitcast = true;
    self
  }

  pub fn verify_custom_call_nested_computation_thread_name(&mut self) -> &mut Self {
    self.verify_custom_call_nested_computation_thread_name = true;
    self
  }

  pub fn with_allow_bitcast_to_have_different_size(
    &mut self, allow_bitcast_to_have_different_size: bool) -> &mut Self
  {
    self.allow_bitcast_to_have_different_size = allow_bitcast_to_have_different_size;
    self    
  }

  pub fn with_instruction_can_change_layout() {
      
  }

  pub fn with_verify_sharding_device_numbers(
    &mut self, verify_sharding_device_numbers: bool) -> &mut Self
  {
    self.verify_sharding_device_numbers = verify_sharding_device_numbers;
    self    
  }

  pub fn with_verify_s4_u4_usage(&mut self, _verify: bool) -> &mut Self {
    self
  }

  pub fn with_allow_unbounded_dynamism(
    &mut self, allow_unbounded_dynamism: bool) -> &mut Self
  {
    self.allow_unbounded_dynamism = allow_unbounded_dynamism;
    self    
  }

  pub fn is_layout_sensitive(&self) -> bool {
    self.layout_sensitive
  }

  pub fn allow_mixed_precision(&self) -> bool {
    self.allow_mixed_precision
  }

  pub fn instruction_can_change_layout(&self) {
      
  }
}

pub struct ShapeVerifier {}

impl ShapeVerifier {
    
}