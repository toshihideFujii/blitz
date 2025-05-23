#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::{shape::Shape, shape_util::ShapeUtil};
use hlo::{
  hlo_computation::HloComputation,
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode
};

use crate::{hlo_dce::HloDCE, hlo_pass_pipeline::HloPassPipeline};

// A pass that tries to lift concatenation out of a while loop, and replace
// piece-wise subcomputations in the loop body with one on the concatenated
// shape.
//
// For example:
//
// loop = while (a, b, c, d) {
//   e = concat(a, b)
//   f = some-op(e) <with the same shape as e>
//   s0 = slice(f) first half
//   s1 = slice(f) second half
//   a_1 = add(a, s0)
//   b_1 = add(b, s1)
//   a_new = add(a_1, c)
//   b_new = add(b_1, d)
//   c_new = add(a_new, c)
//   d_new = add(b_new, d)
//   ROOT tuple(a_new, b_new, c_new, d_new)
// }
//
// will be transformed to
//
// ab = concat(a, b)
// cd = concat(c, d)
// while (ab, cd) {
//   f = some-op(ab)
//   ab_1 = add(ab, f)
//   ab_new = add(ab_1, cd)
//   cd_new = add(ab_new, cd)
//   ROOT tuple(ab_new, cd_new)
// }
// a_new = slice(ab_new) first half
// b_new = slice(ab_new) second half
// c_new = slice(cd_new) first half
// d_new = slice(cd_new) second half
pub struct WhileLoopConcatCodeMotion {
  min_operand_count_to_optimize: i64
}

impl WhileLoopConcatCodeMotion {
  pub fn new(min_operand_count_to_optimize: i64) -> Self {
    WhileLoopConcatCodeMotion {
      min_operand_count_to_optimize: min_operand_count_to_optimize
    }
  }

  pub fn name(&self) -> String {
    "while-loop-concat-code-motion".to_string()
  }

  pub fn run(
    &mut self,
    module: &HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    let mut changed = false;
    for comp in
      module.make_computation_post_order(execution_threads, false) {
      for hlo in comp.mutable_make_instruction_post_order() {
        if hlo.opcode() == HloOpcode::While {
          let loop_changed =
            run_on_loop(hlo, self.min_operand_count_to_optimize);
          changed |= loop_changed.unwrap();
        }
      }
    }
    if changed {
      let mut pipeline = HloPassPipeline::new(
        "loop-concat-motion-cleanup".to_string(), None);
      // TODO
      pipeline.add_pass::<HloDCE>(None);
    }
    Ok(changed)
  }
}

// This algorithm tries to group HLO instructions into concat candidates. Each
// instruction can only belong to a single group.
//
// For simplicity, after finding the groups, it in-place updates the first group
// member to the full shape, and replaces non-grouped uses with slices of it.
// Then it relies on TupleSimplifier, WhileLoopSimplifier, and DCE passes to
// remove other elements.
//
// Represents a group of elements and how to concat them.
#[derive(Debug, Clone)]
struct ConcatGroup {
  pub elements: Vec<HloInstruction>,
  element_sizes: Vec<i64>,
  element_offsets: Vec<i64>,
  concat_dim: i64,
  inserted_concat_dim: bool,
}

impl ConcatGroup {
  pub fn new(
    elements: Vec<HloInstruction>, concat_dim: i64, inserted_concat_dim: bool) -> Self
  {
    let mut element_sizes: Vec<i64> = vec![];
    element_sizes.resize(elements.len(), 1);
    let mut element_offsets: Vec<i64> = vec![];
    element_offsets.resize(elements.len(), 0);

    let mut instance = ConcatGroup {
      elements: elements,
      element_sizes: element_sizes,
      element_offsets: element_offsets,
      concat_dim: concat_dim,
      inserted_concat_dim: inserted_concat_dim
    };

    if !inserted_concat_dim {
      for i in 0..instance.element_sizes.len() {
        instance.element_sizes[i] =
          instance.elements[i].shape().dimensions(concat_dim as usize);
        if i > 0 {
          instance.element_offsets[i] =
            instance.element_offsets[i - 1] + instance.element_sizes[i - 1];
        }
      }
    }
    instance
  }

