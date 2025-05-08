
use common::{array::Array, array2d::Array2D, printer::{append_join, Printer, StringPrinter}, util::product};

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
    dims: &Vec<i64>,
    reshape_dims: &Vec<i64>,
    transpose_perm: &Vec<i64>) -> Self
  {
    let mut instance = IotaTileAssignment {
      ndims: dims.len() as i64,
      reshape_ndims: reshape_dims.len() as i64,
      dims: Vec::new(),
      reshape_dims: Vec::new(),
      transpose_perm: Vec::new(),
      storage: Vec::new(),
    };
    instance.dims.clone_from(dims);
    instance.reshape_dims.clone_from(reshape_dims);
    instance.transpose_perm.clone_from(transpose_perm);
    instance
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
    let reshape_dims = self.reshape_dims();
    let transpose_perm = self.transpose_perm();
    let mut reshape_index = vec![0; self.reshape_ndims as usize];
    for i in (0..self.reshape_ndims).rev() {
      let dim = transpose_perm[i as usize];
      let dim_size = reshape_dims[dim as usize];
      reshape_index[dim as usize] = linear_index % dim_size;
      linear_index /= dim_size;
    }
    let mut value = reshape_index[0];
    for i in 1..self.reshape_ndims {
      value *= reshape_dims[i as usize];
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
    if self.dims.is_empty() {
      return 0;
    }
    let mut num_elements = 1;
    for dim in self.dims() {
      num_elements *= dim;
    }
    num_elements
  }

  pub fn transpose(&self, _perm: &Vec<i64>) -> Option<IotaTileAssignment> {
    None
  }

  pub fn print(&self, printer: &mut dyn Printer) {
    printer.append(&"[".to_string());
    append_join(printer, self.dims(), ",".to_string());
    printer.append(&"]<=[".to_string());
    append_join(printer, self.reshape_dims(), ",".to_string());
    printer.append(&"]".to_string());
    if self.reshape_dims.len() > 1 {
      printer.append(&"T(".to_string());
      append_join(printer, self.transpose_perm(), ",".to_string());
      printer.append(&")".to_string());
    }
  }

  pub fn to_string(&self) -> String {
    let mut printer = StringPrinter::new();
    self.print(&mut printer);
    printer.to_string()
  }

  // Materializes array representation of IotaTileAssignment.
  pub fn to_array(&self) -> Array {
    let mut reshape_dims = vec![];
    reshape_dims.clone_from(self.reshape_dims());
    let mut array = Array::new(reshape_dims);
    array.fill_iota(0);
    //array.transpose_dimensions(self.transpose_perm());
    //array.reshape(self.dims());
    array
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
  array: Option<Array>,
  array_2d: Option<Array2D<i64>>
}

impl TileAssignment {
  pub fn default() -> Self {
    TileAssignment {
      iota: None,
      array: Some(TileAssignment::replicated_array()),
      array_2d: None
    }
  }

  pub fn new_from_vec(dims: &Vec<i64>) -> Self {
    TileAssignment {
      iota: Some(IotaTileAssignment::create(dims)),
      array: None,
      array_2d: None
    }
  }

  pub fn new_from_array(array: Array) -> Self {
    TileAssignment {
      iota: None,
      array: Some(array),
      array_2d: None
    }
  }

  pub fn new_from_array_2d(array_2d: Array2D<i64>) -> Self {
    TileAssignment {
      iota: None, 
      array: None,
      array_2d: Some(array_2d)
    }
  }

  pub fn new_from_device_id(device_id: i64) -> Self {
    let array = Array::new_fill(vec![1], device_id);
    TileAssignment::new_from_array(array)
  }

  pub fn new_from_iota(iota: IotaTileAssignment) -> Self {
    TileAssignment {
      iota: Some(iota),
      array: None,
      array_2d: None
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
      self.array.as_ref().unwrap().values()[0]
    } else {
      0
    }
  }

  #[allow(dead_code)]
  fn value_at(&self, pos: &Vec<i64>) -> i64 {
    if self.array.is_some() {
      return self.array.as_ref().unwrap().value_at(pos);
    } else {
      assert!(self.iota.is_some());
      return self.iota.as_ref().unwrap().value_at(pos);   
    }
  }

  pub fn each<F>(&mut self, func: &mut F)
    where F: FnMut(&Vec<i64>, &mut i64)
  {
    self.maybe_materializa_full_array();
    self.array.as_mut().unwrap().each(func);
  }

  pub fn each_status() {}

  // Returns a tile assignment reshaped to the given dimensions.
  // REQUIRES: new shape has the same number of elements.
  pub fn reshape(&self, new_dimensions: &Vec<i64>) -> Self {
    if self.iota.is_some() {
      assert_eq!(TileAssignment::product(new_dimensions),
        self.iota.as_ref().unwrap().num_elements());
      let iota = IotaTileAssignment::new_detail(new_dimensions,
          self.iota.as_ref().unwrap().reshape_dims(),
          self.iota.as_ref().unwrap().transpose_perm());
      return TileAssignment::new_from_iota(iota);
    }
    assert!(self.array.is_some());
    let mut reshaped = self.array.as_ref().unwrap().clone();
    reshaped.reshape(new_dimensions);
    TileAssignment::new_from_array(reshaped)
  }

  fn product(dims: &Vec<i64>) -> i64 {
    if dims.is_empty() { return 0; }
    let mut product = 1;
    for dim in dims {
      product *= dim;
    }
    product
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
      append_join(printer, self.array().dimensions(), ",".to_string());
      printer.append(&"]".to_string());
      //append_join(printer, self.array().values(), ",".to_string());
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
      if self.array.is_some() {
        for i in self.array.as_ref().unwrap().values() {
          if device == *i { return true; }
        }
      }
      return false;
    }
  }

  // Returns non-nullopt iota tile assignment iff it holds that format.
  pub fn iota(&self) -> &Option<IotaTileAssignment> {
    &self.iota
  }

  // Returns reference to the full array representation. If it holds iota
  // format, reference to a lazily materialized array is returned.
  pub fn array(&self) -> &Array {
    self.array.as_ref().unwrap()
  }

  fn replicated_array() -> Array {
    Array::new(vec![0])
  }

  fn maybe_materializa_full_array(&mut self) {
    if self.array.is_none() {
      assert!(self.iota.is_some());
      self.array = Some(self.iota.as_ref().unwrap().to_array());
    }
  }
}

