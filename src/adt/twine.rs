#![allow(dead_code)]

use std::io::{self, Write};

use crate::adt::string_ref::StringRef;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
  // An empty string;
  // the result of concatenating anything with it is also rmpty.
  NullKind,
  // The empty string.
  EmptyKind,
  // A pointer to a Twine instance.
  TwineKind,
  // A pointer to a string instance.
  StringKind,
  // A pointer and length representation.
  PtrAndLengthKind,
  FormatvObjectKind,
  // A char value, to render as a character.
  CharKind,
  // An unsigned int value, to render as an unsigned decimal integer.
  DecUIKind,
  // An int value, to render as a signed decimal integer.
  DecIKind,
  // A pointer to an unsigned long value, to render as an unsigned
  // decimal integer.
  DecULKind,
  // A pointer to a long value, to render as a signed decimal integer.
  DecLKind,
  // A pointer to an unsigned long long value, to render as an
  // unsigned decimal integer.
  DecULLKind,
  // A pointer to a long long value, to render as a signed decimal integer.
  DecLLKind,
  // A pointer to a u64 value, to render as an unsigned hexadecimal integer.
  UHexKind
}

#[derive(Debug, Clone)]
pub struct Twine {
  // The prefix in the concatenation.
  lhs_: Option<Box<Child>>,
  // The suffix in the concatenation.
  rhs_: Option<Box<Child>>,
  // The NodeKind of the left hand side.
  lhs_kind_: NodeKind,
  // The NodeKind of the right hand side.
  rhs_kind_: NodeKind
}

impl Twine {
  // Construct a nullary twine; the kind must be NullKind or EmptyKind.
  pub fn new_from_kind(kind: NodeKind) -> Self {
    Twine { lhs_: None, rhs_: None, lhs_kind_: kind, rhs_kind_: NodeKind::NullKind }
  }

  // Construct from a string.
  pub fn new_from_string(str: String) -> Self {
    Twine {
      lhs_: Some(Box::new(Child::new_from_string(str))), 
      rhs_: None, 
      lhs_kind_: NodeKind::StringKind, 
      rhs_kind_: NodeKind::EmptyKind 
    }
  }

  pub fn new_from_character(c: char) -> Self {
    Twine {
      lhs_: Some(Box::new(Child::new_from_character(c))), 
      rhs_: None, 
      lhs_kind_: NodeKind::CharKind, 
      rhs_kind_: NodeKind::NullKind 
    }
  }

  pub fn new_from_decui(decui: u32) -> Self {
    Twine {
      lhs_: Some(Box::new(Child::new_from_decui(decui))), 
      rhs_: None, 
      lhs_kind_: NodeKind::DecUIKind, 
      rhs_kind_: NodeKind::NullKind 
    }
  }

  pub fn new_from_deci(deci: i32) -> Self {
    Twine {
      lhs_: Some(Box::new(Child::new_from_deci(deci))), 
      rhs_: None, 
      lhs_kind_: NodeKind::DecIKind, 
      rhs_kind_: NodeKind::NullKind 
    }
  }

  pub fn new_from_decull(decull: u64) -> Self {
    Twine {
      lhs_: Some(Box::new(Child::new_from_decull(decull))), 
      rhs_: None, 
      lhs_kind_: NodeKind::DecULLKind, 
      rhs_kind_: NodeKind::NullKind 
    }
  }

  // Check for the null twine.
  pub fn is_null(&self) -> bool {
    self.get_lhs_kind() == NodeKind::NullKind
  }

  // Check for the empty twine.
  pub fn is_empty(&self) -> bool {
    self.get_lhs_kind() == NodeKind::EmptyKind
  }

  // Check if this is a nullary twine (null or empty).
  pub fn is_nullary(&self) -> bool {
    self.is_null() || self.is_empty()
  }

  // Check if this is a unary twine.
  pub fn is_unary(&self) -> bool {
    self.get_rhs_kind() == NodeKind::EmptyKind && !self.is_nullary()
  }

  // Check if this is a binary twine.
  pub fn is_binary(&self) -> bool {
    self.get_lhs_kind() != NodeKind::NullKind &&
    self.get_rhs_kind() != NodeKind::EmptyKind
  }

  // Check if this is a valid twine.
  pub fn is_valid(&self) -> bool {
    // Nullary twine always have Empty on the RHS.
    if self.is_nullary() && self.get_rhs_kind() != NodeKind::EmptyKind {
      return false;
    }
    // Null should never appear on the RHS.
    if self.get_rhs_kind() == NodeKind::NullKind {
      return false;
    }
    // The RHS cannot be non-empty if the LHS is empty.
    if self.get_rhs_kind() != NodeKind::EmptyKind &&
       self.get_lhs_kind() == NodeKind::EmptyKind {
      return false;
    }
    // A twine child should always be binary.
    if self.get_lhs_kind() == NodeKind::TwineKind &&
      !self.clone().lhs_.unwrap().twine_.unwrap().is_binary() {
      return false;
    }
    if self.get_rhs_kind() == NodeKind::TwineKind &&
      !self.clone().rhs_.unwrap().twine_.unwrap().is_binary() {
      return false;
    }

    true
  }

  // Get the NodeKind of the left-hand side.
  pub fn get_lhs_kind(&self) -> NodeKind {
    self.lhs_kind_.clone()
  }

