#![allow(dead_code)]

struct ConstantRange {}
impl ConstantRange {
  pub fn get_empty() {}
  pub fn get_full() {}
  pub fn get_non_empty() {}
  pub fn from_known_bits() {}
  pub fn make_allowed_icmp_region() {}
  pub fn make_satisfying_icmp_region() {}
  pub fn make_exact_icmp_region() {}
  pub fn icmp() {}
  pub fn are_insensitive_to_signedness_of_icmp_predicate() {}
  pub fn get_equivalent_pred_with_flipped_signedness() {}
  pub fn make_guaranteed_no_wrap_region() {}
  pub fn make_exact_no_wrap_region() {}
  pub fn is_intrinsic_supported() {}
  pub fn intrinsic() {}
  pub fn get_equivalent_icmp() {}
  pub fn get_lower() {}
  pub fn get_upper() {}
}