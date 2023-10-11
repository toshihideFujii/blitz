#![allow(dead_code)]

// This file is a common base class of all globally definable objects. 
// As such, it is subclassed by GlobalVariable, GlobalAlias and by Function.

use std::fmt::Debug;
use std::any::Any;
use crate::{
  ir::{
    constant::Constant,
    metadata::MDNode,
    type_::Type,
    use_::Use,
    value::{ValueType, Value}
  },
  adt::twine::Twine
};

#[derive(Debug, Clone)]
pub enum IntrinsicID {
  FAdd,
  FSub,
  FMul,
  FDiv,
  FRem,
  FPExt,
  SIToFP,
  UIToFP,
  FPToSI,
  FPToUI,
  FPTrunc,

  LifetimeStart,
  LifetimeEnd,

  NotIntrinsic
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkageTypes {
  ExternalLinkage,
  AvailableExternallyLinkage,
  LinkOnceAnyLinkage,
  LinkOnceOdrLinkage,
  WeakAnyLinkage,
  WeakOdrLinkage,
  AppendingLinkage,
  InternalLinkage,
  PrivateLinkage,
  ExternalWeakLinkage,
  CommonLinkage
}

// An enumeration for the kinds of visibility of global values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisibilityTypes {
  DefaultVisibility,
  HiddenVisibility,
  ProtectedVisibility
}

