#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use common::{shape_tree::ShapeTree};
use crate::{
  dfs_hlo_visitor::DfsHloVisitorBase, dfs_hlo_visitor_with_default::DfsHloVisitorWithDefault,
  hlo_computation::HloComputation, hlo_instruction::HloInstruction, hlo_module::HloModule,
  hlo_opcode::HloOpcode, hlo_value::HloPosition
};

struct SendRecvGroup {
  send: HloInstruction,
  recv: HloInstruction,
}

pub struct SendRecvGroupMap {
  host_transfer_rendezvous_map: HashMap<String, SendRecvGroup>
}

impl SendRecvGroupMap {
  pub fn new(_hlo_module: &HloModule) -> Self {
    unimplemented!()
  }

  pub fn get_matching_send_or_recv(
    &self, _send_or_recv: &HloInstruction) -> Result<HloInstruction, String>
  {
    unimplemented!()    
  }
}

pub struct HloPreOrderDFS {
  stack: Vec<HloInstruction>,
  visited: HashSet<HloInstruction>
}

impl HloPreOrderDFS {
  pub fn default() -> Self {
    HloPreOrderDFS { stack: Vec::new(), visited: HashSet::new() }
  }

  pub fn run(
    &self,
    _computation: &HloInstruction,
    _visitor: DfsHloVisitorBase) -> Result<(), String>
  {
    unimplemented!()
  }

  fn is_ready(&self, _instruction: &HloInstruction) -> bool {
    unimplemented!()
  }
}

// The einsum depth is the length of the einsum dependency chain. And we
// distinguish instructions that are used by root and that are not used by
// root.
// The einsum depth of an HLO value A is defined as follows:
// for B = op(A, ...)
// 1) the root instruction has a depth of 0;
// 2) non-root instructions that have zero users have a depth of -1;
// 3) if op is a Dot or Convolution (i.e., einsum),
//    depth(A, B) = depth(B) >= 0 ? depth(B) + 1 : depth(B) - 1.
//    depth(A, B) means the depth of A because of B;
// 4) otherwise depth(A, B) = depth(B);
// 5) depth(A) is computed by merging all depth(A, u) where u is a user of A.
//    See MergeDepth for how user depths are merged.

pub struct EinsumDepthAnalysis {
  base: DfsHloVisitorWithDefault,
  einsum_depth_map: HashMap<HloInstruction, ShapeTree<i64>>,
  send_recv_group_map: SendRecvGroupMap
}

impl EinsumDepthAnalysis {
  pub fn new() {
    unimplemented!()
  }

  pub fn run(
    &self,
    _computation: &HloComputation,
    _send_recv_group_map: &SendRecvGroupMap) -> Result<EinsumDepthAnalysis, String>
  {
    unimplemented!()    
  }

  fn run_internal(
    &self,
    _computation: &HloComputation,
    _root_depth: &ShapeTree<i64>) -> Result<(), String>
  {
    unimplemented!()    
  }

  pub fn default_action(
    &mut self, instruction: &mut HloInstruction) -> Result<(), String>
  {
    let mut depth_tree = self.get_depth_tree_or_die(instruction);
    let max_depth = get_max_depth(&mut depth_tree);
    for operand_index in 0..instruction.operand_count() {
      let operand =
        instruction.mutable_operand(operand_index).unwrap();
      let result =
        self.set_instruction_depth(operand, max_depth);
      check_error(&result);
    }
    Ok(())
  }

  pub fn handle_tuple(&self, tuple: &HloInstruction) -> Result<(), String> {
    self.handle_tuple_like(tuple)
  }

  pub fn handle_get_tuple_element(
    &self, get_tuple_element: &HloInstruction) -> Result<(), String>
  {
    self.base.handle_get_tuple_element(get_tuple_element)    
  }

  pub fn handle_dot(&mut self, dot: &mut HloInstruction) -> Result<(), String> {
    self.handle_depth_increment_instruction(dot)
  }

  pub fn handle_custom_call(
    &mut self, custom_call: &mut HloInstruction) -> Result<(), String>
  {
    if custom_call.custom_call_target().starts_with("sparse_dense_matmul") {
      return self.handle_dot(custom_call);
    }
    self.default_action(custom_call)
  }

  pub fn handle_convolution(
    &mut self, convolution: &mut HloInstruction) -> Result<(), String>
  {
    self.handle_dot(convolution)
  }

  pub fn handle_ragged_dot(
    &mut self, ragged_dot: &mut HloInstruction) -> Result<(), String>
  {
    self.handle_dot(ragged_dot)    
  }

  pub fn handle_call(&self, call: &HloInstruction) -> Result<(), String> {
    let depth_tree = self.get_depth_tree_or_die(call);
    self.handle_called_computation(
      &call.called_computations()[0],
      &depth_tree,
      call.operands())
  }

  pub fn handle_fusion(&self, fusion: &HloInstruction) -> Result<(), String> {
    let depth_tree = self.get_depth_tree_or_die(fusion);
    self.handle_called_computation(
      &fusion.called_computations()[0],
      &depth_tree,
      fusion.operands())
  }

  pub fn handle_while(&self, blitz_while: &HloInstruction) -> Result<(), String> {
    self.base.handle_while(blitz_while)
  }

  pub fn handle_conditional(
    &mut self, conditional: &HloInstruction) -> Result<(), String>
  {
    let depth_tree =
      self.get_depth_tree_or_die(conditional);
    // Conditionals have one more operand than the number of branches. The first
    // operand is the pred.
    //let result = self.set_instruction_depth_by_tree(
      //&conditional.operands()[0], &depth_tree);
    //check_error(&result);
    for i in 0..conditional.branch_count() {
      let result = self.handle_called_computation(
        &conditional.called_computations()[i],
        &depth_tree,
        &vec![conditional.operands()[i+1].clone()]);
      check_error(&result);
    }
    Ok(())    
  }

  pub fn handle_after_all(
    &mut self, after_all: &mut HloInstruction) -> Result<(), String>
  {
    let mut depth_tree =
      self.get_depth_tree_or_die(after_all);
    let max_depth = get_max_depth(&mut depth_tree);
    for operand in after_all.mutable_operands() {
      debug_assert!(operand.shape().is_token());
      let result =
        self.set_instruction_depth(operand, max_depth);
      check_error(&result);
    }
    Ok(())
  }

  pub fn handle_send(&mut self, send: &mut HloInstruction) -> Result<(), String> {
    let mut depth_tree = self.get_depth_tree_or_die(send);
    let send_buffer = send.mutable_operand(0).unwrap();

    #[allow(unused_assignments)]
    let mut send_buffer_depth = None;
    if self.has_depth_tree(send_buffer) {
      send_buffer_depth = Some(self.get_depth_tree_or_die(send_buffer));
    } else {
      send_buffer_depth = Some(self.create_and_insert_depth_tree(send_buffer));
    }
    set_depth_from_tuple_depth(
      &send_buffer_depth.unwrap(), &depth_tree, 0);
    let max_depth = get_max_depth(&mut depth_tree);
    let token = send.mutable_operand(1).unwrap();
    self.set_instruction_depth(token, max_depth)
  }

