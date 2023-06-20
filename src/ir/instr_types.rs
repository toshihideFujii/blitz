#![allow(dead_code)]

struct UnaryInstruction {}
impl UnaryInstruction {
  pub fn new() {}
  pub fn class_of() {}
}

struct UnaryOperator {}
impl UnaryOperator {
  pub fn new() {}
}

struct BinaryOperator {}

struct CastInst {}

struct CmpInst {}

// Base class for all callable instructions (InvokeInst and CallInst) holds
// everything related to calling a function.
struct CallBase {}
impl CallBase {
  pub fn new() {}
  pub fn has_descriptor() {}
  pub fn get_num_subclass_extra_operands() {}
}

struct FuncletPadInst {}
impl FuncletPadInst {
  pub fn new() {}
}