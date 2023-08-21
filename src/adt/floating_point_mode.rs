#![allow(dead_code)]

// Utilities for dealing with flags related to floating
// point properties and mode controls.

use crate::adt::string_ref::StringRef;

// Rounding mode defined in IEEE-754.
#[derive(Debug, Clone, PartialEq)]
pub enum RoundingMode {
  TowardZero = 0,
  NearestTiesToEven = 1,
  TowardPositive = 2,
  TowardNegative = 3,
  NearestTiesToAway = 4,
  Dynamic = 7,
  Invalid= -1
}

// Returns text representation of the given rounding mode.
pub fn spell(mode: RoundingMode) -> StringRef {
  match mode {
    RoundingMode::TowardZero => return StringRef::new_from_string("towardzero"),
    RoundingMode::NearestTiesToEven => return StringRef::new_from_string("tonearest"),
    RoundingMode::TowardPositive => return StringRef::new_from_string("upward"),
    RoundingMode::TowardNegative => return StringRef::new_from_string("downward"),
    RoundingMode::NearestTiesToAway => return StringRef::new_from_string("tonearestaway"),
    RoundingMode::Dynamic => return StringRef::new_from_string("dynamic"),
    _ => return StringRef::new_from_string("invalid")
  };
}

#[derive(Debug, Clone, PartialEq)]
pub enum DenormalModeKind {
  Invalid,
  IEEE,
  PreserveSign,
  PositiveZero
}

// Represent subnormal handling kind for floating point instruction
// inputs and outputs.
#[derive(Debug, Clone, PartialEq)]
pub struct DenormalMode {
  output: DenormalModeKind,
  input: DenormalModeKind
}

impl DenormalMode {
  pub fn new(output: DenormalModeKind, input: DenormalModeKind) -> Self {
    DenormalMode { output: output, input: input }
  }

  pub fn get_invalid() -> DenormalMode {
    DenormalMode::new(DenormalModeKind::Invalid, DenormalModeKind::Invalid)
  }

  pub fn get_ieee() -> DenormalMode {
    DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE)
  }

  pub fn get_preserve_sign() -> DenormalMode {
    DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::PreserveSign)
  }

  pub fn get_positive_zero() -> DenormalMode {
    DenormalMode::new(DenormalModeKind::PositiveZero, DenormalModeKind::PositiveZero)
  }

  pub fn is_simple(&self) -> bool {
    self.input == self.output
  }

  pub fn is_valid(&self) -> bool {
    self.input != DenormalModeKind::Invalid && self.output != DenormalModeKind::Invalid
  }

  pub fn print() {}

  pub fn str(&self) -> StringRef {
    let output = denormal_mode_kind_name(self.output.clone());
    let input = denormal_mode_kind_name(self.input.clone());
    let mut str = output.data();
    str.push(',');
    str.push_str(&input.data());
    StringRef::new_from_string(str.as_str())
  }
}

// Parse the expected names from the denormal-fp-math attribute.
pub fn parse_denormal_fp_attribute_component(str: String) -> DenormalModeKind {
  let empty = String::from("");
  let ieee = String::from("ieee");
  let preserve_sign = String::from("preserve-sign");
  let positive_zero = String::from("positive-zero");
  let string = str.to_ascii_lowercase();
  if string == empty || string == ieee {
    return DenormalModeKind::IEEE;
  } else if string == preserve_sign {
    return DenormalModeKind::PreserveSign;
  } else if string == positive_zero {
    return DenormalModeKind::PositiveZero
  } else {
    return DenormalModeKind::Invalid;
  }
}

// Return the name used for the denormal handling mode used by the
// expected names from the denormal-fp-math attribute.
pub fn denormal_mode_kind_name(mode: DenormalModeKind) -> StringRef {
  match mode {
    DenormalModeKind::IEEE => return StringRef::new_from_string("ieee"),
    DenormalModeKind::PreserveSign => return StringRef::new_from_string("preserve-sign"),
    DenormalModeKind::PositiveZero => return StringRef::new_from_string("positive-zero"),
    _ => return StringRef::new_from_string("")
  };
}

pub fn parse_denormal_fp_attribute(str: StringRef) -> DenormalMode {
  let split_str = str.split(',');
  let mut mode = DenormalMode::new(DenormalModeKind::Invalid,
    DenormalModeKind::Invalid);
  mode.output = parse_denormal_fp_attribute_component(split_str.0);
  if split_str.1.is_empty() {
    mode.input = mode.output.clone();
  } else {
    mode.input = parse_denormal_fp_attribute_component(split_str.1);
  }
  mode
}

// Floating-point class tests, supported by 'is-fpclass' intrinsic.
// Actual test may be an OR combination of basic tests.
#[derive(Debug, Clone, PartialEq)]
pub enum FPClassTest {
  None = 0,
  SNan = 0x0001,
  QNan = 0x0002,
  NegInf = 0x0004,
  NegNormal = 0x0008,
  NegSubnormal = 0x0010,
  NegZero = 0x0020,
  PosZero = 0x0040,
  PosSubnormal = 0x0080,
  PosNormal = 0x0100,
  PosInf = 0x0200
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_denormal_fp_attribute_component() {
    assert_eq!(parse_denormal_fp_attribute_component(String::from("ieee")), DenormalModeKind::IEEE);
    assert_eq!(parse_denormal_fp_attribute_component(String::from("")), DenormalModeKind::IEEE);
    assert_eq!(parse_denormal_fp_attribute_component(String::from("preserve-sign")), DenormalModeKind::PreserveSign);
    assert_eq!(parse_denormal_fp_attribute_component(String::from("positive-zero")), DenormalModeKind::PositiveZero);
    assert_eq!(parse_denormal_fp_attribute_component(String::from("foo")), DenormalModeKind::Invalid);
  }

