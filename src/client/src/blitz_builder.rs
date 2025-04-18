#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}, vec};

use common::{
  blitz_data::{random_distribution_name, ChannelHandle, ConvolutionDimensionNumbers, CustomCallApiVersion, CustomCallSchedule, DotDimensionNumbers, FftType, FrontendAttributes, GatherDimensionNumbers, OpMetadata, OpSharding, PaddingConfig, PaddingType, PrecisionConfig, PrimitiveType, RandomDistribution, ReplicaGroup, ScatterDimensionNummbers, SliceDimensions, SparsityDescriptor, TriangularSolveOptions, Window}, comparison_util::{default_comparison_type, ComparisonDirection, ComparisonType}, layout::Layout, literal::Literal, literal_util::LiteralUtil, permutation_util::is_identity_permutation, shape::{ProgramShape, Shape}, shape_util::ShapeUtil, util::{self, make_no_padding_config}
};
use hlo::{hlo_instruction::HloInstruction, hlo_opcode::HloOpcode, hlo_sharding::HloSharding};
use service::{blitz_computation::BlitzComputation, shape_inference::ShapeInference};

use crate::padding::Padding;

// This represents an instruction that has been enqueued using the BlitzBuilder.
// This is used to pass to subsequent computations that depends upon the
// instruction as an operand.
#[derive(Debug, Clone)]
pub struct BlitzOp {
  handle: i64,
  builder: Option<BlitzBuilder>,
}

impl BlitzOp {
  pub fn default() -> Self {
    BlitzOp { handle: -1, builder: None }
  }

  pub fn new(builder: BlitzBuilder) -> Self {
    BlitzOp { handle: -1, builder: Some(builder) }
  }

  // Precondition: !IsUninitialized().
  //
  // It's very common to do foo.builder()->bar().  Without this precondition, if
  // foo.builder() is null, the call to bar will segfault at some point possibly
  // deep in the callstack when we finally dereference `this`.  The precondition
  // lets us avoid this tricky-to-debug problem.
  pub fn builder(&self) -> &BlitzBuilder {
    assert!(self.builder.is_some());
    self.builder.as_ref().unwrap()
  }

  // Returns true if the BlitzOp represents valid, non-erroneous value.
  pub fn valid(&self) -> bool {
    self.handle >= 0
  }

  // Returns true if the BlitzOp was created by the BlitzOp() constructor and
  // not returned by a builder.
  pub fn is_uninitialized(&self) -> bool {
    self.builder.is_none()
  }

  pub fn is_identical_to(&self, rhs: &BlitzOp) -> bool {
    self.handle == rhs.handle //&& self.builder == rhs.builder
  }

  pub fn handle(&self) -> i64 {
    self.handle
  }
}

// Describes an input/output alias as inserted by the SetUpAlias() API.
struct InputOutputAlias {}

// We don't overload the relational operators (==, !=, <, <=, >, >=) because the
// semantics might be surprising since their result types are usually 'bool'.
// Further programmers may expect == to be a structural equality.
// We also choose not to overload any of the mutating operators (e.g., +=, -=)
// because the semantics might be misleading — Blitz computations are immutable.

// A convenient interface for building up computations.
//
// Thread-compatible.
#[derive(Debug, Clone, PartialEq)]
pub struct BlitzBuilder {
  // Name to use for the built computation.
  name: String,
  // The next sequential ID for every instruction/computation contained within
  // this computation.
  next_id: i64,
  // A map from XlaOp::Handle to the index in the instructions_ vector where the
  // instruction is held.
  handle_to_index: HashMap<i64, i64>,
  metadata: OpMetadata,
  oneshot_metadata: OpMetadata,
  sharding: Option<OpSharding>,
  frontend_attributes: FrontendAttributes,
  // Mode bit that indicates whether to die when a first error is encountered.
  die_immediately_on_error: bool,
  // The first error encountered while building the computation.
  // This is OK until the first error is encountered.
  first_error: Result<(), String>,
  // The instructions of this computation.
  // Use a deque so pointers into this are stable, for example the return
  // value of LookUpInstructionByHandle().
  instructions: Vec<HloInstruction>,
  // A cache for the HloInstructionProto shapes, to avoid recreating Shape
  // objects from protos and to support the GetShapePtr() API.
  instruction_shapes: Vec<Shape>,
  // The unique parameter numbers.
  parameter_numbers: HashSet<i64>,
}

impl BlitzBuilder {
  // Default dimension numbers used for a 2D convolution.
  const CONV_BATCH_DIMENSION: i64 = 0;
  const CONV_FEATURE_DIMENSION: i64 = 1;
  const CONV_FIRST_SPATIAL_DIMENSION: i64 = 2;
  const CONV_SECOND_SPATIAL_DIMENSION: i64 = 3;
  const CONV_KERNEL_OUTPUT_DIMENSION: i64 = 0;
  const CONV_KERNEL_INPUT_DIMENSION: i64 = 1;
  const CONV_KERNEL_FIRST_SPATIAL_DIMENSION: i64 = 2;
  const CONV_KERNEL_SECOND_SPATIAL_DIMENSION: i64 = 3;

  // computation_name: name to use for the built computation.
  pub fn new(computation_name: String) -> Self {
    BlitzBuilder {
      name: computation_name,
      next_id: 0,
      handle_to_index: HashMap::new(),
      metadata: OpMetadata::new(),
      oneshot_metadata: OpMetadata::new(),
      sharding: None,
      frontend_attributes: FrontendAttributes::new(),
      die_immediately_on_error: false,
      first_error: Ok(()),
      instructions: Vec::new(),
      instruction_shapes: Vec::new(),
      parameter_numbers: HashSet::new()
    }
  }

  // Returns the computation name.
  pub fn name(&self) -> &String {
    &self.name
  }

  // Sets OpMetadata that will be added to all instructions until cleared.
  //
  // OpMetadata is often applied to a series of Blitz HLO instructions. As a
  // result, OpMetadata is set on the computation builder. All subsequent
  // instructions generated via this computation builder will have the same
  // OpMetadata attached until a call to ClearOpMetadata.
  pub fn set_op_metadata(&mut self, metadata: OpMetadata) {
    self.metadata = metadata;
  }

  // Swaps the passed op metadata with the ones currently set.
  // Returns the old op metadata.
  pub fn swap_op_metadata(&mut self, metadata: OpMetadata) -> OpMetadata {
    let old_metadata = self.metadata.clone();
    self.metadata = metadata;
    old_metadata
  }

  // Similar to SetOpMetadata, but only set the metadata for the next op.
  pub fn set_oneshot_op_metadata(&mut self, metadata: OpMetadata) {
    self.oneshot_metadata = metadata;
  }

  // Clears the HloMetadata state.
  pub fn clear_op_metadata(&mut self) {
    self.metadata.clear();
  }

  // Sets an OpSharding that will be attached to all instructions until cleared.
  pub fn set_sharding(&mut self, sharding: OpSharding) {
    self.sharding = Some(sharding);
  }

  // Sets the FrontendAttributes that will be added to all instructions until
  // cleared.
  //
  // FrontendAttributes are often applied to a series of Blitz HLO instructions.
  // As a result they are set on the computation builder and all the
  // instructions generated via the computation builder will have the same
  // frontend attributes attached to them.
  pub fn set_frontend_attributes(&mut self, frontend_attributes: FrontendAttributes) {
    self.frontend_attributes = frontend_attributes;
  }

  pub fn swap_frontend_attributes() {
      
  }

  // Returns the FrontendAttributes that will be attached to all instructions.
  pub fn frontend_attributes(&self) -> &FrontendAttributes {
    &self.frontend_attributes
  }

  // Clears all the frontend attributes.
  pub fn clear_frontend_attributes(&mut self) {
    self.frontend_attributes.clear();
  }

  // Clears the sharding. Ops will be sharded according to the default placement
  // policy.
  pub fn clear_sharding(&mut self) {
    self.sharding = None;
  }

  // Returns the OpSharding that will be attached to all instructions.
  pub fn sharding(&self) -> &Option<OpSharding> {
    &self.sharding
  }

  // Sets the builder to a mode where it will die immediately when an error is
  // encountered, rather than producing it in a deferred fashion when Build() is
  // called (which is the default).
  pub fn set_die_immediately_on_error(&mut self, enabled: bool) {
    self.die_immediately_on_error = enabled;
  }

