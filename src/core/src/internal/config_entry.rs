#![allow(dead_code)]

pub struct ConfigEntry<T> {
  key: String,
  prepended_key: Option<String>,
  prepend_separator: String,
  alternatives: Vec<String>,
  default_value: T,
  value_converter: fn(String) -> T,
  string_converter: fn(&T) -> String,
  doc: String,
  is_public: bool,
  version: String,
}

impl<T> ConfigEntry<T> {
  pub fn new(
    key: String,
    prepended_key: Option<String>,
    prepend_separator: String,
    alternatives: Vec<String>,
    default_value: T,
    value_converter: fn(String) -> T,
    string_converter: fn(&T) -> String,
    doc: String,
    is_public: bool,
    version: String,
  ) -> Self {
    ConfigEntry {
      key: key,
      prepended_key: prepended_key,
      prepend_separator: prepend_separator,
      alternatives: alternatives,
      default_value: default_value,
      value_converter: value_converter,
      string_converter: string_converter,
      doc: doc,
      is_public: is_public,
      version: version
    }
  }

  pub fn default_value(&self) -> &T {
    &self.default_value
  }

  pub fn default_value_string(&self) -> String {
    (self.string_converter)(&self.default_value)
  }

  pub fn read_string() {}
  pub fn read_from() {
      
  }
}

pub struct ConfigEntryWithDefaultFunction<T> {
  key: String,
  prepended_key: Option<String>,
  prepend_separator: String,
  alternatives: Vec<String>,
  default_function: fn() -> T,
  value_converter: fn(String) -> T,
  string_converter: fn(&T) -> String,
  doc: String,
  is_public: bool,
  version: String,
}

impl<T> ConfigEntryWithDefaultFunction<T> {
  pub fn new(
    key: String,
    prepended_key: Option<String>,
    prepend_separator: String,
    alternaatives: Vec<String>,
    default_function: fn() -> T,
    value_converter: fn(String) -> T,
    string_converter: fn(&T) -> String,
    doc: String,
    is_public: bool,
    version: String,
  ) -> Self {
    ConfigEntryWithDefaultFunction {
      key: key,
      prepended_key: prepended_key,
      prepend_separator: prepend_separator,
      alternatives: alternaatives,
      default_function: default_function,
      value_converter: value_converter,
      string_converter: string_converter,
      doc: doc,
      is_public: is_public,
      version: version
    }
  }

  //pub fn default_value(&self) -> &T {
    //let val = (self.default_function)();
    //&val
  //}

  //pub fn default_value_string(&self) -> String {
    //(self.string_converter)(&(self.default_function)())
  //}

  pub fn read_string() {}
  pub fn read_from() {
      
  }
}

pub struct ConfigEntryWithDefaultString<T> {
  key: String,
  prepended_key: Option<String>,
  prepend_separator: String,
  alternatives: Vec<String>,
  default_value: String,
  value_converter: fn(&String) -> T,
  string_converter: fn(&T) -> String,
  doc: String,
  is_public: bool,
  version: String,
}

impl<T> ConfigEntryWithDefaultString<T> {
  pub fn new(
    key: String,
    prepended_key: Option<String>,
    prepend_separator: String,
    alternaatives: Vec<String>,
    default_value: String,
    value_converter: fn(&String) -> T,
    string_converter: fn(&T) -> String,
    doc: String,
    is_public: bool,
    version: String,
  ) -> Self {
    ConfigEntryWithDefaultString {
      key: key,
      prepended_key: prepended_key,
      prepend_separator: prepend_separator,
      alternatives: alternaatives,
      default_value: default_value,
      value_converter: value_converter,
      string_converter: string_converter,
      doc: doc,
      is_public: is_public,
      version: version
    }
  }

  pub fn default_value(&self) -> T {
    (self.value_converter)(&self.default_value)
  }

  pub fn default_value_string(&self) -> String {
    self.default_value.clone()
  }

  pub fn read_string() {}
  pub fn read_from() {
      
  }
}

pub struct OptionalConfigEntry<T> {
  key: String,
  prepended_key: Option<String>,
  prepend_separator: String,
  alternatives: Vec<String>,
  default_value: T,
  value_converter: fn(String) -> T,
  string_converter: fn(&T) -> String,
  doc: String,
  is_public: bool,
  version: String,
}

impl<T> OptionalConfigEntry<T> {
  pub fn new(
    key: String,
    prepended_key: Option<String>,
    prepend_separator: String,
    alternaatives: Vec<String>,
    default_value: T,
    value_converter: fn(String) -> T,
    string_converter: fn(&T) -> String,
    doc: String,
    is_public: bool,
    version: String,
  ) -> Self {
    OptionalConfigEntry {
      key: key,
      prepended_key: prepended_key,
      prepend_separator: prepend_separator,
      alternatives: alternaatives,
      default_value: default_value,
      value_converter: value_converter,
      string_converter: string_converter,
      doc: doc,
      is_public: is_public,
      version: version
    }
  }

  pub fn default_value_string(&self) -> String {
    "".to_string()
  }

  pub fn read_string() {}
  pub fn read_from() {}
}

pub struct FallbackConfigEntry<T> {
  key: String,
  prepended_key: Option<String>,
  prepend_separator: String,
  alternatives: Vec<String>,
  default_value: T,
  value_converter: fn(String) -> T,
  string_converter: fn(&T) -> String,
  doc: String,
  is_public: bool,
  version: String,
}

impl<T> FallbackConfigEntry<T> {
  pub fn new(
    key: String,
    prepended_key: Option<String>,
    prepend_separator: String,
    alternaatives: Vec<String>,
    default_value: T,
    value_converter: fn(String) -> T,
    string_converter: fn(&T) -> String,
    doc: String,
    is_public: bool,
    version: String,
  ) -> Self {
    FallbackConfigEntry {
      key: key,
      prepended_key: prepended_key,
      prepend_separator: prepend_separator,
      alternatives: alternaatives,
      default_value: default_value,
      value_converter: value_converter,
      string_converter: string_converter,
      doc: doc,
      is_public: is_public,
      version: version
    }
  }

  pub fn default_value_string(&self) -> String {
    (self.string_converter)(&self.default_value)
  }

  pub fn read_string() {}
  pub fn read_from() {
      
  }
}