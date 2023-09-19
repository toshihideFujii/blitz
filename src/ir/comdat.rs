#![allow(dead_code)]

// THis file contains the declarations of the Comdat class, which
// represents a single COMDAT in Blitz.

//use crate::adt::{/*small_ptr_set::SmallPtrSet,*/ /*string_map::StringMapEntry*/};
//use super::global_object::GlobalObject;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectionKind {
  Any,
  ExactMatch,
  Largest,
  NoDeduplicate,
  SameSize
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comdat {
  sk: SelectionKind,
  //name: StringMapEntry<Box<Comdat>>,
  //users: SmallPtrSet<GlobalObject>
}

impl Comdat {
  pub fn new() -> Self {
    Comdat {
      sk: SelectionKind::Any,
      //name: StringMapEntry::new(),
      //users: SmallPtrSet::new()
    }
  }

  pub fn get_selection_kind(&self) -> SelectionKind {
    self.sk.clone()
  }

  pub fn set_selection_kind(&mut self, sk: SelectionKind) {
    self.sk = sk.clone()
  }

  pub fn get_name(&self) {}

  //pub fn get_users(&self) -> &SmallPtrSet<GlobalObject> {
    //&self.users
  //}

  pub fn print(&self) {
    // print_blitz_name(self.get_name())
    println!(" = comdat ");
    match self.get_selection_kind() {
      SelectionKind::Any => println!("any"),
      SelectionKind::ExactMatch => println!("exactmatch"),
      SelectionKind::Largest => println!("largest"),
      SelectionKind::NoDeduplicate => println!("nodeduplicate"),
      SelectionKind::SameSize => println!("samesize")
    };
  }

  pub fn dump(&self) {
    self.print()
  }

  //fn add_user(&mut self, go: GlobalObject) {
    //self.users.insert(go)
  //}

  //fn remove_user(&mut self, go: GlobalObject) {
    //self.users.erase(go)
  //}
}