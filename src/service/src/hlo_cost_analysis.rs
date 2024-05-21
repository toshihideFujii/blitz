#![allow(dead_code)]

use std::collections::HashMap;

use common::{blitz_data::DotDimensionNumbers, layout_util::LayoutUtil, shape::Shape, shape_util::ShapeUtil};
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode};

struct Properties {
  flops: f64,
  transcendentals: f64,
  bytes_accessed: f64,
  optimal_seconds: f64,
  utilization: f64,
  operand0_utilization: f64,
  operand1_utilization: f64,
  operand0_bytes_accessed: f64,
  operand1_bytes_accessed: f64,
  output_root_bytes_accessed: f64,
  reserved0: f64,
  reserved1: f64,
  named_props: HashMap<String, f64>,
}

impl Properties {
  pub fn new() -> Self {
    Properties {
      flops: 0.0,
      transcendentals: 0.0,
      bytes_accessed: 0.0,
      optimal_seconds: 0.0,
      utilization: 0.0,
      operand0_utilization: 0.0,
      operand1_utilization: 0.0,
      operand0_bytes_accessed: 0.0,
      operand1_bytes_accessed: 0.0,
      output_root_bytes_accessed: 0.0,
      reserved0: 0.0,
      reserved1: 0.0,
      named_props: HashMap::new(),
    }
  }

  pub fn get(&self, property: &str) -> f64 {
    match property {
      "flops" => return self.flops,
      "transcendentals" => return self.transcendentals,
      "byts accessed" => return self.bytes_accessed,
      "optimal_seconds" => return self.optimal_seconds,
      "utilization" => return self.utilization,
      "utilization0{}" => return self.operand0_utilization,
      "utilization1{}" => return self.operand1_utilization,
      "bytes accessed0{}" => return self.operand0_bytes_accessed,
      "bytes accessed1{}" => return self.operand1_bytes_accessed,
      "bytes accessedout{}" => return self.output_root_bytes_accessed,
      "reserved0" => return self.reserved0,
      "reserved1" => return self.reserved1,
      _ => return 0.0
    }
  }

  pub fn set(&mut self, property: &str, value: f64) {
    match property {
      "flops" => self.flops = value,
      "transcendentals" => self.transcendentals = value,
      "bytes accessed" => self.bytes_accessed = value,
      "optimal_seconds" => self.optimal_seconds = value,
      "utilization" => self.utilization = value,
      "utilization0{}" => self.operand0_utilization = value,
      "utilization1{}" => self.operand1_utilization = value,
      "bytes accessed0{}" => self.operand0_bytes_accessed = value,
      "bytes accessed1{}" => self.operand1_bytes_accessed = value,
      "bytes accessedout{}" => self.output_root_bytes_accessed = value,
      "reserved0" => self.reserved0 = value,
      "reserved1" => self.reserved1 = value,
      _ => return
    }   
  }

  pub fn for_each<F>(&self, mut func: F) where F: FnMut(&str, f64) {
    if self.flops != 0.0 {
      func("flops", self.flops);
    }
    if self.transcendentals != 0.0 {
      func("transcendentals", self.transcendentals);
    }
    if self.bytes_accessed != 0.0 {
      func("bytes accessed", self.bytes_accessed);
    }
    if self.optimal_seconds != 0.0 {
      func("optimal_seconds", self.optimal_seconds);
    }
    if self.utilization != 0.0 {
      func("utilization", self.utilization);
    }
    if self.operand0_utilization != 0.0 {
      func("utilization0{}", self.operand0_utilization);
    }
    if self.operand1_utilization != 0.0 {
      func("utilization1{}", self.operand1_utilization);
    }
    if self.operand0_bytes_accessed != 0.0 {
      func("bytes accessed0{}", self.operand0_bytes_accessed);
    }
    if self.operand1_bytes_accessed != 0.0 {
      func("bytes accessed1{}", self.operand1_bytes_accessed);
    }
    if self.output_root_bytes_accessed != 0.0 {
      func("bytes accessedout{}", self.output_root_bytes_accessed);
    }
    if self.reserved0 != 0.0 {
      func("reserved0", self.reserved0);
    }
    if self.reserved1 != 0.0 {
      func("reserved1", self.reserved1);
    }

    for (k, v) in &self.named_props {
      if *v != 0.0 {
        func(k, *v);
      }
    }
  }

  pub fn operand_utilization(&self, operand: i64, shape_index: Vec<usize>) -> f64 {
    if operand == 0 && shape_index.is_empty() {
      return self.operand0_utilization;
    }
    if operand == 1 && shape_index.is_empty() {
      return self.operand1_utilization;
    }
    let value = 
      self.named_props.get(&get_operand_utilization_key(operand, shape_index));
    if value.is_some() {
      return *value.unwrap();
    }
    0.0
  }