  pub fn handle_recv(&self, _recv: &HloInstruction) -> Result<(), String> {
    /*
    let depth_tree = self.get_depth_tree_or_die(recv);
    let send_wrapper =
      self.send_recv_group_map.get_matching_send_or_recv(recv);
    check_error(&send_wrapper);

    let send = send_wrapper.ok().unwrap();
    let send_depth = self.get_or_create_depth_tree(&send);
    let max_depth = get_max_depth(&depth_tree);

    let func = |index: &Vec<i64>, depth: &mut i64| {
      if !send_depth.is_leaf(index) { return; }
      if *index.first().unwrap() == 0 {
        *depth = merge_depth(*depth, *depth_tree.element(index));
        return;
      }
      *depth = merge_depth(*depth, max_depth);
    };
    send_depth.for_each_mutable_element(&mut func);
    let after_all = recv.mutable_operand(0).unwrap();
    self.set_instruction_depth(after_all, max_depth)
    */
    Ok(())
  }

  pub fn handle_send_done(&mut self, send_done: &mut HloInstruction) -> Result<(), String> {
    let mut depth_tree = self.get_depth_tree_or_die(&send_done);
    let mut send = send_done.mutable_operand(0).unwrap();
    let max_depth = get_max_depth(&mut depth_tree);
    self.set_instruction_depth(&mut send, max_depth)
  }

  pub fn handle_recv_done(&self, _recv_done: &mut HloInstruction) -> Result<(), String> {
    /*
    let depth_tree = self.get_depth_tree_or_die(recv_done);
    let max_depth = get_max_depth(&depth_tree);
    let recv = recv_done.mutable_operand(0).unwrap();
    let mut recv_depth = self.get_depth_tree_or_die(recv);

    let mut func = |index: &Vec<i64>, depth: &mut i64| {
      if !recv_depth.is_leaf(index) { return; }
      if *index.first().unwrap() == 0 {
        *depth = merge_depth(*depth, *depth_tree.element(index));
        return;
      }
      *depth = merge_depth(*depth, max_depth);
    };
    recv_depth.for_each_mutable_element(&mut func);
    */
    Ok(())
  }

  pub fn handle_all_reduce(
    &mut self, all_reduce: &mut HloInstruction) -> Result<(), String>
  {
    if all_reduce.shape().is_array() {
      return self.default_action(all_reduce);
    }
    self.handle_tuple_like(all_reduce)
  }

  pub fn handle_async_start(
    &self, async_start: &HloInstruction) -> Result<(), String>
  {
    let depth_tree =
      self.get_depth_tree_or_die(async_start);
    let output_depth_tree =
      depth_tree.sub_shape_tree(&vec![1]);
    check_error(&output_depth_tree);
    self.handle_called_computation(
      async_start.async_wrapped_computation(),
      &output_depth_tree.unwrap(),
      async_start.operands())
  }

  pub fn handle_async_done(
    &self, _async_done: &mut HloInstruction) -> Result<(), String>
  {
    /*
    let depth_tree =
      self.get_depth_tree_or_die(async_done);
    let async_start = async_done.mutable_operand(0).unwrap();
    let mut async_start_depth =
      self.get_or_create_depth_tree(async_start);

    let mut func = |index: &Vec<i64>, depth: &mut i64| {
      if !async_start_depth.is_leaf(index) { return; }
      if *index.first().unwrap() == 1 {
        // TODO
        *depth = merge_depth(*depth, *depth_tree.element(index));
      }
    };
    async_start_depth.for_each_mutable_element(&mut func);
    */
    Ok(())
  }

  pub fn get_einsum_depth_map(&self) -> &HashMap<HloInstruction, ShapeTree<i64>> {
    &self.einsum_depth_map
  }

  fn create_and_insert_depth_tree(
    &mut self, instruction: &mut HloInstruction) -> ShapeTree<i64>
  {
    let new_depth_tree =
      ShapeTree::new_with_value(instruction.mutable_shape(), -1);
    self.einsum_depth_map.insert(instruction.clone(), new_depth_tree.clone());
    new_depth_tree
  }

  fn get_depth_tree_or_die(
    &self, instruction: &HloInstruction) -> ShapeTree<i64>
  {
    let depth = self.einsum_depth_map.get(instruction);
    debug_assert!(depth.is_some(), "No depth tree dound for instruction");
    depth.unwrap().clone()
  }

  fn has_depth_tree(&self, instruction: &HloInstruction) -> bool {
    self.einsum_depth_map.get(instruction).is_some()
  }

  fn set_instruction_depth(
    &mut self, instruction: &mut HloInstruction, depth: i64) -> Result<(), String>
  {
    #[allow(unused_assignments)]
    let mut depth_tree = None;
    if self.has_depth_tree(instruction) {
      depth_tree = Some(self.get_depth_tree_or_die(instruction));
    } else {
      depth_tree = Some(self.create_and_insert_depth_tree(instruction));
    }
    set_depth(&mut depth_tree.unwrap(), depth);
    Ok(())
  }

  fn set_instruction_depth_by_tree(
    &mut self,
    instruction: &mut HloInstruction,
    depth: &ShapeTree<i64>) -> Result<(), String>
  {
    #[allow(unused_assignments)]
    let mut depth_tree = None;
    if self.has_depth_tree(instruction) {
      depth_tree = Some(self.get_depth_tree_or_die(instruction));
    } else {
      depth_tree = Some(self.create_and_insert_depth_tree(instruction))
    }
    set_depth_by_tree(depth_tree.as_ref().unwrap(), depth);
    Ok(())
  }

  fn set_instruction_depth_from_tuple_depth(
    &self,
    _instruction: &HloInstruction,
    _tuple_depth_tre: ShapeTree<i64>,
    _tuple_index: i64) -> Result<(), String>
  {
    unimplemented!()    
  }

  fn handle_depth_increment_instruction(
    &mut self, instruction: &mut HloInstruction) -> Result<(), String>
  {
    let depth_tree = self.get_depth_tree_or_die(instruction);
    let instruction_depth = depth_tree.element(&vec![]);
    for operand in instruction.mutable_operands() {
      let mut depth = instruction_depth - 1;
      if *instruction_depth >= 0 { depth = *instruction_depth + 1; }
      let result =
        self.set_instruction_depth(operand, depth);
      check_error(&result);
    }
    Ok(())
  }

