#![allow(dead_code)]

// This file is a common base class of all globally definable objects. 
// As such, it is subclassed by GlobalVariable, GlobalAlias and by Function.

use crate::{ir::type_::Type, adt::twine::Twine};
use super::{value::ValueType, use_::Use};

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

const GLOBAL_VALUE_SUB_CLASS_DATA_BITS: u32 = 15;

struct SanitizerMetadata {
  no_address: u32,
  no_hw_address: u32,
  mem_tag: u32,
  is_dyn_init: u32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalValue {
  //v_type: Box<dyn Type>,
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

impl GlobalValue {
  pub fn new(_t: Box<dyn Type>, _v_id: ValueType, _ops: Option<Use>,
    _num_ops: usize, linkage: &LinkageTypes, _name: Twine, _addr_space: usize) -> Self
  {
    GlobalValue {
      //v_type: t,
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

  // Returns true if the definition of this global may be replaced by
  // a differently optimized variant of the same source level function
  // at link time.
  pub fn may_be_derefined(&self) -> bool {
    match self.get_linkage() {
      LinkageTypes::WeakOdrLinkage => return true,
      LinkageTypes::LinkOnceOdrLinkage => return true,
      LinkageTypes::AvailableExternallyLinkage => return true,
      _ => self.is_interposable() || self.is_no_builtin_fn_def()
    }
  }

  // Returns true if the global is a function definition with the nobuiltin
  // attribute.
  pub fn is_no_builtin_fn_def(&self) -> bool { false }

  pub fn get_global_value_sub_class_data(&self) -> u32 {
    self.sub_class_data
  }

  pub fn set_global_value_sub_class_data(&mut self, v: u32) {
    debug_assert!(v < (1 << GLOBAL_VALUE_SUB_CLASS_DATA_BITS), "It will not fit.");
    self.sub_class_data = v;
  }

  pub fn set_parent() {}
  pub fn get_address_space() {}

  pub fn has_global_unnamed_addr(&self) -> bool {
    self.unnamed_addr_val == UnnamedAddr::Global
  }

  // Returns true if this value's address is not significant in this module.
  pub fn has_at_least_local_unnamed_addr(&self) -> bool {
    self.unnamed_addr_val != UnnamedAddr::None
  }

  pub fn get_unnamed_addr(&self) -> UnnamedAddr {
    self.unnamed_addr_val.clone()
  }

  pub fn set_unnamed_addr(&mut self, val: UnnamedAddr) {
    self.unnamed_addr_val = val;
  }

  pub fn get_min_unnamed_addr(a: UnnamedAddr, b: UnnamedAddr) -> UnnamedAddr {
    if a == UnnamedAddr::None || b == UnnamedAddr::None {
      return UnnamedAddr::None;
    }
    if a == UnnamedAddr::Local || b == UnnamedAddr::Local {
      return UnnamedAddr::Local;
    }
    UnnamedAddr::Global
  }

  pub fn has_comdat() {}
  pub fn get_comdat() {}

  pub fn get_visibility(&self) -> VisibilityTypes {
    self.visibility.clone()
  }

  pub fn has_default_visibility(&self) -> bool {
    self.visibility == VisibilityTypes::DefaultVisibility
  }

  pub fn has_hidden_visibility(&self) -> bool {
    self.visibility == VisibilityTypes::HiddenVisibility
  }

  pub fn has_protected_visibility(&self) -> bool {
    self.visibility == VisibilityTypes::ProtectedVisibility
  }

  pub fn set_visibility() {}

  // If this value is 'Thread Local', its value isn't shared by the threads.
  pub fn is_thread_local(&self) -> bool {
    self.thread_local != ThreadLocalMode::NotThreadLocal
  }

  pub fn set_thread_local() {}
  pub fn set_thread_local_mode() {}

  pub fn get_thread_local_mode(&self) -> ThreadLocalMode {
    self.thread_local.clone()
  }

  pub fn get_dll_storage_class(&self) -> DllStorageClassTypes {
    self.dll_storage_class.clone()
  }

  pub fn has_dll_import_storage_class(&self) -> bool {
    self.dll_storage_class == DllStorageClassTypes::DllImportStorageClass
  }

  pub fn has_dll_export_storage_class(&self) -> bool {
    self.dll_storage_class == DllStorageClassTypes::DllExportStorageClass
  }

  pub fn set_dll_storage_class(&mut self, c: DllStorageClassTypes) {
    debug_assert!(!self.has_local_linkage() ||
      c == DllStorageClassTypes::DefaultStorageClass,
      "Local linkage requires DefaultStorageClass.");
    self.dll_storage_class = c;
  }

  pub fn has_section() {}
  pub fn get_section() {}
  pub fn get_type() {}
  pub fn get_value_type() {}

  pub fn is_implicit_dso_local(&self) -> bool {
    self.has_local_linkage() || (!self.has_default_visibility() &&
    !self.has_external_weak_linkage())
  }

  pub fn set_dso_local(&mut self, local: bool) {
    self.is_dso_local = local
  }

  pub fn is_dso_local(&self) -> bool {
    self.is_dso_local
  }

  pub fn has_partition(&self) -> bool {
    self.has_partition
  }

  pub fn get_partition() {}
  pub fn set_partition() {}

  pub fn has_sanitizer_metadata(&self) -> bool {
    self.has_sanitizer_metadata
  }

  pub fn get_sanitizer_metadata() {}
  pub fn set_sanitizer_metadata() {}
  pub fn remove_sanitizer_metadata() {}

  pub fn is_tagged(&self) -> bool {
    false //self.has_sanitizer_metadata()
  }

  pub fn get_link_once_linkage(odr: bool) -> LinkageTypes {
    if odr {
      return LinkageTypes::LinkOnceOdrLinkage;
    } else {
      return LinkageTypes::LinkOnceAnyLinkage;
    }
  }

  pub fn get_weak_linkage(odr: bool) -> LinkageTypes {
    if odr {
      return LinkageTypes::WeakOdrLinkage;
    } else {
      return LinkageTypes::WeakAnyLinkage;
    }
  }

  pub fn is_external_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::ExternalLinkage
  }

  pub fn is_available_externally_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::AvailableExternallyLinkage
  }

  pub fn is_link_once_any_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::LinkOnceAnyLinkage
  }

  pub fn is_link_once_odr_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::LinkOnceOdrLinkage
  }

