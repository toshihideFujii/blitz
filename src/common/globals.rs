
use std::mem;

const KB: i32 = 1024;
const MB: i32 = KB * 1024;
const GB: i32 = MB * 1024;

// Minimum stack size in KB required by compilers.
const STACK_SPACE_REQUIRED_FOR_COMPILATION: i32 = 40;

// In order to emit more efficient stack checks in optimized code,
// deoptimization may implicitly exceed the V8 stack limit by this many bytes.
// Stack checks in functions with `difference between optimized and unoptimized
// stack frame sizes <= slack` can simply emit the simple stack check.
const STACK_LIMIT_SLACK_FOR_DEOPTIMIZATION_IN_BYTES: i32 = 256;

//const SHORT_BUILTIN_CALLS_OLD_SPACE_SIZE_THRESHOLD: usize = 2 * GB;

type Byte = u8;

// Constants
const MAX_INT: i32 = 0x7FFFFFFF;
const MIN_INT: i32 = -MAX_INT - 1;
const MAX_INT8: i32 = (1 << 7) - 1;
const MIN_INT8: i32 = -(1 << 7);
const MAX_UINT8: i32 = (1 << 8) - 1;
const MIN_UINT8: i32 = 0;
const MAX_INT16: i32 = (1 << 15) - 1;
const MIN_INT16: i32 = -(1 << 15);
const MAX_UINT16: i32 = (1 << 16) - 1;
const MIN_UINT16: i32 = 0;
const MAX_INT31: i32 = MAX_INT / 2;
const MIN_INT31: i32 = MIN_INT / 2;

const MAX_UINT32: u32 = 0xFFFFFFFF;
const MIN_UINT32: i32 = 0;

const UINT8_SIZE: usize = mem::size_of::<u8>();
const BYTE_SIZE: usize = mem::size_of::<Byte>();