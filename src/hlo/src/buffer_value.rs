#![allow(dead_code)]

// Abstract class describing a value used by one of the dataflow analyses -
// TuplePointsToAnalysis or HloDataflowAnalysis.
// TODO(b/78906445) Delete this class when TuplePointsToAnalysis is unused.
//
// XLA arrays are trivially a single BufferValue. Tuples are made up of more
// than one BufferValue: a BufferValue for the pointer vector, and a
// BufferValue for each child element.
//
// Every BufferValue is defined by a particular instruction and most
// instructions define only a single BufferValue. Instructions which define a
// single BufferValue include array-shaped instructions such as Add but also
// includes Tuple-shaped instructions such as Tuple. The Tuple instruction
// defines a single BufferValue which is a vector of pointers to the values
// containing the Tuple instruction's operands. Though the result of the Tuple
// instruction includes multiple values only the top-level BufferValue (the
// vector of pointers) is defined by the Tuple instruction. The values
// containing the tuple elements are defined by earlier instructions, usually
// the operands of the Tuple instruction.
//
// Instructions which construct both the tuple *and* the tuple elements define
// more than one BufferValue. This includes (at least) tuple-shaped Constant,
// Parameter, Infeed and While instructions. These tuple-shaped instructions do
// not assemble a tuple from existing BufferValues like the Tuple instruction
// does, but rather define all the BufferValues in the tuple.
//
// Some instructions, such as Bitcast, define no buffers. These instructions
// simply forward buffers from their operands.
//
// The BufferValue object describes which HLO instruction defines a buffer and
// where within that instruction's output shape the buffer is defined. The
// location within the output shape is indicated by BufferValue::index() which
// is defined identically to the index used in ShapeUtil::GetSubshape().
// Examples:
//
// %add = Add(%foo, %bar)
// %tuple_constant = Constant({1, {42, 43}})
//
// %add defines a single array-shaped buffer BufferValue(%add, {}) which holds
// the array result of the add operation. The nested-tuple-shaped
// %tuple_constant defines 5 buffers described by the following BufferValue
// objects:
//
//   BufferValue(%tuple_constant, {})      // "Top-level" buffer: vector of
//                                         //  pointers to BufferValues at
//                                         //  indices {0} and {1}
//   BufferValue(%tuple_constant, {0})     // Holds value "1"
//   BufferValue(%tuple_constant, {1})     // Holds nested tuple: vector of
//                                         //  pointers to BufferValues at
//                                         //  indices {1, 0} and {1, 1}
//   BufferValue(%tuple_constant, {1, 0})  // Holds value "42"
//   BufferValue(%tuple_constant, {1, 1})  // Holds value "43"

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferValue {
  id: i64,
  is_array: bool,
  is_tuple: bool,
  color: i64
}

impl BufferValue {
  pub fn new() -> Self {
    BufferValue {
      id: 0,
      is_array: false,
      is_tuple: false,
      color: -1
    }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn instruction() {}
  pub fn index() {}

  // Return the color of the BufferValue.
  pub fn color(&self) -> i64 {
    debug_assert!(self.color != -1);
    self.color
  }

  pub fn set_color(&mut self, color: i64) {
    self.color = color;
  }

  pub fn has_color(&self) -> bool {
    self.color != -1
  }

  pub fn shape() {}
  pub fn is_top_level() {}

  // Whether this buffer contains a tuple.
  pub fn is_tuple(&self) -> bool {
    self.is_tuple
  }

  // Whether this buffer contains an array.
  pub fn is_array(&self) -> bool {
    self.is_array
  }
  
  pub fn to_string() {}
}