// Storage classes of global values for PE targets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DllStorageClassTypes {
  DefaultStorageClass,
  DllImportStorageClass,
  DllExportStorageClass
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThreadLocalMode {
  NotThreadLocal,
  GeneralDynamicTlsModel,
  LocalDynamicTlsModel,
  InitialExecTlsModel,
  LocalExecTlsModel
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnnamedAddr {
  None,
  Local,
  Global
}

pub const GLOBAL_VALUE_SUB_CLASS_DATA_BITS: u32 = 15;

struct SanitizerMetadata {
  no_address: u32,
  no_hw_address: u32,
  mem_tag: u32,
  is_dyn_init: u32
}

pub trait GlobalValue : Debug + Constant {
  fn get_global_value_sub_class_data(&self) -> u32 { 0 }
  fn set_global_value_sub_class_data(&mut self, _v: u32) {}

  // Returns true if the definition of this global may be replaced by
  // a differently optimized variant of the same source level function
  // at link time.
  fn may_be_derefined(&self) -> bool { false }

  // Returns true if the global is a function definition with the
  // nobuiltin attribute.
  fn is_no_builtin_fn_def(&self) -> bool { false }

  fn has_global_unnamed_addr(&self) -> bool { false }

  // Returns true if this value's address is not significant in this module.
  fn has_at_least_local_unnamed_addr(&self) -> bool { false }

  fn get_unnamed_addr(&self) -> UnnamedAddr {
    UnnamedAddr::None
  }

  fn set_unnamed_addr(&mut self, _val: UnnamedAddr) {}

  fn get_min_unnamed_addr(&self, a: UnnamedAddr, b: UnnamedAddr) -> UnnamedAddr {
    if a == UnnamedAddr::None || b == UnnamedAddr::None {
      return UnnamedAddr::None;
    }
    if a == UnnamedAddr::Local || b == UnnamedAddr::Local {
      return UnnamedAddr::Local;
    }
    UnnamedAddr::Global
  }

  fn has_comdat(&self) {}
  fn get_comdat(&self) {}
  fn get_visibility(&self) -> VisibilityTypes { VisibilityTypes::DefaultVisibility }
  fn has_default_visibility(&self) -> bool { false }
  fn has_hidden_visibility(&self) -> bool { false }
  fn has_protected_visibility(&self) -> bool { false }
  fn set_visibility(&self) {}

  // If this value is 'Thread Local', its value isn't shared by the threads.
  fn is_thread_local(&self) -> bool { false }
  fn set_thread_local(&self) {}
  fn set_thread_local_mode(&self) {}
  fn get_thread_local_mode(&self) -> ThreadLocalMode {
    ThreadLocalMode::NotThreadLocal
  }
  fn get_dll_storage_class(&self) -> DllStorageClassTypes {
    DllStorageClassTypes::DefaultStorageClass
  }

  fn has_dll_import_storage_class(&self) -> bool { false }
  fn has_dll_export_storage_class(&self) -> bool { false }
  fn set_dll_storage_class(&mut self, _c: DllStorageClassTypes) {}
  fn has_section(&self) {}
  fn get_section(&self) {}
  fn get_type(&self) {}
  fn get_value_type(&self) {}
  fn is_implicit_dso_local(&self) -> bool {
    self.has_local_linkage() || (!self.has_default_visibility() &&
    !self.has_external_weak_linkage())
  }

  fn set_dso_local(&mut self, _local: bool) {}
  fn is_dso_local(&self) -> bool { false }
  fn has_partition(&self) -> bool { false }
  fn get_partition(&self) {}
  fn set_partition(&self) {}
  fn has_sanitizer_metadata(&self) -> bool { false }
  fn get_sanitizer_metadata(&self) {}
  fn set_sanitizer_metadata(&self) {}
  fn remove_sanitizer_metadata(&self) {}
  fn is_tagged(&self) -> bool { false }

  fn get_link_once_linkage(&self, odr: bool) -> LinkageTypes {
    if odr {
      return LinkageTypes::LinkOnceOdrLinkage;
    } else {
      return LinkageTypes::LinkOnceAnyLinkage;
    }
  }

  fn get_weak_linkage(&self, odr: bool) -> LinkageTypes {
    if odr {
      return LinkageTypes::WeakOdrLinkage;
    } else {
      return LinkageTypes::WeakAnyLinkage;
    }
  }

  fn is_external_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::ExternalLinkage
  }

  fn is_available_externally_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::AvailableExternallyLinkage
  }

  fn is_link_once_any_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::LinkOnceAnyLinkage
  }

  fn is_link_once_odr_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::LinkOnceOdrLinkage
  }

  fn is_link_once_linkage(&self, linkage: &LinkageTypes) -> bool {
    GlobalValue::is_link_once_any_linkage(self, linkage) ||
    GlobalValue::is_link_once_odr_linkage(self, linkage)
  }

  fn is_weak_any_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::WeakAnyLinkage
  }

  fn is_weak_odr_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::WeakOdrLinkage
  }

  fn is_weak_linkage(&self, linkage: &LinkageTypes) -> bool {
    GlobalValue::is_weak_any_linkage(self, linkage) ||
    GlobalValue::is_weak_odr_linkage(self, linkage)
  }

  fn is_appending_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::AppendingLinkage
  }

  fn is_internal_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::InternalLinkage
  }

  fn is_private_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::PrivateLinkage
  }

  fn is_local_linkage(&self, linkage: &LinkageTypes) -> bool {
    GlobalValue::is_internal_linkage(self, linkage) ||
    GlobalValue::is_private_linkage(self, linkage)
  }

  fn is_external_weak_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::ExternalWeakLinkage
  }

  fn is_common_linkage(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::CommonLinkage
  }

  fn is_valid_declaration_linkage(&self, linkage: &LinkageTypes) -> bool {
    GlobalValue::is_external_weak_linkage(self, linkage) ||
    GlobalValue::is_external_linkage(self, linkage)
  }

  fn is_interposable_linkage(&self) {}

  // Whether the definition of this global may be discarded if it is not
  // used in its compilation unit.
  fn is_discardable_if_unused(&self, linkage: &LinkageTypes) -> bool {
    GlobalValue::is_link_once_linkage(self, linkage) ||
    GlobalValue::is_local_linkage(self, linkage) ||
    GlobalValue::is_available_externally_linkage(self, linkage)
  }

  // Whether the definition of this global may be replaced at link time.
  fn is_weak_for_linker(&self, linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::WeakAnyLinkage ||
    *linkage == LinkageTypes::WeakOdrLinkage ||
    *linkage == LinkageTypes::LinkOnceAnyLinkage ||
    *linkage == LinkageTypes::LinkOnceOdrLinkage ||
    *linkage == LinkageTypes::CommonLinkage ||
    *linkage == LinkageTypes::ExternalWeakLinkage
  }

  fn is_definition_exact(&self) {}
  fn has_exact_definition(&self) {}
  fn is_interposable(&self) -> bool { false }
  fn can_benefit_from_local_alias(&self) {}
  fn has_external_linkage(&self) -> bool { false }
  fn has_available_externally_linkage(&self) -> bool { false }
  fn has_link_once_linkage(&self) -> bool { false }
  fn has_link_once_any_linkage(&self) -> bool { false }
  fn has_link_once_odr_linkage(&self) -> bool { false }
  fn has_weak_linkage(&self) -> bool { false }
  fn has_weak_any_linkage(&self) -> bool { false }
  fn has_weak_odr_linkage(&self) -> bool { false }
  fn has_appending_linkage(&self) -> bool { false }
  fn has_internal_linkage(&self) -> bool { false }
  fn has_private_linkage(&self) -> bool { false }
  fn has_local_linkage(&self) -> bool { false }
  fn has_external_weak_linkage(&self) -> bool { false }
  fn has_common_linkage(&self) -> bool { false }
  fn has_valid_declaration_linkage(&self) -> bool { false }
  fn set_linkage(&self) {}
  fn get_linkage(&self) -> LinkageTypes {
    LinkageTypes::AppendingLinkage
  }
  fn drop_blitz_magling_escape(&self) {}
  fn get_global_identifier(&self) {}
  fn get_guid(&self) {}
  fn is_materializable(&self) {}
  fn materialize(&self) {}

  // Return true if the primary definition of this global value is outside
  // of the current translation unit.
  fn is_declaration(&self) -> bool { false }

  fn is_declaration_for_linker(&self) {}
  fn is_strong_definition_for_linker(&self) {}
  fn get_aliasee_object(&self) {}
  fn is_absolute_symbol_ref(&self) {}
  fn get_absolute_symbol_range(&self) {}
  fn remove_from_parent(&self) {}
  fn erase_from_parent(&self) {}
  fn get_parent(&self) {}
  fn class_of(&self) {}
  fn can_be_ommitted_from_symbol_table(&self) {}
}

