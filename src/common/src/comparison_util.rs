use crate::{blitz_data::PrimitiveType, primitive_util::{
  is_array_type, is_complex_type, is_floating_point_type, is_integral_type,
  is_signed_integral_type, is_unsigned_integral_type, primitive_type_name}
};

// Represents the ordering of the comparison.
#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOrder {
  Total,
  Partial,
}

// Represents different comparison operations.
#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonDirection {
  Eq,
  Ne,
  Ge,
  Gt,
  Le,
  Lt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonType {
  Float,
  FloatTotalOrder,
  Signed,
  Unsigned,
}

// Verifies that this is a valid Comparison: (1) not a partial ordering on
// integers, and (2) a valid PrimitiveType.
pub fn is_valid_comparison(t: &PrimitiveType, order: &ComparisonOrder) -> bool {
  if is_floating_point_type(t) || is_complex_type(t) {
    return true;
  }
  if is_integral_type(t) || *t == PrimitiveType::Pred {
    return *order == ComparisonOrder::Total;
  }
  panic!("Unsupported type.");
}

pub fn string_to_comparison_direction(
  _direction: &String) -> Result<ComparisonDirection, String>
{
  unimplemented!()    
}

pub fn string_to_comparison_type(
  _comparison: &String) -> Result<ComparisonType, String>
{
  unimplemented!()    
}

pub fn comparison_direction_to_string(dir: &ComparisonDirection) -> String {
  match dir {
    ComparisonDirection::Eq => "EQ".to_string(),
    ComparisonDirection::Ne => "NE".to_string(),
    ComparisonDirection::Ge => "GE".to_string(),
    ComparisonDirection::Gt => "GT".to_string(),
    ComparisonDirection::Le => "LE".to_string(),
    ComparisonDirection::Lt => "LT".to_string(),
  }
}

pub fn comparison_primitive_type_to_string(t: &PrimitiveType) -> String {
  primitive_type_name(&t)
}

pub fn comparison_order_to_string(order: &ComparisonOrder) -> String {
  match order {
    ComparisonOrder::Partial => "PARTIALORDER".to_string(),
    ComparisonOrder::Total => "TOTALORDER".to_string(),
  }
}

pub fn comparison_type_to_string(t: &ComparisonType) -> String {
  match t {
    ComparisonType::Float => "FLOAT".to_string(),
    ComparisonType::FloatTotalOrder => "TOTALORDER".to_string(),
    ComparisonType::Signed => "SIGNED".to_string(),
    ComparisonType::Unsigned => "UNSIGNED".to_string()
  }
}

pub fn default_comparison_type(t: &PrimitiveType) -> ComparisonType {
  if is_floating_point_type(t) || is_complex_type(t) {
    return ComparisonType::Float;
  }
  if is_signed_integral_type(t) {
    return ComparisonType::Signed;
  }
  if is_unsigned_integral_type(t) || *t == PrimitiveType::Pred {
    return ComparisonType::Unsigned;
  }
  panic!("Unexpected: {:?}", t);
}

pub fn default_primitive_type(t: &ComparisonType) -> PrimitiveType {
  match t {
    ComparisonType::Float => PrimitiveType::F32,
    ComparisonType::FloatTotalOrder => PrimitiveType::F32,
    ComparisonType::Signed => PrimitiveType::S32,
    ComparisonType::Unsigned => PrimitiveType::U32,
  }
}

pub fn default_ordering_by_comparison(t: &ComparisonType) -> ComparisonOrder {
  match t {
    ComparisonType::Float => ComparisonOrder::Partial,
    _ => ComparisonOrder::Total
  }
}

pub fn default_ordering_by_primitive(t: &PrimitiveType) -> ComparisonOrder {
  if is_floating_point_type(t) || is_complex_type(t) {
    return ComparisonOrder::Partial;
  }
  if is_integral_type(t) || *t == PrimitiveType::Pred {
    return ComparisonOrder::Total;
  }
  panic!("Unexpected: {:?}", t);
}

// Returns the converse of `direction`.
pub fn converse(dir: ComparisonDirection) -> ComparisonDirection {
  match dir {
    ComparisonDirection::Eq => ComparisonDirection::Eq,
    ComparisonDirection::Ne => ComparisonDirection::Ne,
    ComparisonDirection::Ge => ComparisonDirection::Le,
    ComparisonDirection::Gt => ComparisonDirection::Lt,
    ComparisonDirection::Le => ComparisonDirection::Ge,
    ComparisonDirection::Lt => ComparisonDirection::Gt,
  }
}