  pub fn set_operand_utilization(
    &mut self, operand: i64, shape_index: Vec<usize>, value: f64)
  {
    if operand == 0 && shape_index.is_empty() {
      self.operand0_utilization = value;
    } else if operand == 1 && shape_index.is_empty() {
      self.operand1_utilization = value;
    } else {
      self.named_props
        .insert(get_operand_utilization_key(operand, shape_index), value);
    }
  }

  pub fn operand_bytes_accessed(&self, operand: i64, shape_index: Vec<usize>) -> f64 {
    if operand == 0 && shape_index.is_empty() {
      return self.operand0_bytes_accessed;
    }
    if operand == 1 && shape_index.is_empty() {
      return self.operand1_bytes_accessed;
    }
    let value = 
      self.named_props.get(&get_operand_bytes_accessed_key(operand, shape_index));
    if value.is_some() {
      return *value.unwrap();
    }
    0.0
  }

  pub fn set_operand_bytes_accessed(
    &mut self, operand: i64, shape_index: Vec<usize>, value: f64)
  {
    if operand == 0 && shape_index.is_empty() {
      self.operand0_bytes_accessed = value;
    } else if operand == 1 && shape_index.is_empty() {
      self.operand1_bytes_accessed = value;
    } else {
      self.named_props
        .insert(get_operand_bytes_accessed_key(operand, shape_index), value);
    }
  }

  pub fn output_bytes_accessed(&self, shape_index: Vec<usize>) -> f64 {
    if shape_index.is_empty() {
      return self.output_root_bytes_accessed;
    }
    let value =
      self.named_props.get(&get_output_bytes_accessed_key(shape_index));
    if value.is_some() {
      return *value.unwrap();
    }
    0.0
  }

  pub fn set_output_bytes_accessed(&mut self, shape_index: Vec<usize>, value: f64) {
    if shape_index.is_empty() {
      self.output_root_bytes_accessed = value;
    } else {
      self.named_props.insert(get_output_bytes_accessed_key(shape_index), value);
    }
  }

  pub fn to_string(&self) -> String {
    let mut out = "HloCostAnalysis::Properties{\n".to_string();
    out.push_str(" flops: ");
    out.push_str(self.flops.to_string().as_str());
    out.push('\n');
    out.push_str(" transcendentals: ");
    out.push_str(self.transcendentals.to_string().as_str());
    out.push('\n');
    out.push_str(" bytes_accessed: ");
    out.push_str(self.bytes_accessed.to_string().as_str());
    out.push('\n');
    out.push_str(" optimal_seconds: ");
    out.push_str(self.optimal_seconds.to_string().as_str());
    out.push('\n');
    out.push_str(" utilization: ");
    out.push_str(self.utilization.to_string().as_str());
    out.push('\n');
    out.push_str(" operand0_utilization: ");
    out.push_str(self.operand0_utilization.to_string().as_str());
    out.push('\n');
    out.push_str(" operand1_utilization: ");
    out.push_str(self.operand1_utilization.to_string().as_str());
    out.push('\n');
    out.push_str(" operand0_bytes_accessed: ");
    out.push_str(self.operand0_bytes_accessed.to_string().as_str());
    out.push('\n');
    out.push_str(" operand1_bytes_accessed: ");
    out.push_str(self.operand1_bytes_accessed.to_string().as_str());
    out.push('\n');
    out.push_str(" output_root_bytes_accessed: ");
    out.push_str(self.output_root_bytes_accessed.to_string().as_str());
    out.push('\n');
    out.push_str(" reserved0: ");
    out.push_str(self.reserved0.to_string().as_str());
    out.push('\n');
    out.push_str(" reserved1: ");
    out.push_str(self.reserved1.to_string().as_str());
    out.push('\n');
    out.push('}');
    out
  }
}

fn get_operand_bytes_accessed_key(operand_num: i64, shape_index: Vec<usize>) -> String {
  let mut out = "bytes accessed".to_string();
  out.push_str(&operand_num.to_string());
  out.push_str(&shape_index[0].to_string()); // check
  out
}

fn get_operand_utilization_key(operand_num: i64, shape_index: Vec<usize>) -> String {
  let mut out = "utilization".to_string();
  out.push_str(&operand_num.to_string());
  out.push_str(&shape_index[0].to_string()); // check
  out
}

fn get_output_bytes_accessed_key(shape_index: Vec<usize>) -> String {
  let mut out = "bytes accessed".to_string();
  out.push_str("out");
  out.push_str(&shape_index[0].to_string()); // check
  out
}

// A struct to encapsulate hardware-related options.
struct Options {
  per_second_rates: Properties,
  count_multiple_input_accesses: bool,
}

impl Options {
  pub fn shape_size(&self, _shape: &Shape) -> i64 {
    0 // TODO
  }

