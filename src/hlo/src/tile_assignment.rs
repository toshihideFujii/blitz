#![allow(dead_code)]

//use std::mem;
use common::{array::Array, printer::{Printer, StringPrinter}, util::product};

// Describes a TileAssignment with a device array generated from reshaping and
// transposing an iota array, a.k.a. HloShardingV2. This is a more scalable
// format for large number of devices since it does not materialize the full
// list of devices, while being less general since it cannot represent
// arbitrary sequence of devices. It is however sufficient to represent the
// most commonly generated SPMD shardings from ML frameworks that arrange
// devices using mesh axes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IotaTileAssignment {
  ndims: i64,
  reshape_ndims: i64,
  dims: Vec<i64>,
  reshape_dims: Vec<i64>,
  transpose_perm: Vec<i64>,
  storage: Vec<u8>,
}

impl IotaTileAssignment {

  pub fn new_detail(
    _dims: &Vec<i64>,
    _reshape_dims: &Vec<i64>,
    _transpose_perm: &Vec<i64>) -> Self
  {
    unimplemented!()
  }

  pub fn new(ndims: i64, reshape_dims: i64) -> Self {
    IotaTileAssignment {
      ndims: ndims,
      reshape_ndims: reshape_dims,
      dims: Vec::new(),
      reshape_dims: Vec::new(),
      transpose_perm: Vec::new(),
      storage: Vec::new(),
    }
  }

  pub fn create(dims: &Vec<i64>) -> Self {
    IotaTileAssignment::new_detail(
      dims, &vec![product(dims)], &vec![0])
  }

  pub fn value_at(&self, index: &Vec<i64>) -> i64 {
    assert_eq!(index.len(), self.ndims as usize);
    let mut linear_index = index[0];
    for i in 1..self.ndims as usize {
      linear_index *= self.dims()[i];
      linear_index += index[i];
    }

    let reshape_ndims = (self.reshape_ndims - 1) as usize;
    let mut reshape_index = vec![0; self.reshape_ndims as usize];
    for i in reshape_ndims..0 {
      let dim = self.transpose_perm()[i];
      let dim_size = self.reshape_dims()[dim as usize];
      reshape_index[dim as usize] = linear_index % dim_size;
      linear_index /= dim_size;
    }

    let mut value = reshape_index[0];
    for i in 1..self.reshape_ndims {
      value *= self.reshape_dims()[i as usize];
      value += reshape_index[i as usize];
    }
    value
  }

  pub fn ndims(&self) -> usize {
    self.ndims as usize
  }

  pub fn dims(&self) -> &Vec<i64> {
    &self.dims
  }

  pub fn dim(&self, n: i64) -> i64 {
    self.dims[n as usize]
  }

  pub fn reshape_dims(&self) -> &Vec<i64> {
    &self.reshape_dims
  }

  pub fn transpose_perm(&self) -> &Vec<i64> {
    &self.transpose_perm
  }

  pub fn num_elements(&self) -> i64 {
    unimplemented!()
  }

  pub fn transpose(&self, _perm: &Vec<i64>) -> Option<IotaTileAssignment> {
    None
  }

  pub fn print(&self, _printer: &dyn Printer) {
    unimplemented!()
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  // Materializes array representation of IotaTileAssignment.
  pub fn to_array(&self) -> &Array<i64> {
    unimplemented!()
  }
}


// Internal class that represents how an ordered list of device IDs are sharded
// along different dimensions. It manages full or compact representation of the
// device IDs without having callers worry about what underlying format is used.
// This class is meant to be included ONLY by HloSharding so it does not return
// error status on invalid arguments but rather assert preconditions in its
// implementation, assuming it should always get valid data.
// NOTE: This class is immutable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TileAssignment {
  iota: Option<IotaTileAssignment>,
  shared_array: Array<i64>,
  array: Option<Array<i64>>
}

impl TileAssignment {
  pub fn new() -> Self {
    TileAssignment {
      iota: None,
      shared_array: Array::new_i64(vec![]),
      array: None
    }
  }

  pub fn dimensions(&self) -> &Vec<i64> {
    if self.array.is_some() {
      self.array.as_ref().unwrap().dimensions()
    } else {
      self.iota.as_ref().unwrap().dims()
    }
  }

  pub fn num_dimensions(&self) -> usize {
    if self.array.is_some() {
      self.array.as_ref().unwrap().num_dimensions()
    } else {
      self.iota.as_ref().unwrap().ndims()
    }
  }

  pub fn dim(&self, n: i64) -> i64 {
    if self.array.is_some() {
      self.array.as_ref().unwrap().dim(n as usize)
    } else {
      self.iota.as_ref().unwrap().dim(n)
    }
  }

  pub fn num_elements(&self) -> i64 {
    if self.array.is_some() {
      self.array.as_ref().unwrap().num_elements() as i64
    } else {
      self.iota.as_ref().unwrap().num_elements()
    }
  }

  pub fn first(&self) -> i64 {
    if self.array.is_some() {
      -1 // TODO
    } else {
      0
    }
  }

  pub fn each() {}
  pub fn each_status() {}

  // Returns a tile assignment reshaped to the given dimensions.
  // REQUIRES: new shape has the same number of elements.
  pub fn reshape(&self, _new_dimensions: &Vec<i64>) -> Self {
    unimplemented!()
  }

  // Returns a tile assignment transposd using the given dimension permutations.
  // REQUIRES: `perm` must a an array of num_dimensions elements, with unique
  // values within [0, num_dimensions).
  pub fn transpose(&self, _perm: &Vec<i64>) -> Self {
    unimplemented!()
  }

  pub fn print(&self, printer: &mut dyn Printer) {
    if self.iota.is_some() {
      printer.append(&"devices=".to_string());
      self.iota.as_ref().unwrap().print(printer);
    } else {
      printer.append(&"devices=[".to_string());
      //printer::apeend_join(printer, separator);
      printer.append(&"]".to_string());
      // TODO
    }
  }

  pub fn to_string(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print(&mut printer);
    printer.to_string()
  }

  pub fn uses_device(&self, device: i64) -> bool {
    if self.iota.is_some() {
      device < self.iota.as_ref().unwrap().num_elements()
    } else {
      false // TODO
    }
  }

  // Returns non-nullopt iota tile assignment iff it holds that format.
  pub fn iota(&self) -> &Option<IotaTileAssignment> {
    &self.iota
  }

  // Returns reference to the full array representation. If it holds iota
  // format, reference to a lazily materialized array is returned.
  pub fn array(&self) -> &Array<i64> {
    self.array.as_ref().unwrap()
  }

  // Similar to array() but returns the underlying shared_ptr to avoid deep
  // copy.
  pub fn shared_array(&self) -> &Array<i64> {
    &self.shared_array
  }

  // Makes a deep copy of shared_array().
  pub fn shared_array_clone(&self) -> &Array<i64> {
    &self.shared_array
  }
}