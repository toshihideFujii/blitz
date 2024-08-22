#![allow(dead_code)]

use std::collections::HashSet;

use common::{
  permutation_util::{inverse_permutation, is_identity_permutation},
  shape::ShapeEqual,
  shape_util::ShapeUtil
};

use hlo::{
  hlo_instruction::{HloInstruction, HloPrintOptions},
  hlo_module::HloModule,
  hlo_opcode::HloOpcode
};

use crate::hlo_creation_utils::{make_reshape_hlo, make_transpose_hlo};

pub struct ReshapeMoverOptions {
  reshape_of_1d_broadcast_is_cheap: bool
}

pub struct ReshapeMover {
  options: ReshapeMoverOptions
}

impl ReshapeMover {
  pub fn new(options: ReshapeMoverOptions) -> Self {
    ReshapeMover { options: options }
  }

  pub fn name(&self) -> String {
    "reshape-mover".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut changed = false;
    for comp in module.make_nonfusion_computations(execution_threads) {
      let mut candidates = HashSet::new();
      for instruction in comp.instructions() {
        if self.is_reshape_move_candidate(instruction) {
          candidates.insert(instruction.clone());
        }
      }
      let did_change = self.try_reshape_move_on_candidates(&candidates);
      if did_change.is_err() {
        return Err(did_change.err().unwrap());
      }
      changed |= did_change.unwrap();
    }

    Ok(changed)
  }

  // Reshape-moves all qualifying instructions in candidates.  Returns true if it
  // makes changes.
  //
  // `candidates` is a set of HloInstructions with rearrange operands, and a
  // instruction in the set can be reshape-moved iff all the users of its
  // rearrange operands can also be reshaped-moved.
  //
  // The algorithm here iteratively finds the rearrange operands with users that
  // are outside the set of `candidates`, and removes their users from
  // `candidates`, until either `candidates` becomes empty or none of the
  // remaining rearrange operands have users outside `candidates`.  In the later
  // case, all the remaining instructions in `candidates` are reshape-moved and
  // the routine returns true.
  fn try_reshape_move_on_candidates(
    &self, _candidates: &HashSet<HloInstruction>) -> Result<bool, String>
  {
    unimplemented!()
  }

  // Actually performs the reshape-move transformation -- that is, sinks the
  // reshape or transpose operands of `instruction` across it.
  fn sink_rearrange_operands(&self, instruction: &mut HloInstruction) -> Result<bool, String> {
    let mut print_no_metadata = HloPrintOptions::new();
    print_no_metadata.set_print_metadata(false);

    let _computation = instruction.parent();
    let rearrange =
      self.first_nontrivial_rearrange(instruction.operands());
    assert!(rearrange.is_some());

    let new_operand_shape = rearrange.as_ref().unwrap().operand(0).shape();
    println!("** Sinking reshape or transpose: {:?}", instruction.to_string(&print_no_metadata));
    println!("first rearrange operand: {:?}", rearrange.as_ref().unwrap().to_string(&print_no_metadata));
    println!("new operand shape: {:?}", ShapeUtil::human_string(new_operand_shape));

    let t = instruction.shape().element_type();
    let _new_shape = ShapeUtil::change_element_type(
      new_operand_shape, &t);
    let operands = instruction.mutable_operands();
    for i in 0..operands.len() {
      println!("Updating operand # {:?}: {:?}", i, operands[i].to_string(&print_no_metadata));
      let result =
        self.apply_inverse_rearrange(rearrange.as_ref().unwrap(), &operands[i]);
      if result.is_err() {
        return Err(result.err().unwrap());
      }
      operands[i] = result.unwrap();
      println!("Updated operand # {:?} to: {:?}", i, operands[i].to_string(&print_no_metadata));    
    }

    // TODO
    //let cloned = instruction.clone_with_new_opereands(
      //&new_shape, operands);

    Ok(true)
  }

  // Returns a reshape/transpose of `operand` according to the inverse of
  // `rearrange`.
  //
  // This will often create redundant operations that we expect to be eliminated
  // by algsimp.  For example, if we have an operand rearrange(x), this will
  // produce rearrange'(rearrange(x)), which can be simplified to x.
  fn apply_inverse_rearrange(
    &self,
    rearrange: &HloInstruction,
    operand: &HloInstruction) -> Result<HloInstruction, String>
  {
    match rearrange.opcode() {
      HloOpcode::Reshape => {
        let new_shape = ShapeUtil::change_element_type(
          rearrange.operand(0).shape(), &operand.shape().element_type());
        if *operand.shape() != new_shape {
          return make_reshape_hlo(&new_shape, operand);
        } else {
          return Ok(operand.clone());
        }
      }
      HloOpcode::Transpose => {
        if !is_identity_permutation(rearrange.dimensions()) {
          return make_transpose_hlo(operand, 
            &inverse_permutation(rearrange.dimensions()));
        } else {
          return Ok(operand.clone());
        }
      }
      _ => unreachable!("Invalid rearrange op: {:?}", rearrange.to_string_default())
    }
  }

