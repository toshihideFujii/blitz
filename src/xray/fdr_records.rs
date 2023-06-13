#![allow(dead_code)]

// Define types and operations on these types that represent the
// different kinds of records we encounter in XRay flight data
// recorder mode traces.

use crate::{adt::string_ref::StringRef, support::data_extractor::DataExtractor};
use super::xray_record::RecordTypes;

#[derive(Debug, PartialEq)]
pub enum RecordKind {
  Metadata,
  MetadataBufferExtents,
  MetadataWallClockTime,
  MetadataNewCpuId,
  MetadataTscWrap,
  MetadataCustomEvent,
  MetadataCustomEventV5,
  MetadataCallArg,
  MetadataPidEntry,
  MetadataNewBuffer,
  MetadataEndOfBuffer,
  MetadataTypedEvent,
  MetadataLastMetadata,
  Function
}

pub trait Record {
  fn get_record_type(&self) -> RecordKind;
  fn apply(&self, v: &mut RecordInitializer);
  fn kind_to_string(&self, k: RecordKind) -> StringRef {
    match k {
      RecordKind::Metadata =>
        return StringRef::new_from_string("Metadata"),
      RecordKind::MetadataBufferExtents =>
        return StringRef::new_from_string("Metadata:BufferExtents"),
      RecordKind::MetadataWallClockTime =>
        return StringRef::new_from_string("Metadata:WallClockTime"),
      RecordKind::MetadataNewCpuId =>
        return StringRef::new_from_string("Metadata:NewCpuId"),
      RecordKind::MetadataTscWrap =>
        return StringRef::new_from_string("Metadata:TscWrap"),
      RecordKind::MetadataCustomEvent =>
        return StringRef::new_from_string("Metadata:CustomEvent"),
      RecordKind::MetadataCustomEventV5 =>
        return StringRef::new_from_string("Metadata:CustomEventV5"),
      RecordKind::MetadataCallArg =>
        return StringRef::new_from_string("Metadata:CallArg"),
      RecordKind::MetadataPidEntry =>
        return StringRef::new_from_string("Metadata:PidEntry"),
      RecordKind::MetadataNewBuffer =>
        return StringRef::new_from_string("Metadata:NewBuffer"),
      RecordKind::MetadataEndOfBuffer =>
        return StringRef::new_from_string("Metadata:EndofBuffer"),
      RecordKind::MetadataTypedEvent =>
        return StringRef::new_from_string("Metadata:TypedEvent"),
      RecordKind::MetadataLastMetadata =>
        return StringRef::new_from_string("Metadata:LastMetadata"),
      RecordKind::Function =>
        return StringRef::new_from_string("Metadata:Function"),
    };
  }
}

pub enum MetadataType {
  Unknown,
  BufferExtents,
  WallClockTime,
  NewCpuId,
  TscWrap,
  CustomEvent,
  CallArg,
  PidEntry,
  NewBuffer,
  EndOfBuffer,
  TypedEvent
}

const METADATA_BODY_SIZE: u32 = 15;

pub struct MetadataRecord {
  t: RecordKind,
  mt: MetadataType
}

impl MetadataRecord {
  pub fn new(t: RecordKind, m: MetadataType) -> Self {
    MetadataRecord { t: t, mt: m }
  }

  pub fn metadata_type(&self) -> &MetadataType {
    &self.mt
  }

  pub fn class_of() {}
}

pub struct BufferExtents {
  t: RecordKind,
  mt: MetadataType,
  size: u64
}

impl BufferExtents {
  pub fn new(s: u64) -> BufferExtents {
    BufferExtents {
      t: RecordKind::MetadataBufferExtents,
      mt: MetadataType::BufferExtents,
      size: s
    }
  }

  pub fn size(&self) -> u64 {
    self.size
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataBufferExtents
  }
}

impl Record for BufferExtents {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataBufferExtents
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_buffer_extents(*self);
  }
}

pub struct WallClockRecord {
  t: RecordKind,
  mt: MetadataType,
  seconds: u64,
  nanos: u32
}

impl WallClockRecord {
  pub fn new(s: u64, n: u32) -> Self {
    WallClockRecord {
      t: RecordKind::MetadataWallClockTime,
      mt: MetadataType::WallClockTime,
      seconds: s,
      nanos: n
    }
  }

  pub fn seconds(&self) -> u64 {
    self.seconds
  }

  pub fn nanos(&self) -> u32 {
    self.nanos
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataWallClockTime
  }
}

impl Record for WallClockRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataWallClockTime
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_wallclock_record(*self);
  }
}

pub struct NewCpuIdRecord {
  t: RecordKind,
  mt: MetadataType,
  cpu_id: u16,
  tsc: u64
}

