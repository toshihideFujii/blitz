use crate::array2d::Array2D;

pub struct Array4D<T> {
  values: Vec<Vec<Vec<Vec<T>>>>
}

impl<T> Array4D<T> where  T: Default + Clone {

  // Creates a 4D array, uninitialized values.
  pub fn new(n1: usize, n2: usize, n3: usize, n4: usize) -> Self {
    let mut instance = Array4D { values: Vec::new() };
    instance.values.resize(n1,
      vec![vec![vec![T::default(); n4]; n3]; n2]);
    for i in 0..n1 {
      let vec2 = &mut instance.values[i];
      vec2.resize(n2, vec![vec![T::default(); n4]; n3]);
      for j in 0..n2 {
        let vec3 = &mut instance.values[i][j];
        vec3.resize(n3, vec![T::default(); n4]);
        for k in 0..n3 {
          let vec4 = &mut instance.values[i][j][k];
          vec4.resize(n4, T::default());
        }
      }
    }
    instance
  }

  // Creates a 4D array, initialized to value.
  pub fn new_fill_value(n1: usize, n2: usize, n3: usize, n4: usize, value: T) -> Self {
    let mut instance = Array4D::new(n1, n2, n3, n4);
    instance.fill(value);
    instance
  }

  pub fn new_from(values: Vec<Vec<Vec<Vec<T>>>>) -> Self {
    Array4D { values: values }
  }

  pub fn n1(&self) -> usize {
    self.values.len()
  }

  pub fn n2(&self) -> usize {
    if self.values.len() == 0 { return 0; }
    self.values[0].len()
  }

  pub fn n3(&self) -> usize {
    if self.values.len() == 0  || self.values[0].len() == 0 { return 0; }
    self.values[0][0].len()
  }

  pub fn n4(&self) -> usize {
    if self.values.len() == 0  ||
      self.values[0].len() == 0 ||
      self.values[0][0].len() == 0 { return 0; }

    self.values[0][0][0].len()
  }

  pub fn width(&self) -> usize {
    self.n4()
  }

  pub fn height(&self) -> usize {
    self.n3()
  }

  pub fn depth(&self) -> usize {
    self.n2()
  }

  pub fn planes(&self) -> usize {
    self.n1()
  }

  pub fn num_elements(&self) -> usize {
    self.n1() * self.n2() * self.n3() * self.n4()
  }

  pub fn data(&self, n1: usize, n2: usize, n3: usize, n4: usize) -> &T {
    &self.values[n1][n2][n3][n4]
  }

  pub fn set_data(&mut self, n1: usize, n2: usize, n3: usize, n4: usize, data: T) {
    self.values[n1][n2][n3][n4] = data;
  }

  pub fn each(&self, f: &dyn Fn(&T, Vec<usize>)) {
    for n1 in 0..self.n1() {
      for n2 in 0..self.n2() {
        for n3 in 0..self.n3() {
          for n4 in 0..self.n4() {
            f(&self.values[n1][n2][n3][n4], vec![n1, n2, n3, n4]);
          }
        }
      }
    }
  }

  pub fn fill(&mut self, value: T) {
    for n1 in 0..self.n1() {
      for n2 in 0..self.n2() {
        for n3 in 0..self.n3() {
          for n4 in 0..self.n4() {
            self.values[n1][n2][n3][n4] = value.clone();
          }
        }
      }
    }
  }

  // Fills the array with the sequence i*multiplier for i=0,1,...
  pub fn fill_with_multiples(&mut self, multiplier: T)
    where T: std::ops::Mul<Output = T>
  {
    for n1 in 0..self.n1() {
      for n2 in 0..self.n2() {
        for n3 in 0..self.n3() {
          for n4 in 0..self.n4() {
            self.values[n1][n2][n3][n4] = 
              self.values[n1][n2][n3][n4].clone() * multiplier.clone();
          }
        }
      }
    }
  }

  // Fills all of the {p,z} with the array provided, which specifies {y,x}.
  pub fn fill_with_yx(&mut self, value: &Array2D<T>) {
    assert_eq!(value.height(), self.height());
    assert_eq!(value.width(), self.width());
    for plane in 0..self.planes() {
      for depth in 0..self.depth() {
        for height in 0..self.height() {
          for width in 0..self.width() {
            self.values[plane][depth][height][width] =
              value.data(height, width).clone();
          }
        }
      }
    }
  }

