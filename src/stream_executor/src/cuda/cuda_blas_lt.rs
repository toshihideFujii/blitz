#![allow(dead_code)]

struct MatrixLayout {}

impl MatrixLayout {
  pub fn new() {}
  pub fn type_() {}
  pub fn get() {}
}

struct MatmulDesc {}

impl MatmulDesc {
  pub fn new() {}
  pub fn compute_type() {}
  pub fn scale_type() {}
  pub fn pointer_mode() {}
  pub fn get() {}
}

struct MatmulPlan {}

impl MatmulPlan {
  pub fn new() {}
  pub fn execute_on_stream() {}
  pub fn get_algorithms() {}
  pub fn validate_inputs() {}
  pub fn do_mat_mul() {}
}

pub struct BlasLt {}

impl BlasLt {
  pub fn new() {}
  pub fn init() {}
  pub fn get_matmul_plan() {}
}