impl NewCpuIdRecord {
  pub fn new(c: u16, t: u64) -> Self {
    NewCpuIdRecord {
      t: RecordKind::MetadataNewCpuId,
      mt: MetadataType::NewCpuId,
      cpu_id: c,
      tsc: t
    }
  }

  pub fn cpu_id(&self) -> u16 {
    self.cpu_id
  }

  pub fn tsc(&self) -> u64 {
    self.tsc
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataNewCpuId
  }
}

impl Record for NewCpuIdRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataNewCpuId
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_new_cpu_id_record(*self);
  }
}

pub struct TscWrapRecord {
  t: RecordKind,
  mt: MetadataType,
  base_tsc: u64
}

impl TscWrapRecord {
  pub fn new(b: u64) -> Self {
    TscWrapRecord {
      t: RecordKind::MetadataTscWrap,
      mt: MetadataType::TscWrap,
      base_tsc: b
    }
  }

  pub fn tsc(&self) -> u64 {
    self.base_tsc
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataTscWrap
  }
}

impl Record for TscWrapRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataTscWrap
  }
  
  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_tsc_wrap_record(*self);
  }
}

pub struct CustomEventRecord {
  t: RecordKind,
  mt: MetadataType,
  size: i32,
  tsc: u64,
  cpu: u16,
  data: String
}

impl CustomEventRecord {
  pub fn new(s: i32, t: u64, c: u16, d: String) -> Self {
    CustomEventRecord {
      t: RecordKind::MetadataCustomEvent,
      mt: MetadataType::CustomEvent,
      size: s,
      tsc: t,
      cpu: c,
      data: d
    }
  }

  pub fn size(&self) -> i32 {
    self.size
  }

  pub fn tsc(&self) -> u64 {
    self.tsc
  }

  pub fn cpu(&self) -> u16 {
    self.cpu
  }

  pub fn data(&self) -> String {
    self.data.clone()
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataCustomEvent
  }
}

impl Record for CustomEventRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataCustomEvent
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_custom_event_record(*self);
  }
}

pub struct CustomEventRecordV5 {
  t: RecordKind,
  mt: MetadataType,
  size: i32,
  delta: i32,
  data: String
}

impl CustomEventRecordV5 {
  pub fn new(s: i32, d: i32, p: String) -> Self {
    CustomEventRecordV5 {
      t: RecordKind::MetadataCustomEventV5,
      mt: MetadataType::CustomEvent,
      size: s,
      delta: d,
      data: p
    }
  }

  pub fn size(&self) -> i32 {
    self.size
  }

  pub fn delta(&self) -> i32 {
    self.delta
  }

  pub fn data(&self) -> String {
    self.data.clone()
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataCustomEventV5
  }
}

impl Record for CustomEventRecordV5 {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataCustomEventV5
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_custom_event_record_v5(*self);
  }
}

pub struct TypedEventRecord {
  t: RecordKind,
  mt: MetadataType,
  size: i32,
  delta: i32,
  event_type: u16,
  data: String
}

impl TypedEventRecord {
  pub fn new(s: i32, d: i32, e: u16, p: String) -> Self {
    TypedEventRecord {
      t: RecordKind::MetadataTypedEvent,
      mt: MetadataType::TypedEvent,
      size: s,
      delta: d,
      event_type: e,
      data: p
    }
  }

  pub fn size(&self) -> i32 {
    self.size
  }

  pub fn delta(&self) -> i32 {
    self.delta
  }

  pub fn event_type(&self) -> u16 {
    self.event_type
  }

  pub fn data(&self) -> String {
    self.data.clone()
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataTypedEvent
  }
}

impl Record for TypedEventRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataTypedEvent
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_typed_event_record(*self);
  }
}

pub struct CallArgRecord {
  t: RecordKind,
  mt: MetadataType,
  arg: u64
}

impl CallArgRecord {
  pub fn new(a: u64) -> Self {
    CallArgRecord {
      t: RecordKind::MetadataCallArg,
      mt: MetadataType::CallArg,
      arg: a
    }
  }

  pub fn arg(&self) -> u64 {
    self.arg
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataCallArg
  }
}

impl Record for CallArgRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataCallArg
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_call_arg_record(*self);
  }
}

pub struct PidRecord {
  t: RecordKind,
  mt: MetadataType,
  pid: i32
}

impl PidRecord {
  pub fn new(p: i32) -> Self {
    PidRecord {
      t: RecordKind::MetadataPidEntry,
      mt: MetadataType::PidEntry,
      pid: p
    }
  }

  pub fn pid(&self) -> i32 {
    self.pid
  }

  pub fn apply() {}
  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataPidEntry
  }
}

impl Record for PidRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataPidEntry
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_pid_record(*self);
  }
}

pub struct NewBufferRecord {
  t: RecordKind,
  mt: MetadataType,
  tid: i32
}

