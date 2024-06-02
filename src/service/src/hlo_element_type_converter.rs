#![allow(dead_code)]

use std::collections::HashSet;

use common::blitz_data::PrimitiveType;
use hlo::{
  hlo_instruction::HloInstruction,
  hlo_module::HloModule,
  hlo_opcode::HloOpcode
};

// A pass that eliminates certain element types as the input or output of ops by
// inserting Convert ops.
pub struct HloElementTypeConverter {
  eliminate_type: PrimitiveType,
  replace_with_type: PrimitiveType
}

impl HloElementTypeConverter {
  pub fn new(element_type: PrimitiveType, replace_with_type: PrimitiveType) -> Self {
    HloElementTypeConverter {
      eliminate_type: element_type,
      replace_with_type: replace_with_type
    }
  }

  pub fn name(&self) -> String {
    "element-type-converter".to_string()
  }

  pub fn run(
    &self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>) -> Result<bool, String>
  {
    println!("HloElementTypeConverter::run(), before:");
    println!("{:?}", module.to_string());

    if self.eliminate_type == self.replace_with_type {
      return Ok(false);
    }

    let mut changed = false;
    for computation in
      module.mutable_computations_by_exec_threads(execution_threads) {
      for hlo in computation.mutable_make_instruction_post_order() {
        let opcode = hlo.opcode();
        
        // There are ops where it does not mokae sense to convert them.
        if opcode == HloOpcode::Parameter || opcode == HloOpcode::Constant ||
           opcode == HloOpcode::Tuple || opcode == HloOpcode::Convert ||
           opcode == HloOpcode::BitcastConvert ||
           opcode == HloOpcode::GetTupleElement ||
           opcode == HloOpcode::Infeed || opcode == HloOpcode::Outfeed
        {
          continue;
        }

        // We cannot change a CustoCall since we have no way of adjusting the called
        // binary to expect the updated type.
        if opcode == HloOpcode::CustomCall {
          continue;
        }

        // There are ops with embedded computations where it suffices to convert the
        // embedded computations instead of convertiing ops themselves.
        if opcode == HloOpcode::While || opcode == HloOpcode::Call ||
           opcode == HloOpcode::AllReduce ||
           opcode == HloOpcode::ReduceScatter ||
           opcode == HloOpcode::AllReduceStart ||
           opcode == HloOpcode::Fusion || opcode == HloOpcode::Map ||
           opcode == HloOpcode::Reduce || opcode == HloOpcode::ReduceWindow ||
           opcode == HloOpcode::Scatter ||
           opcode == HloOpcode::SelectAndScatter ||
           opcode == HloOpcode::Sort || opcode == HloOpcode::Conditional
        {
          continue;
        }

        debug_assert!(hlo.called_computations().is_empty(), "{:?}", hlo.to_string_default());
        let null_ary = hlo.operands().is_empty();
        let wrong_element_type = hlo.shape().element_type() == self.eliminate_type;
        let should_eliminaete_type = (null_ary && wrong_element_type) ||
          has_operand_type(hlo, &self.eliminate_type);
        
        if !should_eliminaete_type {
          debug_assert!(hlo.shape().element_type() != self.eliminate_type);
          continue;
        }

        // Handle instructions that perform arithmetic operations and contain operands
        // with eliminate_type.
        let mut new_operands = vec![];
        let cloned_hlo = hlo.clone();
        for operand in hlo.mutable_operands() {
          if operand.shape().element_type() == self.eliminate_type {
            *operand = to_element_type(&cloned_hlo, &self.replace_with_type);
          }
          new_operands.push(operand.clone());
        }

        let new_hlo = None;
        if hlo.shape().element_type() == self.eliminate_type {

        } else if hlo.shape().is_tuple() {
            
        } else {
          //new_hlo = Some(computation.add_instruction(
            //hlo.clone_with_new_opereands(hlo.shape(), new_operands),
            //"".to_string()));
        }

        let mut result = hlo.replace_all_uses_with(
          &mut new_hlo.unwrap(), "".to_string());
        if result.is_err() { return Err(result.err().unwrap()); }

        result = hlo.drop_all_control_deps();
        if result.is_err() { return Err(result.err().unwrap()); }

        //computation.remove_instruction(&hlo);
        changed = true;
      }
    }

    println!("HloElementTypeConverter::run(), after:\n");
    println!("{:?}", module.to_string());

    Ok(changed)
  }
}

pub fn to_element_type(_hlo: &HloInstruction, _t: &PrimitiveType) -> HloInstruction {
  unimplemented!()
}

pub fn has_operand_type(hlo: &HloInstruction, t: &PrimitiveType) -> bool {
  for operand in hlo.operands() {
    if operand.shape().element_type() == *t { return true; }
  }
  false
}