  pub fn get_concat_shape(&mut self) -> Shape {
    if self.inserted_concat_dim {
      let mut dims: Vec<i64> = vec![];
      let element_shape = self.elements.last().unwrap().shape();
      dims.reserve(element_shape.rank() + 1);
      for i in 0..element_shape.rank() {
        if i == self.concat_dim as usize {
          dims.push(self.elements.len() as i64);
        }
        dims.push(element_shape.dimensions(i));
      }
      if dims.len() == self.concat_dim as usize {
        dims.push(self.elements.len() as i64);
      }
      return ShapeUtil::make_shape(&element_shape.element_type(), dims);
    } else {
      let mut dim_size = 0;
      for size in &self.element_sizes {
        dim_size += *size;
      }
      let shape = self.elements.last_mut().unwrap().mutable_shape();
      shape.set_dimensions(self.concat_dim as usize, dim_size);
      return shape.clone();
    }
  }

  pub fn create_slice(
    &self,
    _full_data: &HloInstruction,
    _element_index: i64,
    _comp: &HloComputation) -> HloInstruction
  {
    unimplemented!()
  }

  pub fn create_concat(
    &mut self, input_elements: Vec<HloInstruction>, comp: &mut HloComputation) //-> &HloInstruction
  {
    if self.inserted_concat_dim {
      for i in 0..input_elements.len() {
        let mut element_shape: Vec<i64> = vec![];
        element_shape.reserve(input_elements[i].shape().rank() + 1);
        for j in 0..input_elements[i].shape().rank() {
          if j == self.concat_dim as usize {
            element_shape.push(1);
          }
          element_shape.push(input_elements[i].shape().dimensions(j));
        }
        if element_shape.len() == self.concat_dim as usize {
          element_shape.push(1);
        }
        // TODO
      }
    }
    let instruction = HloInstruction::create_concatenate(
      &self.get_concat_shape(), input_elements, self.concat_dim);
    comp.add_instruction(instruction, "".to_string());
  }
}

// A collection of ConcatGroup's where each HLO can only belong to a single
// group.
struct ConcatGroups {
  element_to_group: HashMap<HloInstruction, (i64, i64)>,
  groups: Vec<ConcatGroup>,
  concat_disallowed: HashSet<HloInstruction>
}

impl ConcatGroups {
  pub fn new() -> Self {
    ConcatGroups {
      element_to_group: HashMap::new(),
      groups: Vec::new(),
      concat_disallowed: HashSet::new()
    }
  }

  // Returns the group index and element index in group for an HLO, if it
  // belongs to a group.
  pub fn get_group_index(&self, hlo: &HloInstruction) -> Option<&(i64, i64)> {
    let value = self.element_to_group.get(hlo);
    if value.is_none() {
      return None;
    }
    value
  }

  pub fn get_group(&self, index: i64) -> &ConcatGroup {
    self.groups.get(index as usize).unwrap()
  }