  // Fills all of the {p,x} with the array provided, which specifies {z,y}.
  pub fn fill_with_zy(&mut self, value: &Array2D<T>) {
    assert_eq!(value.height(), self.depth());
    assert_eq!(value.width(), self.height());
    for plane in 0..self.planes() {
      for depth in 0..self.depth() {
        for height in 0..self.height() {
          for width in 0..self.width() {
            self.values[plane][depth][height][width] =
              value.data(depth, height).clone();
          }
        }
      }
    }
  }

  // Fills all of the {x,y} with the array provided, which specifies {p,z}.
  pub fn fill_with_pz(&mut self, value: &Array2D<T>) {
    assert_eq!(value.height(), self.planes());
    assert_eq!(value.width(), self.depth());
    for height in 0..self.height() {
      for width in 0..self.width() {
        for plane in 0..self.planes() {
          for depth in 0..self.depth() {
            self.values[plane][depth][height][width] =
              value.data(plane, depth).clone();
          }
        }
      }
    }
  }
  
  // Fills each of the minor-dim matrices with a number designating which minor
  // dim matrix is enclosed by the shape.
  /*
  pub fn fill_with_minor_dim_num(&mut self) {
    for height in 0..self.height() {
      for width in 0..self.width() {
        for plane in 0..self.planes() {
          for depth in 0..self.depth() {
            self.values[plane][depth][height][width] =
              plane * self.depth() + depth;
          }
        }
      }
    }
  }
  */
}

#[cfg(test)]
mod tests {

  use super::*;

  /*
  fn array_4d_linear_index<T>(arr: &Array4D<T>, idx: Vec<usize>) -> usize
    where T: Default + Clone
  {
    assert_eq!(idx.len(), 4);
    idx[3] +
    idx[2] * arr.n4() +
    idx[1] * arr.n3() * arr.n4() +
    idx[0] * arr.n2() * arr.n3() * arr.n4()
  }
  */

  #[test]
  fn test_uninitialized_dims_ctor() {
    let empty: Array4D<i64> = Array4D::new(2, 3, 4, 5);
    assert_eq!(empty.n1(), 2);
    assert_eq!(empty.n2(), 3);
    assert_eq!(empty.n3(), 4);
    assert_eq!(empty.n4(), 5);
    assert_eq!(empty.num_elements(), 120);
  }

  #[test]
  fn test_full_ctor() {
    let full_of_7 = Array4D::new_fill_value(2, 3, 4, 5, 7);
    assert_eq!(full_of_7.n1(), 2);
    assert_eq!(full_of_7.n2(), 3);
    assert_eq!(full_of_7.n3(), 4);
    assert_eq!(full_of_7.n4(), 5);

    let f = |value: &i32, _idx: Vec<usize>| {
      assert_eq!(*value, 7);
    };
    full_of_7.each(&f);
  }

  //fn test_container_ctor() {}

  #[test]
  fn test_initializer_list_ctor() {
    let values = vec![
        vec![vec![vec![1], vec![2]], vec![vec![3], vec![4]],
          vec![vec![5], vec![6]], vec![vec![7], vec![8]]],
        vec![vec![vec![9], vec![10]], vec![vec![11], vec![12]],
          vec![vec![13], vec![14]], vec![vec![15], vec![16]]],
        vec![vec![vec![17], vec![18]], vec![vec![19], vec![20]],
          vec![vec![21], vec![22]], vec![vec![23], vec![24]]],
      ];
    let arr = Array4D::new_from(values);

    assert_eq!(arr.n1(), 3);
    assert_eq!(arr.n2(), 4);
    assert_eq!(arr.n3(), 2);
    assert_eq!(arr.n4(), 1);
    assert_eq!(arr.num_elements(), 24);

    assert_eq!(arr.data(0, 0, 0, 0), &1);
    assert_eq!(arr.data(0, 0, 1, 0), &2);
    assert_eq!(arr.data(0, 1, 0, 0), &3);
    assert_eq!(arr.data(0, 3, 1, 0), &8);
    assert_eq!(arr.data(1, 0, 0, 0), &9);
    assert_eq!(arr.data(1, 1, 1, 0), &12);
    assert_eq!(arr.data(2, 0, 0, 0), &17);
    assert_eq!(arr.data(2, 1, 1, 0), &20);
    assert_eq!(arr.data(2, 2, 0, 0), &21);
    assert_eq!(arr.data(2, 3, 1, 0), &24);
  }