#[derive(Debug)]
pub struct GlobalValueBase {
  v_type: Box<dyn Type>,
  linkage: LinkageTypes,
  visibility: VisibilityTypes,
  unnamed_addr_val: UnnamedAddr,
  dll_storage_class: DllStorageClassTypes,
  thread_local: ThreadLocalMode,
  has_blitz_reserved_name: bool,
  is_dso_local: bool,
  has_partition: bool,
  has_sanitizer_metadata: bool,
  sub_class_data: u32,
  //parent: Module
}

impl GlobalValueBase {
  pub fn new(t: Box<dyn Type>, _v_id: ValueType, _ops: Option<Use>,
    _num_ops: usize, linkage: &LinkageTypes, _name: Twine, _addr_space: usize) -> Self
  {
    GlobalValueBase {
      v_type: t,
      linkage: linkage.clone(),
      visibility: VisibilityTypes::DefaultVisibility,
      unnamed_addr_val: UnnamedAddr::None,
      dll_storage_class: DllStorageClassTypes::DefaultStorageClass,
      thread_local: ThreadLocalMode::NotThreadLocal,
      has_blitz_reserved_name: false,
      is_dso_local: false,
      has_partition: false,
      has_sanitizer_metadata: false,
      sub_class_data: 0
    }
  }

  pub fn get_global_value_sub_class_data(&self) -> u32 {
    self.sub_class_data
  }

  pub fn set_global_value_sub_class_data(&mut self, v: u32) {
    debug_assert!(v < (1 << GLOBAL_VALUE_SUB_CLASS_DATA_BITS), "It will not fit.");
    self.sub_class_data = v;
  }
}

impl Value for GlobalValueBase {
  fn get_type(&self) -> &dyn Type {
    self.v_type.as_ref()
  }

  fn get_value_id(&self) -> ValueType {
    ValueType::GlobalVariableVal
  }

  fn get_subclass_data_from_value(&self) -> u32 {
    self.sub_class_data
  }

  fn set_value_subclass_data(&mut self, val: u32) {
    self.sub_class_data = val;
  }

  fn set_metadata(&mut self, _kind_id: u32, _node: Option<Box<dyn MDNode>>) {}

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Constant for GlobalValueBase {
  //fn as_any(&self) -> &dyn Any {
    //self
  //}
}

impl GlobalValue for GlobalValueBase {
  fn may_be_derefined(&self) -> bool {
    match self.get_linkage() {
      LinkageTypes::WeakOdrLinkage => return true,
      LinkageTypes::LinkOnceOdrLinkage => return true,
      LinkageTypes::AvailableExternallyLinkage => return true,
      _ => self.is_interposable() || self.is_no_builtin_fn_def()
    }
  }

  fn is_no_builtin_fn_def(&self) -> bool { false }

  fn has_global_unnamed_addr(&self) -> bool {
    self.unnamed_addr_val == UnnamedAddr::Global
  }

  // Returns true if this value's address is not significant in this module.
  fn has_at_least_local_unnamed_addr(&self) -> bool {
    self.unnamed_addr_val != UnnamedAddr::None
  }

