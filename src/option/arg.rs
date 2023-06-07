#![allow(dead_code)]

// Defines the Blitz::Arg class for parsed arguments.

use crate::adt::{string_ref::StringRef, /*small_vector::SmallVector*/};
use super::option::Option_;

// A concrete instance of a particular driver option.
// The Arg class encodes just enough information to be able to
// derive the argument values efficiently.
pub struct Arg {
  // The option this argument is an instance of.
  opt: Option_,
  // The argument this argument was derived from (during tool chain
  // argument translation), if any.
  base_arg: Option<Box<Arg>>,
  // How this instance of the option was spelled.
  spelling: StringRef,
  // The index at which this argument appears in the containing ArgList.
  index: usize,
  claimed: bool,
  owns_values: bool,
  // The argument values, as C strings.
  values: Vec<char>,
  // If this arg was created through an alias, this is the original alias arg.
  alias: Option<Box<Arg>>
}

impl Arg {
  pub fn new(opt: Option_, spelling: StringRef, 
    index: usize, base_arg: Option<Box<Arg>>) -> Self
  {
    Arg { opt: opt, base_arg: base_arg, spelling: spelling, index: index,
      claimed: false, owns_values: false, values: Vec::new(), alias: None }
  }

  pub fn get_option(&self) -> &Option_ {
    &self.opt
  }

  // Returns the used prefix and name of the option: For `--foo=bar`, returns `--foo=`.
  pub fn get_spelling(&self) -> &StringRef{
    &self.spelling
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  // Return the base argument which generated this arg.
  pub fn get_base_arg(&self) -> &Arg {
    if self.base_arg.is_some() {
      return self.base_arg.as_ref().unwrap();
    } else {
      return &self;
    }
  }

  pub fn set_base_arg(&mut self, arg: Arg) {
    self.base_arg = Some(Box::new(arg));
  }

  // Args are converted to their unaliased form.
  // For args that originally came from an alias, this returns the alias
  // the arg was produced from.
  pub fn get_alias(&self) -> &Arg {
    &self.alias.as_ref().unwrap()
  }

  pub fn set_alias(&mut self, arg: Arg) {
    self.alias = Some(Box::new(arg));
  }

  pub fn get_owns_values(&self) -> bool {
    self.owns_values
  }

  pub fn set_owns_values(&mut self, value: bool) {
    self.owns_values = value;
  }
  
  pub fn is_claimed(&self) -> bool {
    self.get_base_arg().claimed
  }

  // set the Arg claimed bit.
  pub fn claim(&mut self) {
    if self.base_arg.is_none() {
      self.claimed = true;
    } else {
      self.base_arg.as_mut().unwrap().claimed = true;
    }
  }

  pub fn get_num_values(&self) -> usize {
    self.values.len()
  }

  pub fn get_value(&self, n: usize) -> &char {
    &self.values[n]
  }

  pub fn get_values(&self) -> &Vec<char> {
    &self.values
  }

  pub fn contains_value() {}
  pub fn render() {}
  pub fn render_as_input() {}
  pub fn print() {}
  pub fn dump() {}
  pub fn get_as_string() {}
}