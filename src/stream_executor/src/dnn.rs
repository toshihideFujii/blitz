#![allow(dead_code)]

use std::{collections::HashMap, vec};

// Specifies the data type used by an operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
  Float,
  Double,
  Half,
  Int8,
  Int32,
  ComplexFloat,
  ComplexDouble,
  BF16,
  F8E5M2,
  F8E4M3FN,
  F8E5M2FNUZ,
  F8E4M3FNUZ,
  Int64,
}

// Describes how a convolution input or output layer's data is formatted.
pub enum DataLayout {
  // Naming convention:
  // Y <-> row or height
  // X <-> column or width
  // Batch <-> batch, or N
  // Depth <-> feature, or channel
  //
  // Note: In cudnn, kBatchDepthYX4 and kBatchDepthYX32 are the same layout
  // (namely, NCHW_VECT_C).  It differentiates between these two by using a
  // different data type (int8x4 vs int8x32).  In StreamExecutor we use
  // different layouts for these, because we don't usually pass an explicit data
  // type to StreamExecutor functions.
  YXDepthBatch,
  YXBatchDepth,
  BatchYXDepth,
  BatchDepthYX,
  BatchDepthYX4,
  BatchDepthYX32,
}

// Describes how a convolution filter is laid out in the memory.
pub enum FilterLayout {
  // Naming convention:
  // Y <-> row or height
  // X <-> column or width
  // Output <-> output feature, or N
  // Input <-> input feature, or N
  OutputInputYX,
  OutputYXInput,
  OutputInputYX4,
  OutputInputYX32,
  OutputInputYX32CudnnReordered,
  YXInputOutput,
}

// Specifies an index to use when accessing specific spatial dimensions.
pub enum DimIndex {
  X = 0,
  Y = 1,
  Z = 2,
}

// Return a reordered dims.
pub fn reorder_dims(
  _input: &Vec<i64>, _from: &FilterLayout, _to: &FilterLayout) -> Vec<i64>
{
  unimplemented!()
}

// Helper functions to make methods more readable.
pub fn get_dim(data: &Vec<i64>, dim: DimIndex) -> i64 {
  let mut vec = vec![];
  vec.clone_from(data);
  vec.reverse();
  vec[dim as usize]
}

pub fn set_dim(data: &mut Vec<i64>, dim: DimIndex, value: i64) {
  data.reverse();
  data[dim as usize] = value;
  data.reverse();
}

// int64_t is not the same type as tensorflow::protobuf_int64 in open-source.
// This wrapper function gives an int64_t array slice view of a repeated int64
// protobuf field.
//
// T should be a protobuf RepeatedField.
pub fn as_i64_slice<T>(_repeated_field: &T) -> &Vec<i64> {
  unimplemented!()
}

// Returns a string representation of the given data layout.
pub fn data_layout_string(layout: &DataLayout) -> String {
  match layout {
    DataLayout::YXDepthBatch => return "YXDepthBatch".to_string(),
    DataLayout::YXBatchDepth => return "YXBatchDepth".to_string(),
    DataLayout::BatchYXDepth => return "BatchYXDepth".to_string(),
    DataLayout::BatchDepthYX => return "BatchDepthYX".to_string(),
    DataLayout::BatchDepthYX4 => return "BatchDepthYX4".to_string(),
    DataLayout::BatchDepthYX32 => return "BatchDepthYX32".to_string(),
  }
}

// Specifies a quantization for activations in a given BatchDescriptor.
#[derive(Debug, Clone)]
pub  enum QuantizedActivationMode {
  K8Bit,
  K16Bit,
  K32Bit,
}

// Specifies the types of a RNN model.
pub enum RnnMode {
  Relu,
  Tanh,
  Lstm,
  Gru,
}

// Specifies the input model and whether there is a linear transformation
// between the input state and the first layer hidden state.
pub enum RnnInputMode {
  LinearSkip,
  SkipInput,
}

// Specifies the number of directions used in a RNN model. When bidirection
// is used, the input states and output sequence contain data for both
// directions.
pub enum RnnDirectionMode {
  Undirectional,
  Bidirectional,
}

// Describe the math definition for the conv op. The popular behavior is
// actually called cross-correlation in math, despite the operation is often
// referred as convolution. See cuDNN cudnnConvolutionMode_t.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvolutionMode {
  CrossCorrelation,
  Convolution,
}

pub enum ConvolutionKind {
  Invalid,
  Forward,
  BackwardFilter,
  BackwardData,
  ForwardBiasActivation,
  ForwardFraph,
}

#[derive(Debug, Clone)]
pub struct TensorDiscriptor {
  d_type: DataType,
  dimensions: Vec<i64>,
  minor_to_major: Vec<i64>,
}

impl TensorDiscriptor {
  pub fn default() -> Self {
    TensorDiscriptor {
      d_type: DataType::Int32,
      dimensions: Vec::new(),
      minor_to_major: Vec::new()
    }
  }

  pub fn new(d_type: DataType, dimensions: Vec<i64>, minor_to_major: Vec<i64>) -> Self {
    TensorDiscriptor {
      d_type: d_type,
      dimensions: dimensions,
      minor_to_major: minor_to_major
    }
  }

  pub fn get_physical_dimensions_major_to_minor(&self) -> Result<Vec<i64>, String> {
    let mut logical_to_physical = vec![0; self.minor_to_major.len()];
    for physical in 0..logical_to_physical.len() {
      let logical = self.minor_to_major[self.minor_to_major.len() - 1 - physical];
      logical_to_physical[logical as usize] = physical;
    }
    if self.dimensions.len() != self.minor_to_major.len() {
      return Err("Dimensions size should match the layout size".to_string());
    }
    let mut physical_dims = vec![0; self.dimensions.len()];
    for i in 0..physical_dims.len() {
      physical_dims[logical_to_physical[i]] = self.dimensions[i];
    }
    Ok(physical_dims)
  }

  pub fn get_physical_strides_major_to_minor(&self) -> Vec<i64> {
    let phys_dims = self.get_physical_dimensions_major_to_minor().unwrap();
    let mut phys_strides = vec![0; self.ndims()];
    phys_strides[self.ndims() - 1] = 1;
    for i in self.ndims()-2..0 {
      phys_strides[i] = phys_strides[i + 1] * phys_dims[i + 1];
    }
    phys_strides
  }

  pub fn get_logical_strides(&self) -> Vec<i64> {
    let mut physical_strides = self.get_physical_dimensions_major_to_minor().unwrap();
    physical_strides.reverse();
    let mut logical_strides = vec![0; physical_strides.len()];
    for i in 0..self.ndims() {
      logical_strides[self.minor_to_major()[i] as usize] = physical_strides[i];
    }
    logical_strides
  }

