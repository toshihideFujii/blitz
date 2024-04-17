#![allow(dead_code)]

pub struct AlgebraicSimplifierOptions {
  is_layout_sensitive: bool,
  enable_dot_strength_reduction: bool,
  supports_non_canonical_dots: bool,
  enable_dot_to_multiply_rewrite: bool,
  enable_conv_simplification: bool,
  enable_conv_operand_swap: bool,
  enable_scalar_multiply_reduction: bool,
  enable_floats_are_real: bool,
  enable_window_reduce_to_reduce_replacement: bool,
  enable_reduce_of_reshape: bool,
  enable_negative_padding_replacement: bool,
  enable_sink_broadcast: bool,
  unconditionally_simplify_reduce_of_transpose_or_reshape: bool,
  very_small_gather_size: i64,
  minmax_propagate_nan: bool,
  enable_unconditional_reduce_of_concat_replacement: bool,
  use_associative_reordering: bool,
  associative_reordering_threshold: f64,
}

impl AlgebraicSimplifierOptions {
  pub fn new() {}
  pub fn reshape_is_bitcast() {}
  pub fn conv_is_lowerable() {}
  pub fn set_conv_is_lowerable_callback() {}

  pub fn set_is_layout_sensitive(&mut self, is_layout_sensitive: bool) {
    self.is_layout_sensitive = is_layout_sensitive;
  }

  pub fn is_layout_sensitive(&self) -> bool {
    self.is_layout_sensitive
  }

  pub fn set_use_associative_reordering(&mut self, use_associative_redordering: bool) {
    self.use_associative_reordering = use_associative_redordering;
  }

  pub fn use_associative_reordering(&self) -> bool {
    self.use_associative_reordering
  }

  pub fn set_associative_reordering_threshold(
    &mut self,
    associative_reordering_threshold: f64)
  {
    self.associative_reordering_threshold = associative_reordering_threshold;
  }

  pub fn associative_reordering_threshold(&self) -> f64 {
    self.associative_reordering_threshold
  }

  pub fn set_enable_dot_strength_reduction(
    &mut self,
    enable_dot_strength_reduction: bool)
  {
    self.enable_dot_strength_reduction = enable_dot_strength_reduction;
  }

  pub fn enable_dot_strength_reduction(&self) -> bool {
    self.enable_dot_strength_reduction
  }

  pub fn set_enable_dot_to_multiply_rewrite(
    &mut self,
    enable_dot_to_multiply_rewrite: bool)
  {
    self.enable_dot_to_multiply_rewrite = enable_dot_to_multiply_rewrite;
  }

  pub fn enable_dot_to_multiply_rewrite(&self) -> bool {
    self.enable_dot_to_multiply_rewrite
  }

  pub fn set_supports_non_canonical_dots(&mut self, supports_non_canonical_dots: bool) {
    self.supports_non_canonical_dots = supports_non_canonical_dots;
  }

  pub fn supports_non_canonical_dots(&self) -> bool {
    self.supports_non_canonical_dots
  }

  pub fn set_enable_conv_simplification(&mut self, enable_conv_simplification: bool) {
    self.enable_conv_simplification = enable_conv_simplification;
  }

  pub fn enable_conv_simplification(&self) -> bool {
    self.enable_conv_simplification
  }

  pub fn set_enable_conv_operand_swap(&mut self, enable_conv_operand_swap: bool) {
    self.enable_conv_operand_swap = enable_conv_operand_swap;
  }

  pub fn enable_conv_operand_swap(&self) -> bool {
    self.enable_conv_operand_swap
  }

  pub fn set_enable_scalar_multiply_reduction(
    &mut self,
    enable_scalar_mutiply_reduction: bool)
  {
    self.enable_scalar_multiply_reduction = enable_scalar_mutiply_reduction;
  }

  pub fn enable_scalar_multiply_reduction(&self) -> bool {
    self.enable_scalar_multiply_reduction
  }

  pub fn set_enable_floats_are_real(&mut self, enable_floats_are_real: bool) {
    self.enable_floats_are_real = enable_floats_are_real;
  }

  pub fn enable_floats_are_real(&self) -> bool {
    self.enable_floats_are_real
  }

  pub fn set_enable_window_reduce_to_reduce_replacement(
    &mut self,
    enable_window_reduce_to_reduce_replacement: bool)
  {
    self.enable_window_reduce_to_reduce_replacement =
      enable_window_reduce_to_reduce_replacement;
  }

  pub fn enable_window_reduce_to_reduce_replacement(&self) -> bool {
    self.enable_window_reduce_to_reduce_replacement
  }

  pub fn set_very_small_gather_size(&mut self, size: i64) {
    self.very_small_gather_size = size;
  }

  pub fn very_small_gather_size(&self) -> i64 {
    self.very_small_gather_size
  }

  pub fn set_cudnn_batchnorm_forward_training_metadata() {}
  pub fn get_cudnn_batchnorm_forward_training_metadata() {}

  pub fn set_enable_reduce_of_reshape(&mut self, enable_reduce_of_reshape: bool) {
    self.enable_reduce_of_reshape = enable_reduce_of_reshape;
  }

  pub fn enable_reduce_of_reshape(&self) -> bool {
    self.enable_reduce_of_reshape
  }

  pub fn set_enable_negative_padding_replacement(
    &mut self,
    enable_negative_padding_replacement: bool)
  {
    self.enable_negative_padding_replacement = enable_negative_padding_replacement;
  }

  pub fn enable_negative_padding_replacement(&self) -> bool {
    self.enable_negative_padding_replacement
  }

  pub fn set_enable_sink_broadcast(&mut self, enable_sink_broadcast: bool) {
    self.enable_sink_broadcast = enable_sink_broadcast;
  }

  pub fn enable_sink_broadcast(&self) -> bool {
    self.enable_sink_broadcast
  }

  pub fn unconditionally_simplify_reduce_of_transpose_or_reshape(&self) -> bool {
    self.unconditionally_simplify_reduce_of_transpose_or_reshape
  }

  pub fn set_unconditionally_simplify_reduce_of_transpose_or_reshape(&mut self, val: bool) {
    self.unconditionally_simplify_reduce_of_transpose_or_reshape = val;
  }

  pub fn minmax_propagate_nan(&self) -> bool {
    self.minmax_propagate_nan
  }

  pub fn set_minmax_propagate_nan(&mut self, minmax_propagate_nan: bool) {
    self.minmax_propagate_nan = minmax_propagate_nan;
  }

  pub fn enable_unconditional_reduce_of_concat_replacement(&self) -> bool {
    self.enable_unconditional_reduce_of_concat_replacement
  }

  pub fn set_enable_unconditional_reduce_of_concat_replacement(&mut self, val: bool) {
    self.enable_unconditional_reduce_of_concat_replacement = val;
  }
}

pub struct AlgebraicSimplifier {}

impl AlgebraicSimplifier {
  pub fn new() {}

  pub fn name() -> String {
    "algsimp".to_string()
  }

  pub fn run() {}
}