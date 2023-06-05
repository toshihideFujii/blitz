#![allow(dead_code)]

// This file defines some vectorizer utilities.

pub fn to_vector_type() {}
pub fn is_trivially_vectorizable() {}
pub fn is_vector_intrinsic_with_scalar_op_at_arg() {}
pub fn is_vector_intrinsic_with_overload_type_at_arg() {}
pub fn get_vector_intrinsic_id_for_call() {}
pub fn find_scalar_element() {}
pub fn get_splat_index() {}
pub fn get_splat_value() {}
pub fn is_splat_value() {}
pub fn get_shuffle_demanded_elts() {}
pub fn narrow_shuffle_mask_elts() {}
pub fn widen_shuffle_mask_elts() {}
pub fn get_shuffle_mask_with_widest_elts() {}
pub fn process_shuffle_masks() {}
pub fn compute_minimum_value_sizes() {}
pub fn unite_access_groups() {}
pub fn intersect_access_groups() {}
pub fn propagate_metadata() {}
pub fn create_bit_mask_for_gaps() {}
pub fn create_replicated_mask() {}
pub fn create_interleave_mask() {}
pub fn create_stride_mask() {}
pub fn create_sequential_mask() {}
pub fn create_unary_mask() {}
pub fn concatenate_vectors() {}
pub fn mask_is_all_zero_or_undef() {}
pub fn mask_is_all_one_or_undef() {}
pub fn possibly_demanded_elts_in_mask() {}