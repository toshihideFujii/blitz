#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}, vec};

use common::{
  blitz_data::{
    random_distribution_name, ChannelHandle, ChannelType, ConvolutionDimensionNumbers,
    CustomCallApiVersion, CustomCallSchedule, DotDimensionNumbers, FftType, FrontendAttributes,
    GatherDimensionNumbers, OpMetadata, OpSharding, OpShardingType, PaddingConfig, PaddingType,
    PrecisionConfig, PrimitiveType, RandomAlgorithm, RandomDistribution, ReplicaGroup, ResultAccuracy,
    ScatterDimensionNummbers, SliceDimensions, SparsityDescriptor, TriangularSolveOptions, Window
  },
  comparison_util::{default_comparison_type, ComparisonDirection, ComparisonType},
  layout::Layout, layout_util::LayoutUtil, literal::Literal, literal_util::LiteralUtil,
  primitive_util::{
    bit_width, is_complex_type, is_unsigned_integral_type, primitive_type_name,
    unsigned_integral_type_for_bit_width
  },
  shape::{ProgramShape, Shape, /*ShapeEqual*/},
  shape_util::ShapeUtil, util::{self, make_no_padding_config}
};

use hlo::{
  hlo_input_output_alias_config::AliasKind, hlo_instruction::HloInstruction,
  hlo_opcode::{hlo_opcode_string, HloOpcode}, hlo_sharding::HloSharding
};
use service::{blitz_computation::BlitzComputation, shape_inference::ShapeInference};

use crate::padding::{make_padding, Padding};

// This represents an instruction that has been enqueued using the BlitzBuilder.
// This is used to pass to subsequent computations that depends upon the
// instruction as an operand.
#[derive(Clone)]
pub struct BlitzOp<'builder> {
  // < 0 means "invalid handle".
  handle: i64,
  // Not owned. Non-null for any handle returned by BlitzBuilder, even if the
  // handle is invalid.
  builder: Option<&'builder BlitzBuilder>,
}

impl<'builder> BlitzOp<'builder> {
  pub fn default() -> Self {
    BlitzOp { handle: -1, builder: None }
  }

  pub fn new(builder: &'builder BlitzBuilder) -> Self {
    BlitzOp { handle: -1, builder: Some(builder) }
  }

  pub fn new_from_handle(handle: i64, builder: &'builder BlitzBuilder) -> Self {
    BlitzOp { handle: handle, builder: Some(builder) }
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

  pub fn mutable_builder(&mut self) -> &'builder mut BlitzBuilder {
    assert!(self.builder.is_some());
    //let builder = self.builder.as_mut().unwrap();
    //builder
    unimplemented!()
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
    self.handle == rhs.handle && self.builder == rhs.builder
  }

  pub fn handle(&self) -> i64 {
    self.handle
  }
}

// Describes an input/output alias as inserted by the SetUpAlias() API.
#[derive(Debug, PartialEq)]
struct InputOutputAlias {
  output_index: Vec<i64>,
  param_number: i64,
  param_index: Vec<i64>,
  kind: AliasKind,
}

impl InputOutputAlias {
  pub fn new(
    output_index: Vec<i64>,
    param_number: i64,
    param_index: Vec<i64>,
    kind: AliasKind) -> Self
  {
    InputOutputAlias {
      output_index: output_index,
      param_number: param_number,
      param_index: param_index,
      kind: kind
    }    
  }
}

// We don't overload the relational operators (==, !=, <, <=, >, >=) because the
// semantics might be surprising since their result types are usually 'bool'.
// Further programmers may expect == to be a structural equality.
// We also choose not to overload any of the mutating operators (e.g., +=, -=)
// because the semantics might be misleading — Blitz computations are immutable.

// A convenient interface for building up computations.
//
// Thread-compatible.
#[derive(PartialEq)]
pub struct BlitzBuilder {
  // Name to use for the built computation.
  name: String,

  // The next sequential ID for every instruction/computation contained within
  // this computation.
  next_id: i64,

  // Holds the input/output alias information populated by the SetUpAlias() API.
  input_output_aliases: Vec<InputOutputAlias>,

  // A map from XlaOp::Handle to the index in the instructions_ vector where the
  // instruction is held.
  handle_to_index: HashMap<i64, i64>,
  metadata: OpMetadata,

  // The metadata to attach to each op. This is structured as a "modal"-like
  // operation, in order to simplify client code (and not sprinkle this metadata
  // throughout the TensorFlow op kernel implementations).
  oneshot_metadata: Option<OpMetadata>,
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
      input_output_aliases: Vec::new(),
      handle_to_index: HashMap::new(),
      metadata: OpMetadata::new(),
      oneshot_metadata: None,
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
    self.oneshot_metadata = Some(metadata);
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

  // Swap the passed FrontendAttributes with the ones currently set.
  // Return the old attributes.
  pub fn swap_frontend_attributes(
    &mut self, frontend_attributes: FrontendAttributes) -> FrontendAttributes
  {
    let old_attributes = self.frontend_attributes.clone();
    self.frontend_attributes = frontend_attributes;
    old_attributes
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
    
    dimension_numbers
      .set_input_batch_dimension(BlitzBuilder::CONV_BATCH_DIMENSION);
    dimension_numbers
      .set_input_feature_dimension(BlitzBuilder::CONV_FEATURE_DIMENSION);
    dimension_numbers
      .set_output_batch_dimension(BlitzBuilder::CONV_BATCH_DIMENSION);
    dimension_numbers
      .set_output_feature_dimension(BlitzBuilder::CONV_FEATURE_DIMENSION);
    dimension_numbers
      .set_kernel_input_feature_dimension(BlitzBuilder::CONV_KERNEL_INPUT_DIMENSION);
    dimension_numbers
      .set_kernel_output_feature_dimension(BlitzBuilder::CONV_KERNEL_OUTPUT_DIMENSION);

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

  // Builds an embedded computation from a subbuilder. Returns the ID of the
  // subcomputation in the parent builder.
  pub fn build_sub_computation(
    &self,
    _root: Option<BlitzOp>,
    _remove_dynamic_dimensions: bool) -> Result<i64, String>
  {
    unimplemented!()    
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
    if root.builder.unwrap() == self {
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
    BlitzOp::new(self)
  }

  // A helper function that converts a absl::StatusOr<BlitzOp> into an BlitzOp.
  // If the absl::Status was an error, reports the error to builder and returns
  // an invalid BlitzOp handle.
  pub fn report_error_or_return<'builder>(
    &'builder mut self, op: Result<BlitzOp<'builder>, String>) -> BlitzOp<'builder>
  {
    if !self.first_error.is_ok() {
      return BlitzOp::new(self);
    }
    if !op.is_ok() {
      let err_msg = op.err().unwrap();
      return self.report_error(&Err(err_msg));
    }
    op.ok().unwrap()
  }

  // Returns true if 'operand' is a compile-time constant. A compile-time
  // constant does not depend on any parameters, or on stateful operators such
  // as `RngNormal` or `Infeed`.
  //
  // This tests whether a computation is a compile-time constant without
  // evaluating the computation.
  pub fn is_constant(&self, _operand: &BlitzOp) -> Result<bool, String> {
    if self.first_error.is_err() {
      let err_msg = self.first_error.as_ref().err().unwrap().clone();
      return Err(err_msg);
    }
    let is_constant = false;
    //let visited = HashSet::new();
    //self.is_constant_visitor(
      //operand.handle(), 0, &visited, &is_constant);
    Ok(is_constant)
  }

  // Adds a new input/output alias. Since the input/output shape information are
  // not available until the computation is built, any eventual error in the
  // arguments of this API will be detected only at computation Build() time.
  //
  // Note: Except when 'must-alias' is true, alias is assumed to be 'may-alias'
  // and only donated buffer at runtime will be aliased with output. If a buffer
  // is not donated at runtime, a copy will be inserted by XLA to prevent buffer
  // clobbering.
  pub fn setup_alias(
    &mut self,
    output_index: Vec<i64>,
    param_number: i64,
    param_index: Vec<i64>,
    kind: AliasKind)
  {
    self.input_output_aliases.push(
      InputOutputAlias::new(output_index, param_number, param_index, kind));
  }

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
    frontend_attrs.set_attribute("".to_string(), value); // TODO
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
  pub fn check_op_builder(&self, op: &BlitzOp) -> Result<(), String> {
    if op.builder() != self {
      let mut err_msg = "BlitzOp with handle ".to_string();
      err_msg.push_str(&op.handle.to_string());
      err_msg.push_str(" id built by builder ");
      err_msg.push_str(&op.builder().name());
      err_msg.push_str(", but is trying to use it in builder ");
      err_msg.push_str(&self.name());
      return Err(err_msg);
    }
    Ok(())
  }

  // Description for the methods below can be found in the corresponding public
  // functions section in this file.
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
      &mut instr, HloOpcode::Parameter, &vec![]);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn constant_literal<T>(&mut self, literal: &Literal<T>) -> BlitzOp
    where T: Clone + Default + PartialEq
  {
    if literal.shape().is_array() && literal.element_count(&vec![]) > 1 &&
      literal.is_all_first()
    {
      let scalar = LiteralUtil::get_first_scalar_literal(literal);
      let mut const_instr = HloInstruction::default();
      const_instr.set_shape(scalar.shape().clone());

      let scalar_op = self.add_instruction(
        &mut const_instr, HloOpcode::Constant, &vec![]);
      if scalar_op.is_err() { assert!(false); }

      unimplemented!()
      //return self.broadcast(
        //scalar_op.as_ref().unwrap(),
        //literal.shape().dimensions_vec()); 
    } else {
      let mut const_instr = HloInstruction::default();
      const_instr.set_shape(literal.shape().clone());

      let result = self.add_instruction(
        &mut const_instr, HloOpcode::Constant, &vec![]);
      if result.is_err() { assert!(false); }
      result.unwrap()
    }
  }

  fn broadcast(
    &mut self,
    operand: &BlitzOp,
    broadcast_sizes: &Vec<i64>) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    let operand_shape = operand_shape_wrapper.unwrap();

    let shape = ShapeInference::infer_broadcast_shape(
      &operand_shape, broadcast_sizes);
    if shape.is_err() { assert!(false); }

