#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct Dim3D {
  pub x: u64,
  pub y: u64,
  pub z: u64
}

impl Dim3D {
  pub fn new(x: u64, y: u64, z: u64) -> Self {
    Dim3D { x: x, y: y, z: z }
  }
}

// Types to express dimensionality of a kernel launch. Blocks, threads and
// clusters are (up to) 3-dimensional.
//
// See NVIDIA documentation for a thread hierarchy:
// https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#thread-hierarchy
//
// Thread dimensionality for use in a kernel launch.
// details.
pub struct ThreadDim {
  dim_3d: Dim3D
}

impl ThreadDim {
  pub fn new(x: u64, y: u64, z: u64) -> Self {
    ThreadDim { dim_3d: Dim3D::new(x, y, z) }
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}

// Block dimensionality for use in a kernel launch.
// details.
pub struct BlockDim {
  pub dim_3d: Dim3D
}

impl BlockDim {
  pub fn new(x: u64, y: u64, z: u64) -> Self {
    BlockDim { dim_3d: Dim3D::new(x, y, z) }
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}

// Cluster dimensionality for use in a kernel launch.
pub struct ClusterDim {
  dim_3d: Dim3D
}

impl ClusterDim {
  pub fn new(x: u64, y: u64, z: u64) -> Self {
    ClusterDim { dim_3d: Dim3D::new(x, y, z) }
  }

  pub fn to_string() {}
}