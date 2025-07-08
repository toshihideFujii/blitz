#![allow(dead_code)]

use std::vec;

use common::{blitz_data::PrimitiveType, primitive_util, shape::{Shape, ShapeEqual}, shape_util::ShapeUtil};
use hlo::{hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_opcode::hlo_opcode_string};

use crate::shape_inference::ShapeInference;


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

// Visitor which verifies that the output shape is correctly set.
pub struct ShapeVerifier {}

impl ShapeVerifier {
  pub fn new() {}
  pub fn verify_entry_computation_layout() {}
  pub fn preprocess(_hlo: &HloInstruction) {}

  pub fn handle_elementwise_unary(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_unary_shape(hlo)
  }

  pub fn handle_elementwise_bynary(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_binary_shape(hlo)
  }

  pub fn handle_clamp(&self, clamp: &HloInstruction) -> Result<(), String> {
    self.check_ternary_shape(clamp)
  }

  pub fn handle_select(&self, select: &HloInstruction) -> Result<(), String> {
    self.check_ternary_shape(select)
  }

  pub fn handle_concatenate(&self, concatenate: &HloInstruction) -> Result<(), String> {
    let mut operand_shapes = vec![];
    for operand in concatenate.operands() {
      operand_shapes.push(operand.shape().clone());
    }
    self.check_shape_by_status(concatenate,
      ShapeInference::infer_concat_op_shape(
        &operand_shapes, concatenate.concatenate_dimension()))
  }

  pub fn handle_iota(&self, iota: &HloInstruction) -> Result<(), String> {
    if !iota.shape().is_array() {
      return Err("Iota does not support non-array result.".to_string());
    }
    let rank =  iota.shape().rank();
    if rank == 0 {
      return Err("Iota does not support scalars.".to_string());
    }
    let iota_dimension = iota.iota_dimension();
    if iota_dimension >= rank as i64 || iota_dimension < 0 {
      return Err("The iota dimension go beyond the operation rank or be negative.".to_string());
    }
    let p_type = iota.shape().element_type();
    if !primitive_util::is_integral_type(&p_type) &&
       !primitive_util::is_floating_point_type(&p_type) &&
       !primitive_util::is_complex_type(&p_type)
    {
      let err_msg = "Only support iota of integral, floating point or complex
        primitive types.".to_string();
      return Err(err_msg);
    }

    Ok(())
  }

  pub fn handle_convert(&self, convert: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(convert,
      ShapeInference::infer_convert_shape(
        convert.operand(0).shape(), &convert.shape().element_type()))
  }

  pub fn handle_bitcast_convert(&self, convert: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(convert,
      ShapeInference::infer_bitcast_convert_shape(
        convert.operand(0).shape(), &convert.shape().element_type()))
  }

  pub fn handle_stochastic_convert(&self, convert: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(convert,
      ShapeInference::infer_bitcast_convert_shape(
        convert.operand(0).shape(), &convert.shape().element_type()))
  }

  pub fn handle_copy(&self, copy: &HloInstruction) -> Result<(), String> {
    self.check_unary_shape(copy)
  }

  pub fn handle_dot() {}
  pub fn handle_convolution() {}

  pub fn handle_fft(&self, fft: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(fft,
      ShapeInference::infer_fft_shape(
        fft.operand(0).shape(), &fft.fft_type(), fft.fft_length()))
  }

  pub fn handle_cholsky(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(hlo,
      ShapeInference::infer_cholesky_shape(
        hlo.operand(0).shape()))
  }

  pub fn handle_triangular_solve(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(hlo,
      ShapeInference::infer_triangular_solve_shape(
        hlo.operand(0).shape(), hlo.operand(1).shape(), hlo.triangular_solve_options()))
  }

  pub fn handle_all_gather() {}
  pub fn handle_all_gather_start() {}

