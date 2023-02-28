#![allow(dead_code)]

use super::user::User;
use crate::ir::value::Value;

/*
This file defines the Use class.
The Use class represents the operand of an instruction
or some other User instance which refers to a Value.
*/

// A Use represents the edge between a Value definition
// and its Users.
#[derive(Debug, Clone)]
struct Use {
  val: Option<Value>,
  next: Box<Option<Use>>,
  prev: Box<Option<Use>>,
  parent: User
}

impl Use {
  pub fn new(parent: User) -> Self {
    Use { val: None, next: Box::new(None), prev: Box::new(None), parent: parent }
  }

  pub fn swap() {}

  pub fn get(&self) -> Option<Value> {
    self.val.clone()
  }

  // Returns the User that contains this Use.
  // For an instruction operand, for example, this will return the instruction.
  pub fn get_user(&self) -> User {
    self.parent.clone()
  }

  pub fn set(&mut self, val: Value) {
    self.val = Some(val);
  }

  pub fn get_next(&self) -> Box<Option<Use>> {
    self.next.clone()
  }

  pub fn get_operand_no() {}

  pub fn zap() {}

  fn add_to_list(&mut self, list: Box<Option<Use>>) {
    self.next = list;
    if self.next.is_some() {
      self.next.clone().unwrap().prev = self.next.clone(); 
    }
    //self.prev = list.clone();
  }

  fn remove_from_list(&mut self) {
    self.prev = self.next.clone();
    if self.next.is_some() {
      self.next.clone().unwrap().prev = self.prev.clone();
    }
  }
}