    // The client-level broadcast op just appends dimensions on the left (adds
    // lowest numbered dimensions). The HLO broadcast instruction is more
    // flexible and can add new dimensions anywhere. The instruction's
    // dimensions field maps operand dimensions to dimensions in the broadcast
    // output, so to append dimensions on the left the instruction's dimensions
    // should just be the n highest dimension numbers of the output shape where
    // n is the number of input dimensions.
    let operand_rank = operand_shape.rank();
    let mut dimensions: Vec<i64> = Vec::new();
    for i in 0..operand_rank {
      dimensions[i] = (i + shape.as_ref().unwrap().rank() - operand_rank) as i64;
    }
    let result = self.in_dim_broadcast(
      shape.as_ref().unwrap(), operand, &dimensions);
    if result.is_err() { assert!(false); }
    result.unwrap()
  }

  fn broadcast_in_dim<'builder>(
    _operand: &'builder BlitzOp,
    _out_dim_size: &Vec<i64>,
    _broadcast_dimensions: &Vec<i64>) -> BlitzOp<'builder>
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
    &mut self,
    operand: BlitzOp,
    padding_value: BlitzOp,
    padding_config: &PaddingConfig) -> BlitzOp
  {
    let operand_shape = self.get_shape(&operand);
    if operand_shape.is_err() { assert!(false); }

    let padding_value_shape = self.get_shape(&padding_value);
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
    &mut self,
    operand: BlitzOp,
    padding_value: BlitzOp,
    dimno: i64,
    pad_lo: i64,
    pad_hi: i64) -> BlitzOp
  {
    let shape = self.get_shape(&operand);
    if shape.is_err() { assert!(false); }

    let mut padding_config = make_no_padding_config(
      shape.as_ref().unwrap().rank() as i64);
    let dims = padding_config.mutable_dimensions(dimno);
    dims.set_edge_padding_low(pad_lo);
    dims.set_edge_padding_high(pad_hi);
    
    self.pad(operand, padding_value, &padding_config)
  }

  fn pad_internal(
    &mut self,
    shape: &Shape,
    operand: BlitzOp,
    padding_value: BlitzOp,
    padding_config: &PaddingConfig) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_padding_config(padding_config.clone());
    self.add_instruction(&mut instr,
      HloOpcode::Pad, &vec![operand, padding_value])
  }

  fn reshape<'builder>(
    &'builder mut self,
    _operand: BlitzOp,
    _dimensions: &Vec<i64>,
    _new_sizes: &Vec<i64>,
    _inferred_dimension: i64) -> BlitzOp
  {
    /*
    let operand_shape = self.get_shape(&operand);
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
      transposed,
      inferred_dimension);
    if result.is_err() { assert!(false); }
    result.unwrap()
    */
    unimplemented!()
  }

  pub fn reshape_without_dimensions(
    &mut self,
    operand: BlitzOp,
    new_sizes: &Vec<i64>,
    inferred_dimension: i64) -> BlitzOp
  {
    let shape = self.get_shape(&operand);
    if shape.is_err() { assert!(false); }
    let dimensions: Vec<i64> = vec![0; shape.as_ref().unwrap().dimensions_size()];
    self.reshape(operand, &dimensions, new_sizes, inferred_dimension)
  }

  fn dynamic_reshape(
    &mut self,
    operand: &BlitzOp,
    dim_sizes: &Vec<BlitzOp>,
    new_size_bounds: &Vec<i64>,
    dims_are_dynamic: &Vec<bool>) -> BlitzOp
  {
    let operand_shape = self.get_shape(operand);
    let dim_size_shape_ptrs = 
      self.get_operand_shapes(dim_sizes).unwrap();

    let shape = ShapeInference::infer_dynamic_reshape_shape(
      &operand_shape.unwrap(),
      &dim_size_shape_ptrs,
      new_size_bounds,
      dims_are_dynamic);
    if shape.is_err() {
      let err_msg = shape.as_ref().err().unwrap();
      assert!(false, "{:?}", err_msg);
    }
    let mut operands = vec![];
    operands.push(operand.clone());
    for dim_size in dim_sizes {
      operands.push(dim_size.clone());
    }
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.unwrap());
    let result =
      self.add_instruction(&mut instr, HloOpcode::DynamicReshape, &operands);
    if result.is_err() {
      let err_msg = result.as_ref().err().unwrap();
      assert!(false, "{:?}", err_msg);
    }
    result.unwrap()
  }

  fn mhlo_dynamic_reshape<'builder>(
    &self,
    operand: &'builder mut BlitzOp<'builder>,
    output_shape: &BlitzOp,
    shape: &Shape) -> BlitzOp<'builder>
  {
    let operand_shape_wrapper = self.get_shape(operand);
    if operand_shape_wrapper.is_err() {
      let err_msg = operand_shape_wrapper.as_ref().err().unwrap();
      assert!(false, "{:?}", err_msg);
    }
    let operand_shape = operand_shape_wrapper.unwrap();
    if operand_shape.element_type() != shape.element_type() {
      let mut err_msg = "Element type of operand ".to_string();
      err_msg.push_str(&ShapeUtil::human_string(&operand_shape));
      err_msg.push_str(" and output ");
      err_msg.push_str(&ShapeUtil::human_string(&shape));
      err_msg.push_str(" must match");
      assert!(false, "{:?}", err_msg);
    }
    if operand_shape.is_static() && shape.is_static() &&
      ShapeUtil::elements_in(&operand_shape) != ShapeUtil::elements_in(&shape)
    {
      let mut err_msg =
        "mhlo_dynamic_reshape has mismatched element counts: from=".to_string();
      err_msg.push_str(&ShapeUtil::elements_in(&operand_shape).to_string());
      err_msg.push_str(" (");
      err_msg.push_str(&ShapeUtil::human_string(&operand_shape));
      err_msg.push_str(") to=");
      err_msg.push_str(&ShapeUtil::elements_in(&shape).to_string());
      err_msg.push_str(" (");
      err_msg.push_str(&ShapeUtil::human_string(&shape));
      err_msg.push_str(")");
      assert!(false, "{:?}", err_msg);
    }
    let output_shape_shape_wrapper = self.get_shape(output_shape);
    if output_shape_shape_wrapper.is_err() {
      let err_msg = output_shape_shape_wrapper.as_ref().err().unwrap();
      assert!(false, "{:?}", err_msg);
    }
    let output_shape_shape = output_shape_shape_wrapper.unwrap();
    if output_shape_shape.dimensions(0) != shape.dimensions_vec().len() as i64 {
      let mut err_msg = "output_shape dimension size=".to_string();
      err_msg.push_str(&output_shape_shape.dimensions(0).to_string());
      err_msg.push_str(" (");
      err_msg.push_str(&ShapeUtil::human_string(&output_shape_shape));
      err_msg.push_str(") and rank of shape=");
      err_msg.push_str(&shape.dimensions_vec().len().to_string());
      err_msg.push_str(" (");
      err_msg.push_str(&ShapeUtil::human_string(&shape));
      err_msg.push_str(") must match");
      assert!(false, "{:?}", err_msg);
    }

    custom_call::<i64>(
      operand.mutable_builder(),
      &"mhlo.dynamic_reshape".to_string(),
      &vec![operand.clone(),
      output_shape.clone()],
      shape,
      &"".to_string(),
      false,
      &vec![],
      None,
      CustomCallSchedule::None,
      CustomCallApiVersion::Original)
  }

  fn collapse<'builder>(
    &'builder mut self,
    operand: BlitzOp<'builder>,
    dimensions: &Vec<i64>) -> BlitzOp<'builder>
  {
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
    let original_shape = self.get_shape(&operand);
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
    operand: BlitzOp,
    start_indices: &Vec<i64>,
    limit_indices: &Vec<i64>,
    strides: &Vec<i64>) -> BlitzOp
  {
    let operand_shape = self.get_shape(&operand);
    check_error(&operand_shape);

    let shape = ShapeInference::infer_slice_shape(
      operand_shape.as_ref().unwrap(),
      start_indices,
      limit_indices, strides);
    check_error(&shape);

    let result = self.slice_internal(
      shape.as_ref().unwrap(), operand, start_indices, limit_indices, strides);
    check_error(&result);
    
    result.unwrap()
  }

  fn slice_internal(
    &mut self,
    shape: &Shape,
    operand: BlitzOp,
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
    self.add_instruction(&mut instr, HloOpcode::Slice, &vec![operand])
  }

  fn slice_in_dim(
    &mut self,
    operand: BlitzOp,
    start_index: i64,
    limit_index: i64,
    stride: i64,
    dimno: i64) -> BlitzOp
  {
    let shape = self.get_shape(&operand);
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
    &mut self,
    operand: &BlitzOp,
    start_indices: &Vec<BlitzOp>,
    slice_sizes: &Vec<i64>) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let start_indices_shapes_wrapper =
      self.get_operand_shapes(start_indices);
    check_error(&start_indices_shapes_wrapper);

    let start_indices_shapes = start_indices_shapes_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_dynamic_slice_shape(
        &operand_shape,
        &start_indices_shapes,
        slice_sizes,
        true);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result =
      self.dynamic_slice_internal(&shape, operand, start_indices, slice_sizes);
    check_error(&result);
    
    result.unwrap()
  }

  fn dynamic_slice_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    start_indices: &Vec<BlitzOp>,
    slicee_sizes: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());

    for size in slicee_sizes {
      instr.add_dynamic_slice_sizes(*size);
    }

    let mut operands = vec![operand.clone()];
    for indice in start_indices {
      operands.push(indice.clone());
    }
    self.add_instruction(&mut instr, HloOpcode::DynamicSlice, &operands)
  }

  fn dynamic_update_slice(
    &mut self,
    operand: &BlitzOp,
    update: &BlitzOp,
    start_indices: &Vec<BlitzOp>) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    
    let update_shape_wrapper = self.get_shape(update);
    check_error(&update_shape_wrapper);
    
    let start_indices_shape_wrapper =
      self.get_operand_shapes(start_indices);
    check_error(&start_indices_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let update_shape = update_shape_wrapper.unwrap();
    let start_indices_shape = start_indices_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_dynamic_update_slice_shape(
        &operand_shape,
        &update_shape,
        &start_indices_shape,
        true);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result =
      self.dynamic_update_slice_internal(&shape, operand, update, start_indices);
    check_error(&result);
    
    result.unwrap()
  }

  fn dynamic_update_slice_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    update: &BlitzOp,
    start_indices: &Vec<BlitzOp>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    let mut operands = vec![operand.clone(), update.clone()];
    for op in start_indices {
      operands.push(op.clone());
    }
    self.add_instruction(&mut instr, HloOpcode::DynamicUpdateSlice, &operands)
  }

  fn concat_in_dim(&mut self, operands: &Vec<BlitzOp>, dimension: i64) -> BlitzOp {
    let operand_shapes_wrapper = self.get_operand_shapes(operands);
    check_error(&operand_shapes_wrapper);
    
    let operand_shapes = operand_shapes_wrapper.unwrap();
    let shape_wrapper = ShapeInference::infer_concat_op_shape(
      &operand_shapes, dimension);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    self.concat_in_dim_internal(shape, operands, dimension)
  }

  fn concat_in_dim_internal(
    &mut self,
    shape: Shape,
    operands: &Vec<BlitzOp>,
    dimension: i64) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape);
    instr.add_dimensions(dimension);
    self.add_instruction(&mut instr, HloOpcode::Concatenate, operands).unwrap()
  }

  fn select(
    &mut self,
    pred: &BlitzOp,
    on_true: &BlitzOp,
    on_false: &BlitzOp) -> BlitzOp
  {
    let true_shape_wrapper = self.get_shape(on_true);
    check_error(&true_shape_wrapper);
    
    let false_shape_wrapper = self.get_shape(on_false);
    check_error(&false_shape_wrapper);
    
    let true_shape = true_shape_wrapper.unwrap();
    let false_shape = false_shape_wrapper.unwrap();
    assert!(true_shape.is_tuple() == false_shape.is_tuple());

    let passthrough_computation =
      |shape: &Shape| -> Result<i64, String>
    {
      let mut builder =
        self.create_sub_builder("dummy".to_string());
      let _out = builder.parameter(
        0, shape, &"p".to_string(), &vec![]);
      //builder.build_sub_computation(Some(out), false) // TODO
      unimplemented!()
    };
    if true_shape.is_tuple() {
      let passthrough_true = passthrough_computation(&true_shape);
      check_error(&passthrough_true);
      
      let passthrough_false = passthrough_computation(&false_shape);
      check_error(&passthrough_false);
      
      return self.conditional(
        pred,
        on_true,
        passthrough_true.unwrap(),
        on_false,
        passthrough_false.unwrap());
    }
    self.ternary_op(HloOpcode::Select, pred, on_true, on_false)
  }

  fn tuple(
    &mut self,
    elements: &Vec<BlitzOp>) -> BlitzOp
  {
    let operand_shapes_wrapper =
      self.get_operand_shapes(elements);
    check_error(&operand_shapes_wrapper);
    
    let operand_shapes = operand_shapes_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_variadic_op_shape_by_opshapes(
        HloOpcode::Tuple, &operand_shapes);
    check_error(&shape_wrapper);
    
    let shape = shape_wrapper.unwrap();
    self.tuple_internal(&shape, elements).unwrap()
  }

  fn tuple_internal(
    &mut self,
    shape: &Shape,
    elements: &Vec<BlitzOp>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    self.add_instruction(&mut instr, HloOpcode::Tuple, elements)
  }

  fn get_tuple_element(&mut self, tuple_data: &BlitzOp, index: i64) -> BlitzOp {
    let tuple_shape_wrapper = self.get_shape(tuple_data);
    if tuple_shape_wrapper.is_err() {
      assert!(false, "{:?}", tuple_shape_wrapper.err());
    }
    let tuple_shape = tuple_shape_wrapper.unwrap();
    if !tuple_shape.is_tuple() {
      assert!(false, "Operand to get_tuple_element is not a tuple: {:?}", tuple_shape);
    }
    if index < 0 || (index as usize) >= ShapeUtil::tuple_element_count(&tuple_shape) {
      assert!(false, "get_tuple_element index {:?} out of range for tuple shape {:?}",
        index, tuple_shape);
    }
    self.get_tuple_element_internal(
      ShapeUtil::get_tuple_element_shape(&tuple_shape, index as usize),
      tuple_data,
      index).unwrap()
  }

  fn get_tuple_element_internal(
    &mut self,
    shape: &Shape,
    tuple_data: &BlitzOp,
    index: i64) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_tuple_index(index);
    self.add_instruction(
      &mut instr,
      HloOpcode::GetTupleElement,
      &vec![tuple_data.clone()])
  }

  fn dot(
    &mut self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    precision_config: Option<PrecisionConfig>,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    let lhs_shape_wrapper = self.get_shape(lhs);
    check_error(&lhs_shape_wrapper);
    
    let lhs_shape = lhs_shape_wrapper.unwrap();
    let mut dimension_numbers = DotDimensionNumbers::default();
    let mut dims = 1;
    if lhs_shape.dimensions_vec().len() == 1 { dims = 0; }
    dimension_numbers.add_lhs_contracting_dimensions(dims);
    dimension_numbers.add_rhs_contracting_dimensions(0);
    
    self.dot_general(lhs, rhs, &dimension_numbers, precision_config, preferred_element_t)
  }

  fn dot_general(
    &mut self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    dimension_numbers: &DotDimensionNumbers,
    precision_config: Option<PrecisionConfig>,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    let lhs_shape_wrapper = self.get_shape(lhs);
    check_error(&lhs_shape_wrapper);
    
    let rhs_shape_wrapper = self.get_shape(rhs);
    check_error(&rhs_shape_wrapper);
    
    let lhs_shape = lhs_shape_wrapper.unwrap();
    let rhs_shape = rhs_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_dot_op_shape(
        &lhs_shape, &rhs_shape,
        dimension_numbers,
        preferred_element_t,
        &vec![]);
    check_error(&shape_wrapper);
    
    let shape = shape_wrapper.unwrap();
    let result = self.dot_general_internal(
      shape, lhs.clone(), rhs.clone(), dimension_numbers, precision_config);
    check_error(&result);
    result.unwrap()
  }

  fn dot_general_internal(
    &mut self,
    shape: Shape,
    lhs: BlitzOp,
    rhs: BlitzOp,
    dimension_numbers: &DotDimensionNumbers,
    precision_config: Option<PrecisionConfig>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape);
    instr.set_dot_dimension_numbers(dimension_numbers.clone());
    if precision_config.is_some() {
      instr.set_precision_config(precision_config.unwrap().clone());
    }
    self.add_instruction(&mut instr, HloOpcode::Dot, &vec![lhs, rhs])
  }

  fn sparse_dot(&mut self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    sparse_meta: &Vec<BlitzOp>,
    sparsity: &Vec<SparsityDescriptor>,
    dimension_numbers: &DotDimensionNumbers,
    precision_config: Option<PrecisionConfig>,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    let lhs_shape_wrapper = self.get_shape(lhs);
    check_error(&lhs_shape_wrapper);
    
    let rhs_shape_wrapper = self.get_shape(rhs);
    check_error(&rhs_shape_wrapper);
    
    let lhs_shape = lhs_shape_wrapper.unwrap();
    let rhs_shape = rhs_shape_wrapper.unwrap();
    let shape = ShapeInference::infer_dot_op_shape(
      &lhs_shape, &rhs_shape, dimension_numbers,
      preferred_element_t, sparsity);
    let mut operands = vec![lhs.clone(), rhs.clone()];
    for op in sparse_meta {
      operands.push(op.clone());
    }
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.unwrap());
    instr.set_dot_dimension_numbers(dimension_numbers.clone());
    if precision_config.is_some() {
      instr.set_precision_config(precision_config.unwrap());
    }
    for descriptor in sparsity {
      instr.add_dot_sparsity(descriptor.clone());
    }
    let result =
      self.add_instruction(&mut instr, HloOpcode::Dot, &operands);
    check_error(&result);
    result.unwrap()
  }

  fn conv(
    &mut self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Padding,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    self.conv_with_general_dimensions(
      lhs,
      rhs,
      window_strides,
      padding,
      &BlitzBuilder::create_default_conv_dimension_numbers(
        window_strides.len() as i64),
      feature_group_count,
      batch_group_count,
      precision_config,
      preferred_element_t)
  }

  fn conv_with_general_padding(
    &mut self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    self.conv_general(
      lhs,
      rhs,
      window_strides,
      padding,
      &BlitzBuilder::create_default_conv_dimension_numbers(
        window_strides.len() as i64),
      feature_group_count,
      batch_group_count,
      precision_config,
      preferred_element_t)
  }

  fn conv_with_general_dimensions(
    &mut self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Padding,
    dimension_numbers: &ConvolutionDimensionNumbers,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    let lhs_shape_wrapper = self.get_shape(lhs);
    check_error(&lhs_shape_wrapper);
    
    let rhs_shape_wrapper = self.get_shape(rhs);
    check_error(&rhs_shape_wrapper);
    
    let lhs_shape = lhs_shape_wrapper.unwrap();
    let rhs_shape = rhs_shape_wrapper.unwrap();
    let result =
      self.verify_convolution(&lhs_shape, &rhs_shape, dimension_numbers);
    check_error(&result);
    
    let dim_size = dimension_numbers.input_spatial_dimensions_size();
    let mut base_area_dimensions = vec![];
    for i in 0..dim_size {
      base_area_dimensions.push(lhs_shape.dimensions(
        dimension_numbers.input_spatial_dimensions(i) as usize));
    }
    let win_dim_size = dimension_numbers.kernel_spatial_dimensions_size();
    let mut window_dimensions = vec![];
    for i in 0..win_dim_size {
      window_dimensions.push(rhs_shape.dimensions(
        dimension_numbers.kernel_spatial_dimensions(i) as usize));
    }
    self.conv_general(
      lhs,
      rhs,
      window_strides,
      &make_padding(&base_area_dimensions,
        &window_dimensions, window_strides, padding),
      dimension_numbers,
      feature_group_count,
      batch_group_count,
      precision_config,
      preferred_element_t)
  }

  fn conv_general(
    &mut self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    dimension_numbers: &ConvolutionDimensionNumbers,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    self.conv_general_dilated(
      lhs,
      rhs,
      window_strides,
      padding,
      &vec![],
      &vec![],
      dimension_numbers,
      feature_group_count,
      batch_group_count,
      precision_config,
      preferred_element_t,
      None)
  }

  fn conv_general_dilated(
    &mut self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    lhs_dilation: &Vec<i64>,
    rhs_dilation: &Vec<i64>,
    dimension_numbers: &ConvolutionDimensionNumbers,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    preferred_element_t: Option<PrimitiveType>,
    window_reversal: Option<&Vec<bool>>) -> BlitzOp
  {
    let lhs_shape_wrapper = self.get_shape(lhs);
    check_error(&lhs_shape_wrapper);
    
    let rhs_shape_wrapper = self.get_shape(rhs);
    check_error(&rhs_shape_wrapper);
    
    let lhs_shape = lhs_shape_wrapper.unwrap();
    let rhs_shape = rhs_shape_wrapper.unwrap();
    let result =
      self.verify_convolution(&lhs_shape, &rhs_shape, dimension_numbers);
    check_error(&result);
    
    let dim_size = dimension_numbers.kernel_spatial_dimensions_size();
    let mut window_dimensions = vec![];
    for i in 0..dim_size {
      window_dimensions.push(rhs_shape.dimensions(
        dimension_numbers.kernel_spatial_dimensions(i) as usize));
    }
    let window_wrapper =
      ShapeInference::infer_window_from_dimensions(&window_dimensions, window_strides,
        padding, lhs_dilation, rhs_dilation, window_reversal);
    check_error(&window_wrapper);
    
    let window = window_wrapper.unwrap();
    let shape_wrapper = ShapeInference::infer_convolve_shape(
      &lhs_shape, &rhs_shape, feature_group_count, batch_group_count, &window,
      dimension_numbers, preferred_element_t);
    check_error(&shape_wrapper);
    
    let shape = shape_wrapper.unwrap();
    let result =
      self.conv_general_dilated_internal(&shape, lhs, rhs, &window, window_strides,
        padding, lhs_dilation, rhs_dilation, dimension_numbers, feature_group_count,
        batch_group_count, precision_config);
    check_error(&result);
    
    result.unwrap()
  }

  fn dynamic_conv_forward(
    &mut self,
    lhs: BlitzOp,
    rhs: BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    lhs_dilation: &Vec<i64>,
    rhs_dilation: &Vec<i64>,
    dimension_numbers: &ConvolutionDimensionNumbers,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    padding_t: PaddingType,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    let instr_wrapper = self.dynamic_conv_instruction(
      &lhs, &rhs, window_strides, padding, lhs_dilation, rhs_dilation,
      dimension_numbers, feature_group_count, batch_group_count, precision_config,
      padding_t, preferred_element_t);
    check_error(&instr_wrapper);
    
    let mut instr = instr_wrapper.unwrap();
    instr.set_custom_call_target("dynamic_convolution_forward".to_string());

    let result =
      self.add_instruction(&mut instr, HloOpcode::CustomCall, &vec![lhs, rhs]);
    check_error(&result);

    result.unwrap()
  }

  fn dynamic_conv_input_grad(
    &mut self,
    input_sizes: BlitzOp,
    lhs: BlitzOp,
    rhs: BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    lhs_dilation: &Vec<i64>,
    rhs_dilation: &Vec<i64>,
    dimension_numbers: &ConvolutionDimensionNumbers,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    padding_t: PaddingType,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    let instr_wrapper = self.dynamic_conv_instruction(
      &lhs, &rhs, window_strides, padding, lhs_dilation, rhs_dilation, dimension_numbers,
      feature_group_count, batch_group_count, precision_config, padding_t, preferred_element_t);
    check_error(&instr_wrapper);

    let mut instr = instr_wrapper.unwrap();
    instr.set_custom_call_target("dynamic_convolution_input_grad".to_string());

    let result = self.add_instruction(
      &mut instr, HloOpcode::CustomCall, &vec![input_sizes, lhs, rhs]);
    check_error(&result);
    result.unwrap()
  }

  fn dynamic_conv_kernel_grad(
    &mut self,
    activations: BlitzOp,
    gradients: BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    lhs_dilation: &Vec<i64>,
    rhs_dilation: &Vec<i64>,
    dimension_numbers: &ConvolutionDimensionNumbers,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    padding_t: PaddingType,
    preferred_element_t: Option<PrimitiveType>) -> BlitzOp
  {
    let instr_wrapper = self.dynamic_conv_instruction(
      &activations, &gradients, window_strides, padding, lhs_dilation, rhs_dilation,
      dimension_numbers, feature_group_count, batch_group_count, precision_config, padding_t,
      preferred_element_t);
    check_error(&instr_wrapper);

    let mut instr = instr_wrapper.unwrap();
    instr.set_custom_call_target("dynamic_convolution_kernel_grad".to_string());

    // The gradient of kernel has kernel shape and shouldn't have any dynamic sizes.
    instr.mutable_shape().clear_is_dynamic_dimension();
    let result = self.add_instruction(
      &mut instr, HloOpcode::CustomCall, &vec![activations, gradients]);
    check_error(&result);
    result.unwrap()
  }

  fn dynamic_conv_instruction(
    &self,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    lhs_dilation: &Vec<i64>,
    rhs_dilation: &Vec<i64>,
    dimension_numbers: &ConvolutionDimensionNumbers,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>,
    padding_t: PaddingType,
    preferred_element_t: Option<PrimitiveType>) -> Result<HloInstruction, String>
  {
    let lhs_shape_wrapper = self.get_shape(lhs);
    check_error(&lhs_shape_wrapper);
    let rhs_shape_wrapper = self.get_shape(rhs);
    check_error(&rhs_shape_wrapper);

    let lhs_shape = lhs_shape_wrapper.unwrap();
    let rhs_shape = rhs_shape_wrapper.unwrap();

    let mut window_dimensions = vec![];
    let size = dimension_numbers.kernel_spatial_dimensions_size();
    for i in 0..size {
      window_dimensions.push(rhs_shape.dimensions(
        dimension_numbers.kernel_spatial_dimensions(i) as usize));
    }

    let window_wrapper =
      ShapeInference::infer_window_from_dimensions(
        &window_dimensions, window_strides, padding, lhs_dilation, rhs_dilation,
        None);
    check_error(&window_wrapper);

    let window = window_wrapper.unwrap();
    let shape_wrapper = ShapeInference::infer_convolve_shape(
      &lhs_shape, &rhs_shape, feature_group_count, batch_group_count, &window,
      dimension_numbers, preferred_element_t);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let mut instr = HloInstruction::default();
    instr.set_shape(shape);
    instr.set_window(window);
    instr.set_convolution_dimension_numberes(dimension_numbers.clone());
    instr.set_feature_group_count(feature_group_count);
    instr.set_batch_group_count(batch_group_count);
    instr.set_padding_type(padding_t);

    if precision_config.is_some() {
      instr.set_precision_config(precision_config.unwrap());
    }

    Ok(instr)
  }

  fn conv_general_dilated_internal(
    &mut self,
    shape: &Shape,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    window: &Window,
    _window_strides: &Vec<i64>,
    _padding: &Vec<(i64, i64)>,
    _lhs_dilation: &Vec<i64>,
    _rhs_dilation: &Vec<i64>,
    dimension_numbers: &ConvolutionDimensionNumbers,
    feature_group_count: i64,
    batch_group_count: i64,
    precision_config: Option<PrecisionConfig>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_window(window.clone());
    instr.set_convolution_dimension_numberes(dimension_numbers.clone());
    instr.set_feature_group_count(feature_group_count);
    instr.set_batch_group_count(batch_group_count);

    if precision_config.is_some() {
      instr.set_precision_config(precision_config.unwrap());
    }
    self.add_instruction(
      &mut instr,
      HloOpcode::Convolution,
      &vec![lhs.clone(), rhs.clone()])
  }

  fn fft(
    &mut self,
    operand: &BlitzOp,
    fft_t: &FftType,
    fft_length: &Vec<i64>) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_fft_shape(&operand_shape, fft_t, fft_length);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result =
      self.fft_internal(&shape, operand, fft_t, fft_length);
    check_error(&result);
    result.unwrap()
  }

  fn fft_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    fft_t: &FftType,
    fft_length: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_fft_type(fft_t.clone());
    for i in fft_length {
      instr.add_fft_length(*i);
    }
    self.add_instruction(
      &mut instr, HloOpcode::Fft, &vec![operand.clone()])
  }

  fn triangular_solve_internal(
    &mut self,
    shape: &Shape,
    a: &BlitzOp,
    b: &BlitzOp,
    options: &TriangularSolveOptions) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_triangular_solve_options(options.clone());
    instr.set_shape(shape.clone());
    self.add_instruction(
      &mut instr,
      HloOpcode::TriangularSolve,
      &vec![a.clone(), b.clone()])
  }

  fn cholesky_internal(
    &mut self,
    shape: &Shape,
    a: &BlitzOp,
    lower: bool) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    let options = instr.mutable_cholesky_options();
    options.set_lower(lower);
    instr.set_shape(shape.clone());
    self.add_instruction(&mut instr, HloOpcode::Cholsky, &vec![a.clone()])
  }

  fn infeed(&self, _shape: &Shape, _config: String) -> BlitzOp {
    unimplemented!()
  }

  fn infeed_with_token(
    &mut self,
    token: &BlitzOp,
    shape: &Shape,
    config: String) -> BlitzOp
  {
    if !LayoutUtil::has_layout(shape) {
      assert!(false, "Given shape to Infeed must have a layout.");
    }
    let infeed_instruction_shape = ShapeUtil::make_tuple_shape(
      vec![shape.clone(), ShapeUtil::make_token_shape()]);
    if shape.is_array() && self.sharding().is_some() &&
      self.sharding().as_ref().unwrap().type_() == OpShardingType::Other
    {
      assert!(false, "Tiled sharding is not yet supported for array-shaped infeeds.");
    }
    if self.sharding().is_some() &&
      self.sharding().as_ref().unwrap().type_() == OpShardingType::Replicated
    {
      assert!(false, "Replicated sharding is not yet supported to infeed.");
    }
    let result =
      self.infeed_with_token_internal(infeed_instruction_shape, token, config);
    check_error(&result);

    result.unwrap()
  }

  fn infeed_with_token_internal(
    &mut self,
    infeed_instruction_shape: Shape,
    token: &BlitzOp,
    config: String) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(infeed_instruction_shape);
    instr.set_infeed_config(config);
    self.add_instruction(
      &mut instr, HloOpcode::Infeed, &vec![token.clone()])
  }

  fn outfeed(&self, _operand: &BlitzOp, _shape_with_layout: &Shape, _outfeed_config: String) {
    unimplemented!()
  }

  fn outfeed_with_token(
    &mut self,
    operand: &BlitzOp,
    token: &BlitzOp,
    shape_with_layout: &Shape,
    outfeed_config: String) -> BlitzOp
  {
    if !LayoutUtil::has_layout(shape_with_layout) {
      assert!(false, "Given shape to Outfeed must have a layout.");
    }
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    let operand_shape = operand_shape_wrapper.unwrap();

    if !ShapeUtil::compatible(&operand_shape, shape_with_layout) {
      let mut err_msg = "Outfeed shape ".to_string();
      err_msg.push_str(&ShapeUtil::human_string_with_layout(shape_with_layout));
      err_msg.push_str(" musst be compatible with operand shape ");
      err_msg.push_str(&ShapeUtil::human_string_with_layout(&operand_shape));
    }
    let result = self.outfeed_with_token_internal(
      operand, token, shape_with_layout.clone(), outfeed_config);
    check_error(&result);

    result.unwrap()
  }

  fn outfeed_with_token_internal(
    &mut self,
    operand: &BlitzOp,
    token: &BlitzOp,
    shape_with_layout: Shape,
    outfeed_config: String) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(ShapeUtil::make_token_shape());
    instr.set_outfeed_shape(shape_with_layout);
    instr.set_outfeed_cofig(outfeed_config);
    self.add_instruction(
      &mut instr,
      HloOpcode::Outfeed,
      &vec![operand.clone(), token.clone()])
  }

  fn call(
    &mut self,
    computation: i64,
    operands: &Vec<BlitzOp>) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shapes_wrapper =
      self.get_operand_shapes(operands);
    check_error(&operand_shapes_wrapper);
    let operand_shapes = operand_shapes_wrapper.unwrap();

    let called_program_shape_wrapper =
      self.get_subcomputation_shape(computation);
    check_error(&called_program_shape_wrapper);
    let called_program_shape = called_program_shape_wrapper.unwrap();

    let shape_wrapper = ShapeInference::infer_call_shape(
      &operand_shapes, &called_program_shape);
    check_error(&shape_wrapper);
    let shape = shape_wrapper.unwrap();

    instr.set_shape(shape);
    self.add_called_computation(computation, &mut instr);
    let result =
      self.add_instruction(&mut instr, HloOpcode::Call, operands);
    check_error(&result);

    result.unwrap()
  }

  fn composite_call(
    &mut self,
    computation: i64,
    operands: &Vec<BlitzOp>,
    _name: &String,
    _attributes: Option<String>,
    _version: Option<i64>) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shapes_wrapper =
      self.get_operand_shapes(operands);
    check_error(&operand_shapes_wrapper);

    let operand_shapes = operand_shapes_wrapper.unwrap();
    let called_program_shape_wrapper =
      self.get_subcomputation_shape(computation);
    check_error(&called_program_shape_wrapper);

    let called_program_shape = called_program_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_call_shape(&operand_shapes, &called_program_shape);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);

    self.add_called_computation(computation, &mut instr);
    instr.set_is_composite(true);

    let instruction_wrapper =
      self.add_instruction(&mut instr, HloOpcode::Call, operands);
    check_error(&instruction_wrapper);

    let instruction = instruction_wrapper.unwrap();
