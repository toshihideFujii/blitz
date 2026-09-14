#![allow(dead_code)]

pub enum FailureKind {
  Unknown,
  RedzoneModified,
  WrongResult,
  Disqualified
}

pub struct FailureResult {
  kind: FailureKind,
  msg: String,
  buffer_address: i64
}

pub struct AutotuneResults {
  version: i64
}

impl AutotuneResults {
  pub fn default() -> Self {
    AutotuneResults { version: 1 }
  }

  pub fn add_results(&mut self) -> &mut AutotuneResultsEntry {
    unimplemented!()
  }

  pub fn set_version(&mut self, version: i64) {
    self.version = version;
  }
}

pub struct AutotuneResultsEntry {
  device: String,
  hlo: String,
  version: i64
}

impl AutotuneResultsEntry {
  pub fn default() -> Self {
    AutotuneResultsEntry { device: String::new(), hlo: String::new(), version: 0 }
  }

  pub fn device(&self) -> String {
    self.device.clone()
  }

  pub fn set_device(&mut self, device: String) {
    self.device = device;
  }

  pub fn hlo(&self) -> String {
    self.hlo.clone()
  }

  pub fn set_hlo(&mut self, hlo: String) {
    self.hlo = hlo;
  }

  pub fn version(&self) -> i64 {
    self.version
  }

  pub fn set_version(&mut self, version: i64) {
    self.version = version;
  }

  pub fn ressult(&self) -> &String {
    unimplemented!()
  }

  pub fn mutable_result(&mut self) -> &mut String {
    unimplemented!()
  }
}

// This class is a thin wrapper around AutotuneResults::Entry. It is used to
// provide opaque accessors to an entry's key and value without exposing the
// internal structure of the entry.
pub struct AutotuneResultWrapper {
  autotune_result: AutotuneResultsEntry,
  version: i64
}

impl AutotuneResultWrapper {
  pub fn new() -> Self {
    unimplemented!()
  }

  // Creates an AutotuneResultWrapper from a key and value. The provided key and
  // value must be ones that were previously returned by calls to Key() and
  // Value().
  pub fn from_key_and_value(
    _key: String,
    _value: String) -> Result<AutotuneResultWrapper, String>
  {
    unimplemented!()
  }

  // An opaque string that can be used as a key for this Autotuning result.
  // Do not rely on the format of this string.
  pub fn key(&self) -> String {
    let mut key_proto = AutotuneResults::default();
    key_proto.set_version(self.version);
    let entry = key_proto.add_results();
    entry.set_device(self.autotune_result.device());
    entry.set_hlo(self.autotune_result.hlo());
    entry.set_version(self.autotune_result.version());

    let serialized = String::new();
    // serialize_to_string_deterministic() // TODO
    serialized
  }

  // An opaque string that encodes the autotuning result.
  // Do not rely on the format of this string.
  pub fn value(&self) -> String {
    let mut entry = AutotuneResultsEntry::default();
    *entry.mutable_result() = self.autotune_result.ressult().clone();
    let serialized = String::new();
    // serialize_to_string_deterministic() // TODO
    serialized
  }

  fn serialize_to_string_deterministic(_autotune: &AutotuneResults, _result: &mut String) {
    unimplemented!()
  }
}