  // Creates a new group and returns the index if it doesn't exist, or returns
  // existing group index. If the new group doesn't match exactly with an
  // existing group but shared some of the elements, returns -1 as the index.
  // It also returns whether a new group is created. So the return value is a
  // pair of {whether created, group index}.
  pub fn maybe_create_new_group(&mut self, group: &ConcatGroup) -> (bool, i64) {
    let mut group_id = -1;
    let mut elements_debug = HashSet::new();
    for i in 0..group.elements.len() {
      if !elements_debug.insert(group.elements[i].clone()) {
        println!("Duplicates in group. Element: {:?}",
          group.elements[i].to_string_default());
      }
      if self.concat_disallowed.contains(&group.elements[i]) {
        println!("Failed creating group. Grouping disallowed on {:?}",
          group.elements[i].to_string_default());
        return (false, -1);
      }
      let existing = self.get_group_index(&group.elements[i]);
      if existing.is_some() &&
        (existing.unwrap().1 != i as i64 ||
         self.groups[existing.unwrap().0 as usize].concat_dim != group.concat_dim)
      {
        // We allow mismatched inserted_concat_dim, since that only requires a
        // trivial reshape.
        println!("Failed creating group. Different than existing group. Element: {:?}",
          group.elements[i].to_string_default());
        return (false, -1);
      }
      if i == 0 && existing.is_some() {
        group_id = existing.unwrap().0;
      }
      if i > 0 {
        if existing.is_some() && existing.unwrap().0 != group_id {
          println!("Failed creating group. Different than existing group. Element: {:?}",
            group.elements[i].to_string_default());
          return (false, -1);
        }
        if existing.is_none() && group_id >= 0 {
          println!("Failed creating group. Different than existing group. Element: {:?}",
            group.elements[i].to_string_default());
          return (false, -1);
        }
      }
    }

    if group_id >= 0 {
      println!("Group already exists at {:?} for {:?}",
        group_id, group.elements[0].to_string_default());
      return (false, group_id);
    }
    let index = self.groups.len();
    for i in 0..group.elements.len() {
      self.element_to_group.insert(group.elements[i].clone(), (index as i64, i as i64));
    }
    println!("Created new group at {:?} for {:?}, concat_dim: {:?}, inserted: {:?}",
      index, group.elements[0].to_string_default(), group.concat_dim, group.inserted_concat_dim);
    self.groups.push(group.clone());
    (true, index as i64)
  }

  pub fn groups(&self) -> &Vec<ConcatGroup> {
    &self.groups
  }

  pub fn next_group_index(&self) -> usize {
    self.groups.len()
  }

  pub fn remove_tailing_groups(&mut self, start_index: i64) {
    while self.groups.len() > start_index as usize {
      for element in &self.groups.last().unwrap().elements {
        self.element_to_group.remove(element);
      }
      self.groups.pop();
    }
  }

  pub fn disallow_grouping_on(&mut self, hlo: HloInstruction) {
    println!("Disallow grouping on {:?}", hlo.to_string_default());
    self.concat_disallowed.insert(hlo);
  }
}