  // Returns true if the instruction is a reshape-move candidate:
  //
  //   * at least one operand is a rearrange, and
  //   * all rearrange operands are equivalent (if there's more than one), and
  //   * we can trivially apply the inverse rearrange to all other operands.
  fn is_reshape_move_candidate(&self, instruction: &HloInstruction) -> bool {
    let mut print_no_metadata = HloPrintOptions::new();
    print_no_metadata.set_print_metadata(false);
    println!("** Checking instruction: {:?}", instruction.to_string(&print_no_metadata));

    // Only perform reshape-move for elementwise instructions.
    if !instruction.is_elementwise() { return false; }

    let rearrange =
      self.first_nontrivial_rearrange(instruction.operands());
    if rearrange.is_none() { return false; }

    for operand in instruction.operands() {
      if (is_rearrange(operand) &&
          are_equivalent_rearranges(operand, rearrange.as_ref().unwrap())) ||
         (!is_rearrange(operand) &&
          self.can_trivially_rearrange(operand, rearrange.as_ref().unwrap()))
      {
        continue;
      } else {
        return false;
      }
    }

    true
  }

  fn first_nontrivial_rearrange(&self, instrs: &Vec<HloInstruction>) -> Option<HloInstruction> {
    for instr in instrs {
      if is_rearrange(instr) &&
        !self.can_trivially_rearrange(instr.operand(0), instr)
      {
        return Some(instr.clone());
      }
    }
    None
  }

  // Returns true if `instr` can easily change its shape according to the inverse
  // of `rearrange`, which must be a kReshape or kTranspose op.
  fn can_trivially_rearrange(&self, instr: &HloInstruction, rearrange: &HloInstruction) -> bool {
    assert!(is_rearrange(rearrange));

    if rearrange.opcode() == HloOpcode::Reshape &&
      ShapeEqual::new().equal(rearrange.shape(), rearrange.operand(0).shape())
    {
      return true;
    }

    if rearrange.opcode() == HloOpcode::Transpose &&
      is_identity_permutation(rearrange.dimensions())
    {
      return true;
    }

    if instr.opcode() == HloOpcode::Constant {
      return true;
    }

    if instr.opcode() == HloOpcode::Rng && instr.user_count() == 1 {
      return true;
    }

    if instr.opcode() == HloOpcode::Broadcast {
      // TODO instr.dimensions().is_sorted()

      if rearrange.opcode() == HloOpcode::Reshape {
        // TODO
      }

      if rearrange.opcode() == HloOpcode::Transpose {
        // TODO
      }
    }

    false
  }
}

// In this file, let a "rearrange" op be a reshape or a transpose.
fn is_rearrange(instruction: &HloInstruction) -> bool {
  instruction.opcode() == HloOpcode::Reshape ||
  instruction.opcode() == HloOpcode::Transpose
}

// Returns whether `a` and `b` are equivalent reshapes/transposes.
fn are_equivalent_rearranges(a: &HloInstruction, b: &HloInstruction) -> bool {
  if a.opcode() != b.opcode() ||
     !ShapeUtil::same_dimensions(a.shape(), b.shape())
  {
    return false;
  }

  match a.opcode() {
    HloOpcode::Transpose => return a.dimensions() == b.dimensions(),
    HloOpcode::Reshape => return ShapeUtil::same_dimensions(
      a.operand(0).shape(), b.operand(0).shape()),
    _ => return false
  }
}

// Computes where broadcast dims end up after a transpose.
//
// Consider a simple case:
//
//  bcast = f32[1,2,3,4] broadcast(f32[2,4] x), dimensions={1,3}
//  trans = f32[2,3,1,4] transpose(f32[1,2,3,4] bcast), dimensions={1,2,0,3}.
//
// We want to transform this into
//
//  bcast' = f32[2,3,1,4] broadcast(f32[2,4] x), dimensions={0,3}.
//
// The algorithm is:
//
//  * Invert the permutation {1,2,0,3} to give us p' = {2,0,1,3}.
//
//  * Compute where each broadcast dim ends up after the transpose.  p'[1] = 0,
//    meaning that broadcast dim 1 (size 2) ends up at index 0 after the
//    transpose.  Similarly, p'[3] = 3.
//
// Thus the new broadcast's dims are [p'[dim] for dim in bcast.dimensions()].
fn transposed_bcast_dims(bcast_dims: &Vec<i64>, transpose_dims: &Vec<i64>) -> Vec<i64>{
  let inv_perm = inverse_permutation(transpose_dims);
  let mut new_bcast_dims = vec![];
  for dim in bcast_dims {
    new_bcast_dims.push(inv_perm[*dim as usize]);
  }
  new_bcast_dims
}