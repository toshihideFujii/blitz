#![allow(dead_code)]

// Interface to identify indirect call promotion candidates.

struct ICallPromotionAnalysis {}
impl ICallPromotionAnalysis {
  pub fn new() {}
  pub fn is_promotion_profitable() {}
  pub fn get_profitable_promotion_candidates() {}
  pub fn get_promotion_candidates_for_instruction() {}
}