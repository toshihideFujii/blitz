#![allow(dead_code)]

// Declarations for metadata specific to debug info.

struct DITypeRefArray {}

// Target DWARF-like metadata node.
// A metadata node with a DWARF tag (i.e. a constant named DW_TAG_*,
// defined in binary_format_dwarf.rs).
// Called DINode because it's potentially used for non-DWARF output.
struct DINode {}
impl DINode {
  pub fn mew() {}
  pub fn get_operand_as() {}
  pub fn get_string_operand() {}
  pub fn get_canonical_md_string() {}
  pub fn set_tag() {}
  pub fn get_tag() {}
  pub fn get_flag() {}
  pub fn get_flag_string() {}
  pub fn split_flags() {}
  pub fn class_of() {}
}

struct GenericDINode {}

// Assignment ID.
// Used to link stores (as an attachment) and dbg.assigns (as an operand).
// DIAssignID metadata is never uniqued as we compare instances using
// referential equality (the instance/address is the ID).
struct DIAssignID {}
impl DIAssignID {
  pub fn new() {}
  pub fn replace_operand_with() {}
  pub fn get_distinct() {}
  pub fn get_temporary() {}
  pub fn class_of() {}
}

// Array subrange.
struct DISubrange {}
impl DISubrange {
  pub fn new() {}
  pub fn get_raw_count_node() {}
  pub fn get_raw_lower_bound() {}
  pub fn get_raw_upper_bound() {}
  pub fn get_raw_stride() {}
  pub fn get_count() {}
  pub fn get_lower_bound() {}
  pub fn get_upper_bound() {}
  pub fn get_stride() {}
  pub fn class_of() {}
}

struct DIGenericSubrange {}
impl DIGenericSubrange {
  pub fn new() {}
  pub fn get_raw_count_node() {}
  pub fn get_raw_lower_bound() {}
  pub fn get_raw_upper_bound() {}
  pub fn get_raw_stride() {}
  pub fn get_count() {}
  pub fn get_lower_bound() {}
  pub fn get_upper_bound() {}
  pub fn get_stride() {}
  pub fn class_of() {}
}

// Enumration value.
struct DIEnumerator {}
impl DIEnumerator {
  pub fn new() {}
  pub fn get_value() {}
  pub fn is_unsigned() {}
  pub fn get_name() {}
  pub fn get_raw_name() {}
  pub fn class_of() {}
}

// Base class for scope-like contexts.
// Base class for lexical scopes and types (which are also declaration contexts).
pub trait DIScope {
  fn get_file(&self) {}
  fn get_file_name(&self) {}
  fn get_directory(&self) {}
  fn get_source(&self) {}
  fn get_name(&self) {}
  fn get_scope(&self) {}
  fn get_raw_file(&self) {}
  fn class_of(&self) {}
}

// Which algorithm (e.g. MD5) a checksum was generated with.
enum ChecksumKind {
  MD5 = 1,
  SHA1 = 2,
  SHA256 = 3
}

// File.
struct DIFile {}
impl DIFile {
  pub fn new() {}
  pub fn get_filename() {}
  pub fn get_directory() {}
  pub fn get_checksum() {}
  pub fn get_source() {}
  pub fn get_raw_filename() {}
  pub fn get_raw_directory() {}
  pub fn get_raw_source() {}
  pub fn get_checksum_kind_as_string() {}
  pub fn get_checksum_kind() {}
  pub fn class_of() {}
}