/*
    let mut result = self.set_instruction_frontend_attribute(
      &instruction, &"composite.name".to_string(), name.clone());
    check_error(&result);

    let mut attrs = "{}".to_string();
    if attributes.is_some() {
      attrs = attributes.unwrap().clone();
    }
    result = self.set_instruction_frontend_attribute(
      &instruction, &"composite.attribute".to_string(), attrs);
    check_error(&result);

    let mut ver = "0".to_string();
    if version.is_some() {
      ver = version.unwrap().to_string();
    }
    result = self.set_instruction_frontend_attribute(
      &instruction, &"composite.version".to_string(), ver);
    check_error(&result);
*/
    instruction
  }

  fn custom_call<T>(
    &self,
    _call_target_name: &String,
    _operands: &Vec<BlitzOp>,
    _shape_with_layout: &Shape,
    _opaque: &String,
    _operand_shapes_with_layout: Option<Vec<Shape>>,
    _has_side_effect: bool,
    _output_operand_aliasing: &Vec<(usize, (i64, usize))>,
    _lieteral: Option<&Literal<T>>,
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

  fn optimization_barrier(&mut self, operand: BlitzOp) -> BlitzOp {
    let operand_shape_wrapper = self.get_shape(&operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let mut instr = HloInstruction::default();
    instr.set_shape(operand_shape);

    let result = self.add_instruction(
      &mut instr, HloOpcode::OptimizationBarrier, &vec![operand]);
    check_error(&result);

    result.unwrap()
  }

  fn reduce(
    &mut self,
    operands: &Vec<BlitzOp>,
    init_values: &Vec<BlitzOp>,
    computation: i64,
    dimensions_to_reduce: &Vec<i64>) -> BlitzOp
  {
    let called_program_shape_wrapper =
      self.get_subcomputation_shape(computation);
    check_error(&called_program_shape_wrapper);
    let called_program_shape = called_program_shape_wrapper.unwrap();

    let mut all_operands = vec![];
    for op in operands {
      all_operands.push(op.clone());
    }
    for val in init_values {
      all_operands.push(val.clone());
    }

    let operand_shapes_wrapper =
      self.get_operand_shapes(&all_operands);
    check_error(&operand_shapes_wrapper);

    let operand_shapes = operand_shapes_wrapper.unwrap();
    let shape_wrapper = ShapeInference::infer_reshape_shape(
      &operand_shapes, dimensions_to_reduce, &called_program_shape);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result =
      self.reduce_internal(shape, &all_operands, computation, dimensions_to_reduce);
    check_error(&result);

    result.unwrap()
  }

  fn reduce_internal(
    &mut self,
    shape: Shape,
    all_operands: &Vec<BlitzOp>,
    computation: i64,
    dimensions_to_reduce: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape);

    for dim in dimensions_to_reduce {
      instr.add_dimensions(*dim);
    }

    self.add_called_computation(computation, &mut instr);
    self.add_instruction(&mut instr, HloOpcode::Reduce, all_operands)
  }

  fn reduce_all(
    &mut self,
    operand: &BlitzOp,
    init_value: &BlitzOp,
    computation: i64) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let all_dimnos = vec![0; operand_shape.dimensions_vec().len()];

    self.reduce(
      &vec![operand.clone()],
      &vec![init_value.clone()],
      computation,
      &all_dimnos)
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

  fn cross_replica_sum(
    &self,
    operand: &BlitzOp,
    replica_groups: &Vec<ReplicaGroup>) -> BlitzOp
  {
    let shape_wrapper = self.get_shape(operand);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let mut element_shape = shape.clone();
    if shape.is_tuple() {
      if shape.tuple_shapes_size() == 0 {
        assert!(false, "0 element tuple cross_replica_sum is not supported");
      }
      element_shape = shape.tuple_shapes(0).clone();
    }

    let scalar_shape = ShapeUtil::make_shape(
      &element_shape.element_type(), vec![]);
    let mut b = self.create_sub_builder("sum".to_string());
    let _x = b.parameter(
      0,
      &scalar_shape,
      &"x".to_string(),
      &vec![]);
    /*
    let y = b.parameter(
        1,
        &scalar_shape,
        &"y".to_string(),
        &vec![]);
    */
    if scalar_shape.element_type() == PrimitiveType::Pred {
      //self.or(&x, &y, &vec![]);
    } else {
      //self.add(&x, &y, &vec![]);
    }
    let computation =
      b.build_sub_computation(None, false);
    check_error(&computation);
    
    let computation_id = computation.unwrap();
    self.all_reduce(operand, computation_id, replica_groups,
      None, None, None)
  }

  fn all_gether(
    &mut self,
    operand: &BlitzOp,
    all_gather_dimension: i64,
    shard_count: i64,
    replica_groups: &Vec<ReplicaGroup>,
    channel_id: Option<ChannelHandle>,
    layout: Option<Layout>,
    use_global_device_ids: Option<bool>) -> BlitzOp
  {
    self.all_gather_impl(operand, all_gather_dimension, shard_count,
      replica_groups, channel_id, layout, use_global_device_ids, false)
  }

  fn all_gather_impl(
    &mut self,
    operand: &BlitzOp,
    all_gather_dimension: i64,
    shard_count: i64,
    replica_groups: &Vec<ReplicaGroup>,
    channel_id: Option<ChannelHandle>,
    layout: Option<Layout>,
    use_global_device_ids: Option<bool>,
    async_: bool) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let mut operand_shapes = vec![];
    let mut operands = vec![];
    let operand_shape = operand_shape_wrapper.unwrap();
    if operand_shape.is_tuple() {
      if operand_shape.tuple_shapes_vec().len() == 0 {
        assert!(false, "0 element tuple all_gather is not supported");
      }
      for i in 0..operand_shape.tuple_shapes_vec().len() {
        operand_shapes.push(operand_shape.tuple_shapes(i).clone());
        //operands.push(self.get_tuple_element(operand, i as i64));
      }
    } else {
      operand_shapes.push(operand_shape);
      operands.push(operand.clone());
    }

    let inferred_shape_wrapper =
      ShapeInference::infer_all_gather_shape(
        &operand_shapes, all_gather_dimension, shard_count);

    let mut inferred_shape = inferred_shape_wrapper.unwrap();
    if layout.is_some() {
      inferred_shape.set_layout(layout.unwrap());
      instr.set_constrain_layout(true);
    }
    instr.set_shape(inferred_shape);
    instr.add_dimensions(all_gather_dimension);
    for g in replica_groups {
      instr.add_replica_groups(g.clone());
    }
    if channel_id.is_some() {
      instr.set_channel_id(channel_id.unwrap().handle());
    }
    if use_global_device_ids.is_some() {
      instr.set_use_global_device_ids(use_global_device_ids.unwrap());
    }

    let mut opcode = HloOpcode::AllGather;
    if async_ { opcode = HloOpcode::AllGatherStart; }
    let all_gather =
      self.add_instruction(&mut instr, opcode, &operands);
    check_error(&all_gather);
    all_gather.unwrap()
  }

  fn all_reduce(
    &self,
    _operand: &BlitzOp,
    _computation: i64,
    _replica_groups: &Vec<ReplicaGroup>,
    _channel_id: Option<ChannelHandle>,
    _shape_with_layout: Option<Shape>,
    _use_global_device_ids: Option<bool>) -> BlitzOp
  {
    unimplemented!()    
  }

  fn all_reduce_impl(
    &mut self,
    operand: &BlitzOp,
    computation: i64,
    replica_groups: &Vec<ReplicaGroup>,
    channel_id: Option<ChannelHandle>,
    layout: Option<Shape>,
    use_global_device_ids: Option<bool>,
    async_: bool) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let mut operand_shapes = vec![];
    let mut operands = vec![];

    if operand_shape.is_tuple() {
      if operand_shape.tuple_shapes_vec().len() == 0 {
        assert!(false, "0 element tuple all_reduce is not supported")
      }
      for i in 0..operand_shape.tuple_shapes_vec().len() {
        if operand_shape.tuple_shapes(i).element_type() !=
          operand_shape.tuple_shapes(0).element_type()
        {
          let err_msg = "All the shapes of a tuple input of all_reduce
            must have the same element type".to_string();
          assert!(false, "{:?}",err_msg);
        }
        operand_shapes.push(operand_shape.tuple_shapes(i).clone());
        //operands.push(self.get_tuple_element(operand, i as i64));
      }
    } else {
      operand_shapes.push(operand_shape.clone());
      operands.push(operand.clone());
    }

    let inferred_shape_wrapper =
      ShapeInference::infer_all_reduce_shape(&operand_shapes);
    check_error(&inferred_shape_wrapper);
    
    let inferred_shape = inferred_shape_wrapper.unwrap();
    if layout.is_some() {
      if !LayoutUtil::has_layout(layout.as_ref().unwrap()) {
        let mut err_msg = "shape_with_layout must have the layout set".to_string();
        err_msg.push_str(&ShapeUtil::human_string(layout.as_ref().unwrap()).to_string());
        assert!(false, "{:?}", err_msg);
      }
      if !ShapeUtil::compatible(layout.as_ref().unwrap(), &operand_shape) {
        let mut err_msg = "Provided shape_with_layout must be compatible
          with the operand shape: ".to_string();
        err_msg.push_str(&ShapeUtil::human_string(layout.as_ref().unwrap()));
        err_msg.push_str(&ShapeUtil::human_string(&operand_shape));
        assert!(false, "{:?}", err_msg);
      }
      instr.set_constrain_layout(true);
      if operand_shape.is_tuple() && !inferred_shape.is_tuple() {
        assert!(layout.as_ref().unwrap().tuple_shapes_vec().len() == 1);
        instr.set_shape(layout.as_ref().unwrap().tuple_shapes(0).clone());
      } else {
        instr.set_shape(layout.as_ref().unwrap().clone());
      }
    } else {
      instr.set_shape(inferred_shape.clone());
    }

    for group in replica_groups {
      instr.add_replica_groups(group.clone());
    }
    if channel_id.is_some() {
      instr.set_channel_id(channel_id.unwrap().handle());
    }
    if use_global_device_ids.is_some() {
      instr.set_use_global_device_ids(use_global_device_ids.unwrap());
    }

    self.add_called_computation(computation, &mut instr);
    let mut opcode = HloOpcode::AllReduce;
    if async_ { opcode = HloOpcode::AllReduceStart; }
    let all_reduce_wrapper =
      self.add_instruction(&mut instr, opcode, &operands);
    check_error(&all_reduce_wrapper);

    let all_reduce = all_reduce_wrapper.unwrap();
    if operand_shape.is_tuple() && !inferred_shape.is_tuple() {
      assert!(operand_shapes.len() == 1);
      assert!(ShapeUtil::compatible(&operand_shapes[0], &inferred_shape));
      //return self.tuple(&vec![all_reduce]);
    }
    all_reduce
  }

  fn reduce_scatter(
    &mut self,
    operand: &BlitzOp,
    computation: i64,
    scatter_dimension: i64,
    shard_count: i64,
    replica_groups: &Vec<ReplicaGroup>,
    channel_id: Option<ChannelHandle>,
    layout: Option<Layout>,
    use_global_device_ids: Option<bool>) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let mut operand_shapes = vec![];
    let mut operands = vec![];
    if operand_shape.is_tuple() {
      if operand_shape.tuple_shapes_vec().len() == 0 {
        let err_msg = "0 element tuple reduce_scatter is not supported".to_string();
        assert!(false, "{:?}", err_msg);
      }
      for i in 0..operand_shape.tuple_shapes_vec().len() {
        if operand_shape.tuple_shapes(i).element_type() !=
          operand_shape.tuple_shapes(0).element_type()
        {
          let err_msg = "All the shapes of a tuple input of reduce_scatter
            must have the same element type".to_string();
          assert!(false, "{:?}", err_msg);
        }
        operand_shapes.push(operand_shape.tuple_shapes(i).clone());
        //operands.push(self.get_tuple_element(operand, i as i64));
      }
    } else {
      operand_shapes.push(operand_shape.clone());
      operands.push(operand.clone());
    }

    let inferred_shape_wrapper =
      ShapeInference::infer_reduce_scatter_shape(
        &operand_shapes, scatter_dimension, shard_count);
    check_error(&inferred_shape_wrapper);

    let mut inferred_shape = inferred_shape_wrapper.unwrap();
    if layout.is_some() {
      inferred_shape.set_layout(layout.unwrap());
      instr.set_constrain_layout(true);
    }

    instr.set_shape(inferred_shape);
    self.add_called_computation(computation, &mut instr);
    instr.add_dimensions(scatter_dimension);
    for group in replica_groups {
      instr.add_replica_groups(group.clone());
    }
    if channel_id.is_some() {
      instr.set_channel_id(channel_id.unwrap().handle());
    }
    if use_global_device_ids.is_some() {
      instr.set_use_global_device_ids(use_global_device_ids.unwrap());
    }

    let reduce_scatter_wrapper =
      self.add_instruction(&mut instr, HloOpcode::ReduceScatter, &operands);
    check_error(&reduce_scatter_wrapper);
    reduce_scatter_wrapper.unwrap()
  }

  fn all_to_all() {
      
  }

  fn all_to_all_tuple() {
      
  }

  fn all_to_all_array() {
      
  }
  fn collective_broadcast(
    &mut self,
    operand: &BlitzOp,
    replica_groups: &Vec<ReplicaGroup>,
    channel_id: Option<ChannelHandle>) -> BlitzOp
  {
    self.collective_broadcast_impl(operand, replica_groups, channel_id)    
  }

  fn collective_broadcast_impl(
    &mut self,
    operand: &BlitzOp,
    replica_groups: &Vec<ReplicaGroup>,
    channel_id: Option<ChannelHandle>) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_collective_broadcast_shape(&vec![operand_shape]);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);
    for group in replica_groups {
      instr.add_replica_groups(group.clone());
    }
    if channel_id.is_some() {
      instr.set_channel_id(channel_id.unwrap().handle());
    }
    let result = self.add_instruction(
      &mut instr,
      HloOpcode::CollectiveBroadcast,
      &vec![operand.clone()]);
    check_error(&result);
    result.unwrap()
  }

  fn collective_permute(
    &mut self,
    operand: &BlitzOp,
    source_target_pairs: &Vec<(i64, i64)>,
    channel_id: Option<ChannelHandle>,
    async_: bool,
    inplace: bool) -> BlitzOp
  {
    self.collective_permute_impl(
      operand, source_target_pairs, channel_id, async_, inplace)
  }

  fn collective_permute_impl(
    &mut self,
    operand: &BlitzOp,
    source_target_pairs: &Vec<(i64, i64)>,
    channel_id: Option<ChannelHandle>,
    async_: bool,
    inplace: bool) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_collective_permute_shape(
        &vec![operand_shape], inplace);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);

    for pair in source_target_pairs {
      instr.add_source_target_pairs(pair.clone());
    }
    if channel_id.is_some() {
      instr.set_channel_id(channel_id.unwrap().handle());
    }

    let mut opcode = HloOpcode::CollectivePermute;
    if async_ { opcode = HloOpcode::CollectivePermuteStart; }
    let result =
      self.add_instruction(&mut instr, opcode, &vec![operand.clone()]);
    check_error(&result);
    result.unwrap()
  }

  fn replica_id(&mut self) -> BlitzOp {
    let mut instr = HloInstruction::default();
    instr.set_shape(ShapeUtil::make_shape(&PrimitiveType::F32, vec![]));
    let result =
      self.add_instruction(&mut instr, HloOpcode::ReplicaId, &vec![]);
    check_error(&result);
    result.unwrap()
  }

  fn select_and_scatter(
    &mut self,
    operand: &BlitzOp,
    select: i64,
    window_dimensions: &Vec<i64>,
    window_strides: &Vec<i64>,
    padding: &Padding,
    source: &BlitzOp,
    init_value: &BlitzOp,
    scatter: i64) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let padding_values = make_padding(
      operand_shape.dimensions_vec(),
      window_dimensions,
      window_strides,
      padding);

    let window_wrapper =
      ShapeInference::infer_window_from_dimensions(window_dimensions, window_strides,
        &padding_values, &vec![], &vec![], None);
    check_error(&window_wrapper);

    let mut padding_t = PaddingType::Invalid;
    for i in 0..operand_shape.dimensions_vec().len() {
      if operand_shape.is_dynamic_dimension(i as i64) && *padding == Padding::Same { // TODO: window_util
        padding_t = PaddingType::Same;
      }
    }
    if padding_t == PaddingType::Same {
      let instr_wrapper =
        self.select_and_scatter_internal(
          operand, select, window_dimensions, window_strides, &padding_values,
          source, init_value, scatter);
      check_error(&instr_wrapper);

      let mut instr = instr_wrapper.unwrap();
      instr.set_custom_call_target("dynamic_select_and_scatter_same_padding".to_string());
      let result =
        self.add_instruction(&mut instr, HloOpcode::CustomCall,
        &vec![operand.clone(), source.clone(), init_value.clone()]);
      check_error(&result);
      return result.unwrap();
    }
    
    self.select_and_scatter_with_general_padding(
      operand, select, window_dimensions, window_strides, &padding_values,
      source, init_value, scatter)
  }

  fn select_and_scatter_internal(
    &self,
    operand: &BlitzOp,
    select: i64,
    window_dimensions: &Vec<i64>,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    source: &BlitzOp,
    init_value: &BlitzOp,
    scatter: i64) -> Result<HloInstruction, String>
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    let source_shape_wrapper = self.get_shape(source);
    check_error(&source_shape_wrapper);
    let init_shape_wrapper = self.get_shape(init_value);
    check_error(&init_shape_wrapper);
    let select_shape_wrapper =
      self.get_subcomputation_shape(select); 
    check_error(&select_shape_wrapper);
    let scatter_shape_wrapper =
      self.get_subcomputation_shape(scatter);
    check_error(&scatter_shape_wrapper);

    let window_wrapper =
      ShapeInference::infer_window_from_dimensions(window_dimensions, window_strides,
        padding, &vec![], &vec![], None);
    check_error(&window_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let source_shape = source_shape_wrapper.unwrap();
    let init_shape = init_shape_wrapper.unwrap();
    let select_shape = select_shape_wrapper.unwrap();
    let scatter_shape = scatter_shape_wrapper.unwrap();
    let window = window_wrapper.unwrap();

    let shape_wrapper =
      ShapeInference::infer_select_and_scatter_shape(&operand_shape, &select_shape, &window,
        &source_shape, &init_shape, &scatter_shape);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);

    self.add_called_computation(select, &mut instr);
    self.add_called_computation(scatter, &mut instr);
    Ok(instr)
  }

  fn select_and_scatter_with_general_padding(
    &mut self,
    operand: &BlitzOp,
    select: i64,
    window_dimensions: &Vec<i64>,
    window_strides: &Vec<i64>,
    padding: &Vec<(i64, i64)>,
    source: &BlitzOp,
    init_value: &BlitzOp,
    scatter: i64) -> BlitzOp
  {
    let instr_wrapper =
      self.select_and_scatter_internal(
        operand, select, window_dimensions, window_strides, padding,
        source, init_value, scatter);
    check_error(&instr_wrapper);

    let mut instr = instr_wrapper.unwrap();
    let result = self.add_instruction(
      &mut instr,
      HloOpcode::SelectAndScatter,
      &vec![operand.clone(), source.clone(), init_value.clone()]);
    check_error(&result);
    result.unwrap()
  }

  fn iota(&mut self, shape: &Shape, iota_dimension: i64) -> BlitzOp {
    if !shape.is_static() {
      assert!(false, "The output of iota must not have dynamic dimensions: {:?}", shape);
    }
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.add_dimensions(iota_dimension);
    //let result =
      //self.add_instruction(&instr, HloOpcode::Iota, &vec![]);
    //if result.is_err() { assert!(false); }
    //result.unwrap()
    unimplemented!()
  }

  fn convert_element_type(
    &mut self,
    operand: &mut BlitzOp,
    new_element_t: &PrimitiveType) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);    
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_convert_shape(&operand_shape, new_element_t);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    if is_complex_type(&operand_shape.element_type()) &&
      !is_complex_type(new_element_t)
    {
      //operand = real(operand).clone();
    }
    let result = self.add_op_with_shape(
      HloOpcode::Convert, &shape, &vec![operand.clone()]);
    check_error(&result);
    result.unwrap()
  }

  fn bitcast_convert_type(
    &mut self,
    operand: BlitzOp,
    new_element_t: &PrimitiveType) -> BlitzOp
  {
    let operand_shape = self.get_shape(&operand);
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
    &mut self,
    shape: &Shape,
    operand: BlitzOp) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    self.add_instruction(&mut instr, HloOpcode::BitcastConvert, &vec![operand])
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
      &vec![operand.clone(), random.clone()]);
    if result.is_err() { assert!(false); }

    result.unwrap()
  }

  fn transpose(
    &mut self,
    operand: BlitzOp,
    permutation: &Vec<i64>) -> BlitzOp
  {
    let operand_shape = self.get_shape(&operand);
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
    operand: BlitzOp,
    permutation: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    for dim in permutation {
      instr.add_dimensions(*dim);
    }
    self.add_instruction(&mut instr, HloOpcode::Transpose, &vec![operand])
  }

  fn rev(
    &mut self,
    operand: BlitzOp,
    dimensions: &Vec<i64>) -> BlitzOp
  {
    let operand_shape = self.get_shape(&operand);
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
    operand: BlitzOp,
    dimensions: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    for dim in dimensions {
      instr.add_dimensions(*dim);
    }
    self.add_instruction(&mut instr, HloOpcode::Reverse, &vec![operand])
  }

  fn sort(
    &mut self,
    operands: &Vec<BlitzOp>,
    comparator: i64,
    dimension: i64,
    is_stable: bool) -> BlitzOp
  {
    let operand_shapes_wrapper =
      self.get_operand_shapes(operands);
    check_error(&operand_shapes_wrapper);

    let operand_shapes = operand_shapes_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_variadic_op_shape_by_opshapes(
        HloOpcode::Sort, &operand_shapes);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result =
      self.sort_internal(shape, operands, comparator, dimension, is_stable);
    check_error(&result);
    result.unwrap()
  }

  fn sort_internal(
    &mut self,
    shape: Shape,
    operands: &Vec<BlitzOp>, 
    comparator: i64,
    mut dimension: i64,
    is_stable: bool) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();    
    instr.set_shape(shape);
    instr.set_is_stable(is_stable);
    if dimension == -1 {
      let keys_shape_wrapper = self.get_shape(&operands[0]);
      check_error(&keys_shape_wrapper);
      let keys_shape = keys_shape_wrapper.unwrap();
      dimension = (keys_shape.dimensions_vec().len() - 1) as i64;
    }
    instr.add_dimensions(dimension);
    self.add_called_computation(comparator, &mut instr);
    let result =
      self.add_instruction(&mut instr, HloOpcode::Sort, operands);
    check_error(&result);
    result
  }

  fn top_k(&mut self, operand: &BlitzOp, k: i64, largest: bool) -> BlitzOp {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_topk_shape(&operand_shape, k);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result =
      self.top_k_internal(shape, operand.clone(), k, largest);
    check_error(&result);
    result.unwrap()
  }

  fn top_k_internal(
    &mut self,
    shape: Shape,
    operand: BlitzOp,
    k: i64,
    largest: bool) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape);
    instr.set_k(k);
    instr.set_largest(largest);
    self.add_instruction(&mut instr, HloOpcode::TopK, &vec![operand])
  }

  // Enqueues a clamp instruction onto the computation.
  fn clamp(
    &mut self,
    min: &BlitzOp,
    operand: &BlitzOp,
    max: &BlitzOp) -> BlitzOp
  {
    self.ternary_op(HloOpcode::Clamp, min, operand, max)
  }

  fn map() {
      
  }

  fn rng_normal(
    &mut self,
    mu: BlitzOp,
    sigma: BlitzOp,
    shape: &Shape) -> BlitzOp
  {
    self.rng_op(RandomDistribution::Normal,
      &vec![mu, sigma], shape)
  }

  fn rng_uniform(
    &mut self,
    a: BlitzOp,
    b: BlitzOp,
    shape: &Shape) -> BlitzOp
  {
    self.rng_op(RandomDistribution::Uniform,
      &vec![a, b], shape)
  }

  fn rng_bit_generator(
    &mut self,
    algorithm: RandomAlgorithm,
    initial_state: &BlitzOp,
    shape: &Shape) -> BlitzOp
  {
    let result =
      ShapeUtil::validate_shape_with_optional_layout(shape);
    check_error(&result);

    let state_shape_wrapper = self.get_shape(initial_state);
    check_error(&state_shape_wrapper);

    let state_shape = state_shape_wrapper.unwrap();
    let mut output_shape = Shape::new();
    if shape.is_array() {
      // Make output_shape the same as the input shape, but with an unsigned
      // integral type.
      output_shape = shape.clone();
      output_shape.set_element_type(unsigned_integral_type_for_bit_width(
        bit_width(&shape.element_type())));
    }
    if !is_unsigned_integral_type(&output_shape.element_type()) {
      let mut err_msg = "Unsupported shape for rng_bit_generator: ".to_string();
      err_msg.push_str(&primitive_type_name(&shape.element_type()));
      assert!(false, "{:?}", err_msg);
    }

    let tuple_shape =
      ShapeUtil::make_tuple_shape(vec![state_shape, output_shape]);
    let result =
      self.rng_bit_generator_internal(
        tuple_shape, algorithm, initial_state.clone());
    check_error(&result);
    result.unwrap()
  }

  fn rng_bit_generator_internal(
    &mut self,
    full_result_shape: Shape,
    algorithm: RandomAlgorithm,
    initial_state: BlitzOp) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(full_result_shape);
    instr.set_rng_algorithm(algorithm);
    self.add_instruction(
      &mut instr, HloOpcode::RngBitGenerator, &vec![initial_state])
  }

  fn while_(
    &mut self,
    _condition: &BlitzComputation,
    _body: &BlitzComputation,
    _init: BlitzOp) -> BlitzOp
  {
    /*
    let body_program_shape =  body.get_program_shape();
    if body_program_shape.is_err() { assert!(false); }
    let condition_program_shape = condition.get_program_shape();
    if condition_program_shape.is_err() { assert!(false); }
    let init_shape = self.get_shape(&init);
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
    */
    unimplemented!()
  }

  fn while_internal(
    &mut self,
    shape: &Shape,
    condition: i64,
    body: i64,
    init: BlitzOp) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());

    // Body comes before condition computation in the vector.
    self.add_called_computation(body, &mut instr);
    self.add_called_computation(condition, &mut instr);
    self.add_instruction(&mut instr, HloOpcode::While, &vec![init])
  }

  fn conditional(
    &mut self,
    predicate: &BlitzOp,
    true_operand: &BlitzOp,
    true_computation: i64,
    false_operand: &BlitzOp,
    false_computation: i64) -> BlitzOp
  {
    let shape_wrapper = self.get_shape(predicate);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    if !ShapeUtil::is_scalar(&shape) || shape.element_type() != PrimitiveType::Pred {
      let mut err_msg = "Argument to predicted-conditional is not a scalar
        of Pred type (".to_string();
      err_msg.push_str(&ShapeUtil::human_string(&shape));
      err_msg.push_str(")");
      assert!(false, "{:?}", err_msg);
    }
    // The index of true_computation must be 0 and that of false computation
    // must be 1.
    self.conditional_impl(predicate, 
      &vec![true_computation, false_computation],
      &vec![true_operand.clone(), false_operand.clone()])
  }

  fn conditional_many(
    &mut self,
    branch_index: &BlitzOp,
    branch_computations: &Vec<i64>,
    branch_operands: &Vec<BlitzOp>) -> BlitzOp
  {
    let shape_wrapper = self.get_shape(branch_index);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    if !ShapeUtil::is_scalar(&shape) || shape.element_type() != PrimitiveType::S32 {
      let mut err_msg = "Argument to indexed-conditional is not a scalar
        of S32 type (".to_string();
      err_msg.push_str(&ShapeUtil::human_string(&shape));
      err_msg.push_str(")");
      assert!(false, "{:?}", err_msg);
    }
    self.conditional_impl(branch_index, branch_computations, branch_operands)
  }

  fn conditional_impl(
    &mut self,
    branch_index: &BlitzOp,
    branch_computations: &Vec<i64>,
    branch_operands: &Vec<BlitzOp>) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let branch_index_shape_wrapper =
      self.get_shape(branch_index);
    check_error(&branch_index_shape_wrapper);
    let branch_index_shape = branch_index_shape_wrapper.unwrap();

    let mut branch_operand_shapes = vec![];
    let mut branch_computation_shapes = vec![];
    for j in 0..branch_operands.len() {
      let branch_operand_shapes_wrapper =
        self.get_shape(&branch_operands[j]);
      check_error(&branch_operand_shapes_wrapper);
      branch_operand_shapes.push(branch_operand_shapes_wrapper.unwrap());

      let branch_computation_shapes_wrapper =
        self.get_subcomputation_shape(branch_computations[j]);
      check_error(&branch_computation_shapes_wrapper);
      branch_computation_shapes.push(branch_computation_shapes_wrapper.unwrap());
    }

    let shape_wrapper = ShapeInference::infer_conditional_shape(
      &branch_index_shape,
      &branch_computation_shapes,
      &branch_operand_shapes);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);
    for b_comp in branch_computations {
      self.add_called_computation(*b_comp, &mut instr);
    }

    let mut operands = vec![];
    for b_op in branch_operands {
      operands.push(b_op.clone());
    }

    let result =
      self.add_instruction(&mut instr, HloOpcode::Conditional, &operands);
    check_error(&result);
    result.unwrap()
  }

  fn reduce_precision(
    &mut self,
    operand: BlitzOp,
    exponent_bits: i64,
    mantissa_bits: i64) -> BlitzOp
  {
    let operand_shape = self.get_shape(&operand);
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
    operand: BlitzOp,
    exponent_bits: i64,
    mantissa_bits: i64) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_exponent_bits(exponent_bits);
    instr.set_mantissa_bits(mantissa_bits);
    self.add_instruction(
      &mut instr, HloOpcode::ReducePrecision, &vec![operand])
  }

  fn gather(
    &mut self,
    input: BlitzOp,
    start_indices: BlitzOp,
    dimension_numbers: &GatherDimensionNumbers,
    slice_sizes: &Vec<i64>,
    indices_are_sorted: bool) -> BlitzOp
  {
    let input_shape = self.get_shape(&input);
    if input_shape.is_err() { assert!(false); }
    let start_indices_shape = self.get_shape(&start_indices);
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
    input: BlitzOp,
    start_indices: BlitzOp,
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
      &mut instr, HloOpcode::Gather, &vec![input, start_indices])
  }

  fn scatter(
    &mut self,
    input: &BlitzOp,
    scatter_indices: &BlitzOp,
    updates: &BlitzOp,
    update_computation: i64,
    dimension_numbers: &ScatterDimensionNummbers,
    indices_are_sorted: bool,
    unique_indices: bool) -> BlitzOp
  {
    self.scatter_many(
      &vec![input.clone()],
      scatter_indices,
      &vec![updates.clone()],
      update_computation,
      dimension_numbers,
      indices_are_sorted,
      unique_indices)
  }

  fn scatter_many(
    &mut self,
    inputs: &Vec<BlitzOp>,
    scatter_indices: &BlitzOp,
    updates: &Vec<BlitzOp>,
    update_computation: i64,
    dimension_numbers: &ScatterDimensionNummbers,
    indices_are_sorted: bool,
    unique_indices: bool) -> BlitzOp
  {
    if inputs.is_empty() {
      let err_msg = "Scatter inputs cannot be empty";
      assert!(false, "{:?}", err_msg);
    }
    if inputs.len() != updates.len() {
      let mut err_msg =
        "Scatter should have same number of inputs and updates: ".to_string();
      err_msg.push_str(&inputs.len().to_string());
      err_msg.push_str(" vs ");
      err_msg.push_str(&updates.len().to_string());
      assert!(false, "{:?}", err_msg);
    }

    let mut operand_shapes = vec![];
    for input in inputs {
      let input_shape_wrapper = self.get_shape(input);
      check_error(&input_shape_wrapper);
      let input_shape = input_shape_wrapper.unwrap();
      operand_shapes.push(input_shape);
    }

    let scatter_indices_shape_wrapper =
      self.get_shape(scatter_indices);
    check_error(&scatter_indices_shape_wrapper);
    let scatter_indices_shape = scatter_indices_shape_wrapper.unwrap();
    operand_shapes.push(scatter_indices_shape);

    for update in updates {
      let update_shape_wrapper = self.get_shape(update);
      check_error(&update_shape_wrapper);
      let update_shape = update_shape_wrapper.unwrap();
      operand_shapes.push(update_shape);
    }

    let to_apply_shape_wrapper =
      self.get_subcomputation_shape(update_computation);
    check_error(&to_apply_shape_wrapper);

    let to_apply_shape = to_apply_shape_wrapper.unwrap();
    let shape_wrapper = ShapeInference::infer_scatter_shape(
      &operand_shapes, &to_apply_shape, dimension_numbers);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result = self.scatter_internal(
      &shape, inputs, scatter_indices, updates, update_computation,
      dimension_numbers, indices_are_sorted, unique_indices);
    check_error(&result);

    result.unwrap()
  }

  fn scatter_internal(
    &mut self,
    shape: &Shape,
    inputs: &Vec<BlitzOp>,
    scatter_indices: &BlitzOp,
    updates: &Vec<BlitzOp>,
    update_computation: i64,
    dimension_numbers: &ScatterDimensionNummbers,
    indices_are_sorted: bool,
    unique_indices: bool) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_indices_are_sorted(indices_are_sorted);
    instr.set_unique_indices(unique_indices);
    instr.set_shape(shape.clone());
    instr.set_scatter_dimension_numbers(dimension_numbers.clone());

    self.add_called_computation(update_computation, &mut instr);
    let mut operands = vec![];
    for input in inputs {
      operands.push(input.clone());
    }
    operands.push(scatter_indices.clone());
    for update in updates {
      operands.push(update.clone());
    }
    let result =
      self.add_instruction(&mut instr, HloOpcode::Scatter, &operands);
    check_error(&result);
    result
  }

  fn send(&self, _operand: &BlitzOp, _handle: &ChannelHandle) {
    unimplemented!()
  }

  fn send_with_token(
    &mut self,
    operand: &BlitzOp,
    token: &BlitzOp,
    handle: &ChannelHandle) -> BlitzOp
  {
    if handle.type_() != ChannelType::DeviceToDevice {
      assert!(false, "send must use a device-to-device channel");
    }
    let _send_op = build_send(
      self, operand, token, handle, false);
    //build_send_done(self, &send_op, handle, false)

    unimplemented!()
  }

  fn send_to_host(
    &mut self,
    operand: &BlitzOp,
    token: &BlitzOp,
    shape_with_layout: &Shape,
    handle: &ChannelHandle) -> BlitzOp
  {
    if !LayoutUtil::has_layout(shape_with_layout) {
      let err_msg =
        "shape passed to send_to_host must have a layout".to_string();
      assert!(false, "{:?}", err_msg);
    }
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    if !ShapeUtil::compatible(&operand_shape, shape_with_layout) {
      let mut err_msg = "send_to_host shape ".to_string();
      err_msg.push_str(&ShapeUtil::human_string_with_layout(shape_with_layout));
      err_msg.push_str(" must be compatible with operand shape");
      err_msg.push_str(&ShapeUtil::human_string_with_layout(&operand_shape));
      assert!(false, "{:?}", err_msg);
    }
    if !operand_shape.is_array() {
      let mut err_msg =
        "send_to_host only supports array shapes, shape: ".to_string();
      err_msg.push_str(&ShapeUtil::human_string(&operand_shape));
      assert!(false, "{:?}", err_msg);
    }
    if handle.type_() != ChannelType::DeviceToHost {
      let err_msg = "send_to_host must use a device-to-host channel".to_string();
      assert!(false, "{:?}", err_msg);
    }

    // Send instruction produces a tuple of {aliased operand, U32 context,
    // token}.
    let mut send_instr = HloInstruction::default();
    let tuple_shape = ShapeUtil::make_tuple_shape(vec![
      shape_with_layout.clone(),
      ShapeUtil::make_shape(&PrimitiveType::U32, vec![]),
      ShapeUtil::make_token_shape()
    ]);
    send_instr.set_shape(tuple_shape);
    send_instr.set_channel_id(handle.handle());
    send_instr.set_is_host_transfer(true);
    let send_wrapper = self.add_instruction(
      &mut send_instr,
      HloOpcode::Send,
      &vec![operand.clone(), token.clone()]);
    check_error(&send_wrapper);

    let _send = send_wrapper.unwrap();
    let mut send_done_instr = HloInstruction::default();
    send_done_instr.set_shape(ShapeUtil::make_token_shape());
    send_done_instr.set_channel_id(handle.handle());
    send_done_instr.set_is_host_transfer(true);
    //let send_done_wrapper = self.add_instruction(
      //&mut send_done_instr, HloOpcode::SendDone, &vec![send]);
    //check_error(&send_done_wrapper);

    //send_done_wrapper.unwrap()
    unimplemented!()
  }

  fn recv_from_host(
    &mut self,
    token: &BlitzOp,
    shape: &Shape,
    handle: &ChannelHandle) -> BlitzOp
  {
    if !LayoutUtil::has_layout(shape) {
      let err_msg =
        "shape passed to recv_from_host must have a lauout".to_string();
      assert!(false, "{:?}", err_msg);
    }
    if !shape.is_array() {
      let mut err_msg =
        "recv_from_host only supports array shapes, shape: ".to_string();
      err_msg.push_str(&ShapeUtil::human_string(shape));
      assert!(false, "{:?}", err_msg);
    }
    if handle.type_() != ChannelType::HostToDevice {
      let err_msg =
        "recv_from_host must use a host-to-device channel".to_string();
      assert!(false, "{:?}", err_msg);
    }

    let mut recv_instr = HloInstruction::default();
    let tuple_shape = ShapeUtil::make_tuple_shape(vec![
      shape.clone(),
      ShapeUtil::make_shape(&PrimitiveType::U32, vec![]),
      ShapeUtil::make_token_shape()
    ]);
    recv_instr.set_shape(tuple_shape);
    recv_instr.set_channel_id(handle.handle());
    recv_instr.set_is_host_transfer(true);
    let recv_wrapper = self.add_instruction(
      &mut recv_instr, HloOpcode::Recv, &vec![token.clone()]);
    check_error(&recv_wrapper);

    let _recv = recv_wrapper.unwrap();
    let mut recv_done_instr = HloInstruction::default();
    let tuple_shape = ShapeUtil::make_tuple_shape(vec![
      shape.clone(),
      ShapeUtil::make_token_shape()
    ]);
    recv_done_instr.set_shape(tuple_shape);
    recv_done_instr.set_channel_id(handle.handle());
    recv_done_instr.set_is_host_transfer(true);
    //let result = self.add_instruction(
      //&mut recv_done_instr, HloOpcode::RecvDone, &vec![recv]);
    //check_error(&result);

    //result.unwrap()
    unimplemented!()
  }

  fn create_token(&mut self) -> BlitzOp {
    let mut instr = HloInstruction::default();
    instr.set_shape(ShapeUtil::make_token_shape());
    let result = self.add_instruction(
      &mut instr, HloOpcode::AfterAll, &vec![]);
    check_error(&result);
    result.unwrap()
  }

  fn after_all(
    &mut self,
    tokens: &Vec<BlitzOp>) -> BlitzOp
  {
    for i in 0..tokens.len() {
      let operand = &tokens[i];
      let operand_shape_wrapper = self.get_shape(operand);
      check_error(&operand_shape_wrapper);
      let operand_shape = operand_shape_wrapper.unwrap();
      if !operand_shape.is_token() {
        let mut err_msg =
          "all operands to after_all must be tokens; operand ".to_string();
        err_msg.push_str(&i.to_string());
        err_msg.push_str(" has shape ");
        err_msg.push_str(&ShapeUtil::human_string(&operand_shape));
        assert!(false, "{:?}", err_msg);
      }
    }
    let mut instr = HloInstruction::default();
    instr.set_shape(ShapeUtil::make_token_shape());
    let result = self.add_instruction(
      &mut instr, HloOpcode::AfterAll, tokens);
    check_error(&result);
    result.unwrap()
  }

  fn recv(
    &mut self,
    shape: &Shape,
    _handle: &ChannelHandle) -> BlitzOp
  {
    let mut token_instr = HloInstruction::default();
    token_instr.set_shape(ShapeUtil::make_token_shape());
    let token_wrapper = self.add_instruction(
      &mut token_instr, HloOpcode::AfterAll, &vec![]);
    check_error(&token_wrapper);

    let _token = token_wrapper.unwrap();
    //let recv = self.recv_with_token(&token, shape, handle);

    let mut recv_data = HloInstruction::default();
    recv_data.set_shape(shape.clone());
    recv_data.set_tuple_index(0);
    //let result = self.add_instruction(
      //&mut recv_data, HloOpcode::GetTupleElement, &vec![recv]);
    //check_error(&result);
    //result.unwrap()
    unimplemented!()
  }

  fn recv_with_token(
    &mut self,
    token: &BlitzOp,
    shape: &Shape,
    handle: &ChannelHandle) -> BlitzOp
  {
    if handle.type_() != ChannelType::DeviceToDevice {
      assert!(false, "recv must use a device-to-device channel");
    }
    let _recv_op = build_recv(
      self, token, shape, handle, false);
    //build_recv_done(self, &recv_op, shape, handle, false)

    unimplemented!()
  }

  fn batch_norm_training(
    &mut self,
    operand: &BlitzOp,
    scale: &BlitzOp,
    offset: &BlitzOp,
    epsilon: f64,
    feature_index: i64) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    let scale_shape_wrapper = self.get_shape(scale);
    check_error(&scale_shape_wrapper);
    let offset_shape_wrapper = self.get_shape(offset);
    check_error(&offset_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let scale_shape = scale_shape_wrapper.unwrap();
    let offset_shape = offset_shape_wrapper.unwrap();

    let shape_wrapper = ShapeInference::infer_batch_norm_training_shape(
      &operand_shape, &scale_shape, &offset_shape, feature_index);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);
    instr.set_epsilon(epsilon);
    instr.set_feature_index(feature_index);
    
    let result = self.add_instruction(
      &mut instr,
      HloOpcode::BatchNormTraining,
      &vec![operand.clone(), scale.clone(), offset.clone()]);
    check_error(&result);
    result.unwrap()
  }

  fn batch_norm_inference(
    &mut self,
    operand: &BlitzOp,
    scale: &BlitzOp,
    offset: &BlitzOp,
    mean: &BlitzOp,
    variance: &BlitzOp,
    epsilon: f64,
    feature_index: i64) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    let scale_shape_wrapper = self.get_shape(scale);
    check_error(&scale_shape_wrapper);
    let offset_shape_wrapper = self.get_shape(offset);
    check_error(&offset_shape_wrapper);
    let mean_shape_wrapper = self.get_shape(mean);
    check_error(&mean_shape_wrapper);
    let variance_shape_wrapper = self.get_shape(variance);
    check_error(&variance_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let scale_shape = scale_shape_wrapper.unwrap();
    let offset_shape = offset_shape_wrapper.unwrap();
    let mean_shape = mean_shape_wrapper.unwrap();
    let variance_shape = variance_shape_wrapper.unwrap();

    let shape_wrapper = ShapeInference::infer_batch_norm_inference_shape(
      &operand_shape, &scale_shape, &offset_shape, &mean_shape, &variance_shape, feature_index);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);
    instr.set_epsilon(epsilon);
    instr.set_feature_index(feature_index);
    
    let result = self.add_instruction(
      &mut instr, HloOpcode::BatchNormInference,
      &vec![operand.clone(), scale.clone(), offset.clone(),
        mean.clone(), variance.clone()]);
    check_error(&result);
    result.unwrap()
  }

  fn batch_norm_grad(
    &mut self,
    operand: &BlitzOp,
    scale: &BlitzOp,
    batch_mean: &BlitzOp,
    batch_var: &BlitzOp,
    grad_output: &BlitzOp,
    epsilon: f64,
    feature_index: i64) -> BlitzOp
  {
    let mut instr = HloInstruction::default();

    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    let scale_shape_wrapper = self.get_shape(scale);
    check_error(&scale_shape_wrapper);
    let batch_mean_shape_wrapper = self.get_shape(batch_mean);
    check_error(&batch_mean_shape_wrapper);
    let batch_var_shape_wrapper = self.get_shape(batch_var);
    check_error(&batch_var_shape_wrapper);
    let grad_output_shape_wrapper = self.get_shape(grad_output);
    check_error(&grad_output_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let scale_shape = scale_shape_wrapper.unwrap();
    let batch_mean_shape = batch_mean_shape_wrapper.unwrap();
    let batch_var_shape = batch_var_shape_wrapper.unwrap();
    let grad_output_shape = grad_output_shape_wrapper.unwrap();

    let shape_wrapper = ShapeInference::infer_batch_norm_grad_shape(
      &operand_shape, &scale_shape, &batch_mean_shape, &batch_var_shape,
      &grad_output_shape, feature_index);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);
    instr.set_epsilon(epsilon);
    instr.set_feature_index(feature_index);
    
    let result = self.add_instruction(
      &mut instr, HloOpcode::BatchNormGrad, 
      &vec![operand.clone(), scale.clone(), batch_mean.clone(),
        batch_var.clone(), grad_output.clone()]);
    check_error(&result);
    result.unwrap()
  }

  fn get_dimension_size(
    &mut self,
    operand: &BlitzOp,
    dimension: i64) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let shape_wrapper = ShapeInference::infer_get_dimension_size_shape(
      &operand_shape, dimension);
    check_error(&shape_wrapper);

    // Calling GetDimensionSize on a static dimension returns a constant instruction.
    if operand_shape.is_static_dimension(dimension as usize) {
      // TODO
    }

    let shape = shape_wrapper.unwrap();
    instr.set_shape(shape);
    instr.add_dimensions(dimension);
    let result = self.add_instruction(
      &mut instr, HloOpcode::GetDimensionSize, &vec![operand.clone()]);
    check_error(&result);
    result.unwrap()
  }

  fn set_dimension_size(
    &mut self,
    operand: &BlitzOp,
    val: &BlitzOp,
    dimension: i64) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    let val_shape_wrapper = self.get_shape(&val);
    check_error(&val_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let val_shape = val_shape_wrapper.unwrap();
    let shape_wrapper = ShapeInference::infer_set_dimension_size_shape(
      &operand_shape, &val_shape, dimension);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result = self.set_dimension_size_internal(
      &shape, operand, val, dimension);
    check_error(&result);
    result.unwrap()
  }

  fn set_dimension_size_internal(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    val: &BlitzOp,
    dimension: i64) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.add_dimensions(dimension);
    self.add_instruction(
      &mut instr,
      HloOpcode::SetDimensionSize,
      &vec![operand.clone(), val.clone()])
  }

  fn remove_dynamic_dimension(
    &self,
    operand: &BlitzOp,
    dimension: i64) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let mut operand_shape = operand_shape_wrapper.unwrap();
    operand_shape.set_dynamic_dimension(dimension as usize, false);

    // TODO: constant_r0
    unimplemented!()
  }

  fn add_instruction(
    &mut self,
    instr: &mut HloInstruction,
    opcode: HloOpcode,
    operands: &Vec<BlitzOp>) -> Result<BlitzOp, String>
  {
    let handle = self.get_next_id();
    instr.set_id(handle);
    instr.set_opcode(opcode.clone());
    if instr.name().is_empty() {
      instr.set_name(hlo_opcode_string(&instr.opcode()));
    }
    for operand in operands {
      if operand.builder.is_none() {
        let mut err_msg = "Invalid BlitzOp with handle ".to_string();
        err_msg.push_str(&operand.handle().to_string());
        return Err(err_msg);
      }
      if operand.builder != Some(self) {
        let mut err_msg = "Do not add BlitzOp from builder ".to_string();
        err_msg.push_str(&operand.builder.unwrap().name());
        err_msg.push_str(" to builder ");
        err_msg.push_str(&self.name());
        return Err(err_msg);
      }
      instr.add_operand_ids(operand.handle());
    }

    if self.oneshot_metadata.is_some() {
      let metadata = self.oneshot_metadata.as_ref().unwrap().clone();
      instr.set_metadata(metadata);
      self.oneshot_metadata = None;
    } else {
      instr.set_metadata(self.metadata.clone());
    }

    if self.sharding.is_some() {
      let result =
        normalize_and_assign_sharding(instr, self.sharding.as_ref().unwrap());
      if result.is_err() {
        return Err(result.err().unwrap());
      }
    }
    //instr.set_frontend_attributes(self.frontend_attributes.clone()); // TODO
    self.handle_to_index.insert(handle, self.instructions.len() as i64);
    self.instructions.push(instr.clone());

    let shape = self.instructions.last().unwrap().shape();
    self.instruction_shapes.push(shape.clone());

    let op = BlitzOp::new_from_handle(handle, self);
    Ok(op)
  }

  fn add_called_computation(
    &self,
    computation: i64,
    instr: &mut HloInstruction)
  {
    let result =
      self.get_subcomputation_shape(computation);
    check_error(&result);

    // TODO: calls_computations_from_parent
    instr.add_called_computation_ids(computation);
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
  fn unary_op(
    &mut self,
    unop: HloOpcode,
    operand: &BlitzOp,
    result_accuracy: Option<ResultAccuracy>) -> BlitzOp
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    let shape_wrapper =
      ShapeInference::infer_unary_op_shape_by_shape(&unop, &operand_shape);
    check_error(&shape_wrapper);

    let shape = shape_wrapper.unwrap();
    let result = self.add_op_with_shape_with_accuracy(
      unop, &shape, &vec![operand.clone()], result_accuracy);
    check_error(&result);
    result.unwrap()
  }

  // Internal helper method that does the building for an arbitrary binary op.
  // broadcast_dimensions specifies which dimensions to use for broadcasting
  // when the operation is between tensors of different ranks. The direction is
  // only used if opcode is kCompare.
  fn binary_op(
    &mut self,
    _binop: HloOpcode,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _broadcast_dimensions: &Vec<i64>,
    _direction: Option<ComparisonDirection>,
    _t: Option<ComparisonType>) -> BlitzOp
  {
/*
    let lhs_shape_wrapper = self.get_shape(lhs);
    check_error(&lhs_shape_wrapper);
    let lhs_shape = lhs_shape_wrapper.unwrap();

    let rhs_shape_wrapper = self.get_shape(rhs);
    check_error(&rhs_shape_wrapper);
    let rhs_shape = rhs_shape_wrapper.unwrap();

    let shape_wrapper =
      ShapeInference::infer_binary_op_shape_by_dimensions(
        &binop, &lhs_shape, &rhs_shape, broadcast_dimensions);
    check_error(&shape_wrapper);
    let mut shape = shape_wrapper.unwrap();

    let mut updated_lhs = lhs.clone();
    let mut updated_rhs = rhs.clone();
    if !lhs_shape.is_unbounded_dynamic() && !rhs_shape.is_unbounded_dynamic() {
      if lhs_shape.dimensions_vec().len() < shape.dimensions_vec().len() {
        let updated_lhs_wrapper = broadcast_to_target_rank(
          lhs, &lhs_shape, &shape, broadcast_dimensions);
        check_error(&updated_lhs_wrapper);
        updated_lhs = updated_lhs_wrapper.unwrap();
      }
      if rhs_shape.dimensions_vec().len() < shape.dimensions_vec().len() {
        let updated_rhs_wrapper = broadcast_to_target_rank(
          rhs, &rhs_shape, &shape, broadcast_dimensions);
        check_error(&updated_rhs_wrapper);
        updated_rhs = updated_rhs_wrapper.unwrap();
      }
      let updated_lhs_shape_wrapper =
        self.get_shape(&updated_lhs);
      check_error(&updated_lhs_shape_wrapper);
      let updated_lhs_shape = updated_lhs_shape_wrapper.unwrap();

      let updated_rhs_shape_wrapper =
        self.get_shape(&updated_rhs);
      check_error(&updated_rhs_shape_wrapper);
      let updated_rhs_shape = updated_rhs_shape_wrapper.unwrap();

      if !ShapeUtil::same_dimensions(&shape, &updated_lhs_shape) {
        let updated_lhs_wrapper =
          self.add_broadcast_sequence(&shape, &updated_lhs);
        check_error(&updated_lhs_wrapper);
        updated_lhs = updated_lhs_wrapper.unwrap();
      }
      if !ShapeUtil::same_dimensions(&shape, &updated_rhs_shape) {
        let updated_rhs_wrapper =
          self.add_broadcast_sequence(&shape, &updated_rhs);
        check_error(&updated_rhs_wrapper);
        updated_rhs = updated_rhs_wrapper.unwrap();
      }
    } else {
      if ShapeUtil::is_scalar(&lhs_shape) || ShapeUtil::is_scalar(&rhs_shape) {
        if ShapeUtil::is_scalar(&lhs_shape) {
          let updated_lhs_wrapper =
            broadcast_scalaar_to_output_shape_with_unbounded(
              self, lhs, rhs, &rhs_shape);
          updated_lhs = updated_lhs_wrapper.unwrap();
        }
        if ShapeUtil::is_scalar(&rhs_shape) {
          let updated_rhs_wrapper =
            broadcast_scalaar_to_output_shape_with_unbounded(
              self, rhs, lhs, &lhs_shape);
          updated_rhs = updated_rhs_wrapper.unwrap();
        }
      } else {
        if !ShapeUtil::same_dimensions(&lhs_shape, &rhs_shape) {
          shape.set_element_type(lhs_shape.element_type());
          let broadcast_result = 
            broadcast_to_output_shape_with_unbounded(
            self, lhs, &lhs_shape, rhs, &rhs_shape, &shape, broadcast_dimensions);
          updated_lhs = broadcast_result.lhs.clone();
          updated_rhs = broadcast_result.rhs.clone();
        }
      }
    }

    if binop == HloOpcode::Compare {
      if !direction.is_some() {
        let err_msg =
          "compare expects a comparison_direction, but none provided".to_string();
        assert!(false, "{:?}", err_msg);
      }
      if t.is_none() {
        //let result =
          //self.compare(&shape, updated_lhs, updated_rhs, direction.unwrap());
        //check_error(&result);
        //return result.unwrap();
      } else {
        // TODO
      }
    }
    if direction.is_some() {
      let mut err_msg =
        "a comparison direction is provided for a non-compare opcode: ".to_string();
      err_msg.push_str(&hlo_opcode_string(&binop));
      assert!(false, "{:?}", err_msg);
    }
    //self.binary_op_no_broadcast(binop, &shape, updated_lhs, updated_rhs)
*/
    unimplemented!()
  }

  // Internal helper method that does the building for an arbitrary binary op
  // with same ranked operands that doesn't broadcast.
  fn binary_op_no_broadcast(
    &mut self,
    binop: HloOpcode,
    shape: &Shape,
    lhs: BlitzOp,
    rhs: BlitzOp) -> BlitzOp
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    let result = self.add_instruction(
      &mut instr, binop, &vec![lhs, rhs]);
    check_error(&result);
    result.unwrap()
  }

  fn compare(
    &mut self,
    shape: &Shape,
    lhs: &BlitzOp,
    rhs: &BlitzOp,
    direction: ComparisonDirection) -> Result<BlitzOp, String>
  {
    let operand_shape = self.get_shape(lhs);
    self.compare_type(shape, lhs, rhs, direction, 
      default_comparison_type(&operand_shape.unwrap().element_type()))
  }

  fn compare_type(
    &mut self,
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
    self.add_instruction(
      &mut instr,
      HloOpcode::Compare,
      &vec![lhs.clone(), rhs.clone()])
  }

  // Internal helper method that does the building for an arbitrary ternary op.
  fn ternary_op(
    &mut self,
    _triop: HloOpcode,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _ehs: &BlitzOp) -> BlitzOp
  {
    /*
    let mut updated_lhs = lhs.clone();
    let mut updated_rhs = rhs.clone();
    let mut updated_ehs = ehs.clone();

    // The client API supports implicit broadcast for Select and Clamp, but
    // Blitz does not support implicit broadcast. Make implicit broadcast explicit
    // and update the operands.
    if triop == HloOpcode::Select || triop == HloOpcode::Clamp {
      let lhs_shape_wrapper = self.get_shape(lhs);
      check_error(&lhs_shape_wrapper);
      let rhs_shape_wrapper = self.get_shape(rhs);
      check_error(&rhs_shape_wrapper);
      let ehs_shape_wrapper = self.get_shape(ehs);
      check_error(&ehs_shape_wrapper);

      let lhs_shape = lhs_shape_wrapper.unwrap();
      let rhs_shape = rhs_shape_wrapper.unwrap();
      let ehs_shape = ehs_shape_wrapper.unwrap();

      let output_shape_wrapper = 
        ShapeInference::infer_scalar_broadcast_shape(
          &vec![lhs_shape.clone(), rhs_shape.clone(), ehs_shape.clone()]);
      check_error(&output_shape_wrapper);

      // Scalar broadcast if mix of scalars and non-scalars
      let output_shape = output_shape_wrapper.unwrap();
      if output_shape.is_some() {
        if ShapeUtil::is_scalar(&lhs_shape) {
          let mut target = rhs.clone();
          if !ShapeEqual::new().equal(output_shape.as_ref().unwrap(), &rhs_shape) {
            target = ehs.clone();
          }
          let updated_lhs_wrapper =
            self.broadcast_scalar_to_output_shape(lhs, &target);
          updated_lhs = updated_lhs_wrapper.unwrap();
        }
        if ShapeUtil::is_scalar(&rhs_shape) {
          let mut target = lhs.clone();
          if !ShapeEqual::new().equal(output_shape.as_ref().unwrap(), &lhs_shape) {
            target = ehs.clone();
          }
          let updated_rhs_wrapper =
            self.broadcast_scalar_to_output_shape(rhs, &target);
          updated_rhs = updated_rhs_wrapper.unwrap();
        }
        if ShapeUtil::is_scalar(&ehs_shape) {
          let mut target = lhs.clone();
          if !ShapeEqual::new().equal(output_shape.as_ref().unwrap(), &lhs_shape) {
            target = rhs.clone();
          }
          let updated_ehs_wrapper =
            self.broadcast_scalar_to_output_shape(ehs, &target);
          updated_ehs = updated_ehs_wrapper.unwrap();
        }
      }
    }
    
    let lhs_shape_wrapper = self.get_shape(&updated_lhs);
    check_error(&lhs_shape_wrapper);
    let rhs_shape_wrapper = self.get_shape(&updated_rhs);
    check_error(&rhs_shape_wrapper);
    let ehs_shape_wrapper = self.get_shape(&updated_ehs);
    check_error(&ehs_shape_wrapper);

    let lhs_shape = lhs_shape_wrapper.unwrap();
    let rhs_shape = rhs_shape_wrapper.unwrap();
    let ehs_shape = ehs_shape_wrapper.unwrap();
    let inferred_shape_wrapper =
      ShapeInference::infer_ternary_op_shape_by_shape(
        triop.clone(), &lhs_shape, &rhs_shape, &ehs_shape);
    check_error(&inferred_shape_wrapper);

    let _inferred_shape = inferred_shape_wrapper.unwrap();
    //let result = self.add_op_with_shape(
      //triop,
      //&inferred_shape,
      //&vec![updated_lhs, updated_rhs, updated_ehs]);
    //check_error(&result);
    //result.unwrap()
    */
    unimplemented!()
  }

  fn rng_op(
    &mut self,
    distribution: RandomDistribution,
    parameters: &Vec<BlitzOp>,
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
    parameters: &Vec<BlitzOp>,
    shape: &Shape) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    instr.set_distribution(distribution);
    self.add_instruction(&mut instr, HloOpcode::Rng, parameters)
  }

  fn in_dim_broadcast(
    &mut self,
    shape: &Shape,
    operand: &BlitzOp,
    broadcast_dimensions: &Vec<i64>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    for dim in broadcast_dimensions {
      instr.add_dimensions(*dim);
    }

    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);
    assert!(!shape.is_unbounded_dynamic(), "broadcast op result shapes must be static");

    let operand_shape = operand_shape_wrapper.unwrap();
    for i in 0..shape.dimensions_vec().len() {
      let mut found_idx = -1;
      for j in broadcast_dimensions {
        if *j as usize == i { found_idx = *j; }
      }
      if found_idx != -1 {
        // Broadcast dimensions are permitted to be dynamic iff the operand
        // dimension is dynamic.
        assert!(operand_shape.is_bounded_dynamic_dimension(found_idx) ==
          shape.is_bounded_dynamic_dimension(i as i64));
      } else {
        // Non-broadcast dimensions must be static.
        assert!(shape.is_static_dimension(i))
      }
    }

    self.add_instruction(
      &mut instr,
      HloOpcode::Broadcast,
      &vec![operand.clone()])
  }

  // Internal helper method that creates a sequence of instructions that
  // performs an explicit broadcast of the operand to the target shape.
  // All dimensions of the operand must either be equal to the corresponding
  // output shape dimension, or be exactly 1.  (Such dimensions are the
  // degenerate dimensions.)
  fn add_broadcast_sequence(
    &mut self,
    output_shape: &Shape,
    operand: &BlitzOp) -> Result<BlitzOp, String>
  {
    let operand_shape_wrapper = self.get_shape(operand);
    check_error(&operand_shape_wrapper);

    let operand_shape = operand_shape_wrapper.unwrap();
    assert!(ShapeUtil::is_scalar(&operand_shape) ||
      operand_shape.dimensions_vec().len() == output_shape.dimensions_vec().len());

    let broadcast_shape = ShapeUtil::change_element_type(
      output_shape, &operand_shape.element_type());

    // Do explicit broadcast for scalar.
    if ShapeUtil::is_scalar(&operand_shape) {
      return self.in_dim_broadcast(
        &mut ShapeUtil::make_static_shape(&broadcast_shape),
        operand, &vec![]);
    }

    let mut broadcast_dimensions = vec![];
    let mut reshaped_dimensions = vec![];
    let mut reshaped_dynamic_dimensions = vec![];
    for i in 0..operand_shape.dimensions_vec().len() {
      if operand_shape.dimensions(i) == output_shape.dimensions(i) {
        broadcast_dimensions.push(i as i64);
        reshaped_dimensions.push(operand_shape.dimensions(i));
        reshaped_dynamic_dimensions.push(operand_shape.is_dynamic_dimension(i as i64));
      } else {
        assert!(operand_shape.dimensions(i) == 1 &&
          operand_shape.is_static_dimension(i));
      }
    }

    let reshaped_shape = ShapeUtil::make_shape_dynamic(
      &operand_shape.element_type(),
      reshaped_dimensions,
      reshaped_dynamic_dimensions);

    // Eliminate the size one dimensions.
    // The added reshape reduces the rank of the tensor. Hence we cannot directly
    // apply the broadcast's sharding on reshape.
    let reshaped_operand_wrapper = self.reshape_internal(
      &reshaped_shape, operand.clone(), -1);
    check_error(&reshaped_operand_wrapper);
    let _reshaped_operand = reshaped_operand_wrapper.unwrap();
    
    // Broadcast 'reshape' up to the larger size.
    //self.in_dim_broadcast(&broadcast_shape, &reshaped_operand, &broadcast_dimensions)
    unimplemented!()
  }

  // Internal helper method that broadcasts a scalar to the shape of the output.
  fn broadcast_scalar_to_output_shape(
    &mut self,
    scalar: &BlitzOp,
    output: &BlitzOp) -> Result<BlitzOp, String>
  {
    let scalar_shape_wrapper = self.get_shape(scalar);
    check_error(&scalar_shape_wrapper);
    let output_shape_wrapper = self.get_shape(output);
    check_error(&output_shape_wrapper);

    let scalar_shape = scalar_shape_wrapper.unwrap();
    let mut output_shape = output_shape_wrapper.unwrap();
    let updated_output = scalar.clone();

    if output_shape.is_unbounded_dynamic() {
      output_shape.set_element_type(scalar_shape.element_type());
      let updated_output =
        broadcast_scalaar_to_output_shape_with_unbounded(
          self, scalar, output, &output_shape);
      check_error(&updated_output);
      return updated_output;
    }

    let updated_output =
      self.add_broadcast_sequence(&output_shape, &updated_output);
    check_error(&updated_output);
    updated_output
  }

  // Internal helper method for creating a Reshape op with the already inferred
  // shape.
  fn reshape_internal(
    &mut self,
    shape: &Shape,
    _operand: BlitzOp,
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
    //self.add_instruction(&instr, HloOpcode::Reshape, &vec![operand])
    unimplemented!()
  }

  // A visitor which checks whether an operation is a compile-time constant,
  // meaning that it doesn't depend on any parameters, or on any stateful
  // operation such as `RngNormal` or `Infeed`. The visitor walks the
  // computation starting at a given operation and sets is_constant to false iff
  // a parameter or stateful operation is encountered.
  fn is_constant_visitor(
    &self,
    op_handle: i64,
    _depth: i64,
    visited: &mut HashSet<i64>,
    is_constant: &mut bool)
  {
    if visited.contains(&op_handle) || *is_constant { return; }

    let instr_wrapper =
      self.lookup_instruction_by_handle(op_handle);
    check_error(&instr_wrapper);

    let instr = instr_wrapper.unwrap();
    match instr.opcode() {
      HloOpcode::While => *is_constant = false,
      HloOpcode::Scatter => *is_constant = false,
      HloOpcode::Send => *is_constant = false,
      HloOpcode::Recv => *is_constant = false,
      HloOpcode::Parameter => *is_constant = false,
      _ => unimplemented!(),
    }
    
    visited.insert(op_handle);
  }

  // Checks bounds for convolution parameters.
  fn verify_convolution(
    &self,
    lhs_shape: &Shape,
    rhs_shape: &Shape,
    dimension_numbers: &ConvolutionDimensionNumbers) -> Result<(), String>
  {
    if lhs_shape.dimensions_vec().len() != rhs_shape.dimensions_vec().len() {
      let mut err_msg = "convolution arguments must have same number
        of dimensions. got: ".to_string();
      err_msg.push_str(&ShapeUtil::human_string(lhs_shape));
      err_msg.push_str(" and ");
      err_msg.push_str(&ShapeUtil::human_string(rhs_shape));
      return Err(err_msg);
    }
    let num_dims = lhs_shape.dimensions_vec().len();
    if num_dims < 2 {
      let mut err_msg =
        "convolution expects argument arrays with >= 3 dimensions.".to_string();
      err_msg.push_str("got: ");
      err_msg.push_str(&ShapeUtil::human_string(lhs_shape));
      err_msg.push_str(" and ");
      err_msg.push_str(&ShapeUtil::human_string(rhs_shape));
      return Err(err_msg);
    }
    let num_spatial_dims = num_dims - 2;
    let check_spatial_dimensions =
      |field_name: &String, numbers: &Vec<i64>| -> Result<(), String>
    {
      if numbers.len() != num_spatial_dims {
        let mut err_msg = "expected ".to_string();
        err_msg.push_str(&num_spatial_dims.to_string());
        err_msg.push_str(" elements for ");
        err_msg.push_str(&field_name);
        err_msg.push_str(", but got ");
        err_msg.push_str(&numbers.len().to_string());
        return Err(err_msg);
      }
      for i in 0..numbers.len() {
        if numbers[i] < 0 || numbers[i] >= num_dims as i64 {
          let mut err_msg = "convolution ".to_string();
          err_msg.push_str(&field_name);
          err_msg.push_str("[");
          err_msg.push_str(&i.to_string());
          err_msg.push_str("]");
          err_msg.push_str(" is out of bounds: ");
          err_msg.push_str(&numbers[i].to_string());
          return Err(err_msg);
        }
      }
      Ok(())
    };
    let mut result = check_spatial_dimensions(
      &"input_spatial_dimensions".to_string(),
      &dimension_numbers.input_spatial_dimensions_vec());
    check_error(&result);

    result = check_spatial_dimensions(
      &"kernel_spatial_dimensions".to_string(),
      &dimension_numbers.kernel_spatial_dimensions_vec());
    check_error(&result);
    
    check_spatial_dimensions(
      &"output_spatial_dimensions".to_string(),
      &dimension_numbers.output_spatial_dimensions_vec())
  }

  fn get_next_id(&self) -> i64 {
    self.next_id
  }

  // Creates an op with the given opcode and the output shape.
  fn add_op_with_shape(
    &mut self,
    opcode: HloOpcode,
    shape: &Shape,
    operands: &Vec<BlitzOp>) -> Result<BlitzOp, String>
  {
    self.add_op_with_shape_with_accuracy(opcode, shape, operands, None)
  }

  fn add_op_with_shape_with_accuracy(
    &mut self,
    opcode: HloOpcode,
    shape: &Shape,
    operands: &Vec<BlitzOp>,
    result_accuracy: Option<ResultAccuracy>) -> Result<BlitzOp, String>
  {
    let mut instr = HloInstruction::default();
    instr.set_shape(shape.clone());
    if result_accuracy.is_some() {
      instr.set_result_accuracy(result_accuracy.unwrap());
    }
    self.add_instruction(&mut instr, opcode, operands)
  }

  fn get_subcomputation_shape(&self, _id: i64) -> Result<ProgramShape, String> {
    unimplemented!()
  }

  fn add(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _broadcast_dimensions: &Vec<i64>) -> BlitzOp
  {
    unimplemented!()
  }

  fn or(
    &self,
    _lhs: &BlitzOp,
    _rhs: &BlitzOp,
    _broadcast_dimensions: &Vec<i64>) -> BlitzOp
  {
    unimplemented!()
  }
}

