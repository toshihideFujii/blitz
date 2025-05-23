#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use hlo::{hlo_computation::HloComputation, hlo_instruction::{HloPrintOptions, PrintSubcomputationMode}, hlo_module::HloModule};

// Deduplicate computations inside a HloModule.
// If two computations are identical then keep the first one (in postorder
// terms) and remove the rest.
pub struct HloComputationDeduplicator {
  mark_fusion_duplications: bool
}

impl HloComputationDeduplicator {
  pub fn new(mark_fusion_duplications: bool) -> Self {
    HloComputationDeduplicator {
      mark_fusion_duplications: mark_fusion_duplications
    }
  }

  pub fn name(&self) -> String {
    "computation-deduplicator".to_string()
  }
  
  pub fn run(
    &self,
    module: &HloModule,
    execution_threads: HashSet<String>) -> Result<bool, String>
  {
    let mut unique_comps: HashMap<String, HloComputation> = HashMap::new();
    let mut replacements: HashMap<HloComputation, HloComputation> = HashMap::new();

    let mut options = HloPrintOptions::canonical();
    options.set_print_subcomputation_mode(PrintSubcomputationMode::Off);
    options.set_print_infeed_outfeed_config(false);
    options.set_print_only_essential_constants(true);
    options.set_print_operand_shape(true);
    options.set_print_ids(false);
    options.set_canonicalize_computations(true);

    for comp in
      module.make_computation_post_order(&execution_threads, false) {
      if comp.is_entry_computation() || comp.instruction_count() > 128 ||
        self.contains_large_constants(comp) || comp.is_collective_called_computation()
      {
        continue;
      }
      let comp_str = comp.to_string_with_options(&options);
      let poss_dup = unique_comps.get(&comp_str);
      if poss_dup.is_some() { // TODO
        replacements.insert(comp.clone(), poss_dup.unwrap().clone());
      } else {
        unique_comps.insert(comp_str, comp.clone());
      }
    }

    if self.mark_fusion_duplications {
      module.mark_fusion_duplications(&replacements);
    } else {
      module.replace_computations(&replacements);
    }

    Ok(!replacements.is_empty())
  }

  fn contains_large_constants(&self, _comp: &HloComputation) -> bool {
    unimplemented!()
  }
}