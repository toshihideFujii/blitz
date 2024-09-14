#![allow(dead_code)]

// Describes how to load a kernel on a target platform.
//
// This is an abstract base class, subclassed for specific platforms.
// The filename_or_text field represents the program location (i.e. PTX or
// OpenCL loadable translation unit path) and is simply stored; whether it is a
// filename or text is exposed via more specifically named accessors in
// subclasses.
//
// These kernel loader specifications are typically auto-generated into header
// files at build time, but can also be specified manually.
pub struct KernelLoaderSpec {
  kernel_name: String
}

impl KernelLoaderSpec {
  pub fn new(kernel_name: String) -> Self {
    KernelLoaderSpec { kernel_name: kernel_name }
  }

  // Returns the kernel name to load out of the program.
  pub fn kernel_name(&self) -> &String {
    &self.kernel_name
  }
}

// Loads kernel from in process symbol pointer (e.g. pointer to CUDA C++ device
// function).
pub struct InProcessSymbol {
  kernel_loader: KernelLoaderSpec
}

impl InProcessSymbol {
  pub fn new(kernel_name: String) -> Self {
    InProcessSymbol {
      kernel_loader: KernelLoaderSpec::new(kernel_name)
    }
  }

  pub fn symbol() {}
}

// Kernel loader specification for PTX text that resides in memory.
pub struct CudaPtxInMemory {
  kernel_loader: KernelLoaderSpec,
}

impl CudaPtxInMemory {
  pub fn new() {}
  pub fn add_spec() {}
  pub fn default_text() {}
  pub fn text() {}
}

// Kernel loader specification for a CUBIN blob that resides in memory.
pub struct CudaCubinInMemory {
  kernel_loader: KernelLoaderSpec,
  cubin_bytes: Vec<u8>,
}

impl CudaCubinInMemory {
  pub fn new(cubin_bytes: Vec<u8>, kernel_name: String) -> Self {
    CudaCubinInMemory {
      kernel_loader: KernelLoaderSpec::new(kernel_name),
      cubin_bytes: cubin_bytes
    }
  }

  pub fn cubin_bytes(&self) -> &Vec<u8> {
    &self.cubin_bytes
  }
}

pub struct LlvmHostKernel {
  kernel_loader: KernelLoaderSpec,
  ir: String,
  entry_point: String,
  options: Vec<String>
}

impl LlvmHostKernel {
  pub fn new(
    ir: String,
    entry_point: String,
    options: Vec<String>,
    kernel_name: String) -> Self
  {
    LlvmHostKernel {
      kernel_loader: KernelLoaderSpec::new(kernel_name),
      ir: ir,
      entry_point: entry_point,
      options: options
    }
  }

  pub fn ir(&self) -> &String {
    &self.ir
  }

  pub fn entry_point(&self) -> &String {
    &self.entry_point
  }

  pub fn options(&self) -> &Vec<String> {
    &self.options
  }
}

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