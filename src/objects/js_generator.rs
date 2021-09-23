pub enum ResumeMode {
  Next,
  Return,
  Throw,
}

const GENERATOR_EXECUTING: i64 = -2;
const GENERATOR_CLOSED: i64 = -1;

struct JSGeneratorObject {}

impl JSGeneratorObject {
  pub fn is_closed() {}
  pub fn is_executing() {}
  pub fn is_suspended() {}

  // The source position at which the generator is suspended.
  pub fn source_position() {}
}

struct JSAsyncFunctionObject {}

struct JSAsyncGeneratorObject {}

struct AsyncGeneratorRequest {}
