#![allow(dead_code)]

#[derive(Clone, PartialEq)]
pub enum PrimitiveType {
  Invalid,
  Pred,
  S4,
  S8,
  S16,
  S32,
  S64,
  U4,
  U8,
  U16,
  U32,
  U64,
  Tuple,
  Token,
  OpaqueType,
}