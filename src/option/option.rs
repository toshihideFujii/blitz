#![allow(dead_code)]

use crate::adt::string_ref::StringRef;
use super::{opt_table::{OptTableInfo, OptTable}, opt_specifier::OptSpecifier};

// Base flags for all options. Custom flags may be added after.
enum DriverFlag {
  HelpHidden = (1 << 0),
  RenderAsInput = (1 << 1),
  RenderJoined = (1 << 2),
  RenderSeparate = (1 << 3)
}

#[derive(Debug, Clone)]
pub enum OptionClass {
  GroupClass,
  InputClass,
  UnknownClass,
  FlagClass,
  JoinedClass,
  ValuesClass,
  SeparateClass,
  RemainingArgsClass,
  RemainingArgsJoinedClass,
  CommaJoinedClass,
  MultiArgClass,
  JoinedOrSeparateClass,
  JoinedAndSeparateClass
}

pub enum RenderStyleKind {
  RenderCommaJoinedStyle,
  RenderJoinedStyle,
  RenderSeparateStyle,
  RenderValuesStyle
}

pub struct Option_ {
  info: Option<OptTableInfo>,
  owner: Option<OptTable>
}

impl Option_ {
  pub fn new(info: Option<OptTableInfo>, owner: Option<OptTable>) -> Self {
    Option_ { info: info, owner: owner }
  }

  pub fn is_valid(&self) -> bool {
    self.info.is_some()
  }

  pub fn get_id(&self) -> u32 {
    debug_assert!(self.is_valid(), "Must have a valid info.");
    self.info.as_ref().unwrap().id
  }

  pub fn get_kind(&self) -> OptionClass {
    debug_assert!(self.is_valid(), "Must have a valid info.");
    self.info.as_ref().unwrap().kind.clone()
  }

  // Get the name of this option without any prefix.
  pub fn get_name(&self) -> StringRef {
    debug_assert!(self.is_valid(), "Must have a valid info.");
    self.info.as_ref().unwrap().name.clone()
  }

  pub fn get_group(&self) -> Option_ {
    debug_assert!(self.is_valid(), "Must have a valid info.");
    debug_assert!(self.owner.is_some(), "Must have a valid owner.");
    self.owner.as_ref().unwrap().get_option(self.info.as_ref().unwrap().group_id.clone())
  }

  pub fn get_alias(&self) -> Option_ {
    debug_assert!(self.is_valid(), "Must have a valid info.");
    debug_assert!(self.owner.is_some(), "Must have a valid owner.");
    self.owner.as_ref().unwrap().get_option(self.info.as_ref().unwrap().alias_id.clone())
  }

  // Get the arguments as a \0 separated list.
  // e.g. ["foo", "bar"] would be returned as "foo\0bar\0".
  pub fn get_alias_args(&self) -> String {
    debug_assert!(self.is_valid(), "Must have a valid info.");
    self.info.as_ref().unwrap().alias_args.clone()
  }

  // Get the default prefix for this option.
  pub fn get_prefix(&self) -> String {
    if self.info.as_ref().unwrap().prefixes.len() == 0 {
      return String::new();
    } else {
      return self.info.as_ref().unwrap().prefixes[0].clone();
    }
  }

  // Get the name of this option with the default prefix.
  pub fn get_prefixed_name(&self) -> String {
    let mut name = self.get_prefix();
    name.push_str(&self.get_name().data());
    name
  }

  // Get the help text for this option.
  pub fn get_help_text(&self) -> String {
    debug_assert!(self.is_valid(), "Must have a valid info.");
    self.info.as_ref().unwrap().help_text.clone()
  }

  // Get the meta-variable list for this option.
  pub fn get_meta_var(&self) -> String {
    debug_assert!(self.is_valid(), "Must have a valid info.");
    self.info.as_ref().unwrap().meta_var.clone()
  }

  pub fn get_num_args(&self) -> u32 {
    self.info.as_ref().unwrap().param
  }

  pub fn has_no_opt_as_input(&self) -> bool {
    self.info.as_ref().unwrap().flags & DriverFlag::RenderAsInput as u32 != 0
  }

  pub fn get_render_style(&self) -> RenderStyleKind {
    if self.info.as_ref().unwrap().flags & DriverFlag::RenderJoined as u32 != 0 {
      return RenderStyleKind::RenderJoinedStyle;
    }
    if self.info.as_ref().unwrap().flags & DriverFlag::RenderSeparate as u32 != 0 {
      return RenderStyleKind::RenderSeparateStyle;
    }
    match self.get_kind() {
      OptionClass::GroupClass => return RenderStyleKind::RenderValuesStyle,
      OptionClass::InputClass => return RenderStyleKind::RenderValuesStyle,
      OptionClass::UnknownClass => return RenderStyleKind::RenderValuesStyle,
      OptionClass::JoinedClass => return RenderStyleKind::RenderJoinedStyle,
      OptionClass::JoinedAndSeparateClass => return RenderStyleKind::RenderJoinedStyle,
      OptionClass::CommaJoinedClass => return RenderStyleKind::RenderCommaJoinedStyle,
      _ => return RenderStyleKind::RenderSeparateStyle
    };
  }

  // Test if this option has the flag val.
  pub fn has_flag(&self, val: u32) -> bool {
    self.info.as_ref().unwrap().flags & val != 0
  }

  // Return the final option this option aliases (itself, if the option
  // has no alias).
  pub fn get_unaliased_option(&self) -> &Option_ {
    //let alias = &self.get_alias();
    //if alias.is_valid() {
      //return alias.get_unaliased_option();
    //}
    self
  }

  // Return the name to use when rendering this option.
  pub fn get_render_name(&self) -> StringRef {
    self.get_unaliased_option().get_name()
  }

  pub fn matches(&self, opt: OptSpecifier) -> bool {
    // Aliases are never considered in matching, look through them.
    let alias = self.get_alias();
    if alias.is_valid() {
      return alias.matches(opt);
    }
    // Check exact match.
    if self.get_id() == opt.get_id() {
      return true;
    }
    let group = self.get_group();
    if group.is_valid() {
      return group.matches(opt);
    }
    false
  }

  pub fn accept() {}
  pub fn print() {}
  pub fn dump() {}
  fn accept_internal() {}
}