// Returns the inverse of `direction`.
pub fn inverse(dir: ComparisonDirection) -> ComparisonDirection {
  match dir {
    ComparisonDirection::Eq => ComparisonDirection::Ne,
    ComparisonDirection::Ne => ComparisonDirection::Eq,
    ComparisonDirection::Ge => ComparisonDirection::Lt,
    ComparisonDirection::Gt => ComparisonDirection::Le,
    ComparisonDirection::Le => ComparisonDirection::Gt,
    ComparisonDirection::Lt => ComparisonDirection::Ge,
  }
}

#[derive(Debug, PartialEq)]
pub struct Comparison {
  dir: ComparisonDirection,
  primitive_t: PrimitiveType,
  order: ComparisonOrder,
  t: ComparisonType
}

impl Comparison {
  pub fn new(
    dir: ComparisonDirection,
    primitive_t: PrimitiveType,
    order: ComparisonOrder) -> Self
  {
    let t = default_comparison_type(&primitive_t);
    let instance =
      Comparison { dir: dir, primitive_t: primitive_t, order: order, t: t };
    assert!(is_valid_comparison(&instance.primitive_t, &instance.order));
    instance
  }

  pub fn new_dir_prim(dir: ComparisonDirection, primitive_t: PrimitiveType) -> Self {
    let instance = Comparison {
      dir: dir,
      primitive_t: primitive_t.clone(),
      order: default_ordering_by_primitive(&primitive_t),
      t: default_comparison_type(&primitive_t)
    };
    assert!(is_valid_comparison(&instance.primitive_t, &instance.order));
    instance
  }

  #[inline]
  pub fn get_direction(&self) -> ComparisonDirection {
    self.dir.clone()
  }

  #[inline]
  pub fn get_primitive_type(&self) -> PrimitiveType {
    self.primitive_t.clone()
  }

  #[inline]
  pub fn get_order(&self) -> ComparisonOrder {
    self.order.clone()
  }

  #[inline]
  pub fn is_eq(&self) -> bool {
    self.dir == ComparisonDirection::Eq
  }

  #[inline]
  pub fn is_ne(&self) -> bool {
    self.dir == ComparisonDirection::Ne
  }

  #[inline]
  pub fn is_ge(&self) -> bool {
    self.dir == ComparisonDirection::Ge
  }

  #[inline]
  pub fn is_gt(&self) -> bool {
    self.dir == ComparisonDirection::Gt
  }

  #[inline]
  pub fn is_lt(&self) -> bool {
    self.dir == ComparisonDirection::Lt
  }

  #[inline]
  pub fn is_total_order(&self) -> bool {
    self.order == ComparisonOrder::Total
  }

  #[inline]
  pub fn is_partial_order(&self) -> bool {
    self.order == ComparisonOrder::Partial
  }

  // Returns whether this is a floating point total order comparison.
  #[inline]
  pub fn is_f32_total_order(&self) -> bool {
    self.primitive_t == PrimitiveType::F32 && self.is_total_order()
  }

  // Returns whether this is a standard comparison, i.e., what you would expect
  // as the industry standard on most architectures.
  #[inline]
  pub fn is_standard_f32(&self) -> bool {
    self.primitive_t == PrimitiveType::F32 && self.is_partial_order()
  }

  #[inline]
  pub fn is_standard_s32(&self) -> bool {
    self.primitive_t == PrimitiveType::S32 && self.is_total_order()
  }

  #[inline]
  pub fn is_standard_u32(&self) -> bool {
    self.primitive_t == PrimitiveType::U32 && self.is_total_order()
  }

  #[inline]
  pub fn is_integer_primitive_type(&self) -> bool {
    is_integral_type(&self.primitive_t)
  }

  #[inline]
  pub fn is_floating_point_primitive_type(&self) -> bool {
    is_floating_point_type(&self.primitive_t)
  }

  // Returns whether (a dir a) is always true for this comparison.
  pub fn is_reflexive(&self) -> bool {
    match self.dir {
      ComparisonDirection::Eq => self.is_total_order(),
      ComparisonDirection::Ge => self.is_total_order(),
      ComparisonDirection::Le=> self.is_total_order(),
      _ => false
    }
  }

  // Returns whether (a dir a) is always false for this comparison.
  pub fn is_anti_reflexive(&self) -> bool {
    match self.dir {
      ComparisonDirection::Ne => self.is_total_order(),
      ComparisonDirection::Gt => true,
      ComparisonDirection::Lt => true,
      _ => false
    }
  }

  // Gets the converse of the given comparison direction (e.g. >= turns to <=).
  // Useful when commuting operands to get constants into immediate-accepting
  // positions in the ISA.  
  pub fn converse(&self) -> Self {
    Comparison::new(converse(
      self.dir.clone()),
      self.primitive_t.clone(),
      self.order.clone())
  }