// Enqueues a constant with the value of the given literal onto the
// computation.
pub fn constant_literal<'builder, T>(
  builder: &'builder mut BlitzBuilder, literal: &Literal<T>) -> BlitzOp<'builder>
  where T: Default + Clone + PartialEq
{
  builder.constant_literal(literal)
}

// Enqueues a constant onto the computation. Methods are templated on the
// native host type (NativeT) which corresponds to a specific Blitz
// PrimitiveType as given in the following table:
//
//  Native Type   PrimitiveType
// -----------------------------
//   bool           PRED
//   int32_t        S32
//   int64_t        S64
//   uint32_t       U32
//   uint64_t       U64
//   float          F32
//   double         F64
//
// Note: not all primitive types defined in blitz_data.rs have a
// corresponding native type yet.
pub fn constant_r0<T>(builder: &mut BlitzBuilder, value: T) -> BlitzOp
  where T: Default + Clone + PartialEq + 'static
{
  constant_literal(builder, &LiteralUtil::create_r0(value))
}

// Enqueues a tuple-creation instruction onto the computation.
pub fn tuple<'builder>(
  /*builder: &'builder BlitzBuilder,*/
  elements: &'builder mut Vec<BlitzOp<'builder>>) -> BlitzOp<'builder>
{
  let builder = elements[0].mutable_builder();
  builder.tuple(elements)
}