// Infers an operand's concat dim and whether it's an inserted dim. For example,
// if hlo is f32[2,4,2] broadcast(f32[2,4]), dimensions={0,1} concatenated on
// dim 2, then this function will return {2, true}.
//
// If the operand is already transformed to the combined shape, specify its
// group in combined_operand_group. (Only required for kReshape.)
#[allow(unused_assignments)]
fn get_operand_concat_dim(
  hlo: &HloInstruction,
  operand_index: usize,
  hlo_concat_dim: i64,
  hlo_inserted_concat_dim: bool,
  combined_operand_group: Option<&ConcatGroup>) -> Option<(i64, bool)>
{
  if hlo.is_elementwise() || hlo.opcode() == HloOpcode::AllReduce {
    return Some((hlo_concat_dim, hlo_inserted_concat_dim));
  }
  let mut operand_concat_dim = -1;
  let mut operand_inserted_concat_dim = false;
  let mut operand_shape = hlo.operand(operand_index).shape();
  if combined_operand_group.is_some() {
    operand_shape = combined_operand_group.unwrap().elements.last().unwrap().shape();
  }

  if hlo.opcode() == HloOpcode::Broadcast {
    operand_concat_dim = 0;
    operand_inserted_concat_dim = true;
    // Try to place operand_concat_dim adjacent to dims the same way as the
    // output, if it does not exist in the operand..
    let mut min_dist_to_concat_dim = hlo.shape().rank() as i64;
    for i in 0..operand_shape.rank() {
      if hlo.dimensions_number(i as i64) == hlo_concat_dim {
        operand_concat_dim = i as i64;
        operand_inserted_concat_dim = hlo_inserted_concat_dim;
        break;
      }
      if hlo.dimensions_number(i as i64) < hlo_concat_dim &&
        min_dist_to_concat_dim as i64 > hlo_concat_dim - hlo.dimensions_number(i as i64)
      {
        operand_concat_dim = (i + 1) as i64;
        min_dist_to_concat_dim = hlo_concat_dim - hlo.dimensions_number(i as i64);
      }
      if hlo.dimensions_number(i as i64) > hlo_concat_dim &&
        min_dist_to_concat_dim as i64 > hlo.dimensions_number(i as i64) - hlo_concat_dim
      {
        operand_concat_dim = i as i64;
        min_dist_to_concat_dim = hlo.dimensions_number(i as i64) - hlo_concat_dim;
      }
    }
  } else if hlo.opcode() == HloOpcode::Reduce {
    if operand_index != 0 {
      return None;
    }
    operand_concat_dim = hlo_concat_dim;
    operand_inserted_concat_dim = hlo_inserted_concat_dim;
    let mut sorted_reduce_dims: HashSet<i64> = HashSet::new();
    for dim in hlo.dimensions() {
      sorted_reduce_dims.insert(*dim);
    }
    for dim in &sorted_reduce_dims {
      if (hlo_inserted_concat_dim && *dim < operand_concat_dim) ||
         (!hlo_inserted_concat_dim && *dim <= operand_concat_dim)
      {
        operand_concat_dim += 1;
      }
    }
  } else if hlo.opcode() == HloOpcode::Reshape {
    let mut i = 0;
    let mut j = 0;
    operand_inserted_concat_dim = false;
    // Only support adding/removing trivial dims.
    while i < operand_shape.rank() || j <= hlo_concat_dim {
      if i < operand_shape.rank() && j < hlo.shape().rank() as i64 &&
        operand_shape.dimensions(i) == hlo.shape().dimensions(j as usize)
      {
        if j == hlo_concat_dim {
          operand_inserted_concat_dim =
            hlo_inserted_concat_dim && operand_shape.dimensions(i) != 1;
          operand_concat_dim = i as i64;
          break;
        }
        i += 1;
        j += 1;
        continue;
      }
      if i < operand_shape.rank() && operand_shape.dimensions(i) == 1 {
        if j == hlo_concat_dim && hlo_inserted_concat_dim {
          operand_concat_dim = i as i64;
          break;
        }
        i += 1;
        continue;
      }
      if j == hlo_concat_dim {
        operand_concat_dim = i as i64;
        operand_inserted_concat_dim = true;
        break;
      }
      if j < hlo.shape().rank() as i64 && hlo.shape().dimensions(j as usize) == 1 {
        j += 1;
        continue;
      }
      return None;
    }
  } else {
    return None;
  }
  assert!(operand_concat_dim >= 0);
  Some((operand_concat_dim, operand_inserted_concat_dim))
}

fn modify_hlo_properties_for_concat_shape(_group: &ConcatGroup, _hlo: &HloInstruction) {
  unimplemented!()
}

// Main method to assign groups to HLOs, based on a concat.
fn group_hlos_for_concat(
  _body: &HloComputation,
  _concat: &HloInstruction,
  _topological_order: &HashMap<&HloInstruction, i64>,
  _groups: &ConcatGroups) -> bool
{
  unimplemented!()
}

fn tuple_elements_used_in_cond(loop_: &HloInstruction) -> Vec<bool> {
  let mut result = vec![false; loop_.shape().tuple_shapes_size()];
  for user in
    loop_.while_condition().parameter_instruction(0).unwrap().users()
  {
    if user.opcode() != HloOpcode::GetTupleElement {
      result.fill(true);
      return result;
    }
    result[user.tuple_index() as usize] = true;
  }
  result
}

