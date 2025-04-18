#![allow(dead_code)]

// Set frontend attribute on 'instruction' which indices that in-place
// 'instruction' has disjoint read/write buffer regions.
pub fn set_disjoint_read_write_regions_attr() {}

// Returns 'true' if in-place 'instruction' has the BlitzDisjointReadWriteRegions
// frontend attribute set (returns false otherwise).
pub fn has_disjoinnt_read_write_regions_attr() {}