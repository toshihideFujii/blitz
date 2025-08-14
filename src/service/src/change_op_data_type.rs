
use std::collections::{HashMap, HashSet};
use common::{blitz_data::PrimitiveType, shape::Shape,
  //util::make_no_padding_config
};
use hlo::{hlo_instruction::HloInstruction, hlo_module::HloModule, hlo_opcode::HloOpcode};

use crate::hlo_creation_utils::make_convert_to_hlo;

// Changes `from_ty op(from_ty a, from_ty b)` into
// `from_ty convert(op(to_ty convert(a), to_ty convert(b)))`.
//
// One place where this pass is useful is for fp16 dots/convs in XLA:CPU.
// Although XLA:CPU supports fp16 dots/convs, they are significantly slower than
// fp32 convs.   This pass lets us run the fp16 dot/conv as "convert to fp32,
// run in fp32, then convert back to fp16".  (This is of course not
// mathematically the same, but it's close enough for our purposes.)
//
// This pass only considers ops that match `op_matcher` and where all operands
// have type `from_ty`.  It will not do the correct thing for ops like
// dynamic-slice where only some of the arguments should be converted; it's up
// to you to avoid matching such ops with `op_matcher`.
//
// The pass support multiple <from_ty, to_ty> pairs and will apply the transform
// if all operands match one of the types in from_ty.
//
// It uses provided `cloner` to clone an instruction with shape and converted
// operands. If the cloner is not provided, it will uses `CloneWithNewOperands`.
pub struct ChangeOpDataType {
  to_type_map: HashMap<PrimitiveType, PrimitiveType>
}

impl ChangeOpDataType {
  pub fn new(
    from_to_types: Vec<(PrimitiveType, PrimitiveType)>,
    //_op_matcher: &dyn Fn(&HloInstruction)->bool,
    //_cloner: Option<&dyn Fn(&HloInstruction, &Shape, &Vec<HloInstruction>)->HloInstruction>
    )
    -> Self
  {
    let mut instance =
      ChangeOpDataType { to_type_map: HashMap::new() };
    for pair in from_to_types {
      instance.to_type_map.insert(pair.0, pair.1);
    }
    instance
  }
  
  pub fn name(&self) -> String {
    "change-op-data-type".to_string()
  }

  pub fn run(
    &self,
    module: &mut HloModule,
    execution_threads: &HashSet<String>,
    op_matcher: &dyn Fn(&HloInstruction)->bool,
    _cloner: &dyn Fn(&mut HloInstruction, &Shape, &Vec<HloInstruction>)->HloInstruction)
    -> Result<bool, String>
  {
    let mut changed = false;
    for comp in
      module.mutable_make_nonfusion_computations(execution_threads)
    {
      for instr in comp.mutable_make_instruction_post_order() {
        let operand_type = get_uniform_operand_type(instr);
        if !op_matcher(instr) || operand_type.is_none() ||
          !instr.shape().is_array() || instr.opcode() == HloOpcode::Parameter
        {
          continue;
        }
        let from_type = operand_type.unwrap();
        let val_type = self.to_type_map.get(&from_type);
        if val_type.is_none() {
          continue;
        }
        let to_type = val_type.unwrap();
        let mut new_operands = vec![];
        for operand in instr.mutable_operands() {
          new_operands.push(make_convert_to_hlo(operand, to_type, None));
        }
        let new_shape = instr.mutable_shape();
        new_shape.set_element_type(to_type.clone());

        /*
        let new_instr = comp.add_instruction(
          cloner(instr, new_shape, &new_operands), 
          "".to_string());
        let result = comp.replace_instruction(
          &instr,
          &make_convert_to_hlo(new_instr, &from_type, None),
          false, false, true);
        assert!(result.is_ok());
        */

        changed = true;
      }
    }
    Ok(changed)
  }
}

pub fn get_uniform_operand_type(instr: &HloInstruction) -> Option<PrimitiveType> {
  let mut op_type = None;
  for operand in instr.operands() {
    if op_type.is_none() {
      op_type = Some(operand.shape().element_type());
    } else if &operand.shape().element_type() != op_type.as_ref().unwrap() {
      return None;
    }
  }
  op_type
}