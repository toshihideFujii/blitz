#![allow(dead_code)]

// This file defines simple local analysis for load instruction.

pub fn is_dereferenceable_pointer() {}
pub fn is_dereferenceable_and_aligned_pointer() {}
pub fn is_safe_to_load_unconditionally() {}
pub fn is_dereferenceable_and_aligned_in_loop() {}
pub fn find_available_loaded_value() {}
pub fn find_available_ptr_load_store() {}
pub fn can_replace_pointers_if_equal() {}