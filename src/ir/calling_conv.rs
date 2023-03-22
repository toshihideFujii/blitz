#![allow(dead_code)]

pub enum CallingConv {
  C,
  Fast,
  Cold,
  Anyreg,
  PreserveMost,
  PreserveAll,
  Tail,
  X86StdCall,
  X86FastCall
}