  pub fn for_(d_type: DataType, dimensions: &Vec<i64>, minor_to_major: &Vec<i64>) -> Self {
    let mut dims = vec![0; dimensions.len()];
    let mut minor_to_major_vec = vec![0; minor_to_major.len()];
    assert_eq!(dimensions.len(), minor_to_major.len());

    for i in 0..dimensions.len() {
      dims[i] = dimensions[i];
      minor_to_major_vec[i] = minor_to_major[i];
    }
    TensorDiscriptor::new(d_type, dims, minor_to_major_vec)
  }

  pub fn ndims(&self) -> usize {
    assert_eq!(self.dimensions.len(), self.minor_to_major.len());
    self.dimensions.len()
  }

  pub fn dimensions(&self) -> &Vec<i64> {
    &self.dimensions
  }

  pub fn set_dimensions(&mut self, index: usize, value: i64) {
    self.dimensions[index] = value;
  }

  pub fn minor_to_major(&self) -> &Vec<i64> {
    &self.minor_to_major
  }

  pub fn data_type(&self) -> &DataType {
    &self.d_type
  }

  pub fn filter_layout(&self) -> &FilterLayout {
    unimplemented!()
  }

  pub fn to_string(&self) -> String {
    let mut str = "{dimensions: ".to_string();
    str += "[";
    for dim in self.dimensions() {
      str += &dim.to_string();
      str += ", ";
    }
    str += "] minor_to_major: [";
    for num in self.minor_to_major() {
      str += &num.to_string();
      str += ", ";
    }
    str += "]}";
    str
  }
}

pub struct MatmulTensorDescriptor {
  tensor: TensorDiscriptor,
  batch_dimension_numbers: Vec<i64>,
  contracting_dims: Vec<i64>,
}

impl MatmulTensorDescriptor {
  pub fn new(
    tensor: TensorDiscriptor,
    batch_dimension_numbers: Vec<i64>,
    contracting_dims: Vec<i64>) -> Self
  {
    MatmulTensorDescriptor {
      tensor: tensor,
      batch_dimension_numbers: batch_dimension_numbers,
      contracting_dims: contracting_dims
    }
  }

