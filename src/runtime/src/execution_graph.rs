#![allow(dead_code)]

// Execution graph defines the execution order of operations based on their
// buffer use and resource use dependencies.
//
// In Blitz:GPU and Blitz:CPU we compile HLO programs to a sequence of operations
// executed on the underlying device. Blitz compiler creates a sequential schedule
// that is used to assign buffers to operations. These operation can be
// implemented as thunks, or as commands (only on Blitz:GPU backend with CUDA
// graphs). Each operations reads and writes from/to buffer slices and uses
// resources (i.e. collective communicator).
//
// At run time we can relax sequential schedule and execute operations
// concurrently, as long as we don't create data races (reading and writing
// from/to the same or overlapping buffer slices concurrently), or resource
// races (using the same mutable resource concurrently).
//
// Resources can behave as buffers and require an execution order (operation
// must wait for the completion of execution of all dependencies), or as a
// scheduling barrier (operation must wait for the completion of scheduling of
// all dependencies). See more details in the `NodeEdge::Kind` definition.
//
// We use buffer and resource use conflicts to define an execution order of
// operations as a directed acyclic graph (DAG) that satisfies all dependencies.
//
// Backend-specific runtime relies on the execution graph to execute operations
// concurrently usult the underlying device concurrency mechanism, e.g.
// thread pools on CPU device, or CUDA streams on NVIDIA GPU device.

pub struct ExecutionGraph {}

impl ExecutionGraph {
    
}

pub struct Operation {}

impl Operation {
  pub fn default() -> Self {
    Operation {  }
  }
}