impl NewBufferRecord {
  pub fn new(t: i32) -> Self {
    NewBufferRecord {
      t: RecordKind::MetadataNewBuffer,
      mt: MetadataType::NewBuffer,
      tid: t
    }
  }

  pub fn tid(&self) -> i32 {
    self.tid
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataNewBuffer
  }
}

impl Record for NewBufferRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataNewBuffer
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_new_buffer_record(*self);
  }
}

pub struct EndBufferRecord {
  t: RecordKind,
  mt: MetadataType,
}

impl EndBufferRecord {
  pub fn new() -> Self {
    EndBufferRecord {
      t: RecordKind::MetadataEndOfBuffer,
      mt: MetadataType::EndOfBuffer
    }
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::MetadataEndOfBuffer
  }
}

impl Record for EndBufferRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::MetadataEndOfBuffer
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_end_buffer_record(*self);
  }
}

pub struct FunctionRecord {
  t: RecordKind,
  kind: RecordTypes,
  func_id: i32,
  delta: u32
}

impl FunctionRecord {
  pub fn new(k: RecordTypes, f: i32, d: u32) -> Self {
    FunctionRecord { t: RecordKind::Function, kind: k, func_id: f, delta: d }
  }

  // A function record is a concrete record type which has a number of
  // common properties.
  pub fn record_type(&self) -> &RecordTypes {
    &self.kind
  }

  pub fn function_id(&self) -> i32 {
    self.func_id
  }

  pub fn delta(&self) -> u32 {
    self.delta
  }

  pub fn class_of(r: &dyn Record) -> bool {
    r.get_record_type() == RecordKind::Function
  }
}

impl Record for FunctionRecord {
  fn get_record_type(&self) -> RecordKind {
    RecordKind::Function
  }

  fn apply(&self, _v: &mut RecordInitializer) {
    //v.visit_function_record(*self);
  }
}

pub trait RecordVisitor {
  fn visit_buffer_extents(&mut self, r: BufferExtents) -> Result<(), String>;
  fn visit_wallclock_record(&mut self, r: WallClockRecord) -> Result<(), String>;
  fn visit_new_cpu_id_record(&mut self, r: NewCpuIdRecord) -> Result<(), String>;
  fn visit_tsc_wrap_record(&mut self, r: TscWrapRecord) -> Result<(), String>;
  fn visit_custom_event_record(&mut self, r: CustomEventRecord) -> Result<(), String>;
  fn visit_call_arg_record(&mut self, r: CallArgRecord) -> Result<(), String>;
  fn visit_pid_record(&mut self, r: PidRecord) -> Result<(), String>;
  fn visit_new_buffer_record(&mut self, r: NewBufferRecord) -> Result<(), String>;
  fn visit_end_buffer_record(&mut self, r: EndBufferRecord) -> Result<(), String>;
  fn visit_function_record(&mut self, r: FunctionRecord) -> Result<(), String>;
  fn visit_custom_event_record_v5(&mut self, r: CustomEventRecordV5) -> Result<(), String>;
  fn visit_typed_event_record(&mut self, r: TypedEventRecord) -> Result<(), String>;
}

pub struct RecordInitializer {
  de: DataExtractor,
  offset_ptr: u64,
  version: u16
}

impl RecordInitializer {
  pub fn new(de: DataExtractor, op: u64, v: u16) -> Self {
    RecordInitializer { de: de, offset_ptr: op, version: v }
  }
}

impl RecordVisitor for RecordInitializer {
  fn visit_buffer_extents(&mut self, _r: BufferExtents) -> Result<(), String> { Ok(()) }
  fn visit_wallclock_record(&mut self, _r: WallClockRecord) -> Result<(), String> { Ok(()) }
  fn visit_new_cpu_id_record(&mut self, _r: NewCpuIdRecord) -> Result<(), String> { Ok(()) }
  fn visit_tsc_wrap_record(&mut self, _r: TscWrapRecord) -> Result<(), String> { Ok(()) }
  fn visit_custom_event_record(&mut self, _r: CustomEventRecord) -> Result<(), String> { Ok(()) }
  fn visit_call_arg_record(&mut self, _r: CallArgRecord) -> Result<(), String> { Ok(()) }
  fn visit_pid_record(&mut self, _r: PidRecord) -> Result<(), String> { Ok(()) }
  fn visit_new_buffer_record(&mut self, _r: NewBufferRecord) -> Result<(), String> { Ok(()) }
  fn visit_end_buffer_record(&mut self, _r: EndBufferRecord) -> Result<(), String> { Ok(()) }
  fn visit_function_record(&mut self, _r: FunctionRecord) -> Result<(), String> { Ok(()) }
  fn visit_custom_event_record_v5(&mut self, _r: CustomEventRecordV5) -> Result<(), String> { Ok(()) }
  fn visit_typed_event_record(&mut self, _r: TypedEventRecord) -> Result<(), String> { Ok(()) }
}