  // Creates a default ConvolutionDimensionNumbers. For a 2D convolution, for
  // the input operand {batch, feature, height, width} = {0, 1, 2, 3} and for
  // the kernel operand
  // {output_feature, input_feature, height, width} = {0, 1, 2, 3}.
  pub fn create_default_conv_dimension_numbers(
    num_spatial_dims: i64) -> ConvolutionDimensionNumbers
  {
    let mut dimension_numbers =
      ConvolutionDimensionNumbers::new();
    
    dimension_numbers.set_input_batch_dimension(BlitzBuilder::CONV_BATCH_DIMENSION);
    dimension_numbers.set_input_feature_dimension(BlitzBuilder::CONV_FEATURE_DIMENSION);
    dimension_numbers.set_output_batch_dimension(BlitzBuilder::CONV_BATCH_DIMENSION);
    dimension_numbers.set_output_feature_dimension(BlitzBuilder::CONV_FEATURE_DIMENSION);
    dimension_numbers.set_kernel_input_feature_dimension(BlitzBuilder::CONV_KERNEL_INPUT_DIMENSION);
    dimension_numbers.set_kernel_output_feature_dimension(BlitzBuilder::CONV_KERNEL_OUTPUT_DIMENSION);

    for i in 0..num_spatial_dims {
      dimension_numbers.add_input_spatial_dimensions(i + 2);
      dimension_numbers.add_kernel_spatial_dimensions(i + 2);
      dimension_numbers.add_output_spatial_dimensions(i + 2);
    }

    dimension_numbers
  }

  // Returns an error if the convolution dimension numbers have conflicts.
  pub fn validate(dnum: &ConvolutionDimensionNumbers) -> Result<(), String> {
    if dnum.input_spatial_dimensions_size() < 2 {
      let mut err_msg = "input spatial dimension < 2; ".to_string();
      err_msg.push_str(&dnum.input_spatial_dimensions_size().to_string());
      return Err(err_msg);
    }
    if dnum.kernel_spatial_dimensions_size() < 2 {
      let mut err_msg = "kernel spatial dimension < 2; ".to_string();
      err_msg.push_str(&dnum.kernel_spatial_dimensions_size().to_string());
      return Err(err_msg);
    }
    if dnum.output_spatial_dimensions_size() < 2 {
      let mut err_msg = "output spatial dimension < 2; ".to_string();
      err_msg.push_str(&dnum.output_spatial_dimensions_size().to_string());
      return Err(err_msg);
    }

    let mut unique_checker = HashSet::new();
    unique_checker.insert(dnum.input_batch_dimension());
    unique_checker.insert(dnum.input_feature_dimension());
    unique_checker.insert(dnum.input_spatial_dimensions(0));
    unique_checker.insert(dnum.input_spatial_dimensions(1));
    if unique_checker.len() != 4 {
      let mut err_msg = "dimension numbers for the input are not unique: ".to_string();
      err_msg.push_str(&dnum.input_batch_dimension().to_string());
      err_msg.push_str(&dnum.input_feature_dimension().to_string());
      err_msg.push_str(&dnum.input_spatial_dimensions(0).to_string());
      err_msg.push_str(&dnum.input_spatial_dimensions(1).to_string());
      return Err(err_msg);
    }

    unique_checker.clear();
    unique_checker.insert(dnum.kernel_output_feature_dimension());
    unique_checker.insert(dnum.kernel_input_feature_dimension());
    unique_checker.insert(dnum.kernel_spatial_dimensions(0));
    unique_checker.insert(dnum.kernel_spatial_dimensions(1));
    if unique_checker.len() != 4 {
      let mut err_msg = "dimension numbers for the weight are not unique: ".to_string();
      err_msg.push_str(&dnum.kernel_output_feature_dimension().to_string());
      err_msg.push_str(&dnum.kernel_input_feature_dimension().to_string());
      err_msg.push_str(&dnum.kernel_spatial_dimensions(0).to_string());
      err_msg.push_str(&dnum.kernel_spatial_dimensions(1).to_string());
      return Err(err_msg);
    }

    unique_checker.clear();
    unique_checker.insert(dnum.output_batch_dimension());
    unique_checker.insert(dnum.output_feature_dimension());
    unique_checker.insert(dnum.output_spatial_dimensions(0));
    unique_checker.insert(dnum.output_spatial_dimensions(1));
    if unique_checker.len() != 4 {
      let mut err_msg = "dimension numbers for the output are not unique: ".to_string();
      err_msg.push_str(&dnum.output_batch_dimension().to_string());
      err_msg.push_str(&dnum.output_feature_dimension().to_string());
      err_msg.push_str(&dnum.output_spatial_dimensions(0).to_string());
      err_msg.push_str(&dnum.output_spatial_dimensions(1).to_string());
      return Err(err_msg);
    }
    
    Ok(())
  }

  // Returns a new BlitzBuilder whose resultant Computation is used only by this
  // BlitzBuilder. The sub-BlitzBuilder has the same die_immediately_on_error
  // behavior as the parent.
  pub fn create_sub_builder(&self, computation_name: String) -> Self {
    let mut sub_builder = BlitzBuilder::new(computation_name);
    // TODO: sub_builder.parent_builder = self.parent_builder;
    sub_builder.die_immediately_on_error = self.die_immediately_on_error;
    sub_builder
  }

  // Builds the computation with the requested operations, or returns a non-ok
  // status. Note that all ops that have been enqueued will be moved to the
  // computation being returned. The root of the computation will be the last
  // added operation.
  //
  // `remove_dynamic_dimensions` tells the builder whether to remove the
  // dynamic dimensions information in all ops.
  pub fn build(
    &mut self, remove_dynamic_dimensions: bool) -> Result<BlitzComputation, String>
  {
    if self.get_current_status().is_err() {
      return Err(self.get_current_status().err().unwrap());
    }
    self.build_from_root_id(
      self.instructions.last().unwrap().unique_id(),
    remove_dynamic_dimensions)
  }

  // Overload of Build which specifies a particular root instruction for the
  // computation.
  pub fn build_from_root(
    &mut self,
    root: &BlitzOp,
    remove_dynamic_dimensions: bool) -> Result<BlitzComputation, String>
  {
    //if root.builder.as_ref().unwrap() != self {
      //return Err("Given root operation is not in this computation.".to_string());
    //}
    self.build_from_root_id(root.handle(), remove_dynamic_dimensions)
  }

  fn remove_dynamic_dimension_for_build(shape: &mut Shape) {
    if shape.tuple_shapes_size() != 0 {
      for i in 0..shape.tuple_shapes_size() {
        return BlitzBuilder::remove_dynamic_dimension_for_build(
          shape.mutable_tuple_shapes(i));
      }
    }
    for i in 0..shape.dimensions_size() {
      shape.set_dynamic_dimension(i, false);
    }
  }

  pub fn build_from_root_id(
    &mut self,
    root_id: i64,
    remove_dynamic_dimensions: bool) -> Result<BlitzComputation, String>
  {
    if self.get_current_status().is_err() {
      return Err(self.get_current_status().err().unwrap());
    }
    if remove_dynamic_dimensions {
      for index in 0..self.instructions.len() {
        BlitzBuilder::remove_dynamic_dimension_for_build(
          &mut self.instruction_shapes[index]);
        self.instructions[index].set_shape(self.instruction_shapes[index].clone());
      }
    }

    let mut computation =
      BlitzComputation::new_from_id(self.get_next_id());
    computation.set_name(self.name.clone());
    let program_shape = self.get_program_shape();
    if program_shape.is_err() {
      return Err(program_shape.err().unwrap());
    }
    computation.set_program_shape(program_shape.unwrap().clone());
    computation.set_root_id(root_id);

    for instr in &mut self.instructions {
      let full_name = get_full_name(
        instr.name(), '.', instr.unique_id());
      instr.set_name(full_name);

    }

    // TODO
    let module = computation.mutable_proto();
    module.set_name(self.name.clone());
    module.set_id(self.get_next_id());
    module.set_entry_computation_name(self.name.clone());
    module.set_entry_computation_id(self.get_next_id());
    //module.set_host_program_shape(program_shape.unwrap().clone());
    
    // Clear data held by this builder.
    self.instructions.clear();
    self.instruction_shapes.clear();
    self.handle_to_index.clear();
    self.parameter_numbers.clear();

    Ok(computation)
  }