  #[test]
  fn test_initializer_list_ctor_f64() {
    let values = vec![
      vec![vec![vec![1.0], vec![2.0]], vec![vec![3.0], vec![4.0]],
        vec![vec![5.0], vec![6.0]], vec![vec![7.0], vec![8.0]]],
      vec![vec![vec![9.0], vec![10.0]], vec![vec![11.0], vec![12.0]],
        vec![vec![13.0], vec![14.0]], vec![vec![15.0], vec![16.0]]],
      vec![vec![vec![17.0], vec![18.0]], vec![vec![19.0], vec![20.0]],
        vec![vec![21.0], vec![22.0]], vec![vec![23.0], vec![24.0]]],
    ];
    let arr = Array4D::new_from(values);

    assert_eq!(arr.n1(), 3);
    assert_eq!(arr.n2(), 4);
    assert_eq!(arr.n3(), 2);
    assert_eq!(arr.n4(), 1);
    assert_eq!(arr.num_elements(), 24);

    assert_eq!(arr.data(0, 0, 0, 0), &1.0);
    assert_eq!(arr.data(0, 0, 1, 0), &2.0);
    assert_eq!(arr.data(0, 1, 0, 0), &3.0);
    assert_eq!(arr.data(0, 3, 1, 0), &8.0);
    assert_eq!(arr.data(1, 0, 0, 0), &9.0);
    assert_eq!(arr.data(1, 1, 1, 0), &12.0);
    assert_eq!(arr.data(2, 0, 0, 0), &17.0);
    assert_eq!(arr.data(2, 1, 1, 0), &20.0);
    assert_eq!(arr.data(2, 2, 0, 0), &21.0);
    assert_eq!(arr.data(2, 3, 1, 0), &24.0);
  }

  #[test]
  fn test_fill() {
    let mut full_of_7 =
      Array4D::new_fill_value(2, 3, 4, 5, 7);
    let f = |cell: &i32, _idx: Vec<usize>| {
      assert_eq!(*cell, 7);
    };
    full_of_7.each(&f);

    full_of_7.fill(11);
    let g = |cell: &i32, _idx: Vec<usize>| {
      assert_eq!(*cell, 11);
    };
    full_of_7.each(&g);
  }

  #[test]
  fn test_fill_with_multiples() {
    let mut arr: Array4D<f64> =
      Array4D::new_fill_value(2, 3, 4, 5, 7.0);
    arr.fill_with_multiples(2.0);

    let f = |cell: &f64, _idx: Vec<usize>| {
      //assert_eq!(*cell, 2.0 * (array_4d_linear_index(&arr, idx) as f64));
      assert_eq!(*cell, 14.0);
    };
    arr.each(&f);
  }

  #[test]
  fn test_fill_with_pz_test_depth_one() {
    let mut matrix: Array2D<f64> = Array2D::new(3, 2);
    let values = vec![vec![-3.0, -0.1], vec![0.0, -0.1], vec![3.0, 0.2]];

    let mut rowno = 0;
    for row in &values {
      let mut colno = 0;
      for f in row {
        matrix.set_data(rowno, colno, *f);
        colno += 1;
      }
      rowno += 1;
    }

    let mut actual: Array4D<f64> = Array4D::new(3, 2, 1, 1);
    actual.fill_with_pz(&matrix);
    assert_eq!(*actual.data(0, 0, 0, 0), -3.0);
    assert_eq!(*actual.data(0, 1, 0, 0), -0.1);

    assert_eq!(*actual.data(1, 0, 0, 0), 0.0);
    assert_eq!(*actual.data(1, 1, 0, 0), -0.1);

    assert_eq!(*actual.data(2, 0, 0, 0), 3.0);
    assert_eq!(*actual.data(2, 1, 0, 0), 0.2);
  }
}