  fn handle_called_computation(
    &self,
    computation: &HloComputation,
    root_depth: &ShapeTree<i64>,
    operands: &Vec<HloInstruction>) -> Result<(), String>
  {
    let result = self.run_internal(computation, root_depth);
    check_error(&result);
    for i in 0..operands.len() {
      let _operand = &operands[i];
      let parameter = computation.parameter_instruction(i);
      let mut _param_depth = None;
      if self.has_depth_tree(parameter.unwrap()) {
        _param_depth = Some(self.get_depth_tree_or_die(parameter.unwrap()));
      } else {
        //param_depth = Some(self.create_and_insert_depth_tree(parameter.unwrap()));
      }
      //let result = self.set_instruction_depth_by_tree(
        //operand, param_depth.as_ref().unwrap());
      //check_error(&result);
    }
    Ok(())
  }

  fn handle_tuple_like(&self, _tuple_like: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()    
  }
}

fn merge_depth(original_depth: i64, new_depth: i64) -> i64 {
  // If the instruction has users that are dependent upon by the root, its depth
  // is set by the max of all its users that are dependence of the root.
  if new_depth >= 0 {
    return i64::max(original_depth, new_depth);
  }
  // If the instruction's user is not dependent upon by the root, it affects
  // the depth of the instruction only if all users of the instruction are not
  // ancestors of the root.
  if new_depth < 0 && original_depth < 0 {
    return i64::min(original_depth, new_depth);
  }
  original_depth
}

fn set_depth(_depth_tree: &mut ShapeTree<i64>, _depth: i64) {
  /*
  let mut func =
    |shape_index: &Vec<i64>, depth_ptr: &mut i64|
  {
    if depth_tree.is_leaf(shape_index) {
      *depth_ptr = merge_depth(*depth_ptr, depth);
    }
  };
  depth_tree.for_each_mutable_element(&mut func);
  */
}

fn set_height(_height_tree: &mut ShapeTree<i64>, _height: i64) {
  unimplemented!()
}

fn set_depth_by_tree(_depth_tree: &ShapeTree<i64>, _depth: &ShapeTree<i64>) {
    
}

fn get_max_depth(depth_tree: &mut ShapeTree<i64>) -> i64 {
  let mut max_depth  = -1;
  let mut func =
    |_shape_index: &Vec<i64>, depth: &mut i64|
  {
    max_depth = i64::max(max_depth, *depth);
  };
  depth_tree.for_each_mutable_element(&mut func);
  if max_depth >= 0 {
    return max_depth;
  }
  let mut func =
    |_shape_index: &Vec<i64>, depth: &mut i64|
  {
    max_depth = i64::min(max_depth, *depth);
  };
  depth_tree.for_each_mutable_element(&mut func);
  max_depth
}

fn get_max_operand_height(
  _instruction: &HloInstruction,
  _einsum_height_map: &HashMap<HloInstruction, ShapeTree<i64>>) -> i64
{
  unimplemented!()    
}

fn set_depth_from_tuple_depth(
  _depth_tree: &ShapeTree<i64>,
  _tuple_depth_tree: &ShapeTree<i64>,
  _tuple_index: i64)
{
  unimplemented!()
}

// Einsum height is the maximum number of einsums between this instruction and
// any leaf.
pub struct EinsumHeightAnalysis {
  base: DfsHloVisitorWithDefault,
  einsum_height_map: HashMap<HloInstruction, ShapeTree<i64>>
}

impl EinsumHeightAnalysis {
  pub fn new() {
    unimplemented!()
  }

  pub fn run(
    &self,
    _computation: &HloComputation,
    _send_recv_group_map: &SendRecvGroupMap) -> Result<EinsumHeightAnalysis, String>
  {
    unimplemented!()
  }

  fn run_internal(
    &self,
    _computation: &HloComputation,
    _operands: &Vec<HloInstruction>) -> Result<(), String>
  {
    unimplemented!()    
  }

  pub fn default_action(
    &mut self, instruction: &mut HloInstruction) -> Result<(), String>
  {
    let instruction_height =
      get_max_operand_height(instruction, &self.einsum_height_map);
    self.set_instruction_height(instruction, instruction_height)
  }

  pub fn handle_tuple(&self, tuple: &HloInstruction) -> Result<(), String> {
    self.handle_tuple_like(tuple)
  }

  pub fn handle_get_tuple_element(
    &mut self, get_tuple_element: &mut HloInstruction) -> Result<(), String>
  {
    let mut _height_tree = None;
    if self.has_height_for(get_tuple_element) {
      _height_tree = Some(self.get_height_tree_or_die(
        get_tuple_element));
    } else {
      _height_tree = Some(&self.create_and_insert_height_tree(
        get_tuple_element));
    }

    let _tuple_height_tree = self.get_height_tree_or_die(
      get_tuple_element.operand(0));
    let _tuple_index = get_tuple_element.tuple_index();
    //set_height
    Ok(())
  }

  pub fn handle_dot(&mut self, dot: &mut HloInstruction) -> Result<(), String> {
    self.handle_heihgt_increment_instruction(dot)
  }

  pub fn handle_custom_call(
    &mut self, custom_call: &mut HloInstruction) -> Result<(), String>
  {
    if custom_call.custom_call_target().starts_with("sparse_dense_matmul") {
      return self.handle_dot(custom_call);
    }
    self.default_action(custom_call)
  }

  pub fn handle_convolution(
    &mut self, convolution: &mut HloInstruction) -> Result<(), String>
  {
    self.handle_dot(convolution)
  }

  pub fn handle_ragged_dot(
    &mut self, ragged_dot: &mut HloInstruction) -> Result<(), String>
  {
    self.handle_dot(ragged_dot)    
  }

  pub fn handle_call(&mut self, _call: &mut HloInstruction) -> Result<(), String> {
    /*
    let result = self.handle_called_computation(
      &call.called_computations()[0], &call.mutable_operands());
    check_error(&result);

    let root_height_tree = self.get_height_tree_or_die(
      call.called_computations()[0].root_instruction());
    //let result = self.set_instruction_height_by_tree(
      //&mut call, root_height_tree);
    //check_error(&result);
    */
    Ok(())
  }

  pub fn handle_fusion(&mut self, fusion: &mut HloInstruction) -> Result<(), String> {
    self.handle_call(fusion)
  }

  pub fn handle_while(&mut self, _blitz_while: &mut HloInstruction) -> Result<(), String> {
    /*
    let result = self.handle_called_computation(
      blitz_while.while_condition(), blitz_while.mutable_operands());
    check_error(&result);

    let result = self.handle_called_computation(
      blitz_while.while_body(), blitz_while.mutable_operands());
    check_error(&result);

    let root_height_tree = self.get_height_tree_or_die(
      blitz_while.while_body().root_instruction());
    self.set_instruction_height_by_tree(blitz_while, root_height_tree)
    */
    unimplemented!()
  }