  pub fn is_link_once_linkage(linkage: &LinkageTypes) -> bool {
    GlobalValue::is_link_once_any_linkage(linkage) ||
    GlobalValue::is_link_once_odr_linkage(linkage)
  }

  pub fn is_weak_any_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::WeakAnyLinkage
  }

  pub fn is_weak_odr_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::WeakOdrLinkage
  }

  pub fn is_weak_linkage(linkage: &LinkageTypes) -> bool {
    GlobalValue::is_weak_any_linkage(linkage) ||
    GlobalValue::is_weak_odr_linkage(linkage)
  }

  pub fn is_appending_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::AppendingLinkage
  }

  pub fn is_internal_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::InternalLinkage
  }

  pub fn is_private_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::PrivateLinkage
  }

  pub fn is_local_linkage(linkage: &LinkageTypes) -> bool {
    GlobalValue::is_internal_linkage(linkage) ||
    GlobalValue::is_private_linkage(linkage)
  }

  pub fn is_external_weak_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::ExternalWeakLinkage
  }

  pub fn is_common_linkage(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::CommonLinkage
  }

  pub fn is_valid_declaration_linkage(linkage: &LinkageTypes) -> bool {
    GlobalValue::is_external_weak_linkage(linkage) ||
    GlobalValue::is_external_linkage(linkage)
  }

  pub fn is_interposable_linkage() {}

  // Whether the definition of this global may be discarded if it is not
  // used in its compilation unit.
  pub fn is_discardable_if_unused(linkage: &LinkageTypes) -> bool {
    GlobalValue::is_link_once_linkage(linkage) ||
    GlobalValue::is_local_linkage(linkage) ||
    GlobalValue::is_available_externally_linkage(linkage)
  }

  // Whether the definition of this global may be replaced at link time.
  pub fn is_weak_for_linker(linkage: &LinkageTypes) -> bool {
    *linkage == LinkageTypes::WeakAnyLinkage ||
    *linkage == LinkageTypes::WeakOdrLinkage ||
    *linkage == LinkageTypes::LinkOnceAnyLinkage ||
    *linkage == LinkageTypes::LinkOnceOdrLinkage ||
    *linkage == LinkageTypes::CommonLinkage ||
    *linkage == LinkageTypes::ExternalWeakLinkage
  }

  pub fn is_definition_exact() {}
  pub fn has_exact_definition() {}
  pub fn is_interposable(&self) -> bool { false }
  pub fn can_benefit_from_local_alias() {}

  pub fn has_external_linkage(&self) -> bool {
    GlobalValue::is_external_linkage(&self.linkage)
  }

  pub fn has_available_externally_linkage(&self) -> bool {
    GlobalValue::is_available_externally_linkage(&self.linkage)
  }

  pub fn has_link_once_linkage(&self) -> bool {
    GlobalValue::is_link_once_linkage(&self.linkage)
  }

  pub fn has_link_once_any_linkage(&self) -> bool {
    GlobalValue::is_link_once_any_linkage(&self.linkage)
  }

  pub fn has_link_once_odr_linkage(&self) -> bool {
    GlobalValue::is_link_once_odr_linkage(&self.linkage)
  }

  pub fn has_weak_linkage(&self) -> bool {
    GlobalValue::is_weak_linkage(&self.linkage)
  }

  pub fn has_weak_any_linkage(&self) -> bool {
    GlobalValue::is_weak_any_linkage(&self.linkage)
  }

  pub fn has_weak_odr_linkage(&self) -> bool {
    GlobalValue::is_weak_odr_linkage(&self.linkage)
  }

  pub fn has_appending_linkage(&self) -> bool {
    GlobalValue::is_appending_linkage(&self.linkage)
  }

  pub fn has_internal_linkage(&self) -> bool {
    GlobalValue::is_internal_linkage(&self.linkage)
  }

  pub fn has_private_linkage(&self) -> bool {
    GlobalValue::is_private_linkage(&self.linkage)
  }

  pub fn has_local_linkage(&self) -> bool {
    GlobalValue::is_local_linkage(&self.linkage)
  }

  pub fn has_external_weak_linkage(&self) -> bool {
    GlobalValue::is_external_weak_linkage(&self.linkage)
  }

  pub fn has_common_linkage(&self) -> bool {
    GlobalValue::is_common_linkage(&self.linkage)
  }

  pub fn has_valid_declaration_linkage(&self) -> bool {
    GlobalValue::is_valid_declaration_linkage(&self.linkage)
  }

  pub fn set_linkage() {}

  pub fn get_linkage(&self) -> LinkageTypes {
    self.linkage.clone()
  }

  pub fn drop_blitz_magling_escape() {}
  pub fn get_global_identifier() {}
  pub fn get_guid() {}
  pub fn is_materializable() {}
  pub fn materialize() {}
  pub fn is_declaration(&self) -> bool { false }
  pub fn is_declaration_for_linker() {}
  pub fn is_strong_definition_for_linker() {}
  pub fn get_aliasee_object() {}
  pub fn is_absolute_symbol_ref() {}
  pub fn get_absolute_symbol_range() {}
  pub fn remove_from_parent() {}
  pub fn erase_from_parent() {}
  pub fn get_parent() {}
  pub fn class_of() {}
  pub fn can_be_ommitted_from_symbol_table() {}
}