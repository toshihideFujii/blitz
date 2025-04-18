use crate::{layout_util::LayoutUtil, shape::Shape, shape_util::ShapeUtil};

// Namespaced collection of (static) utilities related to indexing into
// multidimensional arrays.
pub struct IndexUtil {}

impl IndexUtil {
  // Converts a multidimensional index (eg {x, y, z}) into a linear index based
  // on the shape and its layout. The first index in the multi_index is
  // dimension 0.
  pub fn multi_dimensional_index_to_linear_index(
    shape: &Shape, multi_index: &Vec<i64>) -> i64
  {
    IndexUtil::multi_dimensional_index_to_linear_index_minor_to_major(
      shape, LayoutUtil::minor_to_major_from_shape(shape), multi_index)
  }

  // Converts a multidimensional index (eg {x, y, z}) into a linear index based
  // on the shape and its layout. The first index in the multi_index is
  // dimension 0.
  // This version can be used when the caller already has the minor_to_major
  // array for shape available (and can therefore be faster).
  pub fn multi_dimensional_index_to_linear_index_minor_to_major(
    shape: &Shape, minor_to_major: &Vec<i64>,  multi_index: &Vec<i64>) -> i64
  {
    // Let the array be sized like so for dimensions i from 0 to n-1:
    //
    //   [D{n-1} x D{n-2} x .. x D{0}]
    //
    // Let the order of the dimensions in the minor_to_major field in
    // Layout be:
    //
    //   L(0), L(1), ... , L(n-1)
    //
    // where L(0) is the most-minor dimension and L(n-1) the most-major. The
    // multidimensional index:
    //
    //   [I{0}, I{1}, ... , I{n-1}]
    //
    // then corresponds to the following linear index:
    //
    // linear_index =
    //   (((  ... + I{L(2)}) * D{L(1)} + I{L(1)}) * D{L(0)} + I{L(0)}
    //
    // or equivalently:
    //
    // linear_index =
    //   I{L(n-1)} * (D{L(n-2)} * D{L(n-3)} * D{L(n-4)} *     ....    D{L(0)}) +
    //   I{L(n-2)} *             (D{L(n-3)} * D{L(n-4)} *     ....    D{L(0)}) +
    //   I{L(n-3)} *                         (D{L(n-4)} *     ....    D{L(0)}) +
    //                                   ...                                   +
    //   I{L(2)} *                                         (D{L(1)} * D{L(0)}) +
    //   I{L(1)} *                                                    D{L(0)}  +
    //   I{L(0)}
    //
    // We compute the linear index value by accumulating the terms above from
    // I{L(0)} up to I{L(n-1)}. Scale accumulates the product term D{L(0}} *
    // D{L(1)} * ...

    // Scale factor holding the growing product of D{L(i)} terms.
    for i in 0..multi_index.len() {
      assert!(multi_index[i] >= 0);
      assert!(multi_index[i] < shape.dimensions(i));
    }
    if minor_to_major.is_empty() {
      return 0;
    }
    let mut linear_index = multi_index[minor_to_major[0] as usize];
    let mut scale = 1;
    for i in 1..minor_to_major.len() {
      scale *= shape.dimensions(minor_to_major[i-1] as usize);
      linear_index += scale * multi_index[minor_to_major[i] as usize];
    }
    linear_index
  }

  // Converts a linear index into multidimensional index (eg {x, y, z}) based on
  // the shape and its layout. The first index in the returned multidimensional
  // index is dimension 0.
  pub fn linear_index_to_multi_dimensional_index(
    shape: &Shape, linear_index: i64) -> Vec<i64>
  {
    assert!(linear_index >= 0);
    assert!(linear_index < ShapeUtil::elements_in(shape));

    // The following formula computes each element of the multidimensional index
    // (See comments in MultidimensionalIndexToLinearIndex for notation):
    //
    // I{L(0)} = linear_index % D{L(0)}
    // I{L(1)} = (linear_index / D{L(0)}) % D{L(1)}
    // I{L(2)} = (linear_index / (D{L(0)} * D{L(1)})) % D{L(2)}
    // ...
    let mut multi_index = vec![0; shape.dimensions_size()];

    // Accumulated product D{L(0)} * D{L(1)} * ...
    let mut divisor = 1;
    for dimension in LayoutUtil::minor_to_major_from_shape(shape) {
      multi_index[*dimension as usize] =
        (linear_index / divisor) % shape.dimensions(*dimension as usize);
      divisor *= shape.dimensions(*dimension as usize);
    }
    multi_index
  }