  pub fn handle_conditional(
    &mut self, conditional: &mut HloInstruction) -> Result<(), String>
  {
    let mut _height_tree = None;
    if self.has_height_for(conditional) {
      _height_tree = Some(self.get_height_tree_or_die(conditional));
    } else {
      _height_tree = Some(&self.create_and_insert_height_tree(conditional));
    }

    for i in 0..conditional.branch_count() {
      let computation = conditional.branch_computation(i);
      //let result = self.handle_called_computation(
        //computation, conditional.mutable_operands()[i+1]);
      //check_error(&result);

      let _branch_root_height_tree =
        self.get_height_tree_or_die(computation.root_instruction());
      //set_height(height_tree.unwrap(), branch_root_height_tree);
    }
    Ok(())
  }

  pub fn handle_after_all(
    &self, after_all: &HloInstruction) -> Result<(), String>
  {
    self.base.handle_after_all(after_all)    
  }

  pub fn handle_send(&mut self, send: &mut HloInstruction) -> Result<(), String> {
    let send_buffer = send.mutable_operand(0).unwrap();
    let _send_buffer_height_tree =
      self.get_height_tree_or_die(&send_buffer);

    let mut _height_tree = None;
    if self.has_height_for(send) {
      _height_tree = Some(self.get_height_tree_or_die(send));
    } else {
      _height_tree = Some(&self.create_and_insert_height_tree(send));
    }
    //set_height(height_tree.unwrap(), height);
    Ok(())
  }

  pub fn handle_recv(&self, recv: &HloInstruction) -> Result<(), String> {
    self.base.handle_recv(recv)
  }

  pub fn handle_send_done(
    &mut self, send_done: &mut HloInstruction) -> Result<(), String>
  {
    let mut _target = None;
    if self.has_height_for(send_done) {
      _target = Some(self.get_height_tree_or_die(send_done));
    } else {
      _target = Some(&self.create_and_insert_height_tree(send_done));
    }
    Ok(())
  }

  pub fn handle_recv_done(&mut self, recv_done: &mut HloInstruction) -> Result<(), String> {
    let recv = recv_done.mutable_operand(0).unwrap();
    let _recv_height_tree = self.get_height_tree_or_die(recv);
    let mut _height_tree = None;
    if self.has_height_for(recv_done) {
      _height_tree = Some(self.get_height_tree_or_die(recv_done));
    } else {
      _height_tree = Some(&self.create_and_insert_height_tree(recv_done));
    }
    //set_height
    Ok(())
  }

  pub fn handle_all_reduce(
    &mut self, all_reduce: &mut HloInstruction) -> Result<(), String>
  {
    if all_reduce.shape().is_array() {
      return self.default_action(all_reduce);
    }
    self.handle_tuple_like(all_reduce)
  }

  pub fn handle_async_start(
    &self, _async_start: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()  
  }

  pub fn handle_async_done(
    &self, _async_done: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()    
  }

  pub fn get_einsum_depth_map(&self) -> HashMap<HloInstruction, ShapeTree<i64>> {
    unimplemented!()
  }

  fn handle_called_computation(
    &self,
    _computation: &HloComputation,
    _operands: &Vec<HloInstruction>) -> Result<(), String>
  {
    /*
    if !operands.is_empty() {
      if computation.num_parameters() != operands.len() {
        let mut err_msg = operands.len().to_string();
        err_msg.push_str(" operands were passed for the computation ");
        err_msg.push_str(&computation.name());
        err_msg.push_str(" with ");
        err_msg.push_str(&computation.num_parameters().to_string());
        err_msg.push_str(" parameters");
        return Err(err_msg);
      }
      for param_index in 0..computation.num_parameters() {
        let param =
          computation.parameter_instruction(param_index);
        let operand = &operands[param_index];
        //let operand_height_tree = self.get
      }
    }
    for inst in computation.instructions() {
      if inst.user_count() == 0 {
        //inst.accept_visitor(self.base);
      }
    }
    Ok(())
    */
    unimplemented!()
  }

  fn get_height_tree_or_die(
    &self, instruction: &HloInstruction) -> &ShapeTree<i64>
  {
    let height = self.einsum_height_map.get(instruction);
    debug_assert!(height.is_some());
    height.unwrap()
  }

  fn create_and_insert_height_tree(
    &mut self, instruction: &mut HloInstruction) -> ShapeTree<i64>
  {
    let new_height_tree =
      ShapeTree::new_with_value(instruction.mutable_shape(), -1);
    self.einsum_height_map.insert(instruction.clone(), new_height_tree.clone());
    new_height_tree
  }

  fn has_height_for(&self, instruction: &HloInstruction) -> bool {
    self.einsum_height_map.contains_key(instruction)
  }

  fn set_instruction_height(
    &mut self, instruction: &mut HloInstruction, _height: i64) -> Result<(), String>
  {
    let mut _height_tree = None;
    if self.has_height_for(instruction) {
      _height_tree = Some(self.get_height_tree_or_die(instruction));
    } else {
      _height_tree = Some(&self.create_and_insert_height_tree(instruction));
    }
    //set_height(height_tree.unwrap(), height);
    Ok(())
  }

  fn set_instruction_height_by_tree(
    &mut self, _instruction: &mut HloInstruction, _height: &ShapeTree<i64>) -> Result<(), String>
  {
    unimplemented!()    
  }

  fn handle_heihgt_increment_instruction(
    &mut self, instruction: &mut HloInstruction) -> Result<(), String>
  {
    let mut _height_tree = None;
    if self.has_height_for(instruction) {
      _height_tree = Some(self.get_height_tree_or_die(instruction));
    } else {
      _height_tree = Some(&self.create_and_insert_height_tree(instruction));
    }
    for operand in instruction.mutable_operands() {
      let _operand_height_tree =
        self.get_height_tree_or_die(operand);
      //set_height(&mut height_tree.unwrap(),
      //operand_height_tree.element(&vec![]) + 1);
    }
    Ok(())
  }

