#![allow(dead_code)]

use ffi::api::ffi::DataType;

// Specifies an index to use when accessing specific spatial dimensions.
pub enum DimIndex {
  X,
  Y,
  Z,
}

// Return a reordered dims.
pub fn reorder_dims(_input: &Vec<i64>) {
  unimplemented!()
}

// Helper functions to make methods more readable.
pub fn get_dim(_data: &Vec<i64>, _dim: DimIndex) -> i64 {
  unimplemented!()
}

pub fn set_dim(_data: &Vec<i64>, _dim: DimIndex, _value: i64) {
  unimplemented!()
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
pub fn data_layout_string() -> String {
  unimplemented!()
}

// Specifies a quantization for activations in a given BatchDescriptor.
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

pub struct TensorDiscriptor {
  d_type: DataType,
  dimensions: Vec<i64>,
  minor_to_major: Vec<i64>,
}

impl TensorDiscriptor {
  pub fn new(d_type: DataType, dimensions: Vec<i64>, minor_to_major: Vec<i64>) -> Self {
    TensorDiscriptor {
      d_type: d_type,
      dimensions: dimensions,
      minor_to_major: minor_to_major
    }
  }

  pub fn get_physical_dimensions_major_to_minor() {}
  pub fn get_physical_strides_major_to_minor() {}
  pub fn get_logical_strides() {}

  pub fn for_() {}
  pub fn ndims(&self) {}

  pub fn dimensions(&self) -> &Vec<i64> {
    &self.dimensions
  }

  pub fn minor_to_major(&self) -> &Vec<i64> {
    &self.minor_to_major
  }

  pub fn data_type(&self) -> &DataType {
    &self.d_type
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
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

  pub fn get_non_contracting_dims() {}
  pub fn get_cudnn_compatible_dimensions() {}
  pub fn get_cudnn_compatible_strides() {}
  pub fn make_cudnn_compatible() {}

  pub fn for_() {}

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
    unimplemented!()
  }
}

struct ParamsRegion {
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
  pub fn new() {}

  pub fn params_size_in_bytes(&self) -> i64 { -1 }

  pub fn params_weight_regions(&self) {}
  pub fn params_bias_regions(&self) {}
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
pub struct BatchDescriptor {
  tensor: TensorDiscriptor,
  value_max: i64,
  value_min: i64,
}

impl BatchDescriptor {
  pub fn new() {}
  pub fn clone_from() {}
  pub fn to_string() {}
  pub fn to_short_string() {}
  pub fn count() {}
  pub fn feature_map_count() {}
  pub fn height() {}
  pub fn width() {}
  pub fn spatial_dim() {}
  pub fn ndims() {}
  pub fn value_max() {}
  pub fn value_min() {}
  pub fn layout() {}
  pub fn quantized_activation_mode() {}
  pub fn full_dims() {}
  pub fn full_strides() {}
  pub fn vectorized_dims() {}
  pub fn vectorized_strides() {}
  pub fn set_count() {}
  pub fn set_feature_map_count() {}
  pub fn set_height() {}
  pub fn set_width() {}
  pub fn set_spatial_dim() {}
  pub fn set_value_max() {}
  pub fn set_value_min() {}
  pub fn set_layout() {}
  pub fn set_quantized_activation_mode() {}
  pub fn nodes_per_feature_map() {}
  pub fn nodes_across_feature_maps() {}
  pub fn element_count() {}
  pub fn fully_connected_weight_count() {}
  pub fn fully_connected_bias_count() {}
  pub fn depth_concatenate_output_descriptor() {}

  fn spatial_size() {}
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
pub struct FilterDescriptor {
  tensor: TensorDiscriptor
}

impl FilterDescriptor {
  pub fn new() {}
  pub fn set_output_feature_map_count() {}
  pub fn set_input_feature_map_count() {}
  pub fn set_input_filter_height() {}
  pub fn set_input_filter_width() {}
  pub fn set_layout() {}
  pub fn set_spatial_dim() {}
  pub fn ndims() {}
  pub fn clone_from() {}
  pub fn to_string() {}
  pub fn to_short_string() {}
  pub fn compute_weight_count() {}
  pub fn bias_count() {}
  pub fn output_feature_map_count() {}
  pub fn input_feature_map_count() {}
  pub fn input_filter_height() {}
  pub fn input_filter_width() {}
  pub fn input_filter_dim() {}
  pub fn layout() {}
  pub fn input_filter_dims() {}
  pub fn full_dims() {}
  pub fn full_strides() {}
  pub fn vectorized_dims() {}
  pub fn vectorized_strides() {}
}

// Describes how padding should be aligned when the total number of pad
// elements is odd.
pub enum PubAlignment {
  Default,
  CudnnPadding,
  TensorFlowPadding,
}

// Returns a string representation of the given padding alignment.
pub fn pad_alignment_string() {}

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
pub struct CovolutionDescriptor {}

impl CovolutionDescriptor {
  pub fn new() {}
  pub fn to_string() {}
  pub fn to_short_string() {}
  pub fn set_zero_padding_height() {}
  pub fn set_zero_padding_width() {}
  pub fn set_zero_padding() {}
  pub fn set_vertical_filter_stride() {}
  pub fn set_horizontal_filter_stride() {}
  pub fn set_filter_stride() {}
  pub fn set_vertical_dilation_rate() {}
  pub fn set_horizontal_dilaation_rate() {}
  pub fn set_dilation_rate() {}
  pub fn set_group_count() {}
  pub fn set_convolution_not_crosscorr() {}
  pub fn set_name() {}
  pub fn zero_padding_height() {}
  pub fn zero_padding_width() {}
  pub fn vertical_filter_stride() {}
  pub fn horizontal_filter_stride() {}
  pub fn vertical_dilation_rate() {}
  pub fn horizontal_dilation_rate() {}
  pub fn zero_padding() {}
  pub fn filter_stride() {}
  pub fn dilation_rate() {}
  pub fn pad_slignment() {}
  pub fn group_count() {}
  pub fn ndims() {}
  pub fn convolution_not_crosscorr() {}
  pub fn strides() {}
  pub fn dilations() {}
  pub fn padding() {}
  pub fn name() {}
}

// A patch of values in the input can be pooled via either a max or an average
// operation.
// Specify int64_t so there's no padding in PoolingDescriptor.
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
pub fn short_pooling_mode_string() {}

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
  pub fn set_pooling_mode() {}
  pub fn set_window_height() {}
  pub fn set_window_width() {}
  pub fn set_window() {}
  pub fn set_vertical_padding() {}
  pub fn set_horizontal_padding() {}
  pub fn set_padding() {}
  pub fn set_vertical_stride() {}
  pub fn set_horizontal_stride() {}
  pub fn set_stride() {}
  pub fn set_propagate_nans() {}
  pub fn set_name() {}
  pub fn ndims() {}
  pub fn clone_from() {}
  pub fn to_string() {}
  pub fn to_short_string() {}
  pub fn mode() {}
  pub fn window_height() {}
  pub fn window_width() {}
  pub fn window() {}
  pub fn vertical_padding() {}
  pub fn horizontal_padding() {}
  pub fn padding() {}
  pub fn vertical_stride() {}
  pub fn horizontal_stride() {}
  pub fn stride() {}
  pub fn strides() {}
  pub fn propagate_nans() {}
  pub fn name() {}
}

// Collects parameters for DNN algorithms
pub struct AlgoriithmDesc {}

impl AlgoriithmDesc {
  pub fn new() {}
  pub fn is_cudnn_frontend() {}
  pub fn tensor_ops_enabled() {}
  pub fn workspace_size() {}
  pub fn tuning_knobs() {}
  pub fn hash() {}
  pub fn to_string() {}
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

  pub fn set_bias(&mut self, bias: f64) -> &Self {
    self.bias = bias;
    self
  }

  pub fn set_range(&mut self, range: i64) -> &Self {
    self.range = range;
    self
  }

  pub fn set_alpha(&mut self, alpha: f64) -> &Self {
    self.alpha = alpha;
    self
  }

  pub fn set_beta(&mut self, beta: f64) -> &Self {
    self.beta = beta;
    self
  }

  pub fn set_wrap_around(&mut self, wrap_araound: bool) -> &Self {
    self.wrap_around = wrap_araound;
    self
  }

  pub fn set_segment_size(&mut self, segment_size: i64) -> &Self {
    self.segment_size = segment_size;
    self
  }

  pub fn clone_from() {}

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

// Returns a string representation of the given activation mode.
pub fn activation_mode_string() {}

// Describes the operation that DoElementwiseOperation should perform on its
// inputs.
pub enum ElementwiseOperation {
  Add,
  Multiply
}

pub fn elementwise_operation_string(_op: &ElementwiseOperation) -> String {
  unimplemented!()
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