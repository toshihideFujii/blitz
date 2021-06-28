pub struct JSArrayBuffer {}

impl JSArrayBuffer {
  // Length in bytes.
  pub fn byte_length() {}

  // Backing memory for this array.
  pub fn backing_store() {}
  pub fn set_backing_store() {}

  // Extension object used for GC.
  pub fn extension() {}

  // For non-wasm, allocation_length and allocation_base are byte_length
  // and backing_store, respectively.
  pub fn allocation_length() {}
  pub fn allocation_base() {}

  // Boolean flags.
  pub fn bit_field() {}

  // Clear uninitialized padding space.
  // This ensures that the snapshot content is deterministic.
  // Depending on the blitz build mode there could be no padding.
  pub fn clear_padding() {}
}