// Base class for types.
pub trait DIType {
  fn get_line(&self) {}
  fn get_size_in_bits(&self) {}
  fn get_align_in_bits(&self) {}
  fn get_offset_in_bits(&self) {}
  fn get_flags(&self) {}
  fn get_scope(&self) {}
  fn get_name(&self) {}
  fn get_raw_scope(&self) {}
  fn get_raw_name(&self) {}
  fn is_private(&self) -> bool { false }
  fn is_protected(&self) -> bool { false }
  fn is_public(&self) -> bool { false }
  fn is_forward_decl(&self) -> bool { false }
  fn is_apple_block_extension(&self) -> bool { false }
  fn is_virtual(&self) -> bool { false }
  fn is_artificial(&self) -> bool { false }
  fn is_object_pointer(&self) -> bool { false }
  fn is_objc_class_complete(&self) -> bool { false }
  fn is_vector(&self) -> bool { false }
  fn is_bit_field(&self) -> bool { false }
  fn is_static_member(&self) -> bool { false }
  fn is_l_value_reference(&self) -> bool { false }
  fn is_r_value_reference(&self) -> bool { false }
  fn is_type_pass_by_value(&self) -> bool { false }
  fn is_type_pass_by_reference(&self) -> bool { false }
  fn is_big_endian(&self) -> bool { false }
  fn is_little_endian(&self) -> bool { false }
  fn get_export_symbols(&self) {}
  fn class_of(&self) {}
}

// Basic type, like 'int' or 'float'.
struct DIBasicType {}
impl DIBasicType {
  pub fn new() {}
  pub fn get_encoding() {}
  pub fn get_signedness() {}
}

// String type, Fortran CJARACTER(n).
struct DIStringType {}
impl DIStringType {
  pub fn new() {}
  pub fn get_string_length() {}
  pub fn get_string_length_exp() {}
  pub fn get_string_location_exp() {}
  pub fn get_encoding() {}
  pub fn get_raw_string_length() {}
  pub fn get_raw_string_length_exp() {}
  pub fn get_raw_string_location_exp() {}
}

// Derived types.
// This includes qualified types, pointers, references, friends, typedefs,
// and class members.
struct DIDerivedType {}
impl DIDerivedType {
  pub fn new() {}
  pub fn get_base_type() {}
  pub fn get_raw_base_type() {}
  pub fn get_dwarf_address_space() {}
  pub fn get_extra_data() {}
  pub fn get_raw_extra_data() {}
  pub fn get_annotations() {}
  pub fn get_raw_annotations() {}
  pub fn get_class_type() {}
  pub fn get_storage_offset_in_bits() {}
  pub fn get_constant() {}
  pub fn get_discriminat_value() {}
  pub fn class_of() {}
}

// Composite types.
#[derive(Debug, Clone, PartialEq)]
pub struct DICompositeType {}
impl DICompositeType {
  pub fn new() {}
  pub fn get_odr_type() {}
  pub fn build_odr_type() {}
  pub fn get_base_type() {}
  pub fn get_elements() {}
  pub fn get_vtable_holder() {}
  pub fn get_template_params() {}
  pub fn get_identifier() {}
  pub fn get_runtime_lang() {}
  pub fn get_raw_base_type() {}
  pub fn get_raw_elements() {}
  pub fn get_raw_vtable_holder() {}
  pub fn get_raw_template_params() {}
  pub fn get_raw_identifier() {}
  pub fn get_raw_discriminator() {}
  pub fn get_discriminator() {}
  pub fn get_raw_data_location() {}
  pub fn get_data_location() {}
  pub fn get_data_location_exp() {}
  pub fn get_raw_associated() {}
  pub fn get_associated_exp() {}
  pub fn get_raw_allocated() {}
  pub fn get_allocated_exp() {}
  pub fn get_raw_rank() {}
  pub fn get_rank_exp() {}
  pub fn get_raw_annotations() {}
  pub fn get_annotations() {}
  pub fn replace_elements() {}
  pub fn replace_vtable_holder() {}
  pub fn replace_template_params() {}
  pub fn class_of() {}
}

struct DISubroutineType {}

struct DICompileUnit {}

struct DILocalScope {}