  // Builds the computation with the requested operations, or notes an error in
  // the parent BlitzBuilder and returns an empty computation if building failed.
  // This function is intended to be used where the returned BlitzComputation is
  // only used by the parent BlitzBuilder and hence further operation on the
  // returned BlitzComputation will simply be error'ed out if an error occurred
  // while building this computation. If the built computation is to be used by
  // a BlitzBuilder other than the parent BlitzBuilder then Build() should be used
  // instead.
  pub fn build_and_note_error(&mut self) -> BlitzComputation {
    let build_status = self.build(false);
    if build_status.is_err() {
      assert!(false, "Error from: {:?}", self.name);
    }
    build_status.unwrap()
  }

  // Returns a subgraph that roots on the given root. If the root is not a
  // compile-time constant (see `IsConstant`), returns an error.
  //
  // This will copy the needed ops/computations to the subgraph.
  pub fn build_constant_sub_graph(
    &self,
    _root_op: &BlitzOp,
    _dynamic_dimension_is_minus_one: bool) -> Result<BlitzComputation, String>
  {
    unimplemented!()    
  }

  // Returns the first error that was encountered while building the
  // computation. When an error is encountered, by default we return a vacuous
  // XlaOp and inform the user of the error that occurred while
  // building the computation when they make a final call to Build().
  //
  // See also set_die_immediately_on_error().
  pub fn first_error(&self) -> &Result<(), String> {
    &self.first_error
  }

  // Returns the current status of the builder, complete with the stack trace
  // information.
  pub fn get_current_status(&self) -> Result<(), String> {
    if self.first_error.is_err() {
      // TODO
      let backtrace = "".to_string();
      return util::append_status(0, backtrace);
    }
    Ok(())
  }

  // Returns the shape of the given op.
  pub fn get_shape(&self, op: &BlitzOp) -> Result<Shape, String> {
    if self.first_error.is_err() {
      let err_msg = self.first_error.as_ref().err().unwrap().clone();
      return Err(err_msg);
    }
    let check_op = self.check_op_builder(op);
    if check_op.is_err() { return Err(check_op.err().unwrap()); }

    let index = self.handle_to_index.get(&op.handle());
    if index.is_none() {
      let mut err_msg = "No BlitzOp with handle: ".to_string();
      err_msg.push_str(&op.handle().to_string());
      return Err(err_msg);
    }
    Ok(self.instruction_shapes[*index.unwrap() as usize].clone())
  }

  // Returns the OpSharding of the given op. If "op" has no sharding, return
  // std::nullopt.
  pub fn get_op_sharding(&self, op: &BlitzOp) -> Result<Option<OpSharding>, String> {
    let instr = self.lookup_instruction(op);
    if instr.has_sharding() {
      return Ok(Some(instr.sharding().to_proto()));
    }
    Ok(None)
  }

  // Returns the (inferred) result for the current computation's shape. This
  // assumes the root instruction is the last added instruction.
  pub fn get_program_shape(&self) -> Result<ProgramShape, String> {
    assert!(!self.instructions.is_empty());
    self.get_program_shape_internal(
      self.instructions.last().as_ref().unwrap().unique_id())
  }

  // Returns the (inferred) result for the current computation's shape using the
  // given operation as the root.
  pub fn get_program_shape_by_op(&self, root: &BlitzOp) -> Result<ProgramShape, String> {
    if root.builder.as_ref().unwrap() == self {
      return Err("Given root operation is not in this computation".to_string());
    }
    self.get_program_shape_internal(root.handle())
  }

  // Returns the (inferred) result for the program shape using the given root.
  fn get_program_shape_internal(&self, root_id: i64) -> Result<ProgramShape, String> {
    if self.first_error.is_err() {
      return Err(self.first_error.as_ref().err().unwrap().clone());
    }
    let root = self.lookup_instruction_by_handle(root_id);
    if root.is_err() {
      return Err(root.err().unwrap());
    }

    let mut program_shape = ProgramShape::new();
    program_shape.set_result(root.ok().unwrap().shape().clone());

    // Check that the parameter numbers are continuous from 0, and add parameter
    // shapes and names to the program shape.
    let param_count = self.parameter_numbers.len();
    for _i in 0..param_count {
      program_shape.add_parameters();
      program_shape.add_parameter_names_empty();
    }
    for instr in &self.instructions {
      // Parameter number uniqueness is guaranteed in BlitzBuilder::Parameter(). So
      // to verify continuity, we just need to verify that every parameter is in
      // the right range.
      if instr.opcode() == HloOpcode::Parameter {
        let index = instr.parameter_number();
        assert!(index >= 0 && index < param_count as i64);
        program_shape.set_parameter(index as usize, instr.shape().clone());
        program_shape.set_parameter_name(index as usize, instr.name());
      }
    }
    Ok(program_shape)
  }

  // Reports an error to the builder, by
  // * storing it internally and capturing a backtrace if it's the first error
  //   (this deferred value will be produced on the call to
  //    Build()/GetShape()/...)
  // * dying if die_immediately_on_error_ is true.
  // Returns an BlitzOp with an invalid handle but a valid builder. This value can
  // be returned in place of a value in APIs that return an BlitzOp.
  pub fn report_error(&mut self, error: &Result<(), String>) -> BlitzOp {
    assert!(error.is_err());
    if self.die_immediately_on_error {
      println!("Error building computation: {:?}", error.as_ref().err().unwrap());
    }
    if self.first_error.is_ok() {
      self.first_error = error.clone();
      // TODO
    }
    BlitzOp::new(self.clone())
  }

  // A helper function that converts a absl::StatusOr<BlitzOp> into an BlitzOp.
  // If the absl::Status was an error, reports the error to builder and returns
  // an invalid BlitzOp handle.
  pub fn report_error_or_return(&mut self, op: Result<BlitzOp, String>) -> BlitzOp {
    if !self.first_error.is_ok() {
      return BlitzOp::new(self.clone());
    }
    if !op.is_ok() {
      let err_msg = op.err().unwrap();
      return self.report_error(&Err(err_msg));
    }
    op.ok().unwrap().clone()
  }

  // Returns true if 'operand' is a compile-time constant. A compile-time
  // constant does not depend on any parameters, or on stateful operators such
  // as `RngNormal` or `Infeed`.
  //
  // This tests whether a computation is a compile-time constant without
  // evaluating the computation.
  pub fn is_constant(&self, operand: &BlitzOp) -> Result<bool, String> {
    if self.first_error.is_err() {
      let err_msg = self.first_error.as_ref().err().unwrap().clone();
      return Err(err_msg);
    }
    let is_constant = false;
    let visited = HashSet::new();
    self.is_constant_visitor(
      operand.handle(), 0, &visited, &is_constant);
    Ok(is_constant)
  }

  pub fn setup_alias() {}

  // Adds a new buffer donor. The donated buffer may be paired with any valid
  // output. On the contrary, the buffer aliasing bonds the input output pair.
  // The input can only donate the buffer to the paired output.
  pub fn add_buffer_donor(&self, _param_number: i64, _param_index: &Vec<Vec<i64>>) {
    unimplemented!()
  }

  // Looks up the HloInstruction and sets the frontend attribute "attribute" to
  // "value". If the attribute already existed, then its value is updated.
  //
  // The attribute is only added to the HloInstruction, not to the builder.
  pub fn set_instruction_frontend_attribute(
    &mut self, op: &BlitzOp, _attribute: &String, value: String)-> Result<(), String>
  {
    let instr = self.lookup_mutable_instruction(op);
    let frontend_attrs = instr.mutable_frontend_attributes();
    frontend_attrs.set_attribute(value);
    Ok(())
  }

  // Looks up the HloInstruction and sets the sharding. If the sharding already
  // existed, then its value is updated.
  //
  // The sharding is only added to the HloInstruction, not to the builder.
  pub fn set_instruction_sharding(
    &mut self, op: &BlitzOp, sharding: &Option<OpSharding>) -> Result<(), String>
  {
    let instr = self.lookup_mutable_instruction(op);
    if sharding.is_none() {
      instr.clear_sharding();
      return Ok(());
    }
    normalize_and_assign_sharding(instr, sharding.as_ref().unwrap())
  }