  // Set the rates used to calculate the time taken by the computation.
  pub fn set_flops_per_second(&mut self, value: f64) {
    self.per_second_rates.set("flops", value);
  }

  pub fn set_transcendentals_per_second(&mut self, value: f64) {
    self.per_second_rates.set("transcendentals", value);
  }

  pub fn set_bytes_per_second(&mut self, value: f64) {
    self.per_second_rates.set("bytes accessed", value);
  }

  // Returns the specified per-second rate used by cost analysis.
  pub fn per_second_rate(&self, property: &str) -> f64 {
    self.per_second_rates.get(property)
  }

  pub fn to_string(&self) -> String {
    let mut out = "HloCostAnalysis::Options{\n".to_string();
    out.push_str(" per_second_rates:");
    out.push_str(&self.per_second_rates.to_string());
    out.push('\n');
    out.push_str(" counnt_multiple_input_accesses: ");
    out.push_str(&self.count_multiple_input_accesses.to_string());
    out.push('\n');
    out.push('}');
    out
  }
}

// HloCostanalysis treverses an HLO graph and calculates the amount of
// computations required for the graph.
pub struct HloCostAnalysis {
  hlo_properties: HashMap<HloInstruction, Properties>,
  current_should_compute_bottleneck_time: bool,
  current_properties: Properties,
  properties_sum: Properties,
  options: Options,
}

impl HloCostAnalysis {
  pub const FMA_FLOPS: i64 = 2;

  pub fn new() {}

  pub fn handle_elementwise_unary(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.handle_elementwise_op(instruction)
  }

  pub fn handle_elementwise_binary(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.handle_elementwise_op(instruction)
  }

  pub fn handle_constant(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    self.current_should_compute_bottleneck_time = false;
    self.current_properties.set("bytes accessed", 0.0);
    self.current_properties.set_output_bytes_accessed(vec![], 0.0);
    self.current_properties.set("optimal_seconds", 0.0);
    Ok(())
  }