// Subprogram description.
struct DISubprogram {}
impl DISubprogram {
  pub fn new() {}
  pub fn get_flag() {}
  pub fn get_flag_string() {}
  pub fn split_flags() {}
  pub fn to_sp_flags() {}
  pub fn clone_with_flags() {}
  pub fn get_line() {}
  pub fn get_virtuality() {}
  pub fn get_virtual_index() {}
  pub fn get_this_adjustment() {}
  pub fn get_scope_line() {}
  pub fn get_flags() {}
  pub fn get_sp_flags() {}
  pub fn is_local_to_unit() {}
  pub fn is_definition() {}
  pub fn is_optimized() {}
  pub fn is_main_subprogram() {}
  pub fn is_artificial() {}
  pub fn is_private() {}
  pub fn is_protected() {}
  pub fn is_public() {}
  pub fn is_explicit() {}
  pub fn is_prototyped() {}
  pub fn are_all_calls_described() {}
  pub fn is_pure() {}
  pub fn is_elemental() {}
  pub fn is_recursive() {}
  pub fn is_objc_direct() {}
  pub fn is_deleted() {}
  pub fn is_l_value_reference() {}
  pub fn is_r_value_reference() {}
  pub fn is_no_return() {}
  pub fn is_thunk() {}
  pub fn get_scope() {}
  pub fn get_name() {}
  pub fn get_linkage_name() {}
  pub fn replace_linkage_name() {}
  pub fn get_type() {}
  pub fn get_containing_type() {}
  pub fn replace_type() {}
  pub fn get_unit() {}
  pub fn replace_unit() {}
  pub fn get_declaration() {}
  pub fn get_retained_nodes() {}
  pub fn get_thrown_types() {}
  pub fn get_annotations() {}
  pub fn get_target_func_name() {}
  pub fn get_raw_scope() {}
  pub fn get_raw_name() {}
  pub fn get_raw_linkage_name() {}
  pub fn get_raw_type() {}
  pub fn get_raw_unit() {}
  pub fn get_raw_declaration() {}
  pub fn get_raw_retained_nodes() {}
  pub fn get_raw_containing_type() {}
  pub fn get_raw_template_params() {}
  pub fn get_raw_thrown_types() {}
  pub fn get_raw_annotations() {}
  pub fn get_raw_target_func_name() {}
  pub fn replace_raw_linkage_name() {}
  pub fn describes() {}
  pub fn class_of() {}
}

// A debug location in source code, used for debug info and otherwise.
pub struct DILocation {}
impl DILocation {
  pub fn new() {}
  pub fn replace_operand_with() {}
  pub fn get_line() {}
  pub fn get_column() {}
  pub fn get_scope() {}
  pub fn get_subprogram_linkage_name() {}
  pub fn get_inlined_at() {}
  pub fn is_implicit_code() {}
  pub fn set_implicit_code() {}
  pub fn get_file() {}
  pub fn get_filename() {}
  pub fn get_directory() {}
  pub fn get_source() {}
  pub fn get_inlined_at_scope() {}
  pub fn get_discriminator() {}
  pub fn is_pseudo_probe_discriminator() {}
  pub fn clone_with_discriminator() {}
  pub fn clone_with_base_discriminator() {}
  pub fn get_duplication_factor() {}
  pub fn get_copy_identifier() {}
  pub fn get_base_discriminator() {}
  pub fn clone_by_multiplying_duplication_factor() {}
  pub fn get_merged_location() {}
  pub fn get_merged_locations() {}
  pub fn get_masked_discriminator() {}
  pub fn get_base_discriminator_from_discriminator() {}
  pub fn encode_discriminator() {}
  pub fn decode_discriminator() {}
  pub fn get_duplication_factor_from_discriminator() {}
  pub fn get_copy_identifier_from_discriminator() {}
  pub fn get_raw_scope() {}
  pub fn get_raw_inlined_at() {}
  pub fn class_of() {}
}

struct DILexicalBlock {}

struct DILexicalBlockFile {}

struct DINamespace {}

struct DIModule {}

struct DITemplateParameter {}

struct DITemplateTypeParameter {}

struct DITemplateValueParameter {}

struct DIVariable {}

struct DIExpression {}

struct ExprOperand {}

struct DIGlobalVariable {}

struct DICommonBlock {}

struct DILocalVariable {}

struct DILabel {}