  // Returns shapes for the operands.
  pub fn get_operand_shapes(&self, operands: &Vec<BlitzOp>) -> Result<Vec<Shape>, String> {
    let mut operand_shapes = vec![];
    operand_shapes.reserve(operands.len());
    for operand in operands {
      let shape = self.get_shape(operand);
      if shape.is_err() { return Err(shape.err().unwrap()); }
      operand_shapes.push(shape.unwrap().clone());
    }
    Ok(operand_shapes)
  }

  // Converts the op to string for the ease of debugging.
  pub fn op_to_string(&self, _op: &BlitzOp) -> String {
    unimplemented!()
  }

  // Returns OK status if the given op was built using this builder. Otherwise,
  // returns an error.
  pub fn check_op_builder(&self, _op: &BlitzOp) -> Result<(), String> {
    unimplemented!()
  }

  fn parameter(
    &mut self,
    parameter_number: i64,
    shape: &Shape,
    name: &String,
    replicated_at_leaf_buffers: &Vec<bool>) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    if !self.parameter_numbers.insert(parameter_number) {
      assert!(false, "Parameter {:?} already registered.", parameter_number);
    }
    instr.set_parameter_number(parameter_number);
    instr.set_name(name.clone());
    instr.set_shape(shape.clone());
    if !replicated_at_leaf_buffers.is_empty() {
      let replication = instr.mutable_parameter_replication();
      for replicated in replicated_at_leaf_buffers {
        replication.add_replicated_at_leaf_buffers(*replicated);
      }
    }
    let result = self.add_instruction(
      &instr, HloOpcode::Parameter, &vec![]);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn constant_literal<T>(&self, literal: &Literal<T>) -> BlitzOp
    where T: Clone + Default + PartialEq
  {
    if literal.shape().is_array() && literal.element_count() > 1 &&
      literal.is_all_first()
    {
      let scalar = LiteralUtil::get_first_scalar_literal(literal);
      let mut instr = HloInstruction::default();
      instr.set_shape(scalar.shape().clone());
      instr.set_literal(scalar.clone());

      let scalar_op = self.add_instruction(
        &instr, HloOpcode::Constant, &vec![]);
      if scalar_op.is_err() { assert!(false); }
      return self.broadcast(
        scalar_op.as_ref().unwrap(),
        literal.shape().dimensions_vec()); 
    } else {
      let mut instr = HloInstruction::default();
      instr.set_shape(literal.shape().clone());
      instr.set_literal(literal.clone());
      let result = self.add_instruction(
        &instr, HloOpcode::Constant, &vec![]);
      if result.is_err() { assert!(false); }
      result.unwrap()
    }
  }

