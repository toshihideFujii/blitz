pub trait BoundsOp<T, U> {
  fn is_in_range(value: T, lower_limit: U, higher_limit: U) -> bool;
}

pub struct Bounds {}

impl BoundsOp<u32, u32> for Bounds {
  fn is_in_range(value: u32, lower_limit: u32, higher_limit: u32) -> bool {
    (value - lower_limit) <= (higher_limit - lower_limit)
  }
}