  fn handle_tuple_like(&self, _tuple_like: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HloValueSemanticLabel {
  Static,
  Random,
  Weight,
  Activation,
  ActivationGradient,
  WeightGradient,
  TupleOrToken,
}

impl Default for HloValueSemanticLabel {
  fn default() -> Self {
    HloValueSemanticLabel::Static
  }
}

pub fn hlo_value_semantic_label_to_string(
  label: &HloValueSemanticLabel) -> String
{
  match label {
    HloValueSemanticLabel::Static => return "Static".to_string(),
    HloValueSemanticLabel::Random => return "Random".to_string(),
    HloValueSemanticLabel::Weight => return "Weight".to_string(),
    HloValueSemanticLabel::Activation => return "Activation".to_string(),
    HloValueSemanticLabel::ActivationGradient => return "ActivationGradient".to_string(),
    HloValueSemanticLabel::WeightGradient => return "WeightGradient".to_string(),
    HloValueSemanticLabel::TupleOrToken => return "TupleOrToken".to_string()
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HloValueSemantics {
  id: i64,
  label: HloValueSemanticLabel,
  origin: HloPosition
}

impl HloValueSemantics {
  pub fn new(id: i64, label: HloValueSemanticLabel, origin: HloPosition) -> Self {
    HloValueSemantics { id: id, label: label, origin: origin }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn label(&self) -> HloValueSemanticLabel {
    self.label.clone()
  }

  pub fn origin(&self) -> &HloPosition {
    &self.origin
  }

  pub fn to_string(&self) -> String {
    let mut out = "{".to_string();
    out.push_str("label: ");
    out.push_str(&hlo_value_semantic_label_to_string(&self.label));
    out.push_str(", ");
    out.push_str("origin: ");
    out.push_str(&self.origin.to_string());
    out.push_str("}");
    out
  }
}

pub struct HloValueSemanticsAnalysis<'module> {
  module: &'module HloModule,
  next_id: i64,
  value_semantics: HashMap<HloInstruction, ShapeTree<HloValueSemantics>>,
  einsum_depth_map: HashMap<HloInstruction, ShapeTree<i64>>,
  einsum_height_map: HashMap<HloInstruction, ShapeTree<i64>>,
  send_recv_group_map: SendRecvGroupMap,
  execution_threads: HashSet<String>
}

impl<'module> HloValueSemanticsAnalysis<'module> {
  pub fn new() {
  }

  pub fn run(
    &self,
    _module: &HloModule,
    _execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    unimplemented!()
  }

  pub fn has_semantics_for(&self, instruction: &HloInstruction) -> bool {
    self.value_semantics.contains_key(instruction)
  }

  pub fn get_semantics(
    &self, _instruction: &HloInstruction, _index: &Vec<i64>) -> HloValueSemantics
  {
    unimplemented!()
  }

  pub fn get_semantics_map(
    &self) -> &HashMap<HloInstruction, ShapeTree<HloValueSemantics>>
  {
    &self.value_semantics
  }

  pub fn get_einsum_depth_map(
    &self) -> &HashMap<HloInstruction, ShapeTree<i64>>
  {
    &self.einsum_depth_map
  }

  pub fn get_einsum_height_map(
    &self) -> &HashMap<HloInstruction, ShapeTree<i64>>
  {
    &self.einsum_height_map
  }

  pub fn get_depth(
    &self, instruction: &HloInstruction, index: &Vec<i64>) -> i64
  {
    let depth = self.einsum_depth_map.get(instruction);
    debug_assert!(depth.is_some());
    *depth.unwrap().element(index)
  }

  pub fn get_height(
    &self, instruction: &HloInstruction, index: &Vec<i64>) -> i64
  {
    let height = self.einsum_height_map.get(instruction);
    debug_assert!(height.is_some());
    *height.unwrap().element(index)
  }

  pub fn get_send_recv_group_map(&self) -> &SendRecvGroupMap {
    &self.send_recv_group_map
  }

  pub fn get_matching_send_or_recv(
    &self, send_or_recv: &HloInstruction) -> Result<HloInstruction, String>
  {
    self.send_recv_group_map.get_matching_send_or_recv(send_or_recv)
  }

  fn initialize_einsum_depth(&self) -> Result<(), String> {
    unimplemented!()
  }

  fn initialize_einsum_height(&self) -> Result<(), String> {
    unimplemented!()
  }

  fn initialize_send_recv_groups(&self) {
    unimplemented!()
  }

  fn annotate_weights(&self) {
    unimplemented!()
  }

  fn run_on_computation_by_operands(
    &self,
    computation: &HloComputation,
    operands: &Vec<HloInstruction>) -> Result<(), String>
  {
    debug_assert_eq!(computation.num_parameters(), operands.len());
    for i in 0..computation.num_parameters() {
      let semantics =
        self.value_semantics.get(&operands[i]);
      debug_assert!(semantics.is_some());
      self.deep_copy_hlo_value_semantics(
        &computation.parameter_instructions()[i],
        semantics.unwrap(),
        &vec![]
      );
    }
    self.run_on_computation(computation)
  }

  fn run_on_computation(&self, computation: &HloComputation) -> Result<(), String> {
    if HloInstruction::is_thread_included(
      computation.execution_thread(), &self.execution_threads)
    {
      let propagation =
        HloValueSemanticsPropagation::new(self);
      return propagation.run(computation);
    }
    Ok(())
  }

  fn next_id(&self) -> i64 {
    self.next_id
  }

  fn new_hlo_value_semantics(
    &self, _label: HloValueSemanticLabel, _origin: HloPosition) -> HloValueSemantics
  {
    unimplemented!()    
  }

  fn get_instruction_semantics(
    &self, instruction: &HloInstruction) -> &ShapeTree<HloValueSemantics>
  {
    let semantics =
      self.value_semantics.get(instruction);
    debug_assert!(semantics.is_some());
    semantics.unwrap()
  }

  fn deep_copy_hlo_value_semantics(
    &self,
    _target: &HloInstruction,
    _copy_from: &ShapeTree<HloValueSemantics>,
    _source_index: &Vec<i64>)
  {
    unimplemented!()
  }

  fn set_hlo_value_semantics(
    &mut self,
    target: &HloInstruction,
    semantics: ShapeTree<HloValueSemantics>)
  {
    let value_emantic =
      self.value_semantics.get(target);
    if value_emantic.is_some() {
      self.delete_hlo_value_semantics(value_emantic.unwrap());
    }
    self.value_semantics.insert(target.clone(), semantics);
  }

  fn delete_hlo_value_semantics(&self, _to_delete: &ShapeTree<HloValueSemantics>) {
    unimplemented!()
  }
}

pub struct EinsumAndOperandIndex {
  einsum: HloInstruction,
  operand_index: i64,
}

pub struct HloValueSemanticsPropagation<'module> {
  analysis: &'module HloValueSemanticsAnalysis<'module>
}

impl<'module> HloValueSemanticsPropagation<'module> {
  pub fn new(analysis: &'module HloValueSemanticsAnalysis<'module>) -> Self {
    HloValueSemanticsPropagation { analysis: analysis }
  }

  pub fn run(&self, _computation: &HloComputation) -> Result<(), String> {
    unimplemented!()
  }

  pub fn default_action(&self, _instruction: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_parameter(&self, _parameter: &HloInstruction) -> Result<(), String> {
    Ok(())
  }

  pub fn handle_constant(
    &mut self, constant: &mut HloInstruction) -> Result<(), String>
  {
    let constant_semantics = self.analysis.new_hlo_value_semantics(
      HloValueSemanticLabel::Static,
      HloPosition::new(constant.clone(), vec![]));
    
    let _semantics_shape_tree = ShapeTree::new_with_value(
      constant.mutable_shape(), constant_semantics);
    
    //self.analysis.set_hlo_value_semantics(constant, semantics_shape_tree);
    Ok(())
  }

  pub fn handle_iota(&self, iota: &mut HloInstruction) -> Result<(), String> {
    let semantics = self.analysis.new_hlo_value_semantics(
      HloValueSemanticLabel::Static,
      HloPosition::new(iota.clone(), vec![]));
    
    let _semantics_shape_tree = ShapeTree::new_with_value(
      iota.mutable_shape(), semantics);
    
    //self.analysis.set_hlo_value_semantics(iota, semantics_shape_tree);
    Ok(())
  }

  pub fn handle_partition_id(
    &self, partition_id: &mut HloInstruction) -> Result<(), String>
  {
    let semantics = self.analysis.new_hlo_value_semantics(
      HloValueSemanticLabel::Static,
      HloPosition::new(partition_id.clone(), vec![]));
    
    let _semantics_shape_tree = ShapeTree::new_with_value(
      partition_id.mutable_shape(), semantics);
    
    //self.analysis.set_hlo_value_semantics(iota, semantics_shape_tree);
    Ok(())
  }

  pub fn handle_replica_id(&self, replica_id: &mut HloInstruction) -> Result<(), String> {
    let semantics = self.analysis.new_hlo_value_semantics(
      HloValueSemanticLabel::Static,
      HloPosition::new(replica_id.clone(), vec![]));
    
    let _semantics_shape_tree = ShapeTree::new_with_value(
      replica_id.mutable_shape(), semantics);
    
    //self.analysis.set_hlo_value_semantics(iota, semantics_shape_tree);
    Ok(())
  }

  pub fn handle_clamp(&self, clamp: &HloInstruction) -> Result<(), String> {
    let operand_semantics =
      self.analysis.get_instruction_semantics(clamp.operand(1));
    self.analysis.deep_copy_hlo_value_semantics(
      clamp, operand_semantics, &vec![]);
    Ok(())
  }

  pub fn handle_tuple(&self, tuple: &HloInstruction) -> Result<(), String> {
    self.handle_tuple_like(tuple)
  }

  pub fn handle_get_tuple_element(&self, _get_tuple_element: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_call(&self, _call: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_fusion(&self, fusion: &HloInstruction) -> Result<(), String> {
    self.handle_call(fusion)
  }

  pub fn handle_custom_call(&self, _custom_call: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_sparse_dense_matmul(&self, _instruction: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_while(&self, _blitz_while: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_conditional(&self, _conditional: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_select(&self, _select: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_concatenate(&self, concatenate: &HloInstruction) -> Result<(), String> {
    let operand_semantics =
      self.analysis.get_instruction_semantics(concatenate.operand(9));
    self.analysis.deep_copy_hlo_value_semantics(
      concatenate, operand_semantics, &vec![]);
    Ok(())
  }

  pub fn handle_dynamic_slice(&self, _dynamic_slice: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_dynamic_update_slice(
    &self, _dynamic_update_slice: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()
  }

  pub fn handle_copy_start(&self, copy_start: &HloInstruction) -> Result<(), String> {
    self.handle_collective_or_copy_start(copy_start)
  }

  pub fn handle_copy_done(&self, copy_done: &HloInstruction) -> Result<(), String> {
    self.handle_collective_or_copy_done(copy_done)
  }

  pub fn handle_all_gather_start(
    &self, all_gather_start: &HloInstruction) -> Result<(), String>
  {
    self.handle_collective_or_copy_start(all_gather_start)
  }

  pub fn handle_all_gather_done(
    &self, all_gather_done: &HloInstruction) -> Result<(), String>
  {
    self.handle_collective_or_copy_done(all_gather_done)
  }

  pub fn handle_collective_permute_start(
    &self, collective_permute_start: &HloInstruction) -> Result<(), String>
  {
    self.handle_collective_or_copy_start(collective_permute_start)
  }

  pub fn handle_collective_permute_done(
    &self, collective_permute_done: &HloInstruction) -> Result<(), String>
  {
    self.handle_collective_or_copy_done(collective_permute_done)
  }

  pub fn handle_gather(&self, gather: &HloInstruction) -> Result<(), String> {
    let op_sem_shape_tree =
      self.analysis.get_instruction_semantics(gather.operand(0));
    self.analysis.deep_copy_hlo_value_semantics(
      gather, op_sem_shape_tree, &vec![]);
    Ok(())
  }

  pub fn handle_scatter(&mut self, scatter: &mut HloInstruction) -> Result<(), String> {
    let semantics =
      self.compute_semantics_from_operands(
        scatter, &vec![0, 2], &vec![]);
    check_error(&semantics);

    self.add_semantics(semantics.as_ref().unwrap());
    let _semantics_shape_tree =
      ShapeTree::new_with_value(scatter.mutable_shape(), semantics.unwrap());
    //self.analysis.set_hlo_value_semantics(scatter, semantics_shape_tree);
    Ok(())
  }

  pub fn handle_after_all(&self, after_all: &mut HloInstruction) -> Result<(), String> {
    let semantics = self.analysis.new_hlo_value_semantics(
      HloValueSemanticLabel::TupleOrToken,
      HloPosition::new(after_all.clone(), vec![]));
    
    let _semantics_shape_tree = ShapeTree::new_with_value(
      after_all.mutable_shape(), semantics);
    //self.analysis.set_hlo_value_semantics(after_all, semantics_shape_tree);
    Ok(())
  }

  pub fn handle_all_reduce(&self, all_reduce: &HloInstruction) -> Result<(), String> {
    if all_reduce.shape().is_array() {
      return self.default_action(all_reduce);
    }
    if all_reduce.shape().is_tuple() {
      return self.handle_tuple_like(all_reduce);
    }
    Ok(())
  }

  pub fn handle_async_start(&self, _async_start: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_async_done(&self, async_done: &HloInstruction) -> Result<(), String> {
    let operand_semantics_tree =
      self.analysis.get_instruction_semantics(async_done.operand(0));
    self.analysis.deep_copy_hlo_value_semantics(
      async_done, operand_semantics_tree, &vec![1]);
    Ok(())
  }

  pub fn handle_infeed(&self, _infeed: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  pub fn handle_outfeed(&self, outfeed: &mut HloInstruction) -> Result<(), String> {
    let semantics = self.analysis.new_hlo_value_semantics(
      HloValueSemanticLabel::TupleOrToken,
    HloPosition::new(outfeed.clone(), vec![]));

    let _outfeed_semantics_tree = ShapeTree::new_with_value(
      outfeed.mutable_shape(), semantics);
    //self.analysis.set_hlo_value_semantics(outfeed, outfeed_semantics_tree);
    Ok(())
  }

  pub fn handle_domain(&self, domain: &mut HloInstruction) -> Result<(), String> {
    let domain_operand = domain.mutable_operand(0);
    let operand_semantics =
      self.analysis.get_instruction_semantics(domain_operand.unwrap());
    self.analysis.deep_copy_hlo_value_semantics(
      domain, operand_semantics, &vec![]);
    Ok(())
  }

  pub fn handle_optimization_barrier(
    &self, opt_barrier: &mut HloInstruction) -> Result<(), String>
  {
    let opt_barrier_operand = opt_barrier.mutable_operand(0);
    let operand_semantics =
      self.analysis.get_instruction_semantics(opt_barrier_operand.unwrap());
    self.analysis.deep_copy_hlo_value_semantics(
      opt_barrier, operand_semantics, &vec![]);
    Ok(())
  }

  pub fn handle_rng_bit_generator(
    &self, rng_bit_generator: &mut HloInstruction) -> Result<(), String>
  {
    let semantics = self.analysis.new_hlo_value_semantics(
      HloValueSemanticLabel::Static,
      HloPosition::new(rng_bit_generator.clone(), vec![]));
    
    let _semantics_shape_tree = ShapeTree::new_with_value(
      rng_bit_generator.mutable_shape(), semantics);
    
    //self.analysis.set_hlo_value_semantics(iota, semantics_shape_tree);
    Ok(())
  }

  pub fn handle_send(&self, send: &mut HloInstruction) -> Result<(), String> {
    let semantics_tree = ShapeTree::new_with_value(
      send.mutable_shape(), HloValueSemantics::default());
    let source_buffer = send.mutable_operand(0);

    let _source_buffer_semantics =
      self.analysis.get_instruction_semantics(source_buffer.unwrap());
    //self.analysis.deep_copy_hlo_value_semantics(target, copy_from, source_index);
    
    let mut _func =
      |index: &Vec<i64>, semantics: &mut HloValueSemantics|
    {
      if !index.is_empty() {
        if index.first() == Some(&1) && semantics_tree.is_leaf(index) {

        }
        if index.first() == Some(&0) {
          return;
        }
      }
      let mut index_clone = vec![];
      index_clone.clone_from(index);
      *semantics = self.analysis.new_hlo_value_semantics(
        HloValueSemanticLabel::TupleOrToken,
        HloPosition::new(send.clone(), index_clone));
    };
    //semantics_tree.for_each_mutable_element(&mut func);
    //self.analysis.set_hlo_value_semantics(send, semantics_tree);
    Ok(())
  }

  pub fn handle_recv(&self, recv: &mut HloInstruction) -> Result<(), String> {
    let send =
      self.analysis.get_matching_send_or_recv(recv).unwrap();
    //send.accept(visitor, call_finish_visit, ignore_control_predecessors, cross_computation)
    let _semantic_tree = ShapeTree::new_with_value(
      recv.mutable_shape(), HloValueSemantics::default());
    let _send_buffer_semantics =
      self.analysis.get_instruction_semantics(&send);
    //self.analysis.deep_copy_hlo_value_semantics(target, copy_from, source_index);

    let _func =
      |index: &Vec<i64>, semantics: &mut HloValueSemantics|
    {
      if !index.is_empty() {
        if index.first() == Some(&0) {
          return;
        }
      }
      let mut index_clone = vec![];
      index_clone.clone_from(index);
      *semantics = self.analysis.new_hlo_value_semantics(
        HloValueSemanticLabel::TupleOrToken,
        HloPosition::new(recv.clone(), index_clone));
    };
    //semantic_tree.for_each_mutable_element(&mut func);
    //self.analysis.set_hlo_value_semantics(recv, semantic_tree);
    Ok(())
  }

  pub fn handle_send_done(&self, send_done: &mut HloInstruction) -> Result<(), String> {
    let semantics = self.analysis.new_hlo_value_semantics(
      HloValueSemanticLabel::TupleOrToken,
      HloPosition::new(send_done.clone(), vec![]));
    let _send_done_semantics_tree = ShapeTree::new_with_value(
      send_done.mutable_shape(), semantics);
    //self.analysis.set_hlo_value_semantics(send_done, send_done_semantics_tree);
    Ok(())
  }

  pub fn handle_recv_done(&self, _recv_done: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  fn copy_semantics(&self, semantics: &HloValueSemantics) -> HloValueSemantics {
    HloValueSemantics::new(0, semantics.label(), semantics.origin().clone())
  }

  fn copy_semantics_with_new_origin(
    &self,
    semantics: &HloValueSemantics,
    new_origin: &HloInstruction,
    index: Vec<i64>) -> HloValueSemantics
  {
    HloValueSemantics::new(0, semantics.label(),
    HloPosition::new(new_origin.clone(), index))
  }

  fn add_semantics(&self, semantics: &HloValueSemantics) {
    self.analysis.new_hlo_value_semantics(semantics.label(), semantics.origin().clone());
  }

  // Checks if the origin of `semantics` is an einsum that takes
  // `origin_dependence` as an operand.
  // If `recursive` is set to true, recursively checks all ancestors of the
  // `semantics`' origin (including itself) for the above condition.
  // Returns all such einsums and the operand index corresponding to
  // `origin_dependence`.
  // We use this function to find whether the output of an einsum who has an
  // operand X is used in another einsum who takes X as an operand. This is
  // the pattern for gradient.
  // For example, consider C = einsum(A, B), dC / dB = einsum(A, C).
  fn find_einsums_where_origin_depends_on_other(
    &self,
    _semantics: &HloValueSemantics,
    _origin_dependence: &HloPosition,
    _recursive: bool) -> Vec<EinsumAndOperandIndex>
  {
    unimplemented!()    
  }

  fn origin_depends_on(
    &self,
    semantics: &HloValueSemantics,
    origin_dependence: &HloPosition,
    recursive: bool) -> bool
  {
    let dependent_einsums =
      self.find_einsums_where_origin_depends_on_other(
        semantics, origin_dependence, recursive);
    !dependent_einsums.is_empty()
  }

  fn maybe_create_gradient_sementics(
    &self,
    _gradient_candidate: &HloInstruction,
    _fallback_label: HloValueSemanticLabel) -> Result<HloValueSemantics, String>
  {
    unimplemented!()    
  }

  fn compute_semantics_from_static_and_other(&self,
    static_semantics: &HloValueSemantics,
    other_semantics: &HloValueSemantics,
    instruction: &HloInstruction) -> Result<HloValueSemantics, String>
  {  
    debug_assert_eq!(static_semantics.label(), HloValueSemanticLabel::Static);
    if other_semantics.label() == HloValueSemanticLabel::Static {
      return Ok(self.copy_semantics_with_new_origin(
        other_semantics, instruction, vec![]));
    }
    if is_dot_convolution(instruction) &&
      other_semantics.label() == HloValueSemanticLabel::ActivationGradient
    {
      return self.maybe_create_gradient_sementics(
        instruction,
        HloValueSemanticLabel::ActivationGradient);
    }
    Ok(self.copy_semantics(other_semantics))
  }

  fn compute_semantics_from_random_and_other(&self,
    random_semantics: &HloValueSemantics,
    other_semantics: &HloValueSemantics,
    instruction: &HloInstruction) -> Result<HloValueSemantics, String>
  {
    debug_assert_eq!(random_semantics.label(), HloValueSemanticLabel::Random);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Static);
    if other_semantics.label() == HloValueSemanticLabel::Random {
      return Ok(self.copy_semantics_with_new_origin(
        other_semantics, instruction, vec![]));
    }
    Ok(self.copy_semantics(other_semantics))
  }

  fn compute_semantics_from_weight_and_other(&self) {
      
  }

  fn compute_semantics_from_activation_and_other(
    &self,
    activation_semantics: &HloValueSemantics,
    other_semantics: &HloValueSemantics,
    instruction: &HloInstruction) -> Result<HloValueSemantics, String>
  {
    debug_assert_eq!(activation_semantics.label(), HloValueSemanticLabel::Activation);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Static);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Random);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Weight);

    if !is_dot_convolution(instruction) {
      if activation_semantics.origin() == other_semantics.origin() {
        return Ok(self.copy_semantics(other_semantics));
      }
      return Ok(self.copy_semantics_with_new_origin(
        other_semantics, instruction, vec![]));
    }
    if other_semantics.label() == HloValueSemanticLabel::Activation {
      // Like said above, since loss is classified as Activation, an einsum
      // between an Activation X and an Activation Y could be WeightGradient if
      // either X or Y is the loss. This case is different from other Activation
      // einsums because there must a dependency between X and Y.
      let other_depends_on_activation = self.origin_depends_on(
        other_semantics,
        activation_semantics.origin(),
        true);
      let activation_depends_on_other = self.origin_depends_on(
        activation_semantics,
        other_semantics.origin(),
        true);
      debug_assert!(!other_depends_on_activation || !activation_depends_on_other);
      if other_depends_on_activation || activation_depends_on_other {
        return self.maybe_create_gradient_sementics(
          instruction,
          HloValueSemanticLabel::Activation);
      }
      return Ok(self.copy_semantics_with_new_origin(
        other_semantics, instruction, vec![]));
    }
    if other_semantics.label() == HloValueSemanticLabel::ActivationGradient {
      return self.maybe_create_gradient_sementics(
        instruction,
        HloValueSemanticLabel::ActivationGradient);
    }
    debug_assert_eq!(other_semantics.label(), HloValueSemanticLabel::WeightGradient);
    Ok(self.copy_semantics(other_semantics))
  }

  fn compute_semantics_from_activation_gradient_and_other(
    &self,
    activation_gradient_semantics: &HloValueSemantics,
    other_semantics: &HloValueSemantics,
    instruction: &HloInstruction) -> Result<HloValueSemantics, String>
  {
    debug_assert_eq!(activation_gradient_semantics.label(),
      HloValueSemanticLabel::ActivationGradient);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Static);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Random);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Weight);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Activation);
    if other_semantics.label() == HloValueSemanticLabel::ActivationGradient {
      if other_semantics.origin() == activation_gradient_semantics.origin() {
        return Ok(self.copy_semantics(activation_gradient_semantics));
      }
      return Ok(self.copy_semantics_with_new_origin(
        other_semantics, instruction, vec![]));
    }
    debug_assert_eq!(other_semantics.label(), HloValueSemanticLabel::WeightGradient);
    Ok(self.copy_semantics(other_semantics))
  }

  fn compute_semantics_from_weight_gradient_and_other(
    &self,
    weight_gradient_semantics: &HloValueSemantics,
    other_semantics: &HloValueSemantics,
    _instruction: &HloInstruction) -> Result<HloValueSemantics, String>
  {
    debug_assert_eq!(weight_gradient_semantics.label(),
      HloValueSemanticLabel::WeightGradient);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Static);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Random);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Weight);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::Activation);
    debug_assert_ne!(other_semantics.label(), HloValueSemanticLabel::ActivationGradient);

    Ok(self.copy_semantics(weight_gradient_semantics))
  }

  fn merge_semantics_for_an_instruction(
    &self,
    _instruction: &HloInstruction,
    _semantics_vec: &Vec<HloValueSemantics>) -> Result<HloValueSemantics, String>
  {
    unimplemented!()    
  }

  fn compute_semantics_from_operands(
    &self,
    instruction: &HloInstruction,
    operand_indices: &Vec<i64>,
    operand_shape_indices: &Vec<Vec<i64>>) -> Result<HloValueSemantics, String>
  {
    debug_assert!(!operand_indices.is_empty());
    debug_assert!(operand_shape_indices.is_empty() ||
      operand_indices.len() == operand_shape_indices.len());

    let mut semantics_vec: Vec<HloValueSemantics> = vec![];
    for op_index in operand_indices {
      let operand = instruction.operand(*op_index as usize);
      let mut shape_index = vec![];
      if !operand_shape_indices.is_empty() {
        shape_index.clone_from(&operand_shape_indices[*op_index as usize]);
      }
      let operand_semantics = self.analysis.get_semantics(
        operand, &shape_index);
      //let operand_height =
        //self.analysis.get_einsum_height_map().get(operand);
      //debug_assert!(operand_height.is_some());
      semantics_vec.push(operand_semantics);
    }
    self.merge_semantics_for_an_instruction(instruction, &semantics_vec)
  }

  fn handle_tuple_like(&self, _tuple_like: &HloInstruction) -> Result<(), String> {
    unimplemented!()
  }

  fn handle_collective_or_copy_start(
    &self, _op_start: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()
  }

  fn handle_collective_or_copy_done(
    &self, _op_done: &HloInstruction) -> Result<(), String>
  {
    unimplemented!()    
  }
}

fn is_dot_convolution(instruction: &HloInstruction) -> bool {
  if instruction.opcode() == HloOpcode::CustomCall &&
    instruction.custom_call_target().starts_with("sparse_dense_mutmal")
  {
    return true;
  }
  instruction.opcode() == HloOpcode::Dot ||
  instruction.opcode() == HloOpcode::Convolution ||
  instruction.opcode() == HloOpcode::RaggedDot
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}