  fn broadcast(&self, operand: &BlitzOp, broadcast_sizes: &Vec<i64>) -> BlitzOp {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_broadcast_shape(
      operand_shape.as_ref().unwrap(), broadcast_sizes);
    if shape.is_err() { assert!(false); }

    // The client-level broadcast op just appends dimensions on the left (adds
    // lowest numbered dimensions). The HLO broadcast instruction is more
    // flexible and can add new dimensions anywhere. The instruction's
    // dimensions field maps operand dimensions to dimensions in the broadcast
    // output, so to append dimensions on the left the instruction's dimensions
    // should just be the n highest dimension numbers of the output shape where
    // n is the number of input dimensions.
    let operand_rank = operand_shape.as_ref().unwrap().rank();
    let mut dimensions: Vec<i64> = Vec::new();
    for i in 0..operand_rank {
      dimensions[i] = (i + shape.as_ref().unwrap().rank() - operand_rank) as i64;
    }
    let result = self.in_dim_broadcast(
      shape.as_ref().unwrap(), operand, &dimensions);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn broadcast_in_dim(
    _operand: &BlitzOp,
    _out_dim_size: &Vec<i64>,
    _broadcast_dimensions: &Vec<i64>) -> BlitzOp
  {
    unimplemented!()  
  }

  fn mhlo_dynamic_broadcast_in_dim(
    &self,
    _operand: &BlitzOp,
    _output_dimensions: &BlitzOp,
    _broadcast_dimensions: &Vec<i64>,
    _output_shape: &Shape) -> BlitzOp
  {
    unimplemented!()    
  }

  fn pad(
    &self,
    operand: &BlitzOp,
    padding_value: &BlitzOp,
    padding_config: &PaddingConfig) -> BlitzOp
  {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let padding_value_shape = self.get_shape(padding_value);
    if padding_value_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_pad_shape(
      operand_shape.as_ref().unwrap(),
      padding_value_shape.as_ref().unwrap(), padding_config);
    if shape.is_err() { assert!(false); }

    let result = self.pad_internal(
      shape.as_ref().unwrap(), operand, padding_value, padding_config);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn pad_in_dim(
    &self,
    operand: &BlitzOp,
    padding_value: &BlitzOp,
    dimno: i64,
    pad_lo: i64,
    pad_hi: i64) -> BlitzOp
  {
    let shape = self.get_shape(operand);
    if shape.is_err() { assert!(false); }

    let mut padding_config = make_no_padding_config(
      shape.as_ref().unwrap().rank() as i64);
    let dims = padding_config.mutable_dimensions(dimno);
    dims.set_edge_padding_low(pad_lo);
    dims.set_edge_padding_high(pad_hi);
    
    self.pad(operand, padding_value, &padding_config)
  }

  fn pad_internal(
    &self,
    shape: &Shape,
    operand: &BlitzOp,
    padding_value: &BlitzOp,
    padding_config: &PaddingConfig) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_padding_config(padding_config.clone());
    self.add_instruction(&instr,
      HloOpcode::Pad, &vec![operand, padding_value])
  }

  fn reshape(
    &mut self,
    operand: &BlitzOp,
    dimensions: &Vec<i64>,
    new_sizes: &Vec<i64>,
    inferred_dimension: i64) -> BlitzOp
  {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_reshape_shpae(
      operand_shape.as_ref().unwrap(), dimensions, new_sizes, inferred_dimension);
    if shape.is_err() { assert!(false); }

    let mut transposed = operand.clone();
    if !is_identity_permutation(dimensions) {
      transposed = self.transpose(operand, dimensions);
    }

    let result = self.reshape_internal(
      shape.as_ref().unwrap(),
      &transposed,
      inferred_dimension);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  pub fn reshape_without_dimensions(
    &mut self,
    operand: &BlitzOp,
    new_sizes: &Vec<i64>,
    inferred_dimension: i64) -> BlitzOp
  {
    let shape = self.get_shape(operand);
    if shape.is_err() { assert!(false); }
    let dimensions: Vec<i64> = vec![0; shape.as_ref().unwrap().dimensions_size()];
    self.reshape(operand, &dimensions, new_sizes, inferred_dimension)
  }

  fn dynamic_reshape(
    &self,
    _operand: &BlitzOp,
    _dim_sizes: &Vec<BlitzOp>,
    _new_size_bounds: &Vec<i64>,
    _dims_are_dynamic: &Vec<bool>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn mhlo_dynamic_reshape(
    &self, _operand: &BlitzOp, _output_shape: &BlitzOp, _shape: &Shape) -> BlitzOp
  {
    unimplemented!()    
  }

  fn collapse(&mut self, operand: &BlitzOp, dimensions: &Vec<i64>) -> BlitzOp {
    if dimensions.len() <= 1 {
      // Not collapsing anything, trivially we can return the operand versus
      // enqueueing a trivial reshape.
      return operand.clone();
    }

    // Out-of-order collapse is not supported.
    // Checks that the collapsed dimensions are in order and consecutive.
    for i in 1..dimensions.len() {
      if dimensions[i] - 1 != dimensions[i - 1] {
        assert!(false, "Collapsed dimensions are not in consecutive order.");
      }
    }

    // Create a new sizes vector from the old shape, replacing the collapsed
    // dimensions by the product of their sizes.
    let original_shape = self.get_shape(operand);
    if original_shape.is_err() { assert!(false); }

    let mut new_sizes = vec![];
    for i in 0..original_shape.as_ref().unwrap().rank() {
      if i <= *dimensions.first().unwrap() as usize ||
        i > *dimensions.last().unwrap() as usize
      {
        new_sizes.push(original_shape.as_ref().unwrap().dimensions(i));
      } else {
        let val = new_sizes.last().unwrap() *
          original_shape.as_ref().unwrap().dimensions(i);
        new_sizes.push(val);
      }
    }
    
    self.reshape_without_dimensions(operand, &new_sizes, -1)
  }

  fn slice(
    &mut self,
    operand: &BlitzOp,
    start_indices: &Vec<i64>,
    limit_indices: &Vec<i64>,
    strides: &Vec<i64>) -> BlitzOp
  {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_slice_shape(
      operand_shape.as_ref().unwrap(),
      start_indices,
      limit_indices, strides);
    if shape.is_err() { assert!(false); }

    let result = self.slice_internal(
      shape.as_ref().unwrap(), operand, start_indices, limit_indices, strides);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn slice_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    start_indices: &Vec<i64>,
    limit_indices: &Vec<i64>,
    strides: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    for i in 0..start_indices.len() {
      let mut slice_dim = SliceDimensions::default();
      slice_dim.set_start(start_indices[i]);
      slice_dim.set_limit(limit_indices[i]);
      slice_dim.set_stride(strides[i]);
    }
    self.add_instruction(&instr, HloOpcode::Slice, &vec![operand])
  }

  fn slice_in_dim(
    &mut self,
    operand: &BlitzOp,
    start_index: i64,
    limit_index: i64,
    stride: i64,
    dimno: i64) -> BlitzOp
  {
    let shape = self.get_shape(operand);
    let mut starts = vec![0; shape.as_ref().unwrap().rank()];
    let mut limits = vec![];
    limits.clone_from(shape.as_ref().unwrap().dimensions_vec());
    let mut strides = vec![1; shape.as_ref().unwrap().rank()];

    starts[dimno as usize] = start_index;
    limits[dimno as usize] = limit_index;
    strides[dimno as usize] = stride;

    self.slice(operand, &starts, &limits, &strides)
  }

  fn dynamic_slice(
    _operand: &BlitzOp,
    _start_indices: &Vec<BlitzOp>,
    _slicee_sizes: &Vec<i64>) -> BlitzOp
  {
    unimplemented!()  
  }

  fn dynamic_slice_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    start_indices: &Vec<&BlitzOp>,
    slicee_sizes: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());

    for size in slicee_sizes {
      instr.add_dynamic_slice_sizes(*size);
    }

    let mut operands = vec![operand];
    for indice in start_indices {
      operands.push(*indice);
    }
    self.add_instruction(&instr, HloOpcode::DynamicSlice, &operands)
  }

  fn dynamic_update_slice(
    &self,
    _operand: &BlitzOp,
    _update: &BlitzOp,
    _start_indices: &Vec<BlitzOp>) -> BlitzOp
  {
    unimplemented!()  
  }

  fn concat_in_dim(&self, _operands: &Vec<BlitzOp>, _dimension: i64) -> BlitzOp {
    unimplemented!()
  }

  fn select(&self, _pred: &BlitzOp, _on_true: &BlitzOp, _on_false: &BlitzOp) -> BlitzOp {
    unimplemented!()
  }

  fn tuple(&self, _elements: &Vec<BlitzOp>) -> BlitzOp {
    unimplemented!()
  }

  fn tuple_internal(&self, _shape: &Shape, _elements: &Vec<BlitzOp>) -> Result<BlitzOp, String> {
    unimplemented!()
  }

  fn get_tuple_element(&self, _tuple_data: &BlitzOp, _index: i64) -> BlitzOp {
    unimplemented!()
  }

  fn dot(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _precision_config: Option<PrecisionConfig>,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn dot_general(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _dimension_numbers: &DotDimensionNumbers,
    _precision_config: Option<PrecisionConfig>,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn sparse_dot(&self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _sparse_meta: &Vec<BlitzOp>,
    _sparsity: &Vec<SparsityDescriptor>,
    _dimension_numbers: &DotDimensionNumbers,
    _precision_config: Option<PrecisionConfig>,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn conv(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Padding,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn conv_with_general_padding(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Padding,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn conv_with_general_dimensions(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Padding,
    _dimension_numbers: &ConvolutionDimensionNumbers,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn conv_general(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Padding,
    _dimension_numbers: &ConvolutionDimensionNumbers,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn conv_general_dilated(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Vec<(i64, i64)>,
    _lhs_dilation: &Vec<i64>,
    _rhs_dilation: &Vec<i64>,
    _dimension_numbers: &ConvolutionDimensionNumbers,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _preferred_element_t: Option<PrimitiveType>,
    _window_reserval: Option<Vec<bool>>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn dynamic_conv_forward(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Vec<(i64, i64)>,
    _lhs_dilation: &Vec<i64>,
    _rhs_dilation: &Vec<i64>,
    _dimension_numbers: &ConvolutionDimensionNumbers,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _padding_t: PaddingType,
    _preferred_element_t: Option<PrimitiveType>) {
      
  }

  fn dynamic_conv_input_grad(
    &self,
    _input_sizes: &BlitzOp,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Vec<(i64, i64)>,
    _lhs_dilation: &Vec<i64>,
    _rhs_dilation: &Vec<i64>,
    _dimension_numbers: &ConvolutionDimensionNumbers,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _padding_t: PaddingType,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn dynamic_conv_kernel_grad(
    &self,
    _activations: &BlitzOp,
    _gradients: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Vec<(i64, i64)>,
    _lhs_dilation: &Vec<i64>,
    _rhs_dilation: &Vec<i64>,
    _dimension_numbers: &ConvolutionDimensionNumbers,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _padding_t: PaddingType,
    _preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn dynamic_conv_instruction(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window_strides: &Vec<i64>,
    _padding: &Vec<(i64, i64)>,
    _lhs_dilation: &Vec<i64>,
    _rhs_dilation: &Vec<i64>,
    _dimension_numbers: &ConvolutionDimensionNumbers,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>,
    _padding_t: PaddingType,
    _preferred_element_t: Option<PrimitiveType>) -> Result<HloInstruction, String>
  {
    unimplemented!()    
  }

  fn conv_general_dilated_internal(
    &self,
    _shape: &Shape,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _window: &Window,
    _window_strides: &Vec<i64>,
    _padding: &Vec<(i64, i64)>,
    _lhs_dilation: &Vec<i64>,
    _rhs_dilation: &Vec<i64>,
    _dimension_numbers: &ConvolutionDimensionNumbers,
    _feature_group_count: i64,
    _batch_group_count: i64,
    _precision_config: Option<PrecisionConfig>) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  fn fft(&self, _operand: &BlitzOp, _fft_t: &FftType, _fft_length: &Vec<i64>) -> BlitzOp {
    unimplemented!()
  }

  fn triangular_solve_internal(
    &self,
    _shape: &Shape,
    _a: &BlitzOp,
    _b: &BlitzOp,
    _options: &TriangularSolveOptions) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  fn cholesky_internal(
    &self, _shape: &Shape, _a: &BlitzOp, _lower: bool) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  fn infeed(&self, _shape: &Shape, _config: String) -> BlitzOp {
    unimplemented!()
  }

  fn infeed_with_token(
    &self, _token: &BlitzOp, _shape: &Shape, _config: String) -> BlitzOp
  {
    unimplemented!()
  }

  fn infeed_with_token_internal(
    &self, _infeed_instruction_shape: &Shape, _token: &BlitzOp, _config: String) -> BlitzOp
  {
    unimplemented!()
  }

  fn outfeed(&self, _operand: &BlitzOp, _shape_with_layout: &Shape, _outfeed_config: String) {
    unimplemented!()
  }

  fn outfeed_with_token(
    &self,
    _operand: &BlitzOp,
    _token: &BlitzOp,
    _shape_with_layout: &Shape,
    _outfeed_config: String) -> BlitzOp
  {
    unimplemented!()    
  }

  fn outfeed_with_token_internal(
    &self,
    _operand: &BlitzOp,
    _token: &BlitzOp,
    _shape_with_layout: &Shape,
    _outfeed_config: String) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  fn call(&self, _computation: &BlitzComputation, _operands: &Vec<BlitzOp>) -> BlitzOp {
    unimplemented!()
  }

  fn composite_call(
    &self,
    _computation: &BlitzComputation,
    _operands: &Vec<BlitzOp>,
    _name: &String,
    _attributes: Option<String>,
    _version: Option<i64>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn custom_call<T>(
    &self,
    _call_target_name: &String,
    _operands: &Vec<BlitzOp>,
    _shape_with_layout: &Shape,
    _opaque: String,
    _operand_shapes_with_layout: Option<Vec<Shape>>,
    _has_side_effect: bool,
    _output_operand_aliasing: &Vec<(usize, (i64, usize))>,
    _lieteral: &Literal<T>,
    _window: Option<Window>,
    _dnums: Option<&ConvolutionDimensionNumbers>,
    _schedule: CustomCallSchedule,
    _api_version: CustomCallApiVersion) -> BlitzOp
    where T: Clone + Default + PartialEq
  {
    unimplemented!()    
  }

  // Internal version of CustomCall without computation that doesn't do op
  // specific error handling and expects arguments to be legal. CustomCall
  // method above calls this method after error handling.
  fn custom_call_internal<T>(
    &self,
    _call_target_name: &String,
    _operands: &Vec<BlitzOp>,
    _computation: &BlitzComputation,
    _shape_with_layout: &Shape,
    _opaque: String,
    _operand_shapes_with_layout: Option<Vec<Shape>>,
    _has_side_effect: bool,
    _output_operand_aliasing: &Vec<(usize, (i64, usize))>,
    _lieteral: &Literal<T>,
    _window: Option<Window>,
    _dnums: Option<&ConvolutionDimensionNumbers>,
    _schedule: CustomCallSchedule,
    _api_version: CustomCallApiVersion) -> Result<BlitzOp, String>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()    
  }

  fn optimization_barrier(&self, operand: &BlitzOp) -> BlitzOp {
    let shape = self.get_shape(operand);
    if shape.is_err() {
      // TODO
    }
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.unwrap().clone());
    let result = self.add_instruction(
      &instr, HloOpcode::OptimizationBarrier, &vec![operand]);
    result.unwrap()
  }

  fn reduce(
    &self,
    _operand: &BlitzOp,
    _init_value: &BlitzOp,
    _computation: &BlitzComputation,
    _dimensions_to_reduce: &Vec<i64>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn reduce_internal(
    &self,
    _shape: &Shape,
    _all_operands: &Vec<BlitzOp>,
    _computation: &BlitzComputation,
    _dimensions_to_reduce: &Vec<i64>) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  fn reduce_all(
    &self,
    _operand: &BlitzOp,
    _init_value: &BlitzOp,
    _computation: &BlitzComputation) -> BlitzOp
  {
    unimplemented!()    
  }

  fn reduce_window(
    &self,
    _operand: &BlitzOp,
    _init_value: &BlitzOp,
    _computation: &BlitzComputation,
    _window_dimensions: &Vec<i64>,
    _window_strides: &Vec<i64>,
    _padding: &Padding) -> BlitzOp
  {
    unimplemented!()    
  }

  fn reduce_window_with_general_padding(
    &self,
    _operands: &Vec<BlitzOp>,
    _init_values: &Vec<BlitzOp>,
    _computation: &BlitzComputation,
    _window_dimensions: &Vec<i64>,
    _window_strides: &Vec<i64>,
    _base_dilations: &Vec<i64>,
    _window_dilations: &Vec<i64>,
    _padding: &Vec<(i64, i64)>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn reduce_window_internal(
    &self,
    _operands: &Vec<BlitzOp>,
    _init_values: &Vec<BlitzOp>,
    _computation: &BlitzComputation,
    _window_dimensions: &Vec<i64>,
    _window_strides: &Vec<i64>,
    _base_dilations: &Vec<i64>,
    _window_dilations: &Vec<i64>,
    _padding: &Vec<(i64, i64)>) -> Result<HloInstruction, String>
  {
    unimplemented!()    
  }

  fn cross_replca_sum(
    &self, _operand: &BlitzOp, _replica_groups: &Vec<ReplicaGroup>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn all_gether(
    &self,
    _operand: &BlitzOp,
    _all_gather_dimension: i64,
    _shard_count: i64,
    _replica_groups: &Vec<ReplicaGroup>,
    _channel_id: Option<ChannelHandle>,
    _layout: Option<Layout>,
    _use_global_device_ids: Option<bool>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn all_reduce(
    &self,
    _operand: &BlitzOp,
    _computation: &BlitzComputation,
    _replica_groups: &Vec<ReplicaGroup>,
    _channel_id: Option<ChannelHandle>,
    _shape_with_layout: Option<Shape>,
    _use_global_device_ids: Option<bool>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn reduce_scatter() {
      
  }

  fn all_to_all() {
      
  }

  fn all_to_all_tuple() {
      
  }

  fn collective_broadcast() {
      
  }

  fn collective_permute() {
      
  }

  fn replica_id() {
      
  }

  fn select_and_scatter() {
      
  }

  fn select_and_scatter_with_general_padding() {
      
  }

  fn select_and_scatter_internal() {
      
  }

  fn iota(&mut self, shape: &Shape, iota_dimension: i64) -> BlitzOp {
    if !shape.is_static() {
      assert!(false, "The output of iota must not have dynamic dimensions: {:?}", shape);
    }
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.add_dimensions(iota_dimension);
    let result =
      self.add_instruction(&instr, HloOpcode::Iota, &vec![]);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn convert_element_type() {
      
  }

  fn bitcast_convert_type(
    &mut self, operand: &BlitzOp, new_element_t: &PrimitiveType) -> BlitzOp
  {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_bitcast_convert_shape(
      operand_shape.as_ref().unwrap(), new_element_t);
    if shape.is_err() { assert!(false); }

    let result = self.bitcast_convert_type_internal(
      shape.as_ref().unwrap(), operand);
    if result.is_err() { assert!(false); }

    result.unwrap()
  }

  fn bitcast_convert_type_internal(
    &mut self, shape: &Shape, operand: &BlitzOp) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    self.add_instruction(&instr, HloOpcode::BitcastConvert, &vec![operand])
  }

  fn stochastic_convert_type(
    &mut self,
    operand: &BlitzOp,
    random: &BlitzOp,
    new_element_t: &PrimitiveType) -> BlitzOp
  {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let random_shape = self.get_shape(random);
    if random_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_stochastic_convert_shape(
      operand_shape.as_ref().unwrap(), 
      random_shape.as_ref().unwrap(),
      new_element_t);
    if shape.is_err() { assert!(false); }

    let result = self.add_op_with_shape(
      HloOpcode::StochasticConvert,
      shape.as_ref().unwrap(),
      &vec![operand, random]);
    if result.is_err() { assert!(false); }

    result.unwrap()
  }

  fn transpose(&mut self, operand: &BlitzOp, permutation: &Vec<i64>) -> BlitzOp {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false) }

    let shape =
      ShapeInference::infer_tranpose_shape(
        operand_shape.as_ref().unwrap(), permutation);
    if shape.is_err() { assert!(false); }

    let result = self.transpose_internal(
      shape.as_ref().unwrap(), operand, permutation);
    if result.is_err() { assert!(false); }

    result.unwrap()
  }

  fn transpose_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    permutation: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    for dim in permutation {
      instr.add_dimensions(*dim);
    }
    self.add_instruction(&instr, HloOpcode::Transpose, &vec![operand])
  }

  fn rev(&mut self, operand: &BlitzOp, dimensions: &Vec<i64>) -> BlitzOp {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_reverse_shape(
        operand_shape.as_ref().unwrap(), dimensions);
    if shape.is_err() { assert!(false); }

    let result = self.rev_internal(
      shape.as_ref().unwrap(), operand, dimensions);
    if result.is_err() { assert!(false); }

    result.unwrap()
  }

  fn rev_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    dimensions: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    for dim in dimensions {
      instr.add_dimensions(*dim);
    }
    self.add_instruction(&instr, HloOpcode::Reverse, &vec![operand])
  }

  fn sort() {
      
  }

  fn top_k() {
      
  }

  // Enqueues a clamp instruction onto the computation.
  fn clamp(&mut self, min: &BlitzOp, operand: &BlitzOp, max: &BlitzOp) -> BlitzOp {
    self.ternary_op(HloOpcode::Clamp, min, operand, max)
  }

  fn map() {
      
  }

  fn rng_normal(&mut self, mu: &BlitzOp, sigma: &BlitzOp, shape: &Shape) -> BlitzOp {
    self.rng_op(RandomDistribution::Normal, &vec![mu, sigma], shape)
  }

  fn rng_uniform(&mut self, a: &BlitzOp, b: &BlitzOp, shape: &Shape) -> BlitzOp {
    self.rng_op(RandomDistribution::Uniform, &vec![a, b], shape)
  }

  fn rng_bit_generator() {
      
  }

  fn rng_bit_generator_internal() {
      
  }

  fn while_(
    &mut self,
    condition: &BlitzComputation,
    body: &BlitzComputation,
    init: &BlitzOp) -> BlitzOp
  {
    let body_program_shape =  body.get_program_shape();
    if body_program_shape.is_err() { assert!(false); }
    let condition_program_shape = condition.get_program_shape();
    if condition_program_shape.is_err() { assert!(false); }
    let init_shape = self.get_shape(init);
    if init_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_while_shape(
      condition_program_shape.as_ref().unwrap(),
      body_program_shape.as_ref().unwrap(),
        init_shape.as_ref().unwrap());
    if shape.is_err() { assert!(false); }

    let result =
      self.while_internal(shape.as_ref().unwrap(), condition, body, init);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn while_internal(
    &mut self,
    shape: &Shape,
    condition: &BlitzComputation,
    body: &BlitzComputation,
    init: &BlitzOp) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());

    // Body comes before condition computation in the vector.
    self.add_called_computation(body, &instr);
    self.add_called_computation(condition, &instr);
    self.add_instruction(&instr, HloOpcode::While, &vec![init])
  }

  fn conditional() {
      
  }

  fn reduce_precision(
    &mut self, operand: &BlitzOp, exponent_bits: i64, mantissa_bits: i64) -> BlitzOp
  {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_reduce_precision_shape(
      operand_shape.as_ref().unwrap(),
      exponent_bits,
      mantissa_bits);
    if shape.is_err() { assert!(false); }

    let result = self.reduce_precision_internal(
      shape.as_ref().unwrap(), operand, exponent_bits, mantissa_bits);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn reduce_precision_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    exponent_bits: i64,
    mantissa_bits: i64) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_exponent_bits(exponent_bits);
    instr.set_mantissa_bits(mantissa_bits);
    self.add_instruction(&instr, HloOpcode::ReducePrecision, &vec![operand])
  }

  fn gather(
    &mut self,
    input: &BlitzOp,
    start_indices: &BlitzOp,
    dimension_numbers: &GatherDimensionNumbers,
    slice_sizes: &Vec<i64>,
    indices_are_sorted: bool) -> BlitzOp
  {
    let input_shape = self.get_shape(input);
    if input_shape.is_err() { assert!(false); }
    let start_indices_shape = self.get_shape(start_indices);
    if start_indices_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_gather_shape(
      input_shape.as_ref().unwrap(),
      start_indices_shape.as_ref().unwrap(), 
      dimension_numbers,
      slice_sizes);
    if shape.is_err() { assert!(false); }
    
    let result = self.gather_internal(
      shape.as_ref().unwrap(),
      input,
      start_indices,
      dimension_numbers,
      slice_sizes,
      indices_are_sorted);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn gather_internal(
    &mut self,
    shape: &Shape,
    input: &BlitzOp,
    start_indices: &BlitzOp,
    dimension_numbers: &GatherDimensionNumbers,
    slicee_sizes: &Vec<i64>,
    indices_are_sorted: bool) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_indices_are_sorted(indices_are_sorted);
    instr.set_shape(shape.clone());
    instr.set_gather_dimension_numbers(dimension_numbers.clone());

    for bound in slicee_sizes {
      instr.add_gather_slice_sizes(*bound);
    }
    self.add_instruction(
      &instr, HloOpcode::Gather, &vec![input, start_indices])
  }

  fn scatter(
    &mut self,
    input: &BlitzOp,
    scatter_indices: &BlitzOp,
    updates: &BlitzOp,
    update_computation: &BlitzComputation,
    dimension_numbers: &ScatterDimensionNummbers,
    indices_are_sorted: bool,
    unique_indices: bool) -> BlitzOp
  {
    self.scatter_many(
      &vec![input], scatter_indices, &vec![updates],
      update_computation, dimension_numbers, indices_are_sorted, unique_indices)
  }

  fn scatter_many(
    &mut self,
    _inputs: &Vec<&BlitzOp>,
    _scatter_indices: &BlitzOp,
    _updates: &Vec<&BlitzOp>,
    _update_computation: &BlitzComputation,
    _dimension_numbers: &ScatterDimensionNummbers,
    _indices_are_sorted: bool,
    _unique_indices: bool) -> BlitzOp
  {
    unimplemented!()
  }

  fn scatter_internal() {
      
  }

  fn send(&self, _operand: &BlitzOp, _handle: &ChannelHandle) {
    unimplemented!()
  }

  fn send_with_token(
    &self, _operand: &BlitzOp, _token: &BlitzOp, _handle: &ChannelHandle) -> BlitzOp
  {
    unimplemented!()  
  }

  fn send_to_host(
    &self,
    _operand: &BlitzOp,
    _token: &BlitzOp,
    _shape_with_layout: &Shape,
    _handle: &ChannelHandle) -> BlitzOp
  {
    unimplemented!()    
  }

  fn recv_from_host(
    &self, _token: &BlitzOp, _shape: &Shape, _handle: &ChannelHandle) -> BlitzOp
  {
    unimplemented!()    
  }

  fn create_token(&self) -> BlitzOp {
    unimplemented!()
  }

  fn after_all(&self, _tokens: &Vec<BlitzOp>) -> BlitzOp {
    unimplemented!()
  }

  fn recv(&self, _shape: &Shape, _handle: &ChannelHandle) -> BlitzOp {
    unimplemented!()
  }

  fn recv_with_token(
    &self, _token: &BlitzOp, _shape: &Shape, _handle: &ChannelHandle) -> BlitzOp
  {
    unimplemented!()    
  }

  fn batch_norm_training(
    &self,
    _operand: &BlitzOp,
    _scale: &BlitzOp,
    _offset: &BlitzOp,
    _epsilon: f64,
    _feature_index: i64) -> BlitzOp
  {
    unimplemented!()    
  }

  fn batch_norm_inference(
    &self,
    _operand: &BlitzOp,
    _scale: &BlitzOp,
    _offset: &BlitzOp,
    _mean: &BlitzOp,
    _variance: &BlitzOp,
    _epsilon: f64,
    _feature_index: i64) -> BlitzOp
  {
    unimplemented!()    
  }

  fn batch_norm_grad(
    &self,
    _operand: &BlitzOp,
    _scale: &BlitzOp,
    _batch_mean: &BlitzOp,
    _batch_var: &BlitzOp,
    _grad_output: &BlitzOp,
    _epsilon: f64,
    _feature_index: i64) -> BlitzOp
  {
    unimplemented!()    
  }

  fn get_dimension_size(&self, operand: &BlitzOp, dimension: i64) -> BlitzOp {
    let mut instr = HloInstruction::default();
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_get_dimension_size_shape(
      operand_shape.as_ref().unwrap(), dimension);
    if shape.is_err() { assert!(false); }

    // Calling GetDimensionSize on a static dimension returns a constant instruction.
    if operand_shape.as_ref().unwrap().is_static_dimension(dimension as usize) {
      // TODO
    }

    instr.set_shape(shape.as_ref().unwrap().clone());
    instr.add_dimensions(dimension);
    let result = self.add_instruction(
      &instr, HloOpcode::GetDimensionSize, &vec![operand]);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn set_dimension_size(
    &self, operand: &BlitzOp, val: &BlitzOp, dimension: i64) -> BlitzOp
  {
    let operand_shape = self.get_shape(operand);
    if operand_shape.is_err() { assert!(false); }
    let val_shape = self.get_shape(val);
    if val_shape.is_err() { assert!(false); }

    let shape = ShapeInference::infer_set_dimension_size_shape(
      operand_shape.as_ref().unwrap(),
      val_shape.as_ref().unwrap(),
      dimension);
    if shape.is_err() { assert!(false); }

    let result = self.set_dimension_size_internal(
      shape.as_ref().unwrap(), operand, val, dimension);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn set_dimension_size_internal(
    &self,
    shape: &Shape,
    operand: &BlitzOp,
    val: &BlitzOp,
    dimension: i64) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.add_dimensions(dimension);
    self.add_instruction(
      &instr, HloOpcode::SetDimensionSize, &vec![operand, val])
  }

  fn remove_dynamic_dimension(&self, _operand: &BlitzOp, _dimension: i64) -> BlitzOp {
    unimplemented!()
  }

  fn add_instruction(
    &self,
    _instr: &HloInstruction,
    _opcode: HloOpcode,
    _operands: &Vec<&BlitzOp>) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  fn add_called_computation(
    &self, _computation: &BlitzComputation, _instr: &HloInstruction)
  {
    unimplemented!()    
  }

  fn lookup_instruction_by_handle(&self, _handle: i64) -> Result<HloInstruction, String> {
    unimplemented!()
  }

  fn lookup_instruction(&self, _op: &BlitzOp) -> &HloInstruction {
    unimplemented!()
  }

  fn lookup_mutable_instruction(&mut self, _op: &BlitzOp) -> &mut HloInstruction {
    unimplemented!()
  }

  // Internal helper method that does the building for an arbitrary unary op.
  fn unary_op(&self, _unop: HloOpcode, _operand: &BlitzOp) -> BlitzOp {
    unimplemented!()
  }

  // Internal helper method that does the building for an arbitrary binary op.
  // broadcast_dimensions specifies which dimensions to use for broadcasting
  // when the operation is between tensors of different ranks. The direction is
  // only used if opcode is kCompare.
  fn binary_op(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _broadcast_dimensions: &Vec<i64>,
    _direction: Option<ComparisonDirection>,
    _type: Option<ComparisonType>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn compare(&self,
    shape: &Shape,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    direction: ComparisonDirection) -> Result<BlitzOp, String>
  {
    let operand_shape = self.get_shape(lhs);
    self.compare_internal(shape, lhs, rhs, direction, 
      default_comparison_type(&operand_shape.unwrap().element_type()))
  }

  // Internal helper method for binary op compare without broadcast dimensions.
  fn compare_internal(
    &self,
    shape: &Shape,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    direction: ComparisonDirection,
    t: ComparisonType) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_comparison_direction(direction);
    instr.set_comparison_type(t);
    instr.set_shape(shape.clone());
    self.add_instruction(&instr, HloOpcode::Compare, &vec![lhs, rhs])
  }

  // Internal helper method that does the building for an arbitrary binary op
  // with same ranked operands that doesn't broadcast.
  fn binary_op_no_broadcast(
    &self,
    binop: HloOpcode,
    shape: &Shape,
    lhs: &BlitzOp,
    rhs: &BlitzOp) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    let result = self.add_instruction(
      &instr, binop, &vec![lhs, rhs]);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  // Internal helper method that does the building for an arbitrary ternary op.
  fn ternary_op(
    &self,
    _triop: HloOpcode,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _ehs: &BlitzOp) -> BlitzOp
  {
    unimplemented!()    
  }

  fn rng_op(
    &mut self,
    distribution: RandomDistribution,
    parameters: &Vec<&BlitzOp>,
    shape: &Shape) -> BlitzOp
  {
    match distribution {
      RandomDistribution::Normal => {
        if parameters.len() != 2 {
          let mut err_msg = "RNG distribution (".to_string();
          err_msg.push_str(&random_distribution_name(&distribution));
          err_msg.push_str(") expects 2 parameters, but got ");
          err_msg.push_str(&parameters.len().to_string());
          println!("{:?}", err_msg);
          assert!(false);
        }
      }
      RandomDistribution::Uniform => {
        if parameters.len() != 2 {
          let mut err_msg = "RNG distribution (".to_string();
          err_msg.push_str(&random_distribution_name(&distribution));
          err_msg.push_str(") expects 2 parameters, but got ");
          err_msg.push_str(&parameters.len().to_string());
          println!("{:?}", err_msg);
          assert!(false);
        }
      }
      _ => {
        panic!("Unhandled distribution.");
      }
    }
    let validate =
      ShapeUtil::validate_shape_with_optional_layout(shape);
    if validate.is_err() { assert!(false); }

    let result =
      self.rng_op_internal(distribution, parameters, shape);
    if result.is_err() { assert!(false); }

    result.unwrap()
  }

  fn rng_op_internal(
    &mut self,
    distribution: RandomDistribution,
    parameters: &Vec<&BlitzOp>,
    shape: &Shape) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_distribution(distribution);
    self.add_instruction(&instr, HloOpcode::Rng, parameters)
  }

  fn in_dim_broadcast(
    &self,
    _shape: &Shape,
    _operand: &BlitzOp,
    _broadcast_dimensions: &Vec<i64>) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  // Internal helper method that creates a sequence of instructions that
  // performs an explicit broadcast of the operand to the target shape.
  // All dimensions of the operand must either be equal to the corresponding
  // output shape dimension, or be exactly 1.  (Such dimensions are the
  // degenerate dimensions.)
  fn add_broadcast_sequence(
    &self,
    _output_shape: &Shape,
    _operand: &BlitzOp) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  // Internal helper method that broadcasts a scalar to the shape of the output.
  fn broadcast_scalar_to_output_shape(
    &self,
    _caller: &BlitzOp,
    _output: &BlitzOp) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }

  // Internal helper method for creating a Reshape op with the already inferred
  // shape.
  fn reshape_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    inferred_dimension: i64) -> Result<BlitzOp, String>
  {
    if self.first_error.is_err() {
      return Err(self.first_error.as_ref().err().unwrap().clone());
    }
    if shape.is_unbounded_dynamic() {
      return Err("Reshaping with unbounded result shape is not supported.".to_string());
    }
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    if inferred_dimension != -1 {
      instr.add_dimensions(inferred_dimension);
    }
    self.add_instruction(&instr, HloOpcode::Reshape, &vec![operand])
  }

  // A visitor which checks whether an operation is a compile-time constant,
  // meaning that it doesn't depend on any parameters, or on any stateful
  // operation such as `RngNormal` or `Infeed`. The visitor walks the
  // computation starting at a given operation and sets is_constant to false iff
  // a parameter or stateful operation is encountered.
  fn is_constant_visitor(
    &self, _op_handle: i64, _depth: i64, _visited: &HashSet<i64>, _is_constant: &bool)
  {
    unimplemented!()    
  }

  // Checks bounds for convolution parameters.
  fn verify_convolution(
    &self,
    _lhs_shape: &Shape,
    _rhs_shape: &Shape,
    _dimension_numbers: &ConvolutionDimensionNumbers) -> Result<(), String>
  {
    unimplemented!()
  }

  fn get_next_id(&self) -> i64 {
    self.next_id
  }

  // Creates an op with the given opcode and the output shape.
  fn add_op_with_shape(
    &mut self,
    _opcode: HloOpcode,
    _shape: &Shape,
    _operands: &Vec<&BlitzOp>) -> Result<BlitzOp, String>
  {
    unimplemented!()    
  }
}

pub fn normalize_and_assign_sharding(
  instr: &mut HloInstruction, op_sharding: &OpSharding) -> Result<(), String>
{
  let shape = instr.shape();
  let sharding = HloSharding::from_proto(op_sharding);
  if sharding.is_err() {
    return Err(sharding.err().unwrap());
  }
  let normalized_sharding =
    sharding.unwrap().normalize_tuple_sharding(shape);
  let valid = normalized_sharding.validate(shape, None);
  if valid.is_err() {
    return Err(valid.err().unwrap());
  }

  instr.set_sharding(normalized_sharding); // Check.
  Ok(())
}

// Generates a fully qualified computation/instruction name.
pub fn get_full_name(base_name: String, separator: char, id: i64) -> String {
  let mut full_name = base_name.clone();
  full_name.push(separator);
  full_name.push_str(&id.to_string());
  full_name
}

pub struct BlitzScopedShardingAssignment {}

impl BlitzScopedShardingAssignment {
  pub fn new() -> Self {
    unimplemented!()
  }
}