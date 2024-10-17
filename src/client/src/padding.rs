#![allow(dead_code)]

// Describes the padding applied for a windowed operation like
// convolution, where a window is placed inside a base area.
pub enum Padding {
  // Make the output have the same dimensions as the base area. For
  // example, for a 3x3 base area and a 2x2 window, the output will be
  // 3x3, so that requires padding the 3x3 base area to 4x4.
  Same,
  // Use no padding. For example, for a 4x4 base area and a 2x2
  // window, the output will be 3x3.
  Valid,
}

// Validates that the slices are acceptable for determining padding -- this can
// be used to check the preconditions of MakePadding below to produce an error
// message that can be returned to the user.
pub fn validate_padding_values(
  input_dimensions: &Vec<i64>,
  window_dimensions: &Vec<i64>,
  window_strides: &Vec<i64>) -> Result<(), String>
{
  if !(input_dimensions.len() == window_dimensions.len() &&
    input_dimensions.len() == window_strides.len())
  {
    let mut err_msg = "Want input dimensions size ".to_string();
    err_msg.push_str(&input_dimensions.len().to_string());
    err_msg.push_str(" = window dimensions size ");
    err_msg.push_str(&window_dimensions.len().to_string());
    err_msg.push_str(" = window strides size ");
    err_msg.push_str(&window_strides.len().to_string());
    return Err(err_msg);
  }
  for i in 0..input_dimensions.len() {
    if window_dimensions[i] <= 0 {
      let mut err_msg = "Window dimension ".to_string();
      err_msg.push_str(&i.to_string());
      err_msg.push_str(" has non-positive size ");
      err_msg.push_str(&window_dimensions[i].to_string());
      return Err(err_msg);
    }
    if window_strides[i] <= 0 {
      let mut err_msg = "Window dimension ".to_string();
      err_msg.push_str(&i.to_string());
      err_msg.push_str(" has non-positive size ");
      err_msg.push_str(&window_strides[i].to_string());
      return Err(err_msg);
    }
  }
  Ok(())
}

// Returns the padding needed for the base area, given the base area dimensions,
// window dimensions, strides, and the type of padding.
//
// If v is the returned vector, then for each dimension number i,
// v[i].first is the padding to the left (i.e. in the direction of
// lower indices) and v[i].second is the padding to the right (i.e. in
// the direction of higher indices).
//
// Precondition: The number of dimensions (i.e., rank) in input_dimensions,
// window_dimensions, and strides must match, which is equal to the number
pub fn make_padding(
  input_dimensions: &Vec<i64>,
  window_dimensions: &Vec<i64>,
  window_strides: &Vec<i64>,
  padding: Padding) -> Vec<(i64, i64)>
{
  let valid =
    validate_padding_values(input_dimensions, window_dimensions, window_strides);
  assert!(valid.is_ok());

  let mut low_high_padding = vec![];
  match padding {
    Padding::Valid => {
      low_high_padding.resize(window_dimensions.len(), (0, 0));
      return low_high_padding;
    }
    Padding::Same => {
      // TODO
    }
  }
  
  low_high_padding
}