#![allow(dead_code)]

// DependenceAnalysis is a Blitz pass that analyses dependences between
// memory accesses.

struct Dependence {}
impl Dependence {
  pub fn new() {}
  pub fn get_src() {}
  pub fn get_dst() {}
  pub fn is_input() {}
  pub fn is_output() {}
  pub fn is_flow() {}
  pub fn is_anti() {}
  pub fn is_ordered() {}
  pub fn is_unordered() {}
  pub fn is_loop_independent() {}
  pub fn is_confused() {}
  pub fn is_consistent() {}
  pub fn get_levels() {}
  pub fn get_direction() {}
  pub fn get_distance() {}
  pub fn is_direction_negative() {}
  pub fn normalize() {}
  pub fn is_peel_first() {}
  pub fn is_peel_last() {}
  pub fn is_splitable() {}
  pub fn is_scalar() {}
  pub fn get_next_predecessor() {}
  pub fn get_next_successor() {}
  pub fn dump() {}
}

struct FullDependence {}
impl FullDependence {
  pub fn new() {}
  pub fn is_loop_independent() {}
  pub fn is_confused() {}
  pub fn is_consistent() {}
  pub fn get_levels() {}
  pub fn get_direction() {}
  pub fn get_distance() {}
  pub fn is_direction_negative() {}
  pub fn normalize() {}
  pub fn is_peel_first() {}
  pub fn is_peel_last() {}
  pub fn is_splitable() {}
  pub fn is_scalar() {}
}

// This class is the main dependence-analysis driver.
struct DependenceInfo {}
impl DependenceInfo {
  pub fn new() {}
  pub fn invalidate() {}
  pub fn depends() {}
  pub fn get_split_iteration() {}
  pub fn get_function() {}
  pub fn establish_nesting_levels() {}
  pub fn map_src_loop() {}
  pub fn map_dst_loop() {}
  pub fn is_loop_invariant() {}
  pub fn unify_subscript_type() {}
  pub fn remove_matching_extensions() {}
  pub fn collect_common_loops() {}
  pub fn check_src_subscript() {}
  pub fn check_dst_subscript() {}
  pub fn is_known_predicate() {}
  pub fn is_known_less_than() {}
  pub fn is_known_non_negative() {}
  pub fn collect_upper_bound() {}
  pub fn collect_constant_upper_bound() {}
  pub fn classify_pair() {}
  pub fn test_ziv() {}
  pub fn test_siv() {}
  pub fn test_rdiv() {}
  pub fn test_miv() {}
  pub fn strong_siv_test() {}
  pub fn weak_crossing_siv_test() {}
  pub fn exact_siv_test() {}
  pub fn weak_zero_src_siv_test() {}
  pub fn weak_zero_dst_siv_test() {}
  pub fn exact_rdiv_test() {}
  pub fn symbolic_rdiv_test() {}
  pub fn gcd_miv_test() {}
  pub fn banerjee_miv_test() {}
  pub fn collect_coeff_info() {}
  pub fn get_positive_part() {}
  pub fn get_negative_part() {}
  pub fn get_lower_bound() {}
  pub fn get_upper_bound() {}
  pub fn explore_directions() {}
  pub fn test_bounds() {}
  pub fn find_bounds_all() {}
  pub fn find_bounds_lt() {}
  pub fn find_bounds_gt() {}
  pub fn find_bounds_eq() {}
  pub fn intersect_constraints() {}
  pub fn propagate() {}
  pub fn propagate_distance() {}
  pub fn propagate_point() {}
  pub fn propagate_line() {}
  pub fn find_coefficient() {}
  pub fn zero_coefficient() {}
  pub fn add_to_coefficient() {}
  pub fn update_direction() {}
  pub fn try_delinearize() {}
  pub fn try_delinearize_fixed_size() {}
  pub fn try_delinearize_parametric_size() {}
  pub fn check_subscript() {}
}

struct DependenceAnalysis {}
impl DependenceAnalysis {
  pub fn new() {}
  pub fn run() {}
}

struct DependenceAnalysisPrinterPass {}
impl DependenceAnalysisPrinterPass {
  pub fn new() {}
  pub fn run() {}
}

struct DependenceAnalysisWrapperPass {}
impl DependenceAnalysisWrapperPass {
  pub fn new() {}
  pub fn run_on_function() {}
  pub fn release_memory() {}
  pub fn get_analysis_usage() {}
  pub fn print() {}
  pub fn get_di() {}
}

pub fn create_dependence_analysis_wrapper_pass() {}