  // Bumps a sequence of indices; e.g. {0,0,0,0} up by one index value; e.g. to
  // {0,0,0,1}. This is akin to std::next_permutation. If the index hits a limit
  // for the provided shape, the next most significant index is bumped, in a
  // counting-up process.
  //
  // E.g. for shape f32[2,3]
  //  {0,0}=>{0,1}
  //  {0,1}=>{0,2}
  //  {0,2}=>{1,0}
  //  etc.
  //
  // This is useful for traversing the indices in a literal.
  //
  // Returns true iff the indices were successfully bumped; false if we've hit
  // the limit where it can no longer be bumped in-bounds.
  pub fn bump_indices(shape: &Shape, indices: &mut Vec<i64>) -> bool {
    for dimno in (0..indices.len()).rev() {
      let limit = shape.dimensions(dimno);
      if indices[dimno] + 1 < limit {
        indices[dimno] += 1;
        // Whenever an index of a dimension is increased, it means that all
        // following dimensions have maxed out, so they must go to 0.
        for i in dimno+1..indices.len() {
          indices[i] = 0;
        }
        return true;
      }
    }
    false
  }

  // Calculates the stride size (in number of elements, not byte size) of a
  // given logical shape dimension (from 0 to rank-1).
  // Example:
  //  GetDimensionStride(F32[5,8,10,4]{3,2,1,0}, 1) ==
  //    sizeof(dimension(3)) * sizeof(dimension(2)) == 4 * 10
  pub fn get_dimension_stride(shape: &Shape, dimension: i64) -> i64 {
    let mut stride = 1;
    for dim in LayoutUtil::minor_to_major_from_shape(shape) {
      if *dim == dimension { break; }
      stride *= shape.dimensions_vec()[*dim as usize];
    }
    stride
  }

  // Returns true iff the given multi-index is contained in the bounds for the
  // shape.
  pub fn index_in_bounds(shape: &Shape, index: &Vec<i64>) -> bool {
    let rank = shape.rank();
    if rank != index.len() {
      return false;
    }
    for d in 0..rank {
      if index[d] >= shape.dimensions(d) {
        return false;
      }
    }
    true
  }

  // Compares the given indices in lexicographic order.  lhs[0] and rhs[0] are
  // compared first, and lhs[rank-1] and rhs[rank-1] last.  If lhs is larger,
  // then -1 is returned. If rhs is larger, then 1 is returned.  Otherwise, 0 is
  // returned.
  pub fn compare_indices(lhs: &Vec<i64>, rhs: &Vec<i64>) -> i64 {
    assert_eq!(lhs.len(), rhs.len());
    let rank = lhs.len();
    for dim in 0..rank {
      if lhs[dim] < rhs[dim] {
        return -1;
      } else if lhs[dim] > rhs[dim] {
        return 1;
      }
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::blitz_data::PrimitiveType;

  fn set_minor_to_major_layout(shape: &mut Shape, dimensions: &Vec<i64>) {
    shape.mutable_layout().as_mut().unwrap().clear_minor_to_major();
    for dim in dimensions {
      shape.mutable_layout().as_mut().unwrap().add_minor_to_major(*dim);
    }
  }

  #[test]
  fn test_vector_indexing() {
    // Vectors are trivially laid out and the linear index should always be the
    // same as the "multidimensional" index.
    let vector_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![100]);
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &vector_shape, &vec![42]), 42);
    
