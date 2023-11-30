#![allow(dead_code)]

use base::build_config::PAGE_SIZE_BITS;
use common::{globals::*, blitz_internal::{MB, KB}};
use flags::flags::BLITZ_FLAGS;

pub struct Heap {}

impl Heap {
  const POINTER_MULTIPLIER: usize = TAGGED_SIZE / 4;
  const HEAP_LIMIT_MULTIPLIER: usize = SYSTEM_POINTER_SIZE / 4;
  const MAX_INITIAL_OLD_GENERATION_SIZE: usize = 256 * MB * Heap::HEAP_LIMIT_MULTIPLIER;
  const PHISICAL_MEMORY_TO_OLD_GENERATION_RATIO: usize = 4;
  const OLD_GENERATION_LOW_MEMORY: usize = 128 * MB * Heap::HEAP_LIMIT_MULTIPLIER;
  const NEW_LARGE_OBJECT_SPACE_TO_SEMI_SPACE_RATIO: usize = 1;
  const TRACE_RING_BUFFER_SIZE: usize = 512;
  const STACKTRACE_BUFFER_SIZE: usize = 512;
  const MIN_OBJECT_SIZE_IN_TAGGED_WORDS: usize = 2;

  pub fn default_min_semi_space_size() -> usize {
    let min_semi_space_size = 512 * KB * Heap::POINTER_MULTIPLIER;
    assert!(min_semi_space_size % (1 << PAGE_SIZE_BITS) == 0);
    min_semi_space_size
  }

  pub fn default_max_semi_space_size() -> usize {
    let max_semi_space_capacity_base_unit = MB * Heap::POINTER_MULTIPLIER;
    assert!(max_semi_space_capacity_base_unit & (1 << PAGE_SIZE_BITS) == 0);
    let mut max_new_space_capacity_mb =
      BLITZ_FLAGS.scavenger_max_new_space_capacity_mb;
    if BLITZ_FLAGS.minor_ms {
      max_new_space_capacity_mb = BLITZ_FLAGS.minor_ms_max_new_space_capacity_mb;
    }
    let max_semi_space_size =
      max_new_space_capacity_mb * max_semi_space_capacity_base_unit;
    debug_assert_eq!(0, max_semi_space_size % (1 << PAGE_SIZE_BITS));
    max_semi_space_size
  }

  pub fn old_generation_to_semi_space_ratio() -> usize {
    debug_assert!(!BLITZ_FLAGS.minor_ms);
    let old_gen_to_semi_scope_ratio =
      128 * Heap::HEAP_LIMIT_MULTIPLIER / Heap::POINTER_MULTIPLIER;
    old_gen_to_semi_scope_ratio
  }

  pub fn old_generation_to_semi_space_ratio_low_memory() -> usize {
    let mut minor_ms = 1;
    if BLITZ_FLAGS.minor_ms {
      minor_ms = 2;
    }
    let old_gen_to_semi_scope_ratio_low_mem =
      256 * Heap::HEAP_LIMIT_MULTIPLIER / Heap::POINTER_MULTIPLIER;
    old_gen_to_semi_scope_ratio_low_mem / minor_ms
  }

  pub fn get_maximum_fill_to_align() {}
  pub fn get_fill_to_align() {}
  pub fn get_code_range_reserved_area_size() {}
  pub fn fatal_process_out_of_memory() {}
  pub fn is_valid_allocation_space() {}
  pub fn get_code_flush_mode() {}
  pub fn is_young_generation_collector() {}
  pub fn young_generation_collector() {}
}