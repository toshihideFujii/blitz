#![allow(dead_code)]

use hlo::hlo_instruction::HloInstruction;

// Set frontend attribute on 'instruction' which indices that in-place
// 'instruction' has disjoint read/write buffer regions.
pub fn set_disjoint_read_write_regions_attr(instruction: &mut HloInstruction) {
  instruction.set_frontend_attribute(
    "_blitz_disjoint_read_write_regions".to_string(),
    "true".to_string());
}

// Returns 'true' if in-place 'instruction' has the BlitzDisjointReadWriteRegions
// frontend attribute set (returns false otherwise).
pub fn has_disjoinnt_read_write_regions_attr(instruction: &HloInstruction) -> bool {
  instruction.frontend_attributes()
    .has_attribute("_blitz_disjoint_read_write_regions".to_string())
}