    let multi_index = IndexUtil::linear_index_to_multi_dimensional_index(
      &vector_shape, 42);
    assert_eq!(multi_index.len(), 1);
    assert_eq!(multi_index[0], 42);
  }

  #[test]
  fn test_matrix_indexing_row_major() {
    // Set layout to [0, 1]. That is, row major.
    let mut matrix_shape_01 = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10, 20]);
    set_minor_to_major_layout(&mut matrix_shape_01, &vec![0, 1]);

    // If index is {a, b} then linear index should be: a + b * 10
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &matrix_shape_01, &vec![0, 0]), 0);
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &matrix_shape_01, &vec![9, 19]), 199);
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &matrix_shape_01, &vec![3, 5]), 53);

    assert_eq!(IndexUtil::linear_index_to_multi_dimensional_index(
      &matrix_shape_01, 53), vec![3, 5]);
  }

  #[test]
  fn test_matrix_indexing_column_major() {
    let mut matrix_shape_10 = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10, 20]);
    set_minor_to_major_layout(&mut matrix_shape_10, &vec![1, 0]);

    // If index is {a, b} then linear index should be: a * 20 + b
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &matrix_shape_10, &vec![0, 0]), 0);
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(&
      matrix_shape_10, &vec![9, 19]), 199);
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &matrix_shape_10, &vec![3, 5]), 65);

    assert_eq!(IndexUtil::linear_index_to_multi_dimensional_index(
      &matrix_shape_10, 65), vec![3, 5]);
  }

  #[test]
  fn test_3_d_array_indexing_210() {
    // Set layout to [2, 1, 0]. That is, column major.
    let mut shape_210 = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10, 20, 30]);
    set_minor_to_major_layout(&mut shape_210, &vec![2, 1, 0]);

    // If index is {a, b, c} then linear index should be:
    // a * 20 * 30 + b * 30 + c
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &shape_210, &vec![3, 5, 7]), 1957);
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &shape_210, &vec![8, 15, 27]), 5277);
  }

  #[test]
  fn test_3_d_array_indexing_120() {
    // Set layout to [1, 2, 0]
    let mut shape_120 = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10, 20, 30]);
    set_minor_to_major_layout(&mut shape_120, &vec![1, 2, 0]);

    // If index is {a, b, c} then linear index should be:
    // a * 20 * 30 + b + c * 20
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &shape_120, &vec![3, 5, 7]), 1945);
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &shape_120, &vec![8, 15, 27]), 5355);
  }

  #[test]
  fn test_4_d_array_indexing_3210() {
    // Set layout to [3, 2, 1, 0]. That is, column major.
    let mut shape_3210 = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10, 20, 30, 40]);
    set_minor_to_major_layout(&mut shape_3210, &vec![3, 2, 1, 0]);

    // If index is {a, b, c, d} then linear index should be:
    // a * 20 * 30 * 40 + b * 30 * 40 + c * 40 + d
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &shape_3210, &vec![3, 5, 7, 9]), 78289);
    assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
      &shape_3210, &vec![8, 15, 27, 33]), 211113);
  }

  #[test]
  fn test_linear_to_multi_to_linear() {
    // Verify that converting a linear index to a multidimensional index and back
    // always returns the same value for different crazy shapes.  Shape has
    // 1440000000 elements. Inputs are randomly-ish selected.
    let linear_indexes = vec![
      0, 1439999999, 1145567336, 43883404, 617295214, 1117613654];
    let mut minor_to_major_orders = vec![];
    minor_to_major_orders.push(vec![6, 5, 4, 3, 2, 1, 0]);
    minor_to_major_orders.push(vec![0, 1, 2, 3, 4, 5, 6]);
    minor_to_major_orders.push(vec![4, 5, 1, 2, 6, 0, 3]);

    for minor_to_major_order in &minor_to_major_orders {
      let mut shape = ShapeUtil::make_shape(
        &PrimitiveType::F32, vec![10, 20, 30, 40, 30, 20, 10]);
      set_minor_to_major_layout(&mut shape, minor_to_major_order);
      for linear_index in &linear_indexes {
        let multi_index =
          IndexUtil::linear_index_to_multi_dimensional_index(&shape, *linear_index);
        assert_eq!(IndexUtil::multi_dimensional_index_to_linear_index(
          &shape, &multi_index), *linear_index);
      }
    }
  }

  #[test]
  fn test_bump_indices_2x2() {
    let shape = ShapeUtil::make_shape(
      &PrimitiveType::S32, vec![2, 2]);
    let mut indices = vec![0, 0];

    assert_eq!(IndexUtil::bump_indices(&shape, &mut indices), true);
    assert_eq!(indices, vec![0, 1]);
    assert_eq!(IndexUtil::bump_indices(&shape, &mut indices), true);
    assert_eq!(indices, vec![1, 0]);
    assert_eq!(IndexUtil::bump_indices(&shape, &mut indices), true);
    assert_eq!(indices, vec![1, 1]);
    assert_eq!(IndexUtil::bump_indices(&shape, &mut indices), false);
  }
}