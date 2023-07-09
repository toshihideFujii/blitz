#![allow(dead_code)]

// This file implements the StringSwitch template, which mimics
// a switch() statement whose cases are string literals.

use super::string_ref::StringRef;

#[derive(Debug, Clone)]
pub struct StringSwitch<T> {
  // The string we are matching.
  str: StringRef,
  // The pointer to the result of this switch statement, once known,
  // null before that.
  result: Option<T>
}

impl<T> StringSwitch<T> {
  pub fn new(s: StringRef) -> Self {
    StringSwitch { str: s, result: None }
  }

  // Case-sensitive case matchers
  pub fn case(&mut self, s: StringRef, value: Option<T>) -> &mut Self {
    if self.result.is_none() && self.str == s {
      self.result = value;
    }
    self
  }

  pub fn ends_with(&mut self, s: StringRef, value: Option<T>) -> &mut Self {
    if self.result.is_none() && self.str.ends_with(s.data().as_str()) {
      self.result = value;
    }
    self
  }

  pub fn starts_with(&mut self, s: StringRef, value: Option<T>) -> &mut Self {
    if self.result.is_none() && self.str.starts_with(s.data().as_str()) {
      self.result = value;
    }
    self
  }

  pub fn cases_2(&mut self, s0: StringRef, s1: StringRef,
    value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).case(s1, v)
  }

  pub fn cases_3(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).cases_2(s1, s2, v)
  }

  pub fn cases_4(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).cases_3(s1, s2, s3, v)
  }

  pub fn cases_5(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, s4: StringRef, value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).cases_4(s1, s2, s3, s4, v)
  }

  pub fn cases_6(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, s4: StringRef, s5: StringRef,
    value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).cases_5(s1, s2, s3, s4, s5, v)
  }

  pub fn cases_7(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, s4: StringRef, s5: StringRef, s6: StringRef,
    value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).cases_6(s1, s2, s3, s4, s5,
        s6, v)
  }

  pub fn cases_8(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, s4: StringRef, s5: StringRef, s6: StringRef, s7: StringRef,
    value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).cases_7(s1, s2, s3, s4, s5,
        s6, s7, v)
  }

  pub fn cases_9(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, s4: StringRef, s5: StringRef, s6: StringRef, s7: StringRef,
    s8: StringRef, value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).cases_8(s1, s2, s3, s4, s5,
        s6, s7, s8, v)
  }

  pub fn cases_10(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, s4: StringRef, s5: StringRef, s6: StringRef, s7: StringRef,
    s8: StringRef, s9: StringRef, value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case(s0, value).cases_9(s1, s2, s3, s4, s5,
        s6, s7, s8, s9, v)
  }

  // Case-insensitive case matchers.
  pub fn case_lower(&mut self, s: StringRef, value: Option<T>) -> &mut Self {
    if self.result.is_none() && self.str.equals_insensitive(s) {
      self.result = value;
    }
    self
  }

  pub fn ends_with_lower(&mut self, s:StringRef, value: Option<T>) -> &mut Self {
    if self.result.is_none() && self.str.ends_with_insensitive(s.data().as_str()) {
      self.result = value;
    }
    self
  }

  pub fn starts_with_lower(&mut self, s: StringRef, value: Option<T>) -> &mut Self {
    if self.result.is_none() && self.str.starts_with_insensitive(s.data().as_str()) {
      self.result = value;
    }
    self
  }
  
  pub fn cases_lower_2(&mut self, s0: StringRef, s1: StringRef,
    value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case_lower(s0, value).case_lower(s1, v)
  }

  pub fn cases_lower_3(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case_lower(s0, value).cases_lower_2(s1, s2, v)
  }

  pub fn cases_lower_4(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case_lower(s0, value).cases_lower_3(s1, s2, s3, v)
  }

  pub fn cases_lower_5(&mut self, s0: StringRef, s1: StringRef, s2: StringRef,
    s3: StringRef, s4: StringRef, value: Option<T>) -> &mut Self where T: Clone
  {
    let v = value.clone();
    self.case_lower(s0, value).cases_lower_4(s1, s2, s3, s4, v)
  }

  pub fn default(&self, value: Option<T>) -> Option<T> where T: Clone {
    if self.result.is_some() {
      return self.result.clone();
    }
    value
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::adt::string_ref::StringRef;

  #[test]
  fn test_case() {
    fn translate(s: StringRef) -> Option<i64> {
      let mut switch: StringSwitch<i64> = StringSwitch::new(s);
      let result = switch.case(StringRef::new_from_string("0"), Some(0))
        .case(StringRef::new_from_string("1"), Some(1))
        .case(StringRef::new_from_string("2"), Some(2))
        .case(StringRef::new_from_string("3"), Some(3))
        .case(StringRef::new_from_string("4"), Some(4))
        .case(StringRef::new_from_string("5"), Some(5))
        .case(StringRef::new_from_string("6"), Some(6))
        .case(StringRef::new_from_string("7"), Some(7))
        .case(StringRef::new_from_string("8"), Some(8))
        .case(StringRef::new_from_string("9"), Some(9))
        .case(StringRef::new_from_string("A"), Some(10))
        .case(StringRef::new_from_string("B"), Some(11))
        .case(StringRef::new_from_string("C"), Some(12))
        .case(StringRef::new_from_string("D"), Some(13))
        .case(StringRef::new_from_string("E"), Some(14))
        .case(StringRef::new_from_string("F"), Some(15))
        .default(Some(-1));
      result
    }
    assert_eq!(translate(StringRef::new_from_string("1")), Some(1));
    assert_eq!(translate(StringRef::new_from_string("2")), Some(2));
    assert_eq!(translate(StringRef::new_from_string("B")), Some(11));
    assert_eq!(translate(StringRef::new_from_string("b")), Some(-1));
    assert_eq!(translate(StringRef::new_from_string("")), Some(-1));
    assert_eq!(translate(StringRef::new_from_string("Test")), Some(-1));
  }

  #[test]
  fn test_case_lower() {
    fn translate(s: StringRef) -> Option<i64> {
      let mut switch: StringSwitch<i64> = StringSwitch::new(s);
      let result = switch.case(StringRef::new_from_string("0"), Some(0))
        .case(StringRef::new_from_string("1"), Some(1))
        .case(StringRef::new_from_string("2"), Some(2))
        .case(StringRef::new_from_string("3"), Some(3))
        .case(StringRef::new_from_string("4"), Some(4))
        .case(StringRef::new_from_string("5"), Some(5))
        .case(StringRef::new_from_string("6"), Some(6))
        .case(StringRef::new_from_string("7"), Some(7))
        .case(StringRef::new_from_string("8"), Some(8))
        .case(StringRef::new_from_string("9"), Some(9))
        .case_lower(StringRef::new_from_string("A"), Some(10))
        .case_lower(StringRef::new_from_string("B"), Some(11))
        .case_lower(StringRef::new_from_string("C"), Some(12))
        .case_lower(StringRef::new_from_string("D"), Some(13))
        .case_lower(StringRef::new_from_string("E"), Some(14))
        .case_lower(StringRef::new_from_string("F"), Some(15))
        .default(Some(-1));
      result
    }
    assert_eq!(translate(StringRef::new_from_string("1")), Some(1));
    assert_eq!(translate(StringRef::new_from_string("2")), Some(2));
    assert_eq!(translate(StringRef::new_from_string("B")), Some(11));
    assert_eq!(translate(StringRef::new_from_string("b")), Some(11));
    assert_eq!(translate(StringRef::new_from_string("")), Some(-1));
    assert_eq!(translate(StringRef::new_from_string("Test")), Some(-1));
  }

  #[test]
  fn test_starts_with() {
    fn translate(s: StringRef, x: i64, y: i64) -> Option<i64> {
      //let mut switch: StringSwitch<&dyn Fn(i64, i64) -> i64> = StringSwitch::new(s);
      let mut switch: StringSwitch<i64> = StringSwitch::new(s);
      let result = switch
        .starts_with(StringRef::new_from_string("add"), Some(x+y))
        .starts_with(StringRef::new_from_string("sub"), Some(x-y))
        .starts_with(StringRef::new_from_string("mul"), Some(x*y))
        .starts_with(StringRef::new_from_string("div"), Some(x/y))
        .default(Some(0));
      result
    }
    assert_eq!(translate(StringRef::new_from_string("adder"), 10, 5), Some(15));
    assert_eq!(translate(StringRef::new_from_string("subtracter"), 10, 5), Some(5));
    assert_eq!(translate(StringRef::new_from_string("multiplier"), 10, 5), Some(50));
    assert_eq!(translate(StringRef::new_from_string("divider"), 10, 5), Some(2));
    assert_eq!(translate(StringRef::new_from_string("nothing"), 10, 5), Some(0));
    assert_eq!(translate(StringRef::new_from_string("ADDER"), 10, 5), Some(0));
  }

  #[test]
  fn test_starts_with_lower() {
    fn translate(s: StringRef, x: i64, y: i64) -> Option<i64> {
      let mut switch: StringSwitch<i64> = StringSwitch::new(s);
      let result = switch
        .starts_with_lower(StringRef::new_from_string("add"), Some(x+y))
        .starts_with_lower(StringRef::new_from_string("sub"), Some(x-y))
        .starts_with_lower(StringRef::new_from_string("mul"), Some(x*y))
        .starts_with_lower(StringRef::new_from_string("div"), Some(x/y))
        .default(Some(0));
      result
    }
    assert_eq!(translate(StringRef::new_from_string("adder"), 10, 5), Some(15));
    assert_eq!(translate(StringRef::new_from_string("subtracter"), 10, 5), Some(5));
    assert_eq!(translate(StringRef::new_from_string("multiplier"), 10, 5), Some(50));
    assert_eq!(translate(StringRef::new_from_string("divider"), 10, 5), Some(2));

    assert_eq!(translate(StringRef::new_from_string("AdDeR"), 10, 5), Some(15));
    assert_eq!(translate(StringRef::new_from_string("SuBtRaCtEr"), 10, 5), Some(5));
    assert_eq!(translate(StringRef::new_from_string("MuLtIpLiEr"), 10, 5), Some(50));
    assert_eq!(translate(StringRef::new_from_string("DiViDeR"), 10, 5), Some(2));

    assert_eq!(translate(StringRef::new_from_string("nothing"), 10, 5), Some(0));
  }

  #[test]
  fn test_ends_with() {
    #[derive(Debug,Clone, PartialEq)]
    enum Suffix {
      Possible,
      PastTense,
      Process,
      InProgressAction,
      Unknown
    }
    fn translate(s: StringRef) -> Option<Suffix> {
      let mut switch: StringSwitch<Suffix> = StringSwitch::new(s);
      let result = switch
        .ends_with(StringRef::new_from_string("able"), Some(Suffix::Possible))
        .ends_with(StringRef::new_from_string("ed"), Some(Suffix::PastTense))
        .ends_with(StringRef::new_from_string("ation"), Some(Suffix::Process))
        .ends_with(StringRef::new_from_string("ing"), Some(Suffix::InProgressAction))
        .default(Some(Suffix::Unknown));
      result
    }
    assert_eq!(translate(StringRef::new_from_string("optimizable")), Some(Suffix::Possible));
    assert_eq!(translate(StringRef::new_from_string("optimized")), Some(Suffix::PastTense));
    assert_eq!(translate(StringRef::new_from_string("optimization")), Some(Suffix::Process));
    assert_eq!(translate(StringRef::new_from_string("optimizing")), Some(Suffix::InProgressAction));
    assert_eq!(translate(StringRef::new_from_string("optimizer")), Some(Suffix::Unknown));
    assert_eq!(translate(StringRef::new_from_string("OPTIMIZABLE")), Some(Suffix::Unknown));
  }

  #[test]
  fn test_ends_with_lower() {
    #[derive(Debug,Clone, PartialEq)]
    enum Suffix {
      Possible,
      PastTense,
      Process,
      InProgressAction,
      Unknown
    }
    fn translate(s: StringRef) -> Option<Suffix> {
      let mut switch: StringSwitch<Suffix> = StringSwitch::new(s);
      let result = switch
        .ends_with_lower(StringRef::new_from_string("able"), Some(Suffix::Possible))
        .ends_with_lower(StringRef::new_from_string("ed"), Some(Suffix::PastTense))
        .ends_with_lower(StringRef::new_from_string("ation"), Some(Suffix::Process))
        .ends_with_lower(StringRef::new_from_string("ing"), Some(Suffix::InProgressAction))
        .default(Some(Suffix::Unknown));
      result
    }
    assert_eq!(translate(StringRef::new_from_string("optimizable")), Some(Suffix::Possible));
    assert_eq!(translate(StringRef::new_from_string("OPTIMIZABLE")), Some(Suffix::Possible));
    assert_eq!(translate(StringRef::new_from_string("optimized")), Some(Suffix::PastTense));
    assert_eq!(translate(StringRef::new_from_string("optimization")), Some(Suffix::Process));
    assert_eq!(translate(StringRef::new_from_string("optimizing")), Some(Suffix::InProgressAction));
    assert_eq!(translate(StringRef::new_from_string("optimizer")), Some(Suffix::Unknown));
  }

  #[test]
  fn test_cases() {
    #[derive(Debug,Clone, PartialEq)]
    enum OSType {
      Windows,
      Linux,
      Unknown
    }
    fn translate(s: StringRef) -> Option<OSType> {
      let mut switch: StringSwitch<OSType> = StringSwitch::new(s);
      let result = switch
        .cases_3(StringRef::new_from_string("windows"),
          StringRef::new_from_string("win32"),
          StringRef::new_from_string("winnt"),
          Some(OSType::Windows))
        .cases_4(StringRef::new_from_string("linux"),
          StringRef::new_from_string("unix"),
          StringRef::new_from_string("*nix"),
          StringRef::new_from_string("posix"),
          Some(OSType::Linux))
        .default(Some(OSType::Unknown));
      result
    }

    assert_eq!(translate(StringRef::new_from_string("windows")), Some(OSType::Windows));
    assert_eq!(translate(StringRef::new_from_string("win32")), Some(OSType::Windows));
    assert_eq!(translate(StringRef::new_from_string("winnt")), Some(OSType::Windows));

    assert_eq!(translate(StringRef::new_from_string("linux")), Some(OSType::Linux));
    assert_eq!(translate(StringRef::new_from_string("unix")), Some(OSType::Linux));
    assert_eq!(translate(StringRef::new_from_string("*nix")), Some(OSType::Linux));
    assert_eq!(translate(StringRef::new_from_string("posix")), Some(OSType::Linux));

    assert_eq!(translate(StringRef::new_from_string("winod")), Some(OSType::Unknown));
    assert_eq!(translate(StringRef::new_from_string("Win32")), Some(OSType::Unknown));
    assert_eq!(translate(StringRef::new_from_string("")), Some(OSType::Unknown));
  }

  #[test]
  fn test_cases_lower() {
    #[derive(Debug,Clone, PartialEq)]
    enum OSType {
      Windows,
      Linux,
      Unknown
    }
    fn translate(s: StringRef) -> Option<OSType> {
      let mut switch: StringSwitch<OSType> = StringSwitch::new(s);
      let result = switch
        .cases_lower_3(StringRef::new_from_string("windows"),
          StringRef::new_from_string("win32"),
          StringRef::new_from_string("winnt"),
          Some(OSType::Windows))
        .cases_lower_4(StringRef::new_from_string("linux"),
          StringRef::new_from_string("unix"),
          StringRef::new_from_string("*nix"),
          StringRef::new_from_string("posix"),
          Some(OSType::Linux))
        .default(Some(OSType::Unknown));
      result
    }

    assert_eq!(translate(StringRef::new_from_string("windows")), Some(OSType::Windows));
    assert_eq!(translate(StringRef::new_from_string("WINDOWS")), Some(OSType::Windows));
    assert_eq!(translate(StringRef::new_from_string("WIN32")), Some(OSType::Windows));
    assert_eq!(translate(StringRef::new_from_string("WINNT")), Some(OSType::Windows));

    assert_eq!(translate(StringRef::new_from_string("linux")), Some(OSType::Linux));
    assert_eq!(translate(StringRef::new_from_string("LINUX")), Some(OSType::Linux));
    assert_eq!(translate(StringRef::new_from_string("UNIX")), Some(OSType::Linux));
    assert_eq!(translate(StringRef::new_from_string("*NIX")), Some(OSType::Linux));
    assert_eq!(translate(StringRef::new_from_string("POSIX")), Some(OSType::Linux));

    assert_eq!(translate(StringRef::new_from_string("wind")), Some(OSType::Unknown));
    assert_eq!(translate(StringRef::new_from_string("")), Some(OSType::Unknown));
  }
}