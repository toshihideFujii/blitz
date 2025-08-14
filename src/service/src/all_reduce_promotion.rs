
use std::collections::HashSet;
use common::{blitz_data::PrimitiveType, shape::Shape, shape_util::ShapeUtil};
use hlo::{hlo_computation::{HloComputation, HloComputationBuilder},
  hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};
use crate::change_op_data_type::ChangeOpDataType;

pub struct AllReducePromotion {
  pass: ChangeOpDataType
}

impl AllReducePromotion {
  pub fn new(from_to_types: Vec<(PrimitiveType, PrimitiveType)>) -> Self {
    AllReducePromotion {
      pass: ChangeOpDataType::new(from_to_types)
    }
  }

  pub fn name() -> String {
    "all-reduce-promotion".to_string()
  }

  pub fn run(
    &self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    self.pass.run(
      module,
      execution_threads,
      &is_all_reduce,
      &clone_all_reduce)
  }
}

fn is_all_reduce(inst: &HloInstruction) -> bool {
  inst.opcode() == HloOpcode::AllReduce ||
  inst.opcode() == HloOpcode::ReduceScatter
}

fn clone_all_reduce(
  inst: &mut HloInstruction,
  shape: &Shape,
  operands: &Vec<HloInstruction>) -> HloInstruction
{
  // clone an all-reduce or reduce-scatter and also clone the attached
  // computation to match the type.
  let mut new_inst = inst.clone_with_new_opereands(shape, operands);
  let to_apply = new_inst.to_apply();

  let mut to_apply_promoted = || -> HloComputation {
    let t = shape.element_type();
    let mut name = to_apply.name();
    name.push_str("_promoted");

    let mut promoted = HloComputationBuilder::new(name);
    let _x = promoted.add_instruction(
      HloInstruction::create_parameter(0,
        &ShapeUtil::make_shape(&t, vec![]),
        "x".to_string()));

    let _y = promoted.add_instruction(
      HloInstruction::create_parameter(1,
        &ShapeUtil::make_shape(&t, vec![]),
        "y".to_string()));

    //promoted.add_instruction(
      //HloInstruction::create_binary(
        //&ShapeUtil::make_shape(&t, vec![]),
        //to_apply.root_instruction().opcode(),
        //x, y));

    inst.get_mutable_module().unwrap()
      .add_embedded_computation(promoted.build(None)).clone()
  };

  let to_apply_comp = to_apply_promoted();
  new_inst.set_to_apply(to_apply_comp);
  new_inst
}