#![allow(dead_code)]

use crate::hlo_instruction::HloInstruction;

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
  pub fn new() {}
  pub fn verify_entry_computation_layout() {}
  pub fn preprocess(_hlo: HloInstruction) {}

  pub fn handle_elementwise_unary() {}
  pub fn handle_elementwise_bynary() {}
  pub fn handle_clamp() {}
  pub fn handle_select() {}
  pub fn handle_concatenate() {}
  pub fn handle_iota() {}
  pub fn handle_convert() {}
  pub fn handle_bitcast_convert() {}
  pub fn handle_stochastic_convert() {}
  pub fn handle_copy() {}
  pub fn handle_dot() {}
  pub fn handle_convolution() {}
  pub fn handle_fft() {}
  pub fn handle_cholsky() {}
  pub fn handle_triangular_solve() {}
  pub fn handle_all_gather() {}
  pub fn handle_all_gather_start() {}
  pub fn handle_all_gather_done() {}
  pub fn handle_all_reduce() {}
  pub fn handle_all_reduce_start() {}
  pub fn handle_all_reduce_done() {}
  pub fn handle_all_to_all() {}
  pub fn handle_collective_permute() {}
  pub fn handle_collective_permute_start() {}
  pub fn handle_collective_permute_done() {}
  pub fn handle_partition_id() {}
  pub fn handle_replica_id() {}
  pub fn handle_reduce_precision() {}
  pub fn handle_infeed() {}
  pub fn handle_optimization_barrier() {}
  pub fn handle_outfeed() {}
  pub fn handle_rng() {}
  pub fn handle_rng_bit_generator() {}
  pub fn handle_rng_get_and_update_state() {}
  pub fn handle_reverse() {}
  pub fn handle_sort() {}
  pub fn handle_top_k() {}
  pub fn handle_constant() {}
  pub fn handle_get_tuple_element() {}
  pub fn handle_reduce() {}
  pub fn handle_bitcast() {}
  pub fn handle_broadcast() {}
  pub fn handle_reshape() {}
  pub fn handle_dynamic_reshape() {}
  pub fn handle_transpose() {}
  pub fn handle_parameter() {}
  pub fn handle_fusion() {}
  pub fn handle_call() {}
  pub fn handle_custom_call() {}
  pub fn handle_slice() {}
  pub fn handle_dynamic_slice() {}
  pub fn handle_dynamic_update_slice() {}
  pub fn handle_tuple() {}
  pub fn handle_map() {}
  pub fn handle_reduce_scatter() {}
  pub fn handle_reduce_window() {}
  pub fn handle_select_and_scatter() {}
  pub fn handle_while() {}
  pub fn handle_cinditional() {}
  pub fn handle_pad() {}
  pub fn handle_async_start() {}
  pub fn handle_async_update() {}
  pub fn handle_done() {}
  pub fn handle_copy_start() {}
  pub fn handle_copy_done() {}
  pub fn handle_send() {}
  pub fn handle_send_done() {}
  pub fn handle_recv() {}
  pub fn handle_recv_done() {}
  pub fn handle_batch_norm_training() {}
  pub fn handle_batch_norm_inference() {}
  pub fn handle_batch_norm_grad() {}
  pub fn handle_gather() {}
  pub fn handle_scatter() {}
  pub fn handle_after_all() {}
  pub fn handle_get_dimension_size() {}
  pub fn handle_set_dimension_size() {}
  pub fn handle_add_dependency() {}
  pub fn finish_visit() {}
}