  fn get_unnamed_addr(&self) -> UnnamedAddr {
    self.unnamed_addr_val.clone()
  }

  fn set_unnamed_addr(&mut self, val: UnnamedAddr) {
    self.unnamed_addr_val = val;
  }

  fn get_visibility(&self) -> VisibilityTypes {
    self.visibility.clone()
  }

  fn has_default_visibility(&self) -> bool {
    self.visibility == VisibilityTypes::DefaultVisibility
  }

  fn has_hidden_visibility(&self) -> bool {
    self.visibility == VisibilityTypes::HiddenVisibility
  }

  fn has_protected_visibility(&self) -> bool {
    self.visibility == VisibilityTypes::ProtectedVisibility
  }

  // If this value is 'Thread Local', its value isn't shared by the threads.
  fn is_thread_local(&self) -> bool {
    self.thread_local != ThreadLocalMode::NotThreadLocal
  }

  fn get_thread_local_mode(&self) -> ThreadLocalMode {
    self.thread_local.clone()
  }

  fn get_dll_storage_class(&self) -> DllStorageClassTypes {
    self.dll_storage_class.clone()
  }

  fn has_dll_import_storage_class(&self) -> bool {
    self.dll_storage_class == DllStorageClassTypes::DllImportStorageClass
  }

  fn has_dll_export_storage_class(&self) -> bool {
    self.dll_storage_class == DllStorageClassTypes::DllExportStorageClass
  }

  fn set_dll_storage_class(&mut self, c: DllStorageClassTypes) {
    debug_assert!(!self.has_local_linkage() ||
      c == DllStorageClassTypes::DefaultStorageClass,
      "Local linkage requires DefaultStorageClass.");
    self.dll_storage_class = c;
  }

  fn is_implicit_dso_local(&self) -> bool {
    self.has_local_linkage() || (!self.has_default_visibility() &&
    !self.has_external_weak_linkage())
  }

  fn set_dso_local(&mut self, local: bool) {
    self.is_dso_local = local
  }

  fn is_dso_local(&self) -> bool {
    self.is_dso_local
  }

  fn has_partition(&self) -> bool {
    self.has_partition
  }

  fn has_sanitizer_metadata(&self) -> bool {
    self.has_sanitizer_metadata
  }

  fn has_external_linkage(&self) -> bool {
    GlobalValueBase::is_external_linkage(self, &self.linkage)
  }

  fn has_available_externally_linkage(&self) -> bool {
    GlobalValueBase::is_available_externally_linkage(self, &self.linkage)
  }

  fn has_link_once_linkage(&self) -> bool {
    GlobalValueBase::is_link_once_linkage(self, &self.linkage)
  }

  fn has_link_once_any_linkage(&self) -> bool {
    GlobalValueBase::is_link_once_any_linkage(self, &self.linkage)
  }

  fn has_link_once_odr_linkage(&self) -> bool {
    GlobalValueBase::is_link_once_odr_linkage(self, &self.linkage)
  }

  fn has_weak_linkage(&self) -> bool {
    GlobalValueBase::is_weak_linkage(self, &self.linkage)
  }

  fn has_weak_any_linkage(&self) -> bool {
    GlobalValueBase::is_weak_any_linkage(self, &self.linkage)
  }

  fn has_weak_odr_linkage(&self) -> bool {
    GlobalValueBase::is_weak_odr_linkage(self, &self.linkage)
  }

  fn has_appending_linkage(&self) -> bool {
    GlobalValueBase::is_appending_linkage(self, &self.linkage)
  }

  fn has_internal_linkage(&self) -> bool {
    GlobalValueBase::is_internal_linkage(self, &self.linkage)
  }

  fn has_private_linkage(&self) -> bool {
    GlobalValueBase::is_private_linkage(self, &self.linkage)
  }

  fn has_local_linkage(&self) -> bool {
    GlobalValueBase::is_local_linkage(self, &self.linkage)
  }

  fn has_external_weak_linkage(&self) -> bool {
    GlobalValueBase::is_external_weak_linkage(self, &self.linkage)
  }

  fn has_common_linkage(&self) -> bool {
    GlobalValueBase::is_common_linkage(self, &self.linkage)
  }

  fn has_valid_declaration_linkage(&self) -> bool {
    GlobalValueBase::is_valid_declaration_linkage(self, &self.linkage)
  }

  fn get_linkage(&self) -> LinkageTypes {
    self.linkage.clone()
  }
}