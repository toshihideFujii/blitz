#![allow(dead_code)]

use super::user;
//use crate::ir::value::Value;

/*
This file defines the Use class.
The Use class represents the operand of an instruction
or some other User instance which refers to a Value.
*/

// A Use represents the edge between a Value definition
// and its Users.
#[derive(Clone)]
struct Use {
  //val_: Value,
  next_: Box<Use>,
  prev_: Box<Use>,
  parent_: user::User
}

impl Use {
  pub fn new() {}

  pub fn swap() {}

  pub fn get() {}

  // Returns the User that contains this Use.
  pub fn get_user(&self) -> user::User {
    self.parent_.clone()
  }

  pub fn set() {}

  pub fn get_next(&self) -> Box<Use> {
    self.next_.clone()
  }

  pub fn get_operand_no() {}

  pub fn zap() {}

  fn add_to_list(&mut self, list: Box<Use>) {
    self.next_ = list;
    //if self.next_ {
      // 
    //}
  }

  fn remove_from_list(&mut self) {
    //self.prev_ = self.next_;
    // if self.next_ {}
  }
}