// Enqueues a tuple-element-get instruction onto the computation.
pub fn get_tuple_element<'builder>(
  tuple_data: &'builder mut BlitzOp, index: i64) -> BlitzOp<'builder>
{
  let b = tuple_data.mutable_builder();
  b.get_tuple_element(&tuple_data, index)
}

pub fn real<'builder>(_operand: &BlitzOp) -> BlitzOp<'builder> {
  unimplemented!()
}

// Enqueues a custom call instruction onto the computation. A custom call
// invokes code external to Blitz. The |operands| are passed to the external code,
// and the external code is expected to produce a result of the given
// |shape|. The exact mechanism is backend-specific. For example, in the CPU
// backend, a call instruction is emitted which targets a symbol with the name
// |call_target_name|.  |call_target_name| and |opaque| can arbitrary strings,
// but |call_target_name| should be short as it may be used in labels. |opaque|
// can encode arbitrarily large amounts of information. |has_side_effect|
// specifies whether the instruction can have side effects.
// |output_operand_aliasing| specifies a list of output/operand buffer pairs
// that alias each other, where the output buffer is represented as a
// ShapeIndex, and the operand buffer is represented as the operand index and
// the ShapeIndex.
pub fn custom_call<'builder, T>(
  builder: &'builder mut BlitzBuilder,
  call_target_name: &String,
  operands: &Vec<BlitzOp>,
  shape: &Shape,
  opaque: &String,
  has_side_effect: bool,
  output_operand_aliasing: &Vec<(usize, (i64, usize))>,
  lieteral: Option<&Literal<T>>,
  schedule: CustomCallSchedule,
  api_version: CustomCallApiVersion
) -> BlitzOp<'builder>
  where T: Default + Clone + PartialEq
{
  builder.custom_call(
    call_target_name,
    operands,
    shape,
    opaque,
    None,
    has_side_effect,
    output_operand_aliasing,
    lieteral,
    None,
    None, 
    schedule,
    api_version)
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

pub fn build_send<'builder>(
  builder: &'builder mut BlitzBuilder,
  operand: &BlitzOp,
  token: &BlitzOp,
  handle: &ChannelHandle,
  is_host_transfer: bool) -> BlitzOp<'builder>
{
  let mut send_instr = HloInstruction::default();
  let shape_wrapper = builder.get_shape(operand);
  check_error(&shape_wrapper);

  let shape = shape_wrapper.unwrap();
  let tuple_shape = ShapeUtil::make_tuple_shape(
    vec![
      shape,
      ShapeUtil::make_shape(&PrimitiveType::U32, vec![]),
      ShapeUtil::make_token_shape()
    ]
  );
  send_instr.set_shape(tuple_shape);
  send_instr.set_channel_id(handle.handle());
  send_instr.set_is_host_transfer(is_host_transfer);
  let result = builder.add_instruction(
    &mut send_instr,
    HloOpcode::Send,
    &vec![operand.clone(), token.clone()]);
  check_error(&result);

  result.unwrap()
}