  #[test]
  fn test_denormal_attribute_name() {
    assert_eq!(denormal_mode_kind_name(DenormalModeKind::IEEE), StringRef::new_from_string("ieee"));
    assert_eq!(denormal_mode_kind_name(DenormalModeKind::PreserveSign), StringRef::new_from_string("preserve-sign"));
    assert_eq!(denormal_mode_kind_name(DenormalModeKind::PositiveZero), StringRef::new_from_string("positive-zero"));
    assert_eq!(denormal_mode_kind_name(DenormalModeKind::Invalid), StringRef::new_from_string(""));
  }

  #[test]
  fn test_parse_denormal_fp_attribute() {
    let mut s = StringRef::new_from_string("ieee");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE));
    s = StringRef::new_from_string("ieee,ieee");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE));
    s = StringRef::new_from_string("ieee,");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE));
    s = StringRef::new_from_string("");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE));
    s = StringRef::new_from_string(",");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE));

    s = StringRef::new_from_string("preserve-sign");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::PreserveSign));
    s = StringRef::new_from_string("preserve-sign,");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::PreserveSign));
    s = StringRef::new_from_string("preserve-sign,preserve-sign");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::PreserveSign));

    s = StringRef::new_from_string("positive-zero");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::PositiveZero, DenormalModeKind::PositiveZero));
    s = StringRef::new_from_string("positive-zero,positive-zero");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::PositiveZero, DenormalModeKind::PositiveZero));

    s = StringRef::new_from_string("ieee,positive-zero");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::PositiveZero));
    s = StringRef::new_from_string("positive-zero,ieee");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::PositiveZero, DenormalModeKind::IEEE));
    s = StringRef::new_from_string("preserve-sign,ieee");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::IEEE));
    s = StringRef::new_from_string("ieee,preserve-sign");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::PreserveSign));

    s = StringRef::new_from_string("foo");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::Invalid, DenormalModeKind::Invalid));
    s = StringRef::new_from_string("foo,foo");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::Invalid, DenormalModeKind::Invalid));
    s = StringRef::new_from_string("foo,bar");
    assert_eq!(parse_denormal_fp_attribute(s), DenormalMode::new(DenormalModeKind::Invalid, DenormalModeKind::Invalid));
  }

  #[test]
  fn test_render_denormal_fp_attribute() {
    assert_eq!(DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE).str(),
      StringRef::new_from_string("ieee,ieee"));
    assert_eq!(DenormalMode::new(DenormalModeKind::Invalid, DenormalModeKind::Invalid).str(),
      StringRef::new_from_string(","));
    assert_eq!(DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::PreserveSign).str(),
      StringRef::new_from_string("preserve-sign,preserve-sign"));
    assert_eq!(DenormalMode::new(DenormalModeKind::PositiveZero, DenormalModeKind::PositiveZero).str(),
      StringRef::new_from_string("positive-zero,positive-zero"));
    assert_eq!(DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::PreserveSign).str(),
      StringRef::new_from_string("ieee,preserve-sign"));
    assert_eq!(DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::IEEE).str(),
      StringRef::new_from_string("preserve-sign,ieee"));
    assert_eq!(DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::PositiveZero).str(),
      StringRef::new_from_string("preserve-sign,positive-zero"));
  }

  #[test]
  fn test_denormal_mode_is_simple() {
    assert_eq!(DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE).is_simple(), true);
    assert_eq!(DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::Invalid).is_simple(), false);
    assert_eq!(DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::PositiveZero).is_simple(), false);
  }

  #[test]
  fn test_denormal_mode_is_valid() {
    assert_eq!(DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE).is_valid(), true);
    assert_eq!(DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::Invalid).is_valid(), false);
    assert_eq!(DenormalMode::new(DenormalModeKind::Invalid, DenormalModeKind::IEEE).is_valid(), false);
    assert_eq!(DenormalMode::new(DenormalModeKind::Invalid, DenormalModeKind::Invalid).is_valid(), false);
  }

  #[test]
  fn test_denormal_mode_constructor() {
    assert_eq!(DenormalMode::new(DenormalModeKind::Invalid, DenormalModeKind::Invalid), DenormalMode::get_invalid());
    assert_eq!(DenormalMode::new(DenormalModeKind::IEEE, DenormalModeKind::IEEE), DenormalMode::get_ieee());
    assert_eq!(DenormalMode::new(DenormalModeKind::PreserveSign, DenormalModeKind::PreserveSign), DenormalMode::get_preserve_sign());
    assert_eq!(DenormalMode::new(DenormalModeKind::PositiveZero, DenormalModeKind::PositiveZero), DenormalMode::get_positive_zero());
  }
}