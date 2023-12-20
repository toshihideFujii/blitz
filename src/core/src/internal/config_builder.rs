#![allow(dead_code)]

use std::{str::FromStr, fmt::Debug, time::Duration};
use super::config_entry::ConfigEntry;

fn to_number<T>(s: String) -> T
  where T: FromStr, <T as FromStr>::Err: Debug
{
  s.parse::<T>().unwrap()
}

fn to_boolean(s: String) -> bool {
  let val = s.trim().to_ascii_lowercase();
  if val.as_str() == "true" {
    true
  } else {
    false
  }
}

fn to_string(s: String) -> String {
  s
}

fn string_to_seq() {}
fn seq_to_string() {}

fn time_from_string(s: String) -> Duration {
  let millis = s.parse::<u64>().unwrap();
  Duration::from_nanos(millis)
}

fn time_to_string(d: &Duration) -> String {
  d.as_millis().to_string() + "ms"
}

fn byte_from_string(s: String) -> Vec<u8> {
  s.as_bytes().to_vec()
}

fn byte_to_string(b: Vec<u8>) -> String {
  String::from_utf8(b).unwrap()
}

fn regex_from_string() {}

pub struct TypedConfigBuilder<T> {
  parent: ConfigBuilder,
  converter: fn(String) -> T,
  string_converter: Option<fn(&T) -> String>,
}

impl<T> TypedConfigBuilder<T> {
  pub fn new(
    parent: ConfigBuilder,
    converter: fn(String) -> T,
    string_converter: fn(&T) -> String) -> Self
  {
    TypedConfigBuilder {
      parent: parent,
      converter: converter,
      string_converter: Some(string_converter)
    }
  }

  pub fn from_converter(
    parent: ConfigBuilder,
    converter: fn(String) -> T) -> Self
  {
    TypedConfigBuilder {
      parent: parent,
      converter: converter,
      string_converter: None,
    }
  }

  pub fn this() {}

  pub fn transform(&self, _transformer: fn(T) -> T) /*-> Self*/ {
    //et converter: fn(String) -> T = (transformer)((self.converter)(String));
    //TypedConfigBuilder::new(
      //self.parent,
      //(transformer)(self.converter),
      //self.string_converter)
  }

  pub fn check_value() {}
  pub fn check_values() {}
  pub fn to_sequence() {}
  pub fn create_optional() {}

  pub fn create_with_default(&self, default: T) -> ConfigEntry<T> {
    ConfigEntry::new(
      self.parent.key.clone(),
      self.parent.prepended_key.clone(),
      self.parent.prepend_separator.clone(),
      self.parent.alternatives.clone(),
      default,
      self.converter,
      self.string_converter.unwrap(),
      self.parent.doc.clone(),
      self.parent.public,
      self.parent.version.clone()
    )
  }

  pub fn create_with_default_function() {}
  pub fn create_with_default_string() {}
}

#[derive(Debug, Clone)]
pub struct ConfigBuilder {
  key: String,
  prepended_key: Option<String>,
  prepend_separator: String,
  public: bool,
  doc: String,
  version: String,
  // on_create
  alternatives: Vec<String>,
}

impl ConfigBuilder {
  pub fn new(key: &str) -> Self {
    ConfigBuilder {
      key: key.to_string(),
      prepended_key: None,
      prepend_separator: "".to_string(),
      public: true,
      doc: "".to_string(),
      version: "".to_string(),
      alternatives: Vec::new(),
    }
  }
  pub fn internal(&mut self) -> &mut ConfigBuilder {
    self.public = false;
    self
  }

  pub fn doc(&mut self, s: &str) -> &mut ConfigBuilder {
    self.doc = s.to_string();
    self
  }

  pub fn version(&mut self, v: &str) -> &mut ConfigBuilder {
    self.version = v.to_string();
    self
  }

  pub fn on_create() {}

  pub fn with_prepended(&mut self, key: String, separator: String) -> &mut ConfigBuilder {
    self.prepended_key = Some(key);
    self.prepend_separator = separator;
    self
  }

  pub fn with_alternative(&mut self, key: String) -> &mut ConfigBuilder {
    self.alternatives.push(key);
    self
  }

  pub fn int_conf(&self) -> TypedConfigBuilder<i64> {
    assert!(self.prepended_key.is_some());
    TypedConfigBuilder::from_converter(self.clone(), to_number)
  }

  pub fn long_conf(&self) -> TypedConfigBuilder<u64> {
    assert!(self.prepended_key.is_some());
    TypedConfigBuilder::from_converter(self.clone(), to_number)
  }

  pub fn double_conf(&self) -> TypedConfigBuilder<f64> {
    assert!(self.prepended_key.is_some());
    TypedConfigBuilder::from_converter(self.clone(), to_number)
  }

  pub fn boolean_conf(&self) -> TypedConfigBuilder<bool> {
    assert!(self.prepended_key.is_some());
    TypedConfigBuilder::from_converter(self.clone(), to_boolean)
  }

  pub fn string_conf(&self) -> TypedConfigBuilder<String> {
    TypedConfigBuilder::from_converter(self.clone(), to_string)
  }

  pub fn time_conf(&self) -> TypedConfigBuilder<Duration> {
    TypedConfigBuilder::new(
      self.clone(),
      time_from_string,
      time_to_string
    )
  }
}