  // Gets the inverse of the given comparison if it exists (e.g. >= turns to <).
  // Returns optional value because not all inversions may be supported.
  pub fn inverse(&self) -> Option<Self> {
    if self.is_partial_order() {
      // We assume comparisons don't have inverses unless they are total order,
      // e.g., a partial order floating point comparison can return true if one
      // operand is NaN.
      return None;
    }
    if is_array_type(&self.primitive_t) {
      return Some(Comparison::new(
        inverse(self.dir.clone()),
        self.primitive_t.clone(),
        self.order.clone()));
    }
    None
  }

  // Returns a string version of this comparison, e.g., ".GT.F32.TOTALORDER"
  pub fn to_string(&self,
    prefix1: &'static str,
    prefix2: &'static str,
    prefix3: &'static str) -> String
  {
    let mut result = prefix1.to_string();
    result.push_str(&comparison_direction_to_string(&self.dir));
    result.push_str(&prefix2);
    result.push_str(&comparison_primitive_type_to_string(&self.primitive_t));
    result.push_str(&prefix3);
    result.push_str(&comparison_order_to_string(&self.order));
    result
  }

  pub fn compare<T>(&self, a: T, b: T) -> bool
    where T: PartialEq + PartialOrd
  {
    match self.get_direction() {
      ComparisonDirection::Eq => a == b,
      ComparisonDirection::Ne => a != b,
      ComparisonDirection::Ge => a >= b,
      ComparisonDirection::Gt => a > b,
      ComparisonDirection::Le => a <= b,
      ComparisonDirection::Lt => a < b,
    }
  }
}

#[cfg(test)]
mod tests {
  use core::f64;
  use super::*;

