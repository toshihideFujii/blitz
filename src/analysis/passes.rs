#![allow(dead_code)]

// This header file defines prototypes for accessor functions that
// expose passes in the analysis libraries.

pub fn create_lazy_value_info_pass() {}
pub fn create_dependence_analysis_wrapper_pass() {}
pub fn create_cost_model_analysis_pass() {}
pub fn create_delinearization_pass() {}
pub fn create_inst_count_pass() {}
pub fn create_region_info_pass() {}
pub fn create_module_debug_info_printer_pass() {}
pub fn create_mem_dep_printer() {}
pub fn create_mem_deref_printer() {}
pub fn create_must_execute_printer() {}
pub fn create_must_be_executed_context_printer() {}