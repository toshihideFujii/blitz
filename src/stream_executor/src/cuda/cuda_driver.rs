#![allow(dead_code)]

// CUDAContext wraps a cuda CUcontext handle, and includes a unique id.
pub struct GpuContext {
  id: i64
}

impl GpuContext {
  pub fn new() {}
  pub fn context() {}
  pub fn id() {}
}

// Manages the singleton map of contexts that we've created, mapping from
// the CUcontext to the GpuContext that we pass around internally.
pub struct CreatedContexts {}

impl CreatedContexts {
  pub fn new() {}
}