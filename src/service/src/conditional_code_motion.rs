#![allow(dead_code)]

use std::collections::HashMap;

use hlo::hlo_instruction::HloInstruction;

#[derive(Debug, Clone, PartialEq)]
enum Position {
  InsideBranch,
  OutsideBranchUser,
  OutsideBranchOperand,
  Undefined,
}

struct Boundary {
  operands: Vec<HloInstruction>,
  position: Position
}

impl Boundary {
  pub fn new(position: Position) -> Self {
    Boundary { operands: Vec::new(), position: position }
  }

  pub fn mutable_operands(&mut self) -> &mut Vec<HloInstruction> {
    &mut self.operands
  }

  pub fn operands(&self) -> &Vec<HloInstruction> {
    &self.operands
  }

  pub fn is_inside_branch(&self) -> bool {
    self.position == Position::InsideBranch
  }

  pub fn is_outside_branch_user(&self) -> bool {
    self.position == Position::OutsideBranchUser
  }

  pub fn is_outside_branch_operand(&self) -> bool {
    self.position ==Position::OutsideBranchOperand
  }

  pub fn get_position(&self) -> &Position {
    &self.position
  }

  pub fn is_empty(&self) -> bool {
    self.operands.is_empty()
  }

  pub fn to_string() {}
}

enum Direction {
  MoveOutOfBranch,
  MoveIntoBranch,
  NoChange,
}

struct Decision {
  direction: Direction,
  benefit: i64
}

impl Decision {
  pub fn new(direction: Direction, benefit: i64) -> Self {
    Decision { direction: direction, benefit: benefit }
  }

  pub fn get_direction(&self) -> &Direction {
    &self.direction
  }

  pub fn get_benefit(&self) -> i64 {
    self.benefit
  }
}

pub struct ConditionalCodeMotion {
  is_layout_sensitive: bool,
  pursue_full_conditional_code_motion: bool,
  search_config: Vec<i64>,
  search_config_index: i64,
  search_config_map: HashMap<i64, Vec<i64>>,
  move_config: Vec<Vec<i64>>,
  reuse_config: Vec<Vec<i64>>,
  memory_increase_allowance: i64,
  memory_increase: i64
}

impl ConditionalCodeMotion {
  const MAX_POS: i64 = 16;
  const START_POS: i64 = 0;
  const STRIDE_POS: i64 = 32;
  const VALUE_MASK: i64 = 0xffff;

  pub fn new() {}
  pub fn parse_search_configuration() {}
  pub fn make_search_config() {}
  pub fn flip_start() {}
  pub fn flip_stride() {}
  pub fn decrement_max_flip() {}

  pub fn name() -> String { "conditional-code-motion".to_string() }
  pub fn run() {}
}