  #[test]
  fn test_floats_default_to_partial_order() {
    let mut comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ge, PrimitiveType::BF16);
    assert_eq!(comparison.get_order(), ComparisonOrder::Partial);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ge, PrimitiveType::F32);
    assert_eq!(comparison.get_order(), ComparisonOrder::Partial);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ge, PrimitiveType::C64);
    assert_eq!(comparison.get_order(), ComparisonOrder::Partial);
  }

  #[test]
  fn test_integer_default_to_total_order() {
    let mut comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ge, PrimitiveType::S32);
    assert_eq!(comparison.get_order(), ComparisonOrder::Total);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ge, PrimitiveType::U8);
    assert_eq!(comparison.get_order(), ComparisonOrder::Total);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ge, PrimitiveType::Pred);
    assert_eq!(comparison.get_order(), ComparisonOrder::Total);
  }

  #[test]
  fn test_partial_order_reflexivity() {
    let mut comparison = Comparison::new_dir_prim(
      ComparisonDirection::Eq, PrimitiveType::F32);
    assert_eq!(comparison.is_reflexive(), false);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Le, PrimitiveType::F32);
    assert_eq!(comparison.is_reflexive(), false);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Lt, PrimitiveType::S32);
    assert_eq!(comparison.is_reflexive(), false);
  }

  #[test]
  fn test_total_order_reflexivity() {
    let mut comparison = Comparison::new(ComparisonDirection::Le,
      PrimitiveType::BF16, ComparisonOrder::Total);
    assert_eq!(comparison.is_reflexive(), true);

    comparison = Comparison::new(ComparisonDirection::Ge, 
      PrimitiveType::F32, ComparisonOrder::Total);
    assert_eq!(comparison.is_reflexive(), true);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Eq, PrimitiveType::S32);
    assert_eq!(comparison.is_reflexive(), true);

    comparison = Comparison::new(ComparisonDirection::Ne,
      PrimitiveType::F32, ComparisonOrder::Total);
    assert_eq!(comparison.is_reflexive(), false);

    comparison = Comparison::new(ComparisonDirection::Lt,
      PrimitiveType::F64, ComparisonOrder::Total);
    assert_eq!(comparison.is_reflexive(), false);
  }

  #[test]
  fn test_partial_order_anti_reflexivity() {
    let mut comparison = Comparison::new_dir_prim(
      ComparisonDirection::Gt, PrimitiveType::F32);
    assert_eq!(comparison.is_anti_reflexive(), true);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Lt, PrimitiveType::F32);
    assert_eq!(comparison.is_anti_reflexive(), true);

    comparison = Comparison::new(ComparisonDirection::Eq,
      PrimitiveType::F32, ComparisonOrder::Total);
    assert_eq!(comparison.is_anti_reflexive(), false);
  }

  #[test]
  fn test_total_order_anti_reflexivity() {
    let mut comparison = Comparison::new(ComparisonDirection::Ne,
      PrimitiveType::BF16, ComparisonOrder::Total);
    assert_eq!(comparison.is_anti_reflexive(), true);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ne, PrimitiveType::S32);
    assert_eq!(comparison.is_anti_reflexive(), true);

    comparison = Comparison::new(ComparisonDirection::Eq,
      PrimitiveType::F32, ComparisonOrder::Total);
    assert_eq!(comparison.is_anti_reflexive(), false);

    comparison = Comparison::new(ComparisonDirection::Le,
      PrimitiveType::F64, ComparisonOrder::Total);
    assert_eq!(comparison.is_anti_reflexive(), false);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Le, PrimitiveType::S8);
    assert_eq!(comparison.is_anti_reflexive(), false);
  }

  #[test]
  fn test_converse() {
    let mut converse = Comparison::new_dir_prim(
      ComparisonDirection::Le, PrimitiveType::S8).converse();
    assert_eq!(converse, Comparison::new_dir_prim(ComparisonDirection::Ge, PrimitiveType::S8));

    converse = Comparison::new_dir_prim(
      ComparisonDirection::Eq, PrimitiveType::U16).converse();
    assert_eq!(converse, Comparison::new_dir_prim(ComparisonDirection::Eq, PrimitiveType::U16));

    converse = Comparison::new_dir_prim(
      ComparisonDirection::Gt, PrimitiveType::F32).converse();
    assert_eq!(converse, Comparison::new_dir_prim(ComparisonDirection::Lt, PrimitiveType::F32));
  }

  #[test]
  fn test_partial_order_floats_should_not_have_inverse() {
    let comparison = Comparison::new_dir_prim(
      ComparisonDirection::Gt, PrimitiveType::F32);
    assert_eq!(comparison.inverse().is_some(), false);
  }

  #[test]
  fn test_inverse() {
    let mut inverse = Comparison::new_dir_prim(
      ComparisonDirection::Le, PrimitiveType::S64).inverse();
    assert_eq!(inverse.unwrap(),
      Comparison::new_dir_prim(ComparisonDirection::Gt, PrimitiveType::S64));

    inverse = Comparison::new_dir_prim(
      ComparisonDirection::Eq, PrimitiveType::U16).inverse();
    assert_eq!(inverse.unwrap(),
      Comparison::new_dir_prim(ComparisonDirection::Ne, PrimitiveType::U16));

    inverse = Comparison::new(ComparisonDirection::Gt,
      PrimitiveType::F32, ComparisonOrder::Total).inverse();
    assert_eq!(inverse.unwrap(),
      Comparison::new(ComparisonDirection::Le, PrimitiveType::F32, ComparisonOrder::Total));
  }

  #[test]
  fn test_to_string() {
    let mut comparison = Comparison::new_dir_prim(
      ComparisonDirection::Lt, PrimitiveType::F32);
    assert_eq!(comparison.to_string(".", ".", "."), ".LT.F32.PARTIALORDER".to_string());

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Eq, PrimitiveType::S8);
    assert_eq!(comparison.to_string(".", ".", "."), ".EQ.S8.TOTALORDER".to_string());

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ge, PrimitiveType::C128);
    assert_eq!(comparison.to_string("_1_", "_2_", "_3_"), "_1_GE_2_C128_3_PARTIALORDER".to_string());
  }

  #[test]
  fn test_total_order_float_comparison() {
    //let mut comparison = Comparison::new(ComparisonDirection::Eq,
      //PrimitiveType::F64, ComparisonOrder::Total);
    //assert_eq!(comparison.compare(f64::NAN, f64::NAN), true);

    let mut comparison = Comparison::new(ComparisonDirection::Lt,
      PrimitiveType::F64, ComparisonOrder::Total);
    assert_eq!(comparison.compare(1.0, 2.0), true);

    comparison = Comparison::new(ComparisonDirection::Lt,
      PrimitiveType::F64, ComparisonOrder::Total);
    assert_eq!(comparison.compare(f64::INFINITY, f64::NEG_INFINITY), false);

    //comparison = Comparison::new(ComparisonDirection::Lt,
      //PrimitiveType::F64, ComparisonOrder::Total);
    //assert_eq!(comparison.compare(-0.0, 0.0), true);

    //comparison = Comparison::new(ComparisonDirection::Ne,
      //PrimitiveType::F64, ComparisonOrder::Total);
    //assert_eq!(comparison.compare(0.0, -0.0), true);

    comparison = Comparison::new(ComparisonDirection::Gt,
      PrimitiveType::F64, ComparisonOrder::Total);
    assert_eq!(comparison.compare(-0.1, 0.1), false);
  }

  #[test]
  fn test_compare() {
    let mut comparison = Comparison::new_dir_prim(
      ComparisonDirection::Lt, PrimitiveType::F64);
    assert_eq!(comparison.compare(1.0, 2.0), true);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Ne, PrimitiveType::S64);
    assert_eq!(comparison.compare(1_000_000, 1_000_000), false);

    comparison = Comparison::new_dir_prim(
      ComparisonDirection::Eq, PrimitiveType::U8);
    assert_eq!(comparison.compare(63, 63), true);
  }
}