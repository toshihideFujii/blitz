#![allow(dead_code)]

use crate::{
  adt::{small_ptr_set::SmallPtrSet,
    dense_map::DenseMap,
    ap_int::APInt,
    ap_float::APFloat,
    folding_set::FoldingSet,
    string_map::StringMap, string_ref::StringRef,
  },
  remarks::remark_streamer::RemarkStreamer
};
use super::{
  module::Module,
  diagnostic_handler::DiagnositicHandler,
  blitz_remark_streamer::BlitzRemarkStreamer,
  constants::{ConstantInt, ConstantFP, ConstantTokenNone},
  attribute_impl::{AttributeImpl, AttributeListImpl, AttributeSetNode},
  metadata::{MDString, /*ValueAsMetadata,*/ Metadata, MetadataAsValue, MDNode},
  /*value::Value,*/
  debug_info_metadata::DICompositeType,
  type_::{IntegerType}, global_object::GlobalObject, global_value::GlobalValue
};

#[derive(Debug, Clone, PartialEq)]
pub struct BlitzContextImpl {
  // The set of modules instantiated in this context, and which
  // will be automatically deleted if this context is deleted.
  owned_modules: SmallPtrSet<Module>,
  // The main remark streamer used by all the other streamers (e.g.
  // Ir, MIR, frontends, etc.).
  // This should only be used by the specific srreamers, and never
  // directly.
  main_remark_streamer: Option<RemarkStreamer>,
  diag_handler: Option<DiagnositicHandler>,
  respect_diagnostic_filters: bool,
  diagnostics_hotness_requested: bool,
  // The minimum hotness value a diagnostic needs in order to be
  // included in optimization diagnostics.
  diagnostics_hotness_threshold: Option<u64>,
  // The percentage of difference between profiling branch weights
  // and expect branch weights to tolerate when emiting MisExpect diagnostics.
  diagnostics_mis_expect_tolerance: Option<u32>,
  mis_expect_warning_requested: bool,
  // The specialized remark streamer used by Blitz's OptimizationRemarkEmitter.
  blitz_rs: Option<BlitzRemarkStreamer>,

  pub int_constants: DenseMap<APInt, ConstantInt>,
  fp_constants: DenseMap<APFloat, ConstantFP>,

  attrs_set: FoldingSet<AttributeImpl>,
  attrs_lists: FoldingSet<AttributeListImpl>,
  attrs_set_nodes: FoldingSet<AttributeSetNode>,

  md_string_cache: StringMap<MDString>,
  //values_as_metadata: DenseMap<Value, ValueAsMetadata>,
  metadata_as_values: DenseMap<Metadata, MetadataAsValue>,

  di_type_map: DenseMap<MDString, DICompositeType>,
  distinct_md_nodes: Vec<MDNode>,

  //caz_constants: DenseMap<&dyn Type, ConstantAggregateZero>,

  the_true_val: Option<ConstantInt>,
  the_false_val: Option<ConstantInt>,

  // Basic type instances.
  /*
  void_type: Box<dyn Type>,
  label_type: Box<dyn Type>,
  half_type: Box<dyn Type>,
  b_float_type: Box<dyn Type>,
  float_type: Box<dyn Type>,
  double_type: Box<dyn Type>,
  metadata_type: Box<dyn Type>,
  token_type: Box<dyn Type>,

  x86_fp80_type: Box<dyn Type>,
  fp128_type: Box<dyn Type>,
  ppc_fp128_type: Box<dyn Type>,
  x86_mmx_type: Box<dyn Type>,
  x86_amx_type: Box<dyn Type>,
  */

  int_1_type: IntegerType,
  int_8_type: IntegerType,
  int_16_type: IntegerType,
  int_32_type: IntegerType,
  int_64_type: IntegerType,
  int_128_type: IntegerType,

  the_none_token: ConstantTokenNone,

  integer_types: DenseMap<u32, IntegerType>,
  //function_types: DenseSet<FunctionType>,
  //anon_struct_types: DenseSet<>


  custom_md_kind_names: StringMap<u32>,
  //value_metadata: DenseMap<Value, >

  global_object_sections: DenseMap<GlobalObject, StringRef>,
  global_value_partitions: DenseMap<GlobalValue, StringRef>,


  bundle_tag_cache: StringMap<u32>
}

impl BlitzContextImpl {
  pub fn new() {}
  pub fn get_or_insert_bundle_tag() {}
  pub fn get_operand_bundle_tags() {}
  pub fn get_operand_bundle_tag_id() {}
  pub fn get_or_insert_sync_scope_id() {}
  pub fn get_sync_scope_names() {}
  pub fn drop_trivially_dead_constant_arrays() {}
  pub fn get_opt_pass_gate() {}
  pub fn set_opt_pass_gate() {}
  pub fn get_opaque_pointers() {}
  pub fn set_opaque_pointers() {}

  pub fn get_true_value(&self) -> Option<ConstantInt> {
    self.the_true_val.clone()
  }

  pub fn set_true_value(&mut self, value: Option<ConstantInt>) {
    self.the_true_val = value;
  }

  pub fn get_false_value(&self) -> Option<ConstantInt> {
    self.the_false_val.clone()
  }

  pub fn set_false_value(&mut self, value: Option<ConstantInt>) {
    self.the_false_val = value;
  }

  pub fn get_int_1_type(&self) -> IntegerType {
    self.int_1_type.clone()
  }
}