  // Get the NodeKind of the left-hand side.
  pub fn get_rhs_kind(&self) -> NodeKind {
    self.rhs_kind_.clone()
  }

  // Check if this twine is trivially empty.
  pub fn is_trivially_empty(&self) -> bool {
    self.is_nullary()
  }

  // Return true if this twine can be dynamically accessed as a
  // single StringRef value with get_single_string_ref().
  pub fn is_single_string_ref(&self) -> bool {
    if self.get_rhs_kind() != NodeKind::EmptyKind {
      return false;
    }
    match self.get_lhs_kind() {
      NodeKind::EmptyKind => return true,
      NodeKind::StringKind => return true,
      NodeKind::PtrAndLengthKind => return true,
      _ => return false
    };
  }

  pub fn concat() {}

  // Return the twine contents as a string.
  pub fn str(&self) -> Option<String> {
    // If we're storing only a string, just return it.
    if self.lhs_kind_ == NodeKind::StringKind && 
       self.rhs_kind_ == NodeKind::EmptyKind {
      return self.lhs_.clone().unwrap().string_;
    }

    if self.lhs_kind_ == NodeKind::FormatvObjectKind &&
       self.rhs_kind_ == NodeKind::EmptyKind {
      return None; // TODO
    }

    None
  }

  pub fn to_vector() {}

  // This returns the twine as a single StringRef.
  pub fn get_single_string_ref(&self) -> StringRef {
    debug_assert!(self.is_single_string_ref(), "This cannot be had as a single stringref!");
    match self.get_lhs_kind() {
      NodeKind::EmptyKind => return StringRef::new(),
      NodeKind::StringKind =>
        return StringRef::new_from_string(self.lhs_.clone().unwrap().string_.unwrap().as_str()),
      _ => return StringRef::new()  
    };
  }

  // This returns the twine as a single StringRef if it can be
  // represented as such.
  pub fn to_string_ref(&self) -> StringRef {
    if self.is_single_string_ref() {
      return self.get_single_string_ref();
    }
    StringRef::new()
  }

  // Write the concatenated string represented by this twine.
  pub fn print(&self) {
    self.print_one_child(self.lhs_.clone(), self.get_lhs_kind());
    self.print_one_child(self.rhs_.clone(), self.get_rhs_kind());
  }

  // Print one child from a twine.
  pub fn print_one_child(&self, child: Option<Box<Child>>, kind: NodeKind) {
    match kind {
      NodeKind::NullKind => return,
      NodeKind::EmptyKind => return,
      NodeKind::StringKind => {
        let val = child.unwrap().string_;
        let _result = io::stdout().write(val.unwrap().as_bytes());
      },
      _ => return
    };
  }

  pub fn dump() {}

  pub fn print_repr() {}
  pub fn print_one_child_repr() {}

  pub fn dump_repr() {}
}

#[derive(Debug, Clone)]
pub struct Child {
  twine_: Option<Twine>,
  string_: Option<String>,
  character_: Option<char>,
  dec_ui_: Option<u32>,
  dec_i_: Option<i32>,
  dec_ul_: Option<u32>,
  dec_ull_: Option<u64>,
  u_hex_: Option<u64>
}

impl Child {
  pub fn new() -> Self {
    Child { 
      twine_: None,
      string_: None, 
      character_: None, 
      dec_ui_: None, 
      dec_i_: None, 
      dec_ul_: None, 
      dec_ull_: None, 
      u_hex_: None 
    }
  }

  pub fn new_from_string(str: String) -> Self {
    Child { 
      twine_: None,
      string_: Some(str), 
      character_: None, 
      dec_ui_: None, 
      dec_i_: None, 
      dec_ul_: None, 
      dec_ull_: None, 
      u_hex_: None 
    }
  }

  pub fn new_from_character(c: char) -> Self {
    Child { 
      twine_: None,
      string_: None, 
      character_: Some(c), 
      dec_ui_: None, 
      dec_i_: None, 
      dec_ul_: None, 
      dec_ull_: None, 
      u_hex_: None 
    }
  }

  pub fn new_from_decui(decui: u32) -> Self {
    Child { 
      twine_: None,
      string_: None, 
      character_: None, 
      dec_ui_: Some(decui), 
      dec_i_: None, 
      dec_ul_: None, 
      dec_ull_: None, 
      u_hex_: None 
    }
  }

  pub fn new_from_deci(deci: i32) -> Self {
    Child { 
      twine_: None,
      string_: None, 
      character_: None, 
      dec_ui_: None, 
      dec_i_: Some(deci), 
      dec_ul_: None, 
      dec_ull_: None, 
      u_hex_: None 
    }
  }

  pub fn new_from_decull(decull: u64) -> Self {
    Child { 
      twine_: None,
      string_: None, 
      character_: None, 
      dec_ui_: None, 
      dec_i_: None, 
      dec_ul_: None, 
      dec_ull_: Some(decull), 
      u_hex_: None 
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construction() {
    assert_eq!(Twine::new_from_string(String::from("hi")).str(), Some(String::from("hi")));
  }

  #[test]
  fn test_characters() {
    //assert_eq!(Twine::new_from_character('x').str(), Some(String::from("x")));
  }
}