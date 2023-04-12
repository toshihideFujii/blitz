#![allow(dead_code)]

// This file contains descriptions of the various Blitz values.
// This is used as a central place for enumerating the defferent
// values.

// Constant
pub const FUNCTION: u32 = 1;
pub const GLOBAL_ALIAS: u32 = 2;
pub const GLOBAL_I_FUNC: u32 = 3;
pub const GLOBAL_VARIABLE: u32 = 4;
pub const BLOCK_ADDRESS: u32 = 5;
pub const CONSTANT_EXPR: u32 = 6;
pub const DS0_LOCAL_EQUIVALENT: u32 = 7;

// ConstantAggregate.
pub const CONSTANT_ARRAY: u32 = 8;
pub const CONSTANT_STRUCT: u32 = 9;
pub const CONSTANT_VECTOR: u32 = 10;

// ConstantData
pub const UNDEF_VALUE: u32 = 11;
pub const POISON_VALUE: u32 = 12;
pub const CONSTANT_AGGREGATE_ZERO: u32 = 13;
pub const CONSTANT_DATA_ARRAY: u32 = 14;
pub const CONSTANT_DATA_VECTOR: u32 = 15;
pub const CONSTANT_INT: u32 = 16;
pub const CONSTANT_FP: u32 = 17;
pub const CONSTANT_TARGET_NONE: u32 = 18;
pub const CONSTANT_POINTER_NULL: u32 = 19;
pub const CONSTANT_TOKEN_NONE: u32 = 20;