pub fn build_send_done<'builder>(
  builder: &'builder mut BlitzBuilder,
  operand: &BlitzOp,
  handle: &ChannelHandle,
  is_host_transfer: bool) -> BlitzOp<'builder>
{
  let mut send_done_instr = HloInstruction::default();
  send_done_instr.set_shape(ShapeUtil::make_token_shape());
  send_done_instr.set_channel_id(handle.handle());
  send_done_instr.set_is_host_transfer(is_host_transfer);

  let result = builder.add_instruction(
    &mut send_done_instr, HloOpcode::SendDone, &vec![operand.clone()]);
  check_error(&result);

  result.unwrap()
}

pub fn build_recv<'builder>(
  builder: &'builder mut BlitzBuilder,
  token: &BlitzOp,
  shape: &Shape,
  handle: &ChannelHandle,
  is_host_transfer: bool) -> BlitzOp<'builder>
{
  let mut recv_instr = HloInstruction::default();
  recv_instr.set_shape(ShapeUtil::make_tuple_shape(vec![
    shape.clone(),
    ShapeUtil::make_shape(&PrimitiveType::U32, vec![]),
    ShapeUtil::make_token_shape()
  ]));
  recv_instr.set_channel_id(handle.handle());
  recv_instr.set_is_host_transfer(is_host_transfer);
  let result = builder.add_instruction(
    &mut recv_instr, HloOpcode::Recv, &vec![token.clone()]);
  check_error(&result);
  result.unwrap()
}