  pub fn handle_iota(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_get_tuple_element(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    self.current_should_compute_bottleneck_time = false;
    self.current_properties.set("bytes accessed", 0.0);
    self.current_properties.set_output_bytes_accessed(vec![], 0.0);
    self.current_properties.set_operand_bytes_accessed(0, vec![], 0.0);
    self.current_properties.set("optimal_seconds", 0.0);
    Ok(())
  }

  pub fn handle_select(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.handle_elementwise_op(instruction)
  }

  pub fn handle_compare(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.handle_elementwise_op(instruction)
  }

  pub fn handle_clamp(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.handle_elementwise_op(instruction)
  }

  pub fn handle_reduce_precision(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.handle_elementwise_op(instruction)
  }

  pub fn handle_concatenate(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_async_start() {}

  pub fn handle_async_update(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_async_done(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_copy_start(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_copy_done(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_send(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_send_done(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_recv(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_recv_done(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_convert(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.handle_elementwise_op(instruction)
  }

  pub fn handle_copy(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_domain(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.current_should_compute_bottleneck_time = false;
    self.current_properties.set("bytes accessed", 0.0);
    self.current_properties.set_output_bytes_accessed(vec![], 0.0);
    for i in 0..instruction.operand_count() {
      self.current_properties.set_operand_bytes_accessed(
        i as i64, vec![], 0.0);
    }
    self.current_properties.set("optimal_seconds", 0.0);
    Ok(())
  }

  pub fn handle_dot(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    let value = HloCostAnalysis::get_dot_flops(
      instruction.operand(0).shape(),
      instruction.shape(),
      instruction.dot_dimension_numbers()) as f64;

    self.current_properties.set("flops", value);
    Ok(())
  }

  pub fn handle_convolution() {}
  pub fn handle_fft() {}
  pub fn handle_triangular_solve() {}
  pub fn handle_cholsky() {}

  pub fn handle_optimization_barrier(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_all_gather(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_all_gather_start(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_all_gather_done(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_all_reduce(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    // TODO
    Ok(())
  }

  pub fn handle_reduce_scatter(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_all_reduce_start(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.handle_all_reduce(instruction)
  }

  pub fn handle_all_reduce_done(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_all_to_all(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_collective_broadcast(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_collective_permute(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_collective_permute_start(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_collective_permute_done(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_replica_id(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_partition_id(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_infeed() {}
  pub fn handle_outfeed() {}
  pub fn handle_rng() {}
  pub fn handle_rng_bit_generator() {}

  pub fn handle_rng_get_and_update_state(
    &mut self, _nstruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_reverse(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_sort() {}

  pub fn handle_parameter(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    self.current_should_compute_bottleneck_time = false;
    self.current_properties.set("bytes accessed", 0.0);
    self.current_properties.set_output_bytes_accessed(vec![], 0.0);
    self.current_properties.set("optimal_seconds", 0.0);
    Ok(())
  }

  pub fn handle_reduce() {}

  pub fn handle_batch_norm_training(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_batch_norm_inference(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_batch_norm_grad(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_fusion() {}
  pub fn handle_call() {}
  pub fn handle_custom_call() {}
  pub fn handle_slice() {}
  pub fn handle_dynamic_slice() {}
  pub fn handle_dynamic_update_slice() {}

  pub fn handle_tuple(
    &mut self, instruction: &HloInstruction) -> Result<(), String>
  {
    self.current_properties.set("bytes accessed",
      self.get_shape_size(instruction.shape()) as f64);
    self.current_properties.set_output_bytes_accessed(vec![],
      self.get_shape_size(instruction.shape()) as f64);
    
    for i in 0..instruction.operand_count() {
      self.current_properties.set_operand_bytes_accessed(
        i as i64, vec![], 0.0);
    }

    Ok(())
  }

  pub fn handle_map() {}
  pub fn handle_reduce_window() {}
  pub fn handle_select_and_scatter() {}
  pub fn handle_bitcast() {}
  pub fn handle_broadcast() {}
  pub fn handle_pad() {}

  pub fn handle_reshape(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_dynamic_reshape() {}
  pub fn handle_add_dependency() {}
  pub fn handle_after_all() {}
  pub fn handle_transpose() {}
  pub fn handle_while() {}
  pub fn handle_conditional() {}
  pub fn handle_gather() {}
  pub fn handle_scatter() {}

  pub fn handle_get_dimension_size(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_set_dimension_size(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn handle_top_k(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn finish_visit(
    &mut self, _instruction: &HloInstruction) -> Result<(), String>
  {
    Ok(())
  }

  pub fn preprocess(&mut self, _instruction: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn postprocess(&mut self, _instruction: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  // Enable efficient update if a known small set of instructions within an
  // HLO graph was modified.
  pub fn remove_instruction(&mut self, instruction: &HloInstruction) -> Result<(), String> {
    let target = self.hlo_properties.get(instruction);
    if target.is_some() {
      let func = |key: &str, val: f64| {
        self.properties_sum.set(key, val);
        self.hlo_properties.remove(instruction);
      };
      self.current_properties.for_each(func);
    }
    Ok(())
  }

  // Updates the cost analysis by re-doing the analysis of one instruction.
  pub fn revisit_instruction(&mut self, instruction: &HloInstruction) -> Result<(), String> {
    let mut result = self.remove_instruction(instruction);
    if result.is_err() { return result; }

    result = self.preprocess(instruction);
    if result.is_err() { return result; }
    
    // TODO

    result = self.postprocess(instruction);
    if result.is_err() { return result; }

    Ok(())
  }

  // Decorates shape_size by returning 0 immediately if the shape does not have
  // a layout.
  pub fn get_shape_size(&self, shape: &Shape) -> i64 {
    if !LayoutUtil::has_layout(shape) { return 0; }
    if LayoutUtil::is_sparse_array(shape) { return 0; }
    self.options.shape_size(shape)
  }

  // Returns properties for the computation.
  pub fn flop_count(&self) -> f64 {
    self.properties_sum.get("flops")
  }

  pub fn transcendental_count(&self) -> f64 {
    self.properties_sum.get("transcendentalls")
  }

  pub fn bytes_accessed(&self) -> f64 {
    self.properties_sum.get("bytes accessed")
  }

  pub fn optimal_seconds(&self) -> f64 {
    self.properties_sum.get("optimal_seconds")
  }

  pub fn get_dot_flops(
    lhs_shape: &Shape, result_shape: &Shape, dnums: &DotDimensionNumbers) -> i64
  {
    // Count of elements along the reduction dimensions.
    let mut reduction_width = 1;
    for dim in 0..dnums.lhs_contracting_dimensions() {
      reduction_width += lhs_shape.dimensions(dim as usize);
    }
    // Each output element requires resuction_width FMA operations.
    HloCostAnalysis::FMA_FLOPS * ShapeUtil::elements_in(result_shape) * reduction_width
  }

  fn handle_elementwise_op(&mut self, instruction: &HloInstruction) -> Result<(), String> {
    let shape = instruction.shape();
    let computation_count = ShapeUtil::elements_in(shape);
    let opcode = instruction.opcode();

    match opcode {
      HloOpcode::Erf =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Exp =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Log =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Logistic =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Power =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Sqrt =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Cbrt =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Rsqrt =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Tanh =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Sin =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Cos =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Expm1 =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Log1p =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Atan2 =>
        self.current_properties.set("transcendentals", computation_count as f64),
      HloOpcode::Tan =>
        self.current_properties.set("transcendentals", computation_count as f64),
      _ => self.current_properties.set("flops", computation_count as f64),
    }

    Ok(())
  }
}