#[cfg(test)]
mod tests {
  use  super::*;

  fn to_vector_using_each(tile: &mut TileAssignment) -> Vec<i64> {
    let mut result = vec![];
    let mut func = |_index: &Vec<i64>, device: &mut i64| {
      result.push(*device);
    };
    tile.each(&mut func);
    result
  }

  #[test]
  fn test_replicated() {
    let tile = TileAssignment::default();
    assert_eq!(tile.num_dimensions(), 1);
    assert_eq!(tile.dim(0), 0);
  }

  #[test]
  fn test_maximal() {
    let mut tile = TileAssignment::new_from_device_id(5);
    assert_eq!(tile.num_dimensions(), 1);
    assert_eq!(tile.dim(0), 1);
    assert_eq!(tile.value_at(&vec![0]), 5);
    assert_eq!(tile.iota(), &None);
    assert_eq!(tile.uses_device(5), true);
    assert_eq!(tile.first(), 5);
    assert_eq!(tile.uses_device(0), false);
    assert_eq!(to_vector_using_each(&mut tile), vec![5]);
  }

  #[test]
  fn test_trivial_iota_tile() {
    let mut tile = TileAssignment::new_from_vec(&vec![4, 4, 2]);
    assert_eq!(tile.to_string(), "devices=[4,4,2]<=[32]");

    let tile2 = TileAssignment::new_from_vec(&vec![4, 4, 2]);
    assert_eq!(&tile, &tile2);
    assert_eq!(tile.num_dimensions(), 3);
    assert_eq!(tile.dim(0), 4);
    assert_eq!(tile.dim(1), 4);
    assert_eq!(tile.dim(2), 2);
    assert_eq!(tile.value_at(&vec![0, 0, 0]), 0);
    assert_eq!(tile.value_at(&vec![3, 2, 1]), 29);
    
    assert_eq!(tile.uses_device(0), true);
    assert_eq!(tile.uses_device(31), true);
    assert_eq!(tile.uses_device(32), false);
    assert_eq!(to_vector_using_each(&mut tile),
      vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
  }

  #[test]
  fn testreshape_trivial_iota_tile() {
    let tile = TileAssignment::new_from_vec(&vec![4, 4, 2]);
    let mut reshaped = tile.reshape(&vec![2, 8, 2]);
    assert_ne!(tile, reshaped);
    assert_eq!(reshaped, TileAssignment::new_from_vec(&vec![2, 8, 2]));
    assert_eq!(reshaped.num_dimensions(), 3);
    assert_eq!(reshaped.dim(0), 2);
    assert_eq!(reshaped.dim(1), 8);
    assert_eq!(reshaped.dim(2), 2);
    assert_eq!(reshaped.value_at(&vec![0, 0, 0]), 0);
    assert_eq!(reshaped.value_at(&vec![1, 3, 1]), 23);
    assert_eq!(reshaped.uses_device(0), true);
    assert_eq!(reshaped.uses_device(31), true);
    assert_eq!(reshaped.uses_device(32), false);
    assert_eq!(to_vector_using_each(&mut reshaped),
      vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
  }
}