pub fn build_recv_done<'builder>(
  builder: &'builder mut BlitzBuilder,
  token: &BlitzOp,
  shape: &Shape,
  handle: &ChannelHandle,
  is_host_transfer: bool) -> BlitzOp<'builder>
{
  let mut recv_done_instr = HloInstruction::default();
  recv_done_instr.set_shape(ShapeUtil::make_tuple_shape(vec![
    shape.clone(),
    ShapeUtil::make_token_shape()
  ]));
  recv_done_instr.set_channel_id(handle.handle());
  recv_done_instr.set_is_host_transfer(is_host_transfer);
  let result = builder.add_instruction(
    &mut recv_done_instr, HloOpcode::RecvDone, &vec![token.clone()]);
  check_error(&result);
  result.unwrap()
}

fn broadcast_to_target_rank<'builder>(
  _option: &BlitzOp,
  _origin_shape: &Shape,
  _target_shape: &Shape,
  _broadcast_dimensions: &Vec<i64>) -> Result<BlitzOp<'builder>, String>
{
  unimplemented!()    
}

fn broadcast_scalaar_to_output_shape_with_unbounded<'builder>(
  _builder: &'builder mut BlitzBuilder,
  _scalar: &BlitzOp,
  _output: &BlitzOp,
  _output_shape: &Shape) -> Result<BlitzOp<'builder>, String>
{
  unimplemented!()
}

