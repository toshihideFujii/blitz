#![allow(dead_code)]

// Definition of ModRefInfo and MemoryEffects, which are used to
// describe the memory effects of instructions.

// Flags indicating whether a memory access modifies or references
// memory.
#[derive(Debug, Clone, PartialEq)]
pub enum ModRefInfo {
  NoModRef,
  Ref,
  Mod,
  ModRef
}

pub fn is_no_mod_ref(mri: ModRefInfo) -> bool {
  mri == ModRefInfo::NoModRef
}

pub fn is_mod_or_ref_set(mri: ModRefInfo) -> bool {
  mri != ModRefInfo::NoModRef
}

pub fn is_mod_and_ref_set(mri: ModRefInfo) -> bool {
  mri == ModRefInfo::ModRef
}

pub fn is_mod_set(mri: ModRefInfo) -> bool {
  mri == ModRefInfo::Mod
}

pub fn is_ref_set(mri: ModRefInfo) -> bool {
  mri == ModRefInfo::Ref
}


// The location at which a function might access memory.
#[derive(Debug, Clone)]
pub enum Location {
  ArgMem,
  InaccessibleMem,
  Other
}

const BITS_PER_LOCK: u32 = 2;
const LOC_MASK: u32 = (1 << 2) - 1;

// Summary of how a function affects memory in the program.
// Loads from constant globals are not considered memoey accesses
// for this interface.
pub struct MemoryEffects {
  data: u32
}

impl MemoryEffects {
  pub fn new(data: u32) -> Self {
    MemoryEffects { data: data }
  }

  // Create MemoryEffects that can access only the given location 
  // with the given ModRefInfo.
  pub fn new_from_loc_and_info(loc: Location, mri: ModRefInfo) -> Self {
    let mut instance = MemoryEffects { data: 0 };
    instance.set_mod_ref(loc, mri);
    instance
  }

  // Create MemoryEffects that can access any location with the
  // given ModRefInfo.
  pub fn new_from_info(mri: ModRefInfo) -> Self {
    let mut instance = MemoryEffects { data: 0 };
    instance.set_mod_ref(Location::ArgMem, mri.clone());
    instance.set_mod_ref(Location::InaccessibleMem, mri.clone());
    instance.set_mod_ref(Location::Other, mri.clone());
    instance
  }

  // Create MemoryEffects that can read and write any memory.
  pub fn unknown() -> Self {
    MemoryEffects::new_from_info(ModRefInfo::ModRef)
  }

  // Create MemoryEffects that cannot read or write any memory.
  pub fn none() -> Self {
    MemoryEffects::new_from_info(ModRefInfo::NoModRef)
  }

  // Create MemoryEffects that can read any memory.
  pub fn read_only() -> Self {
    MemoryEffects::new_from_info(ModRefInfo::Ref)
  }

  // Create MemoryEffects that can write any memory.
  pub fn write_only() -> Self {
    MemoryEffects::new_from_info(ModRefInfo::Mod)
  }

  // Create MemoryEffects that can only access argument memory.
  pub fn arg_mem_only(mri: ModRefInfo) -> Self {
    MemoryEffects::new_from_loc_and_info(Location::ArgMem, mri)
  }

  // Create MemoryEffects that can only access inaccessible memory.
  pub fn inaccessible_mem_only(mri: ModRefInfo) -> Self {
    MemoryEffects::new_from_loc_and_info(Location::InaccessibleMem, mri)
  }

  // Create MemoryEffects that can obly access inaccessible or argument memory.
  pub fn inaccessible_or_arg_mem_only(mri: ModRefInfo) -> Self {
    let mut instance = MemoryEffects { data: 0 };
    instance.set_mod_ref(Location::ArgMem, mri.clone());
    instance.set_mod_ref(Location::InaccessibleMem, mri.clone());
    instance
  }

  // Create MemoryEffects from an encoded integer value (used by memory
  // attribute).
  pub fn create_from_int_value(data: u32) -> Self {
    MemoryEffects::new(data)
  }

  // Convert MemoryEffects into an encoded integer value (used by memory
  // attribute)/
  pub fn to_int_value(&self) -> u32 {
    self.data
  }

  // Get ModRefInfo for the given location.
  pub fn get_mod_ref_given_loc(&self, loc: Location) -> ModRefInfo {
    let val = (self.data >> MemoryEffects::get_location_pos(loc)) & LOC_MASK;
    match val {
      0 => return ModRefInfo::NoModRef,
      1 => return ModRefInfo::Ref,
      2 => return ModRefInfo::Mod,
      3 => return ModRefInfo::ModRef,
      _ => panic!("Unexpected value: {}.", val)
    };
  }

  // Get new MemoryEffects with modified ModRefInfo for Loc.
  pub fn get_with_mod_ref(&mut self, loc: Location, mri: ModRefInfo) -> &Self {
    self.set_mod_ref(loc, mri);
    self
  }

  // Get new MemoryEffects with NoModRef on the given loc.
  pub fn get_without_loc(&mut self, loc: Location) -> &Self {
    self.set_mod_ref(loc, ModRefInfo::NoModRef);
    self
  }

  pub fn get_mod_ref_any_loc(&self) -> ModRefInfo {
    ModRefInfo::Mod
  }

  // Whether this function accesses no memory.
  pub fn does_not_access_memory(&self) -> bool {
    self.data == 0
  }

  // Whether this function only (at most) reads memory.
  pub fn only_reads_memory(&self) -> bool {
    !is_mod_set(self.get_mod_ref_any_loc())
  }

  // Whether this function only (at most) writes memory.
  pub fn only_writes_memory(&self) -> bool {
    !is_ref_set(self.get_mod_ref_any_loc())
  }

  // Whether this function only (at most) accesses argument memory.
  pub fn only_accesses_arg_pointees(&mut self) -> bool {
    self.get_without_loc(Location::ArgMem).does_not_access_memory()
  }

  // Whether this function may access argument memory.
  pub fn does_access_arg_pointees(&self) -> bool {
    is_mod_or_ref_set(self.get_mod_ref_given_loc(Location::ArgMem))
  }

  // Whether this function only (at most) accesses inaccessible memory.
  pub fn only_accesses_inaccessible_mem(&mut self) -> bool {
    self.get_without_loc(Location::InaccessibleMem).does_not_access_memory()
  }

  // Whether this function only (at most) accesses argument and inaccessible
  // memory.
  pub fn only_accesses_inaccessible_or_arg_mem(&self) -> bool {
    is_no_mod_ref(self.get_mod_ref_given_loc(Location::Other))
  }

  fn get_location_pos(loc: Location) -> u32 {
    loc as u32 * BITS_PER_LOCK
  }

  fn set_mod_ref(&mut self, loc: Location, mri: ModRefInfo) {
    self.data &= !(LOC_MASK << MemoryEffects::get_location_pos(loc.clone()));
    self.data |= (mri as u32) << MemoryEffects::get_location_pos(loc.clone());
  }
}