// Adds copies to returned values to keep RewriteLoopWithConcatGroups simple:
// the copies do not have other users and only appear once in the root tuple.
fn add_copies_to_root(
  _body: &HloComputation,
  _param_gtes: &Vec<Option<HloInstruction>>,
  _groups: &ConcatGroups) -> Result<(), String>
{
  unimplemented!()
}

fn remove_copies_from_root(body: &mut HloComputation) -> Result<(), String>
{
  let root = body.mutable_root_instruction();
  assert_eq!(root.opcode(), HloOpcode::Tuple);
  for i in 0..root.operand_count() {
    let copy = root.mutable_operand(i);
    if copy.unwrap().opcode() == HloOpcode::Copy {
      //let result =
        //root.replace_operand_with(i as i64, 
          //copy.unwrap().mutable_operand(0).unwrap().clone());
    }
  }
  Ok(())
}

fn rewrite_loop_with_concat_groups(
  _loop_: &HloInstruction,
  _param_gtes: &Vec<Option<HloInstruction>>,
  _groups: &ConcatGroups) -> Result<(), String>
{
  unimplemented!()
}

fn run_on_loop(
  loop_: &mut HloInstruction,
  _min_operand_count_to_optimize: i64) -> Result<bool, String>
{
  let body = loop_.mutable_while_body();
  let param = body.parameter_instruction(0).unwrap();
  let root = body.root_instruction();
  if !param.shape().is_tuple() || root.opcode() != HloOpcode::Tuple {
    return Ok(false);
  }
/*
  let mut gtes: Vec<Option<HloInstruction>> =
    vec![None; param.shape().tuple_shapes_size()];
  let mut groups = ConcatGroups::new();
  let indices_used_in_cond = tuple_elements_used_in_cond(loop_);
  for user in param.users() {
    if user.opcode() != HloOpcode::GetTupleElement {
      // Unhandled user opcode.
      return Ok(false);
    }
    let idx = user.tuple_index();
    if gtes[idx as usize].is_some() {
      // Seen this index before.
      return Ok(false);
    }
    gtes[idx as usize] = Some(user.clone());
    if indices_used_in_cond[idx as usize] {
      groups.disallow_grouping_on(user.clone());
    }
  }

  let mut concats: Vec<Option<HloInstruction>> = vec![];
  let body_instructions = body.make_instruction_post_order();
  let mut topological_order: HashMap<&HloInstruction, i64> = HashMap::new();
  for i in 0..body_instructions.len() {
    let hlo = &body_instructions[i];
    topological_order.insert(hlo, i as i64);
    if hlo.opcode() == HloOpcode::Concatenate &&
       hlo.operand_count() >= min_operand_count_to_optimize as usize
    {
      concats.push(Some(hlo.clone()));
    }
  }

  for concat in &mut concats {
    if !group_hlos_for_concat(
      body, &concat.as_ref().unwrap().clone(), &topological_order, &groups)
    {
      *concat = None;
    }
  }
  if groups.groups().is_empty() {
    return Ok(false);
  }

  let mut result =
    add_copies_to_root(body, &gtes, &groups);
  if result.is_err() {
    return Err(result.err().unwrap());
  }
  result = rewrite_loop_with_concat_groups(loop_, &gtes, &groups);
  if result.is_err() {
    return Err(result.err().unwrap());
  }
  for concat in &mut concats {
    if *concat == None { continue; }
    // We have repalced the operands of the concat with slices of full data.
    let new_slice = concat.as_mut().unwrap().mutable_operand(0);
    assert_eq!(new_slice.unwrap().opcode(), HloOpcode::Slice);
    //let result = concat.as_mut().unwrap().replace_all_uses_with(
      //new_slice.unwrap().mutable_operand(0).unwrap(), "".to_string());
    //let result = body.remove_instruction(&concat.unwrap().clone());
    //if result.is_err() {
      //return Err(result.err().unwrap());
    //}
  }
  result = remove_copies_from_root(body);

  for gte in &gtes {
    // TODO
  }
*/
  Ok(true)
}