// Helper struct to store the result of `BroadcastToOutputShapeWithUnbounded`.
struct UnboundedBroadcastResult<'builder> {
  lhs: BlitzOp<'builder>,
  rhs: BlitzOp<'builder>
}

// Broadcast `lhs` and `rhs` to `output_shape` with unbounded dimensions where
// `lhs` or `rhs` are possibly different ranks than `output_shape`.
fn broadcast_to_output_shape_with_unbounded<'builder>(
  _builder: &'builder mut BlitzBuilder,
  _lhs: &BlitzOp,
  _lhs_shape: &Shape,
  _rhs: &BlitzOp,
  _rhs_shape: &Shape,
  _output_shape: &Shape,
  _broadcast_dimensions: &Vec<i64>) -> UnboundedBroadcastResult<'builder>
{
  unimplemented!()
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}

pub struct BlitzScopedShardingAssignment {}

impl BlitzScopedShardingAssignment {
  pub fn new() -> Self {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_constant() {
    let name = "test_is_constant".to_string();
    let mut b = BlitzBuilder::new(name);
    let cst = constant_r0(&mut b, 1.0);
    
    let mut elements = vec![cst.clone(), cst.clone()];
    let _tuple = tuple(&mut elements);
    //let _get_tuple_element = get_tuple_element(&tuple, 0);
    //let is_constant = b.is_constant(&get_tuple_element);
    //assert_eq!(is_constant.unwrap(), true);
  }
}