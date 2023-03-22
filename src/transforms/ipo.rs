#![allow(dead_code)]

pub fn create_annotation_2_metadata_legacy_pass() {}

pub fn create_constant_merge_pass() {}

// This transform is designed to eliminate unreachable
// internal globals (functions or global variables)
pub fn create_global_dce_pass() {}

pub fn create_gv_extraction_pass() {}

// This pass removes arguments from functions which are not
// used by the body of function.
pub fn create_dead_arg_elimination_pass() {}

pub fn create_dead_arg_hacking_pass() {}

// This pass extracts all natural loops from the program into
// a function if it can.
pub fn create_loop_extractor_pass() {}

pub fn create_single_loop_extractor_pass() {}

pub fn create_barrier_noop_pass() {}