  pub fn get_non_contracting_dims(&self) -> Result<Vec<i64>, String> {
    let mut non_contracting_dims = vec![];
    for dim in 0..self.tensor.dimensions().len() {
      let mut is_batch = false;
      for batch_dim in &self.batch_dimension_numbers {
        if *batch_dim == dim as i64 { is_batch = true; }
      }
      let mut is_contracting = false;
      for contracting_dim in &self.contracting_dims {
        if *contracting_dim == dim as i64 { is_contracting = true; }
      }
      if is_batch && is_contracting {
        return Err("A dimension cannot be both a batch dimension and a
          contracting dimension.".to_string());
      }
      if !(is_batch || is_contracting) {
        non_contracting_dims.push(dim as i64);
      }
    }

    if self.batch_dimension_numbers.len() + self.contracting_dims.len() +
      non_contracting_dims.len() != self.tensor.dimensions.len()
    {
      return Err("Batch_dimension_numbers, contracting_dim and non_contracting_dims
        should sum up to the total number of dimensions.".to_string());
    }

    Ok(non_contracting_dims)
  }

  pub fn get_cudnn_compatible_dimensions(&self, is_lhs: bool) -> Vec<i64> {
    self.make_cudnn_compatible(self.tensor.dimensions(), is_lhs).unwrap()
  }

  pub fn get_cudnn_compatible_strides(&self, is_lhs: bool) -> Vec<i64> {
    self.make_cudnn_compatible(&self.tensor.get_logical_strides(), is_lhs).unwrap()
  }

  pub fn make_cudnn_compatible(
    &self, v: &Vec<i64>, is_lhs: bool) -> Result<Vec<i64>, String>
  {
    let mut cudnn_compatible = vec![0; v.len()];
    let batch_dim_size = self.batch_dimension_numbers.len();
    assert_eq!(batch_dim_size, v.len());
    for i in 0..batch_dim_size {
      cudnn_compatible[i] = v[self.batch_dimension_numbers[i] as usize];
    }

    let non_contracting_dims = self.get_non_contracting_dims().unwrap();
    if self.batch_dimension_numbers.len() + self.contracting_dims.len() +
      non_contracting_dims.len() != v.len()
    {
      return Err("Batch_dimension_numbers, contracting_dim and non_contracting_dims 
        should sum up to the total number of dimensions.".to_string());
    }

    if is_lhs /* lhs -> {b0, b1,....bk, m, k} */ {
      for i in 0..non_contracting_dims.len() {
        cudnn_compatible[batch_dim_size + i] = v[non_contracting_dims[i] as usize];
      }
      for i in 0..self.contracting_dims.len() {
        cudnn_compatible[batch_dim_size + non_contracting_dims.len() + i]
          = v[self.contracting_dims[i] as usize];
      }
    } else /* rhs -> {b0, b1, ... bk, k, n} */ {
      for i in 0..self.contracting_dims.len() {
        cudnn_compatible[batch_dim_size + i] = v[self.contracting_dims[i] as usize];
      }
      for i in 0..non_contracting_dims.len() {
        cudnn_compatible[batch_dim_size + self.contracting_dims.len() + i]
          = v[non_contracting_dims[i] as usize];
      }
    }
    Ok(cudnn_compatible)
  }

  pub fn for_(
    d_type: DataType,
    dimensions: &Vec<i64>,
    minor_to_major: &Vec<i64>,
    batch_dims: &Vec<i64>,
    contracting_dims: &Vec<i64>) -> Self
  {
    let mut batch_dims_vec = vec![0; batch_dims.len()];
    let mut contracting_dims_vec = vec![0; contracting_dims.len()];
    for i in 0..batch_dims.len() {
      batch_dims_vec[i] = batch_dims[i];
    }
    for i in 0..contracting_dims.len() {
      contracting_dims_vec[i] = contracting_dims[i];
    }
    let tensor =
      TensorDiscriptor::for_(d_type, dimensions, minor_to_major);
    MatmulTensorDescriptor::new(
      tensor, batch_dims_vec, contracting_dims_vec)
  }

  pub fn dimensions(&self) -> &Vec<i64> {
    &self.tensor.dimensions
  }

  pub fn minor_to_major(&self) -> &Vec<i64> {
    &self.tensor.minor_to_major
  }

  pub fn data_type(&self) -> &DataType {
    &self.tensor.d_type
  }

  pub fn to_string(&self) -> String {
    let mut str = "{".to_string();
    str += &self.tensor.to_string();
    str += " batch_dimension_numbers: [";
    for batch_dim in &self.batch_dimension_numbers{
      str += &batch_dim.to_string();
      str += ", ";
    }
    str += "], contracting_dims: [";
    for contracting_dim in &self.contracting_dims{
      str += &contracting_dim.to_string();
      str += ", ";
    }
    str += "]}";
    str
  }
}

pub struct ParamsRegion {
  offset: i64,
  size: i64,
}

// Specifies the descriptor for a RNN model.
//
// An example use case:
//   * The user first creates a model through CreateRnnDescriptor.
//   * The user queries the size of the underlying opaque parameter buffer.
//   * The user creates and initializes a parameter buffer of the proper size.
//   * The user runs forward and backward operations using this RNN descriptor.
//   * Once a while, user queries maintainable weights and bias regions from
//       the underlying parameter buffer. They are more likely to be forward
//       compatible and should used in saving and restoring a model.
//   * The user releases the RNN descriptor when the model is no longer in use.
pub struct RnnDescriptor {}

impl RnnDescriptor {
  pub fn new() -> Self {
    RnnDescriptor {  }
  }

  pub fn params_size_in_bytes(&self) -> i64 {
    -1
  }

  pub fn params_weight_regions(&self) -> Vec<ParamsRegion> {
    Vec::new()
  }

  pub fn params_bias_regions(&self) -> Vec<ParamsRegion> {
    Vec::new()
  }
}

// Specifies the sequence in a RNN model.
//
// The user is responsible for releasing this descriptor when it is no longer
// in use. The destructor releases the underlying descriptors.
pub struct RnnSequenceTensorDescriptor {}

// Specifies either the input and hidden state in a RNN model.
//
// The user is responsible for releasing this descriptor when it is no longer
// in use. The destructor releases the underlying descriptors.
pub struct RnnStateTensorDescriptor {}

// Returns a string representation of the given quantization mode.
pub fn quantize_activation_mode_string() {}

// Describes the dimensions that a layer consumes/produces.
//
// This is a matrix (height, width), its "depth" (feature_map_count),
// how many of these matrices are present (count),
// and the maximum and minimum values expected in the matrix (value_max,
// value_min).
// If input is quantized, all values greater
// than value_max will be clipped to value_max and all values less than
// value_min will be clipped to value_min.
// When quantized output is dequantized no value will be greater than
// value_max or less than value_min.
//
// Uses the named argument construction form:
//
//  auto input_batch_dimensions =
//      BatchDescriptor().set_count(42).set_feature_map_count(7)...
//
// Details:
//
// For a convolutional layer, a single inference takes a 3-dimensional matrix
// of input and produces a 3-dimensional matrix of output. We call the three
// dimensions height, width and feature_map_count, where for an image, the
// height and width correspond to the Y and X pixel indices, respectively, and
// the feature_map_count corresponds to the RGB dimension of the input data.
// Then the count indicates how many 3D matrices are being presented to be
// processed at once; this corresponds to the neural network concept of
// minibatch size.
//
// For a fully connected layer, it's better to put the nodes of the layer in
// the feature_map_count, and leave the height and weight as degenerate (== 1).
// Count indicates how many input vectors (degenerate 3D matrices) are to be
// processed.
//
// If unspecified, value_max and value_min default to 0.0.
// If value_max == value_min the Stream will attempt to derive valid values -
// for example the output of Relu6 activation will always be in the range
// [0.0, 6.0].
//
// If unspecified, layout defaults to kYXDepthBatch.
#[derive(Debug, Clone)]
pub struct BatchDescriptor {
  tensor: TensorDiscriptor,
  value_max: f64,
  value_min: f64,
  quantized_activation_mode: QuantizedActivationMode,
}

impl BatchDescriptor {
  pub fn default() -> Self {
    BatchDescriptor::new(2)
  }

  pub fn new(_ndims: i64) -> Self {
    /*
    BatchDescriptor {
      tensor: TensorDiscriptor::new(d_type, dimensions, minor_to_major),
      value_max: 0.0,
      value_min: 0.0,
      quantized_activation_mode: QuantizedActivationMode::K8Bit
    }
    */
    unimplemented!()
  }

  pub fn to_string(&self) -> String {
    let mut spatial = "".to_string();
    for i in 0..self.ndims() {
      spatial += &self.spatial_size()[i].to_string();
    }
    let mut str = "{count: ".to_string();
    str += &self.count().to_string();
    str += "feature_map_count: ";
    str += &self.feature_map_count().to_string();
    str += &spatial;
    str += &self.value_min.to_string();
    str += &self.value_max.to_string();
    //str += data_layout_string(self.layout());
    str
  }

  pub fn to_short_string(&self) -> String {
    unimplemented!()
  }

  pub fn count(&self) -> i64 {
    self.tensor.dimensions[0]
  }

  pub fn feature_map_count(&self) -> i64 {
    self.tensor.dimensions[1]
  }
  
  pub fn height(&self) -> i64 {
    get_dim(self.spatial_size(), DimIndex::Y)
  }

  pub fn width(&self) -> i64 {
    get_dim(self.spatial_size(), DimIndex::X)
  }

  pub fn spatial_dim(&self, dim: DimIndex) -> i64 {
    get_dim(self.spatial_size(), dim)
  }

  pub fn ndims(&self) -> usize {
    self.spatial_size().len()
  }

  pub fn value_max(&self) -> f64 {
    self.value_max
  }

  pub fn value_min(&self) -> f64 {
    self.value_min
  }

  pub fn layout(&self) {
    unimplemented!()
  }

  pub fn quantized_activation_mode(&self) -> QuantizedActivationMode {
    self.quantized_activation_mode.clone()
  }

  // Full dimensions of the underlying data, ordered according to a specific
  // layout.
  pub fn full_dims(&self, _layout: &DataLayout) -> &Vec<i64> {
    unimplemented!()
  }

  // Full strides of the underlying data, ordered according to a specific
  // layout.
  pub fn full_strides(&self, _layout: &DataLayout) -> &Vec<i64> {
    unimplemented!()
  }

  // Vectorized dimensions where users can specify the dimension that the number
  // of dimensions is reported rather than the full number of elements.
  pub fn vectorized_dims(
    &self, _layout: &DataLayout, _vec_size: i64, _vec_dim: i64) -> &Vec<i64>
  {
    unimplemented!()
  }

  // Vectorized strides correspond to the vectorized_dims.
  pub fn vectorized_strides(
    &self, _layout: &DataLayout, _vec_size: i64, _vec_dim: i64) -> &Vec<i64>
  {
    unimplemented!()
  }

  // Named-argument helpers for avoiding user error during construction.
  pub fn set_count(&mut self, value: i64) -> &mut Self {
    self.tensor.set_dimensions(0, value);
    self
  }

  pub fn set_feature_map_count(&mut self, value: i64) -> &mut Self{
    self.tensor.set_dimensions(1, value);
    self
  }

  pub fn set_height(&mut self, value: i64) -> &mut Self {
    set_dim(self.spatial_size(), DimIndex::Y, value);
    self
  }

  pub fn set_width(&mut self, value: i64) -> &mut Self {
    set_dim(self.spatial_size(), DimIndex::X, value);
    self
  }

  pub fn set_spatial_dim(&mut self, dim: DimIndex, value: i64) -> &mut Self {
    set_dim(self.spatial_size(), dim, value);
    self
  }

  pub fn set_value_max(&mut self, value: f64) -> &mut Self {
    self.value_max = value;
    self
  }

  pub fn set_value_min(&mut self, value: f64) -> &mut Self {
    self.value_min = value;
    self
  }

  pub fn set_layout() {}

  pub fn set_quantized_activation_mode(
    &mut self, mode: QuantizedActivationMode) -> &mut Self
  {
    self.quantized_activation_mode = mode;
    self
  }

  // Return the number of nodes in a single feature map.
  pub fn nodes_per_feature_map(&self) -> i64 {
    let mut ret = 0;
    for i in 0..self.ndims() {
      ret *= self.spatial_size()[i];
    }
    ret
  }

  // Return the number of nodes across all feature maps. Note that this is not
  // affected by the batch count.
  pub fn nodes_across_feature_maps(&self) -> i64 {
    self.nodes_per_feature_map() * self.feature_map_count()
  }

  // Returns the number of elements (e.g. RGB pixel values) required to hold a
  // given batch descriptor, given a no-padding assumption. Note that this is
  // affected by the batch count.
  pub fn element_count(&self) -> i64 {
    self.count() * self.feature_map_count() * self.nodes_per_feature_map()
  }
  
  // Return the number of weights required to fully connect a layer with
  // dimensions given by the 'input' descriptor with a layer with dimensions
  // given by the 'output' descriptor.
  pub fn fully_connected_weight_count(
    input: &BatchDescriptor, output: &BatchDescriptor) -> i64
  {
    input.nodes_across_feature_maps() * output.nodes_across_feature_maps()
  }

  // Return the number of biases required to fully connect to an output layer
  // with dimensions given the 'output' descriptor.
  pub fn fully_connected_bias_count(output: &BatchDescriptor) -> i64 {
    output.nodes_across_feature_maps()
  }

  // Return a BatchDescriptor for the output of a depth concatenation
  // with the given input descriptors. The inputs should have the same
  // dimensions, except possibly for feature_map_count(), though this
  // function does not verify that.
  pub fn depth_concatenate_output_descriptor(inputs: &Vec<BatchDescriptor>) -> Self {
    if inputs.is_empty() {
      return BatchDescriptor::default();
    }
    let mut feature_map_count = 0;
    for dimensions in inputs {
      feature_map_count += dimensions.feature_map_count();
    }
    let mut output = inputs[0].clone();
    output.set_feature_map_count(feature_map_count);
    output
  }

  fn spatial_size(&self) -> &mut Vec<i64> {
    unimplemented!()
  }
}

// Returns a string representation of the given filter layout.
pub fn filter_layout_string() {}

// Describes a filter for the convolution. This is the "window" from
// height-by-width patches of each of the feature maps in the input layer to the
// cells within the output feature map.
//
// Uses the named argument construction form:
//
//  FilterDescriptor filter_dimensions;
//  filter_dimensions
//    .set_output_feature_map_count(42)
//    .set_input_feature_map_count(7)
//    ...
//
// Arguments:
// - output_feature_map_count: number of feature maps in the output layer.
// - input_feature_map_count: number of feature maps in the input layer (from
//      which the filter patch is taken).
// - input_filter_height: "height" number of neurons used in the sliding window
//      over the input layer.
// - input_filter_width: "width" number of neurons used in the sliding window
//      over the input layer.
//
// Sometimes names like "filter input height" are referred to by synonymous
// terminology, such as "kernel y size".
//
// If unspecified, layout defaults to kOutputInputYX.
#[derive(Debug, Clone)]
pub struct FilterDescriptor {
  tensor: TensorDiscriptor
}

impl FilterDescriptor {
  pub fn default() -> Self {
    unimplemented!()
  }

  pub fn new(_ndims: i64) -> Self {
    unimplemented!()
  }

  pub fn set_output_feature_map_count(&mut self, value: i64) -> &mut Self {
    self.tensor.set_dimensions(0, value);
    self
  }

  pub fn set_input_feature_map_count(&mut self, value: i64) -> &mut Self {
    self.tensor.set_dimensions(1, value);
    self
  }

  pub fn set_input_filter_height(&mut self, value: i64) -> &mut Self {
    set_dim(self.input_filter_dims(), DimIndex::Y, value);
    self
  }

  pub fn set_input_filter_width(&mut self, value: i64) -> &mut Self {
    set_dim(self.input_filter_dims(), DimIndex::X, value);
    self
  }

  pub fn set_layout() {}

  pub fn set_spatial_dim(&mut self, dim: DimIndex, value: i64) -> &mut Self {
    set_dim(self.input_filter_dims(), dim, value);
    self
  }

  pub fn ndims(&self) -> usize {
    self.input_filter_dims().len()
  }

  pub fn to_string() {}
  pub fn to_short_string() {}

  pub fn compute_weight_count(&self) -> i64 {
    let mut ret =
      self.output_feature_map_count() * self.input_feature_map_count();
    for i in 0..self.ndims() {
      ret *= self.input_filter_dims()[i];
    }
    ret
  }

  pub fn bias_count() {}

  pub fn output_feature_map_count(&self) -> i64 {
    self.tensor.dimensions[0]
  }

  pub fn input_feature_map_count(&self) -> i64 {
    self.tensor.dimensions[1]
  }

  pub fn input_filter_height(&self) -> i64 {
    get_dim(&self.input_filter_dims(), DimIndex::Y)
  }

  pub fn input_filter_width(&self) -> i64 {
    get_dim(self.input_filter_dims(), DimIndex::X)
  }

  pub fn input_filter_dim(&self, dim: DimIndex) -> i64 {
    get_dim(&self.input_filter_dims(), dim)
  }

  pub fn layout(&self) -> &FilterLayout {
    self.tensor.filter_layout()
  }

  pub fn input_filter_dims(&self) -> &mut Vec<i64> {
    unimplemented!()
  }

  // Full dimensions of the underlying filter,
  // ordered according to a specific layout.
  pub fn full_dims(&self, layout: &FilterLayout) -> Vec<i64> {
    let mut oiyx_dims = vec![0; self.ndims()];
    oiyx_dims[0] = self.output_feature_map_count();
    oiyx_dims[1] = self.input_feature_map_count();

    for i in 0..self.input_filter_dims().len() {
      oiyx_dims[i+2] = self.input_filter_dims()[i];
    }
    reorder_dims(&oiyx_dims, &FilterLayout::OutputInputYX, layout)
  }

  // Full strides of the underlying filter,
  // ordered according to a specific layout.
  pub fn full_strides(&self, layout: &FilterLayout) -> Vec<i64> {
    let phys_dims = self.full_dims(self.layout());
    let mut phys_strides = vec![0; phys_dims.len()];

    phys_strides[self.ndims() + 1] = 1;
    for i in self.ndims()..=0 {
      phys_strides[i] = phys_strides[i + 1] * phys_dims[i + 1];
    }
    reorder_dims(&phys_strides, self.layout(), layout)
  }

  // Vectorized dimensions where users can specify the dimension that the number
  // of dimensions is reported rather than the full number of elements.
  pub fn vectorized_dims(
    &self, layout: &FilterLayout, vector_size: i64, vector_dim: i64) -> Vec<i64>
  {
    let mut oiyx_dims = self.full_dims(&FilterLayout::OutputInputYX);
    if vector_dim != -1 {
      oiyx_dims[vector_dim as usize] /= vector_size;
    }
    reorder_dims(&oiyx_dims, &FilterLayout::OutputInputYX, layout)
  }

  // Vectorized strides correspond to the vectorized_dims.
  pub fn vectorized_strides(
    &self, layout: &FilterLayout, vector_size: i64, vector_dim: i64) -> Vec<i64>
  {
    let phys_dims = self.vectorized_dims(self.layout(), vector_size, vector_dim);
    let mut phys_strides = vec![0; phys_dims.len()];
    phys_strides[phys_dims.len() - 1] = 1;
    for i in phys_dims.len()-2..=0 {
      phys_strides[i] = phys_strides[i + 1] * phys_dims[i + 1];
    }
    reorder_dims(&phys_strides, self.layout(), layout)
  }
}

// Describes how padding should be aligned when the total number of pad
// elements is odd.
pub enum PadAlignment {
  Default,
  CudnnPadding,
  TensorFlowPadding,
}

// Returns a string representation of the given padding alignment.
pub fn pad_alignment_string(alignment: &PadAlignment) -> String {
  match alignment {
    PadAlignment::Default => return "default".to_string(),
    PadAlignment::CudnnPadding => return "cuDNN pading".to_string(),
    PadAlignment::TensorFlowPadding => return "TensorFlow padding".to_string()
  }
}

// Convolution-specific parameters.
struct ConvolutionDescriptorProro {
  paddings: Vec<i64>,
  strides: Vec<i64>,
  dilations: Vec<i64>,
  compute_mode: DataType,
  group_count: i64,
  convolution_mode: ConvolutionMode,
  name: String
}

impl ConvolutionDescriptorProro {
  pub fn set_group_count(&mut self, group_count: i64) {
    self.group_count = group_count;
  }

  pub fn set_convolution_mode(&mut self, mode: ConvolutionMode) {
    self.convolution_mode = mode;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
}

// Describes a convolution.
//
// Uses the named argument construction form:
//
//  ConvolutionDescriptor convolution_dimensions;
//  convolution_dimensions
//    .set_vertical_filter_stride(2)
//    .set_horizontal_filter_stride(2)
//    ...
//
// Arguments:
// - zero_padding_height: padding of the "y dimension" of the input data. Note
//    that this is different from the height of the filter.
// - zero_padding_width: analogous to the height above, but in the "x
//    dimension".
// - vertical_filter_stride: the convolution slides a 2-dimensional window of
//    filter-height-by-filter-width over the input layer -- the center of that
//    window is moved in the "y dimension" according to this stride value.
// - horizontal_filter_stride: analogous to the vertical stride above, but in
//    the "x dimension".
// - vertical_dilation_rate: there will be (vertical_dilation_rate - 1) skipped
//   cells between each filter element in the "y dimension".
// - horizontal_dilation_rate: there will be (horizontal_dilation_rate - 1)
//   skipped cells between each filter element in the "x dimension".
// - convolution_not_crosscor: By default (convolution_not_crosscor == false),
//   we perform cross correlation rather than convolution. With the flag set,
//   we perform convolution. Convolution and cross correlation are related by
//   rotating the filter by 180 degrees (or equivalently flipping all spatial
//   dimensions).
pub struct CovolutionDescriptor {
  proto: ConvolutionDescriptorProro
}

impl CovolutionDescriptor {
  pub fn default() -> Self {
    CovolutionDescriptor::new(2)
  }

  pub fn new(_ndims: i64) -> Self {
    unimplemented!()
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  pub fn to_short_string(&self) -> String {
    unimplemented!()
  }

  pub fn set_zero_padding_height(&mut self, value: i64) -> &mut Self {
    set_dim(self.padding(), DimIndex::Y, value);
    self
  }

  pub fn set_zero_padding_width(&mut self, value: i64) -> &mut Self {
    set_dim(self.padding(), DimIndex::X, value);
    self
  }

  pub fn set_zero_padding(&mut self, dim: DimIndex, value: i64) -> &mut Self {
    set_dim(self.padding(), dim, value);
    self
  }

  pub fn set_vertical_filter_stride(&mut self, value: i64) -> &mut Self {
    set_dim(self.strides(), DimIndex::Y, value);
    self
  }

  pub fn set_horizontal_filter_stride(&mut self, value: i64) -> &mut Self {
    set_dim(self.strides(), DimIndex::X, value);
    self
  }

  pub fn set_filter_stride(&mut self, dim: DimIndex, value: i64) -> &mut Self {
    set_dim(self.strides(), dim, value);
    self
  }

  pub fn set_vertical_dilation_rate(&mut self, value: i64) -> &mut Self {
    set_dim(self.dilations(), DimIndex::Y, value);
    self
  }

  pub fn set_horizontal_dilaation_rate(&mut self, value: i64) -> &mut Self {
    set_dim(self.dilations(), DimIndex::X, value);
    self
  }

  pub fn set_dilation_rate(&mut self, dim: DimIndex, value: i64) -> &mut Self {
    set_dim(self.dilations(), dim, value);
    self
  }

  pub fn set_group_count(&mut self, group_count: i64) -> &mut Self {
    self.proto.set_group_count(group_count);
    self
  }

  pub fn set_convolution_not_crosscorr(&mut self, conv: bool) -> &mut Self {
    if conv {
      self.proto.set_convolution_mode(ConvolutionMode::Convolution);
    } else {
      self.proto.set_convolution_mode(ConvolutionMode::CrossCorrelation);
    }
    self
  }

  pub fn set_name(&mut self, name: String) -> &mut Self {
    self.proto.set_name(name);
    self
  }

  pub fn zero_padding_height(&self) -> i64 {
    get_dim(&self.proto.paddings, DimIndex::Y)
  }

  pub fn zero_padding_width(&self) -> i64 {
    get_dim(&self.proto.paddings, DimIndex::X)
  }

  pub fn vertical_filter_stride(&self) -> i64 {
    get_dim(&self.proto.strides, DimIndex::Y)
  }

  pub fn horizontal_filter_stride(&self) -> i64 {
    get_dim(&self.proto.strides, DimIndex::X)
  }

  pub fn vertical_dilation_rate(&self) -> i64 {
    get_dim(&self.proto.dilations, DimIndex::Y)
  }

  pub fn horizontal_dilation_rate(&self) -> i64 {
    get_dim(&self.proto.dilations, DimIndex::X)
  }

  pub fn zero_padding(&self, dim: DimIndex) -> i64 {
    get_dim(&self.proto.paddings, dim)
  }

  pub fn filter_stride(&self, dim: DimIndex) -> i64 {
    get_dim(&self.proto.strides, dim)
  }

  pub fn dilation_rate(&self, dim: DimIndex) -> i64 {
    get_dim(&self.proto.dilations, dim)
  }

  pub fn pad_slignment(&self) -> PadAlignment {
    PadAlignment::Default
  }

  pub fn group_count(&self) -> i64 {
    self.proto.group_count
  }

  pub fn ndims(&self) -> usize {
    self.proto.paddings.len()
  }

  pub fn convolution_not_crosscorr(&self) -> bool {
    self.proto.convolution_mode == ConvolutionMode::CrossCorrelation
  }

  pub fn strides(&mut self) -> &mut Vec<i64> {
    &mut self.proto.strides
  }

  pub fn dilations(&mut self) -> &mut Vec<i64> {
    &mut self.proto.dilations
  }

  pub fn padding(&mut self) -> &mut Vec<i64> {
    &mut self.proto.paddings
  }

  pub fn name(&self) -> &String {
    &self.proto.name
  }
}

// A patch of values in the input can be pooled via either a max or an average
// operation.
// Specify int64_t so there's no padding in PoolingDescriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolingMode {
  Maximum,
  Average,
}

// Specify the dimension in which to concatenate inputs in space.
// Specify int64_t so there's no padding in SpaceConcatenateMode.
pub enum SpaceConcatenateMode {
  XDirecton,
  YDirection,
}

// Returns a short name for the pooling mode, e.g. "Avg".
pub fn short_pooling_mode_string(mode: PoolingMode) -> String {
  match mode {
    PoolingMode::Maximum => return "Max".to_string(),
    PoolingMode::Average => return "Avg".to_string(),
  }
}

// Describes a pooling operation to be enqueued onto a stream via a platform's
// DnnSupport.
//
// Arguments:
//  pooling_mode: pooling operator to use on the input patch
//  window_height: height of input window
//  window_width: width of input window
//  vertical_stride: vertical delta for center of the input patch
//  horizontal_stride: horizontal delta for center of the input patch
pub struct PoolingDescriptor {
  mode: PoolingMode,
  ndims: i64,
  propagate_nans: bool,
  name: String,
  window: Vec<i64>,
  padding: Vec<i64>,
  strides: Vec<i64>,
}

impl PoolingDescriptor {
  pub fn new() {}

  pub fn set_pooling_mode(&mut self, value: PoolingMode) -> &mut Self {
    self.mode = value;
    self
  }

  pub fn set_window_height(&mut self, value: i64) -> &mut Self {
    set_dim(&mut self.window, DimIndex::Y, value);
    self
  }

  pub fn set_window_width(&mut self, value: i64) -> &mut Self {
    set_dim(&mut self.window, DimIndex::X, value);
    self
  }

  pub fn set_window(&mut self, dim: DimIndex, value: i64) -> &mut Self {
    set_dim(&mut self.window, dim, value);
    self
  }

  pub fn set_vertical_padding(&mut self, value: i64) -> &mut Self {
    set_dim(&mut self.padding, DimIndex::Y, value);
    self
  }

  pub fn set_horizontal_padding(&mut self, value: i64) -> &mut Self {
    set_dim(&mut self.padding, DimIndex::X, value);
    self
  }

  pub fn set_padding(&mut self, dim: DimIndex, value: i64) -> &mut Self {
    set_dim(&mut self.padding, dim, value);
    self
  }

  pub fn set_vertical_stride(&mut self, value: i64) -> &mut Self {
    set_dim(&mut self.strides, DimIndex::Y, value);
    self
  }

  pub fn set_horizontal_stride(&mut self, value: i64) -> &mut Self {
    set_dim(&mut self.strides, DimIndex::X, value);
    self
  }

  pub fn set_stride(&mut self, dim: DimIndex, value: i64) -> &mut Self {
    set_dim(&mut self.strides, dim, value);
    self
  }

  pub fn set_propagate_nans(&mut self, value: bool) -> &mut Self {
    self.propagate_nans = value;
    self
  }

  pub fn set_name(&mut self, name: String) -> &mut Self {
    self.name = name;
    self
  }

  pub fn ndims(&self) -> i64 {
    self.ndims
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  pub fn to_short_string(&self) -> String {
    unimplemented!()
  }

  pub fn mode(&self) -> PoolingMode {
    self.mode.clone()
  }

  pub fn window_height(&self) -> i64 {
    get_dim(&self.window, DimIndex::Y)
  }

  pub fn window_width(&self) -> i64 {
    get_dim(&self.window, DimIndex::X)
  }

  pub fn window_dim(&self, dim: DimIndex) -> i64 {
    get_dim(&self.window, dim)
  }

  pub fn vertical_padding(&self) -> i64 {
    get_dim(&self.padding, DimIndex::Y)
  }

  pub fn horizontal_padding(&self) -> i64 {
    get_dim(&self.padding, DimIndex::X)
  }

  pub fn padding_dim(&self, dim: DimIndex) -> i64 {
    get_dim(&self.padding, dim)
  }

  pub fn vertical_stride(&self) -> i64 {
    get_dim(&self.strides, DimIndex::Y)
  }

  pub fn horizontal_stride(&self) -> i64 {
    get_dim(&self.strides, DimIndex::X)
  }

  pub fn stride_dim(&self, dim: DimIndex) -> i64 {
    get_dim(&self.strides, dim)
  }

  pub fn window(&self) -> &Vec<i64> {
    &self.window
  }

  pub fn padding(&self) -> &Vec<i64> {
    &self.padding
  }

  pub fn strides(&self) -> &Vec<i64> {
    &self.strides
  }

  pub fn propagate_nans(&self) -> bool {
    self.propagate_nans
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MathType {
  Default,
  // The GPU may operate 4x4 matrix FMA.
  // See cuDNN's documentation for CUDNN_TENSOR_OP_MATH.
  TensorOp,
}

struct AlgorithmProto {
  algo_id: i64,
  math_type: MathType,
  tuning_knobs: HashMap<i64, i64>,
  // Legacy algorithm enums and cuDNN Frontend engine numbers need to coexist in
  // the same proto medium-term, until we can be confident of no longer needing
  // the legacy cuDNN convolution API.  Once the migration is complete, we can
  // stop producing legacy algorithm enums and remove this field.
  is_cudnn_frontend: bool,
  workspace_size: i64,
}

impl AlgorithmProto {
  pub fn new() {}
  pub fn has_workspace_size(&self) -> bool {
    unimplemented!()
  }
}

// Collects parameters for DNN algorithms
//#[derive(Debug)]
pub struct AlgoriithmDesc {
  proto: AlgorithmProto
}

impl AlgoriithmDesc {
  pub fn new() {}

  pub fn is_cudnn_frontend(&self) -> bool {
    self.proto.is_cudnn_frontend
  }

  pub fn tensor_ops_enabled(&self) -> bool {
    self.proto.math_type == MathType::TensorOp
  }

  pub fn workspace_size(&self) -> Option<i64> {
    if self.proto.has_workspace_size() {
      return Some(self.proto.workspace_size)
    }
    None
  }

  pub fn algo_id(&self) -> i64 {
    self.proto.algo_id
  }

  pub fn tuning_knobs() {}
  pub fn hash() {}

  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}

// Describes the result from a perf experiment.
//
// Arguments:
//  algorithm: returns the exact algorithm that was used.
//  elapsed_time_in_ms: returns the measured elapsed time in milliseconds.
pub struct ProfileResult {
  algorithm: Option<AlgoriithmDesc>,
  elapsed_time_in_ms: f64,
  scratch_size: usize,
  warmup_run_executed: bool,
}

impl ProfileResult {
  pub fn new() {}

  pub fn is_valid(&self) -> bool {
    self.algorithm.is_some() && self.elapsed_time_in_ms != f64::MAX
  }

  pub fn warmup_run_exxcuted(&self) -> bool {
    self.warmup_run_executed
  }

  pub fn set_warmup_run_executed(&mut self, value: bool) {
    self.warmup_run_executed = value;
  }

  pub fn algorithm(&self) -> &Option<AlgoriithmDesc> {
    &self.algorithm
  }

  pub fn set_algorithm(&mut self, value: AlgoriithmDesc) {
    self.algorithm = Some(value);
  }

  pub fn elapsed_time_in_ms(&self) -> f64 {
    self.elapsed_time_in_ms
  }

  pub fn set_elapsed_time_in_ms(&mut self, value: f64) {
    self.elapsed_time_in_ms = value;
  }

  pub fn scratch_size(&self) -> usize {
    self.scratch_size
  }

  pub fn set_scratch_size(&mut self, value: usize) {
    self.scratch_size = value;
  }
}

// Describes the configuration for the algorithms that will used.
//
// Arguments:
//  algorithm: the primary algorithm that should be used.
//  algorithm_no_scratch: a secondary algorithm that should be used, if the
//    the allocation for the scratch memory fails.
//  scrach_size: specify the size of scratch memory in bytes needed for the
//    algorithm used.
//
// On CUDA platform with CUDNN library, algorithm and algorithm_no_scratch
// would be used. On ROCm platform with MIOpen library, algorithm and
// scratch_size would be used. The major difference between the two platforms
// are whether it's possible to get an algorithm without scratch memory. On
// CUDA + CUDNN it's possible, and algorithm_no_scratch can be used to track
// such information, whereas on ROCm + MIOpen there is no guarantee to getting
// one without scratch memory, and scratch_size field is used to track it.
pub struct AlgorithmConfig {
  algorithm: Option<AlgoriithmDesc>,
  algorithm_no_scratch: Option<AlgoriithmDesc>,
  scratch_size: Option<usize>,
}

impl AlgorithmConfig {
  pub fn new(
    algorithm: AlgoriithmDesc,
    algorithm_no_scratch: AlgoriithmDesc,
    scratch_size: usize) -> Self
  {
    AlgorithmConfig {
      algorithm: Some(algorithm),
      algorithm_no_scratch: Some(algorithm_no_scratch),
      scratch_size: Some(scratch_size)
    }
  }

  pub fn algorithm(&self) -> &Option<AlgoriithmDesc> {
    &self.algorithm
  }

  pub fn set_algorithm(&mut self, value: AlgoriithmDesc) {
    self.algorithm = Some(value);
  }

  pub fn algorithm_no_scratch(&self) -> &Option<AlgoriithmDesc> {
    &self.algorithm_no_scratch
  }

  pub fn set_algorithm_no_scratch(&mut self, value: AlgoriithmDesc) {
    self.algorithm_no_scratch = Some(value);
  }

  pub fn scratch_size(&self) -> &Option<usize> {
    &self.scratch_size
  }

  pub fn set_scratch_size(&mut self, value: usize) {
    self.scratch_size = Some(value);
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}

// Describes a local response normalization (LRN). LRN is used e.g. in
// dist_belief.
//
// Let V be the vector of feature maps at some (batch, y, x)
// coordinate. LRN applies independently to each vector V in the
// input, across all coordinates (batch, y, x), by mapping each V to
// another vector U of the same size using the formula
//
//   U_i = V_i / ((bias + alpha * (sum_j V_j^2)) ^ beta)
//
// where the sum is taken over j in the closed range [i - range, i + range].
//
// When calculating U_i the j in the sum can extend beyond the bounds
// of V. If wrap_around is true, then V_j = V_{j mod F} where F is the
// size of V, which is the number of feature maps. If wrap_around is
// false, then V_j = 0 for j outside [0, F-1].
//
// If segment_size <= F, where F is the number of feature_maps, then
// segment_size has no effect. Otherwise, each consecutive segment of
// segment_size entries in V are normalized separately.
//
// Not all StreamExecutors allow wrap_around == true or segment_size
// != 64. Some do not implement normalization at all.
pub struct NormalizeDescriptor {
  bias: f64,
  range: i64,
  alpha: f64,
  beta: f64,
  wrap_around: bool,
  segment_size: i64,
}

impl NormalizeDescriptor {
  pub fn new() {}

  pub fn set_bias(&mut self, bias: f64) -> &mut Self {
    self.bias = bias;
    self
  }

  pub fn set_range(&mut self, range: i64) -> &mut Self {
    self.range = range;
    self
  }

  pub fn set_alpha(&mut self, alpha: f64) -> &mut Self {
    self.alpha = alpha;
    self
  }

  pub fn set_beta(&mut self, beta: f64) -> &mut Self {
    self.beta = beta;
    self
  }

  pub fn set_wrap_around(&mut self, wrap_araound: bool) -> &mut Self {
    self.wrap_around = wrap_araound;
    self
  }

  pub fn set_segment_size(&mut self, segment_size: i64) -> &mut Self {
    self.segment_size = segment_size;
    self
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  pub fn blas(&self) -> f64 {
    self.bias
  }

  pub fn range(&self) -> i64 {
    self.range
  }

  pub fn alpha(&self) -> f64 {
    self.alpha
  }

  pub fn beta(&self) -> f64 {
    self.beta
  }

  pub fn wrap_around(&self) -> bool {
    self.wrap_around
  }

  pub fn segment_size(&self) -> i64 {
    self.segment_size
  }
}

pub enum ActivationMode {
  None,
  Sigmoid,
  // Rectified linear activation: f(x) = x < 0 ? 0 : x
  Relu,
  // Rectified linear activation; where upper maximum is 6.0.
  Relu6,
  // Rectified linear activation; where upper maximum specified by
  // BatchDescriptor::value_max().
  ReluX,
  Tanh,
  // Like ReluX; but passes all values in the range [-X,X].
  BandPass,
  // Exponential linear activation: f(x) = x < 0 ? e^x - 1 : x
  Elu,
  // Leaky Rectified linear activation: f(x) = x < 0 ? alpha * x : x
  LeakyRelu,
  // Gaussian Error linear unit activation:
  GeluExact,
}

// Returns a string representation of the given activation mode.
pub fn activation_mode_string(mode: ActivationMode) -> String {
  match mode {
    ActivationMode::None => return "none".to_string(),
    ActivationMode::Sigmoid => return "sigmoid".to_string(),
    ActivationMode::Relu => return "relu".to_string(),
    ActivationMode::Relu6 => return "relu6".to_string(),
    ActivationMode::ReluX => return "reluX".to_string(),
    ActivationMode::Tanh => return "tanh".to_string(),
    ActivationMode::BandPass => return "bandpass".to_string(),
    ActivationMode::Elu => return "elu".to_string(),
    ActivationMode::LeakyRelu => return "leakyrelu".to_string(),
    _ => return "unknown".to_string(),
  }
}

// Describes the operation that DoElementwiseOperation should perform on its
// inputs.
pub enum ElementwiseOperation {
  Add,
  Multiply
}

pub fn elementwise_operation_string(op: &ElementwiseOperation) -> String {
  match op {
    ElementwiseOperation::Add => return "add".to_string(),
    ElementwiseOperation::Multiply => return "multiply".to_string(),
  }
}

// A simple class representing the version of the backing library, to
// workaround the "too perfect forwarding" issue in gcc6+ compilers.
// See PR#16309 and issue #18402 for links discussing the issue.
pub struct VersionInfo {
  major: i64,
  minor: i64,
  patch: i64,
}

impl VersionInfo {
  pub fn new(major: i64, minor: i64, patch: i64) -> Self {
    VersionInfo { major: major, minor: minor, patch: patch }
  }

  pub fn major_version(&self) -> i64 {
    self.major
  }

  pub fn minor_version(&self) -> i64 {
    self.minor
  }

  pub fn patch(&self) -> i64 {
    self.patch
  }

  pub fn as_tuple(&self) -> (i64, i64, i64) {
    (self.major, self.minor, self.patch)
  }
}

pub struct DnnGraph {}

impl DnnGraph {
  pub fn new() {}
  pub fn prepare() {}
  pub fn build() {}
  pub fn execute() {}
}

// Suite of operations typically used for implementing Deep/Convolutional Neural
// Nets. Note: A false return value of an operation indicates the
// implementation is not available.
pub struct DnnSupport {}

impl DnnSupport {
  pub fn new() {}
  pub fn init() {}
  pub fn get_version() {}
  pub fn do_batch_normalization_forward() {}
  pub fn do_batch_normalization_backward() {}
  pub fn do_fused_convolve() {}
  pub fn fused_convolve_with_algorithm() {}
  pub fn prepare_for_convolution() {}
  pub fn cudnn_reorder_convolution_filter_and_bias() {}
  pub fn do_convolve() {}
  pub fn convolve_with_algorithm() {}
  pub fn get_convolve_runners() {}
  pub fn convolve_runner_from_desc() {}
  pub fn get_graph_convolve_runners() {}
  pub fn graph_convolve_runner_from_desc() {}
  pub fn get_fused_convolve_runners() {}
  pub fn get_fused_matmul_runners() {}
  pub fn fused_convolve_runner_from_desc() {}
  pub fn norm_runner_from_desc() {}
  pub fn deserialize_graph() {}
  pub fn fused_mha_runner_from_desc() {}
  pub fn fused_mha_backward_runner_from_desc() {}
  pub fn get_mo_open_convolve_algorithms() {}
  pub fn get_rnn_algorithms() {}
  pub fn pool_forward() {}
  pub fn pool_backward() {}
  pub fn do_pool_forward() {}
  pub fn do_pool_backward() {}
  pub fn do_normalize_with_dimensions() {}
  pub fn do_normalize_backward_with_dimensions() {}
  pub fn create_rnn_descriptor() {}
  pub fn create_rnn_sequence_tensor_descriptor() {}
  pub fn create_rnn_state_tensor_descriptor() {}
  pub fn do_rnn_forward() {}
  pub fn do_rnn_backward() {}
  pub fn prepare_for_ctc_loss() {}
  pub fn do_ctc_loss() {}
  pub fn do_transform_tensor() {}
  pub fn notify_stream_destroyed() {}

  fn do_prepare_for_convolution() {}
  fn fo_prepare_for_stc_loss() {}
}