  pub fn handle_all_gather_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(hlo,
      ShapeInference::infer_all_gether_done_shape(
        hlo.operand(0).shape()))
  }

  pub fn handle_all_reduce() {}
  pub fn handle_all_reduce_start() {}

  pub fn handle_all_reduce_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(hlo,
      ShapeInference::infer_all_reduce_done_shape(
        hlo.operand(0).shape()))
  }

  pub fn handle_all_to_all() {}
  pub fn handle_collective_permute() {}
  pub fn handle_collective_permute_start() {}

  pub fn handle_collective_permute_done(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(hlo,
      ShapeInference::infer_collective_permute_done_shape(
        hlo.operand(0).shape()))
  }

  pub fn handle_partition_id(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_shape(hlo,
      &ShapeUtil::make_shape(&PrimitiveType::U32, vec![]),
      false)
  }

  pub fn handle_replica_id(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_shape(hlo,
      &ShapeUtil::make_shape(&PrimitiveType::U32, vec![]),
      false)
  }

  pub fn handle_reduce_precision(&self, reduce_precision: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(reduce_precision,
      ShapeInference::infer_reduce_precision_shape(
        reduce_precision.operand(0).shape(),
        reduce_precision.operand_bits(),
        reduce_precision.mantissa_bits()))
  }

  pub fn handle_infeed() {}

  pub fn handle_optimization_barrier(&self, hlo: &HloInstruction) -> Result<(), String> {
    let operand_count = check_operand_count(hlo, 1);
    if operand_count.is_err() {
      return Err(operand_count.err().unwrap());
    }
    self.check_shape(hlo,
      hlo.operand(0).shape(),
      false)
  }

  pub fn handle_outfeed() {}
  pub fn handle_rng() {}
  pub fn handle_rng_bit_generator() {}
  pub fn handle_rng_get_and_update_state() {}

  pub fn handle_reverse(&self, reverse: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(reverse,
      ShapeInference::infer_reverse_shape(
        reverse.operand(0).shape(), reverse.dimensions()))
  }

  pub fn handle_sort() {}

  pub fn handle_top_k(&self, hlo: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(hlo,
      ShapeInference::infer_topk_shape(
        hlo.operand(0).shape(), hlo.k()))
  }

  pub fn handle_constant() {}

  pub fn handle_get_tuple_element(&self, get_tuple_element: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(get_tuple_element,
      ShapeInference::infer_get_tuple_element_shape(
        get_tuple_element.operand(0).shape(), get_tuple_element.tuple_index()))
  }

  pub fn handle_reduce() {}
  pub fn handle_bitcast() {}
  pub fn handle_broadcast() {}
  pub fn handle_reshape() {}
  pub fn handle_dynamic_reshape() {}

  pub fn handle_transpose(&self, transpose: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(transpose,
      ShapeInference::infer_tranpose_shape(
        transpose.operand(0).shape(), transpose.dimensions()))
  }

  pub fn handle_parameter(&self, _hlo: &HloInstruction) -> Result<(), String> {
    Ok(())
  }

  pub fn handle_fusion() {}
  pub fn handle_call() {}
  pub fn handle_custom_call() {}

  pub fn handle_slice(&self, slice: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(slice,
      ShapeInference::infer_slice_shape(
        slice.operand(0).shape(),
        slice.slice_starts(),
        slice.slice_limits(),
        slice.slice_strides()))
  }

  pub fn handle_dynamic_slice(&self, dynamic_slice: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(dynamic_slice,
      ShapeInference::infer_dynamic_slice_shape(
        dynamic_slice.operand(0).shape(),
        dynamic_slice.index_shapes(),
        dynamic_slice.dynamic_slice_sizes(),
        true))
  }

  pub fn handle_dynamic_update_slice(
    &self, dynamic_update_slice: &HloInstruction) -> Result<(), String>
  {
    self.check_shape_by_status(dynamic_update_slice,
      ShapeInference::infer_dynamic_update_slice_shape(
        dynamic_update_slice.operand(0).shape(),
        dynamic_update_slice.operand(1).shape(),
        dynamic_update_slice.index_shapes(),
        true))
  }

  pub fn handle_tuple(&self, tuple: &HloInstruction) -> Result<(), String> {
    self.check_variadic_shape(tuple)
  }

  pub fn handle_map() {}
  pub fn handle_reduce_scatter() {}
  pub fn handle_reduce_window() {}
  pub fn handle_select_and_scatter() {}
  pub fn handle_while() {}
  pub fn handle_cinditional() {}

  pub fn handle_pad(&self, pad: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(pad,
      ShapeInference::infer_pad_shape(
        pad.operand(0).shape(),
        pad.operand(1).shape(),
        pad.padding_config()))
  }

  pub fn handle_async_start() {}
  pub fn handle_async_update() {}
  pub fn handle_done() {}

  pub fn handle_copy_start(&self, copy_start: &HloInstruction) -> Result<(), String> {
    self.check_shape(copy_start,
      &ShapeUtil::make_tuple_shape(
        vec![copy_start.operand(0).shape().clone(),
        copy_start.operand(0).shape().clone(),
        ShapeUtil::make_shape(&PrimitiveType::U32, vec![])]
      ),
      true)
  }

  pub fn handle_copy_done() {}

  pub fn handle_send(&self, send: &HloInstruction) -> Result<(), String> {
    self.check_shape(send,
      &ShapeUtil::make_tuple_shape(
        vec![send.operand(0).shape().clone(),
        ShapeUtil::make_shape(&PrimitiveType::U32, vec![]),
        ShapeUtil::make_token_shape()]
      ),
      true)
  }

  pub fn handle_send_done(&self, send_done: &HloInstruction) -> Result<(), String> {
    self.check_shape(send_done,
      &ShapeUtil::make_token_shape(),
      false)
  }

  pub fn handle_recv(&self, recv: &HloInstruction) -> Result<(), String> {
    self.check_shape(recv,
      &ShapeUtil::make_tuple_shape(
        vec![ShapeUtil::get_tuple_element_shape(recv.shape(), 0).clone(),
        ShapeUtil::make_shape(&PrimitiveType::U32, vec![]),
        ShapeUtil::make_token_shape()]
      ),
      true)
  }

  pub fn handle_recv_done(&self, recv_done: &HloInstruction) -> Result<(), String> {
    self.check_shape(recv_done,
      &ShapeUtil::make_tuple_shape(
        vec![ShapeUtil::get_tuple_element_shape(recv_done.operand(0).shape(), 0).clone(),
        ShapeUtil::make_token_shape()]
      ),
      false)
  }

  pub fn handle_batch_norm_training(
    &self, batch_norm_training: &HloInstruction) -> Result<(), String>
  {
    self.check_shape_by_status(batch_norm_training,
      ShapeInference::infer_batch_norm_training_shape(
        batch_norm_training.operand(0).shape(),
        batch_norm_training.operand(1).shape(),
        batch_norm_training.operand(2).shape(),
        batch_norm_training.feature_index()))
  }

  pub fn handle_batch_norm_inference(
    &self, batch_norm_inference: &HloInstruction) -> Result<(), String>
  {
    self.check_shape_by_status(batch_norm_inference,
      ShapeInference::infer_batch_norm_inference_shape(
        batch_norm_inference.operand(0).shape(),
        batch_norm_inference.operand(1).shape(),
        batch_norm_inference.operand(2).shape(),
        batch_norm_inference.operand(3).shape(),
        batch_norm_inference.operand(4).shape(),
        batch_norm_inference.feature_index()))
  }

  pub fn handle_batch_norm_grad(&self, batch_norm_grad: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(batch_norm_grad,
      ShapeInference::infer_batch_norm_grad_shape(
        batch_norm_grad.operand(0).shape(),
        batch_norm_grad.operand(1).shape(),
        batch_norm_grad.operand(2).shape(),
        batch_norm_grad.operand(3).shape(),
        batch_norm_grad.operand(4).shape(),
        batch_norm_grad.feature_index()))
  }

  pub fn handle_gather(&self, gather: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(gather,
      ShapeInference::infer_gather_shape(
        gather.operand(0).shape(), 
        gather.operand(1).shape(),
        gather.gather_dimension_numbers(),
        gather.gather_slice_sizes()))
  }

  pub fn handle_scatter() {}

  pub fn handle_after_all(&self, token: &HloInstruction) -> Result<(), String> {
    let mut operand_shapes = vec![];
    for operand in token.operands() {
      operand_shapes.push(operand.shape().clone())
    }
    self.check_shape(token,
      &ShapeUtil::make_token_shape(),
      false)
  }

  pub fn handle_get_dimension_size(&self, get_size: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(get_size,
      ShapeInference::infer_get_dimension_size_shape(
        get_size.operand(0).shape(), get_size.dimension()))
  }

  pub fn handle_set_dimension_size(&self, set_size: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(set_size,
      ShapeInference::infer_set_dimension_size_shape(
        set_size.operand(0).shape(), set_size.operand(1).shape(), set_size.dimension()))
  }

  pub fn handle_add_dependency(
    &self, add_dependency: &HloInstruction) -> Result<(), String>
  {
    let is_token =
      self.check_is_token_operand(add_dependency, 1);
    if is_token.is_err() { return is_token; }

    self.check_shape(add_dependency,
      add_dependency.operand(0).shape(),
      false)
  }

  pub fn finish_visit() {}

  fn shape_same(&self, _a: &Shape, _b: &Shape) -> bool {
    false
  }

  fn check_shape(
    &self,
    _instruction: &HloInstruction,
    _inferred_shape: &Shape,
    _only_compare_minor_to_major_in_layout: bool) -> Result<(), String>
  {
    unimplemented!()    
  }

  fn check_shape_by_status(
    &self,
    _instruction: &HloInstruction,
    _inferred_shape_status: Result<Shape, String>) -> Result<(), String>
  {
    unimplemented!()
  }

  fn check_parameter_count(
    calling_instruction: &HloInstruction,
    computation: &HloComputation,
    expected: i64) -> Result<(), String>
  {
    if computation.num_parameters() != expected as usize {
      let mut err_msg = "Expecetd computation ".to_string();
      err_msg.push_str(&computation.name());
      err_msg.push_str(" calling from ");
      err_msg.push_str(&calling_instruction.name());
      err_msg.push_str(" to have ");
      err_msg.push_str(&expected.to_string());
      err_msg.push_str(" parameters, has ");
      err_msg.push_str(&computation.num_parameters().to_string());
      return Err(err_msg);
    }
    Ok(())
  }

  fn check_unary_shape(&self, instruction: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(instruction,
      ShapeInference::infer_unary_op_shape(
        &instruction.opcode(), instruction.operand(0)))
  }

  fn check_binary_shape(&self, instruction: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(instruction,
      ShapeInference::infer_binary_op_shape(
        &instruction.opcode(), instruction.operand(0), instruction.operand(1)))
  }

  fn check_ternary_shape(&self, instruction: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(instruction,
      ShapeInference::infer_ternary_op_shape(
        &instruction.opcode(),
        instruction.operand(0),
        instruction.operand(1),
        instruction.operand(2)))
  }

  fn check_variadic_shape(&self, instruction: &HloInstruction) -> Result<(), String> {
    self.check_shape_by_status(instruction,
      ShapeInference::infer_variadic_op_shape(
        instruction.opcode(), instruction.operands()))
  }

  fn check_is_token_operand(
    &self, instruction: &HloInstruction, operand_no: usize) -> Result<(), String>
  {
    let token = instruction.operand(operand_no);
    if !ShapeEqual::new().equal(token.shape(), &ShapeUtil::make_token_shape()) {
      let mut err_msg = "Expected operand ".to_string();
      err_msg.push_str(&operand_no.to_string());
      err_msg.push_str(" to be token-shaped, actual shape is ");
      err_msg.push_str(&instruction.to_string_default());
      return Err(err_msg);
    }
    Ok(())
  }
}

pub fn check_operand_count(hlo: &HloInstruction, expected: usize) -> Result<(), String> {
  if hlo.operand_count() != expected {
    let mut err_msg = "Expected ".to_string();
    err_msg.push_str(&expected.to_string());
    err_msg.push_str("operands for ");
    err_msg.push_str(&hlo_opcode_string(&hlo.opcode()));
    err_msg.push_str(" instruction: ");
    err_msg.push_str(&hlo.to_string_default());
    return Err(err_msg);
  }
  Ok(())
}