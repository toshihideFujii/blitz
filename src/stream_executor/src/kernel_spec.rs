#![allow(dead_code)]

pub struct KernelLoaderSpec {}

impl KernelLoaderSpec {
  pub fn new() {}
  pub fn kernel_name() {}
}

pub struct InProcessSymbol {}

pub struct CudaPtxInMemory {}

impl CudaPtxInMemory {
  pub fn new() {}
  pub fn add_spec() {}
  pub fn default_text() {}
  pub fn original_default_text() {}
  pub fn text() {}
  pub fn original_text() {}
  pub fn decompress_ptx() {}
}

pub struct CudaCubinInMemory {}

// Describes how to load a kernel on any subset of a number of target platforms.
pub struct MultiKernelLoaderSpec {
  in_process_symbol: Option<InProcessSymbol>,
  cuda_cubin_in_memory: Option<CudaCubinInMemory>,
  cuda_ptx_in_memory: Option<CudaPtxInMemory>,
  arity: usize,
}

impl MultiKernelLoaderSpec {
  pub fn new() {}

  // Returns the number of arguments that this kernel accepts.
  pub fn arity(&self) -> usize {
    self.arity
  }

  pub fn has_in_process_symbol(&self) -> bool {
    self.in_process_symbol.is_some()
  }

  pub fn has_cuda_cubin_in_memory(&self) -> bool {
    self.cuda_cubin_in_memory.is_some()
  }

  pub fn has_cuda_ptx_in_memory(&self) -> bool {
    self.cuda_ptx_in_memory.is_some()
  }

  // Accessors for platform variant kernel load specifications.
  pub fn in_process_symbol(&self) -> &InProcessSymbol {
    assert!(self.has_in_process_symbol());
    self.in_process_symbol.as_ref().unwrap()
  }

  pub fn cuda_cubin_in_memory(&self) -> &CudaCubinInMemory {
    assert!(self.has_cuda_cubin_in_memory());
    self.cuda_cubin_in_memory.as_ref().unwrap()
  }

  pub fn cuda_ptx_in_memory(&self) -> &CudaPtxInMemory {
    assert!(self.has_cuda_ptx_in_memory());
    self.cuda_ptx_in_memory.as_ref().unwrap()
  }

  pub fn add_in_process_symbol() {}
  pub fn add_cuda_cubin_in_memory() {}
  pub fn add_cuda_ptx_in_memory() {}

  pub fn kernel_args_packing() {}
}