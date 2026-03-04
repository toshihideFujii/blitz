use std::collections::{HashMap, HashSet};
use common::primitive_util::is_primitive_type_name;

// Simple stateful class that helps generate "unique" names. To use it, simply
// call GetUniqueName as many times as needed. The names returned by
// GetUniqueName are guaranteed to be distinct for this instance of the class.
// Note that the names will be sanitized to match regexp
// "[a-zA-Z_][a-zA-Z0-9_.-]*".
#[derive(Clone, PartialEq)]
pub struct NameUniquer {
  // The string to use to separate the prefix of the name from the uniquing
  // integer value.
  separator: String,
  // Map from name prefix to the generator data structure which tracks used
  // identifiers and generates new ones.
  generated_names: HashMap<String, SequentialIdGenerator>
}

impl NameUniquer {
  // The separator must contain allowed characters only: "[a-zA-Z0-9_.-]".
  pub fn new(separator: String) -> Self {
    for c in separator.chars() {
      if !is_allowed(&c) {
        assert!(false, "separator should comprises allowed characters only");
      }
    }
    Self {
      separator: separator,
      generated_names: HashMap::new()
    }
  }

  // Get a sanitized unique name in a string, with an optional prefix for
  // convenience.
  pub fn get_unique_name(&mut self, prefix: &String) -> String {
    let mut root = NameUniquer::get_sanitized_name(&"name".to_string());
    if !prefix.is_empty() {
      root = NameUniquer::get_sanitized_name(prefix);
    }
    // Strip away numeric suffix (if any). Only recognize separator if it is in
    // the middle of the name.
    let mut has_numeric_suffix = false;
    let mut numeric_suffix = 0;
    let separator_idx = root.rfind(&self.separator);
    if separator_idx.is_some() && separator_idx.unwrap() > 0 &&
      (separator_idx.unwrap() < root.len() - self.separator.len())
    {
      let after_suffix =
        root.get(separator_idx.unwrap() + self.separator.len()..).unwrap();
      let parsed_suffix = after_suffix.parse::<i64>();
      if parsed_suffix.is_ok() {
        numeric_suffix = parsed_suffix.unwrap();
        has_numeric_suffix = true;
        // Remove numeric suffix from root.
        root = root.get(0..separator_idx.unwrap()).unwrap().to_string();
      }
    }

    if !self.generated_names.contains_key(&root) {
      self.generated_names.insert(root.clone(), SequentialIdGenerator::default());
    }
    let id_generator: Option<&mut SequentialIdGenerator> =
      self.generated_names.get_mut(&root);
    numeric_suffix = id_generator.unwrap().register_id(numeric_suffix);
    if numeric_suffix == 0 {
      if has_numeric_suffix {
        root.push_str(&self.separator);
        root.push_str(&0.to_string());
      }
      return root;
    }
    root.push_str(&self.separator);
    root.push_str(&numeric_suffix.to_string());
    root
  }

  // Sanitizes and returns the name. Unallowed characters will be replaced with
  // '_'. The result will match the regexp "[a-zA-Z_][a-zA-Z0-9_.-]*".
  pub fn get_sanitized_name(name: &String) -> String {
    if name.is_empty() {
      return "".to_string();
    }
    let mut result = name.clone();
    let c = result.chars().nth(0).unwrap();
    if !c.is_alphanumeric() && c != '_' {
      result.remove(0);
      result.insert(0, '_');
    }
    for i in 1..result.len() {
      let c = result.chars().nth(i).unwrap();
      if !is_allowed(&c) {
        result.remove(i);
        result.insert(i, '_');
      }
    }
    // HLO primitive type names (with the exception of 'tuple' and 'buffer') are
    // keywords in the HLO text representation and cannot be names, so append an
    // underscore if the name is a primitive type.
    if is_primitive_type_name(&result) && result != "tuple".to_string() &&
      result != "buffer".to_string()
    {
      result.push_str("_");
    }
    if result.starts_with("__") && !result.starts_with("__blitz_") {
      // Morph name prefix __ that is not __xla_, to avoid using name prefixes
      // reserved by the backends, such as __llvm_retpoline_ reserved by the LLVM
      // x86 backend.
      result.remove(0);
      result.insert(0, 'a');
    }
    result
  }
}

// Used to track and generate new identifiers for the same instruction name root.
#[derive(Clone, PartialEq)]
struct SequentialIdGenerator {
  next: i64,
  used: HashSet<i64>
}

impl SequentialIdGenerator {
  fn default() -> Self {
    SequentialIdGenerator { next: 0, used: HashSet::new() }
  }

  // Tries to register id as used identifier. If id is not already used, the
  // id itself will be returned. Otherwise a new one will be generated, and
  // returned.
  fn register_id(&mut self, id: i64) -> i64 {
    if self.used.insert(id) {
      return id;
    }
    while !self.used.insert(self.next) {
      self.next += 1;
    }
    let result = self.next;
    self.next += 1;
    result
  }
}

fn is_allowed(c: &char) -> bool {
  c.is_alphanumeric() || *c == '_' || *c == '.' || *c == '-'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_uniquer() {
    let mut uniquer = NameUniquer::new("__".to_string());

    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo__1".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo__2".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar".to_string()), "bar".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo__3".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar".to_string()), "bar__1".to_string());
    assert_eq!(uniquer.get_unique_name(&"qux".to_string()), "qux".to_string());
  }

  #[test]
  fn test_different_separator() {
    let mut uniquer = NameUniquer::new(".".to_string());

    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo.1".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo.2".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar".to_string()), "bar".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo.3".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar".to_string()), "bar.1".to_string());
  }

  #[test]
  fn test_numeric_suffixes() {
    let mut uniquer = NameUniquer::new(".".to_string());

    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.54".to_string()), "foo.54".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo.1".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.55.1".to_string()), "foo.55.1".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.55.1".to_string()), "foo.55.0".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar.1000".to_string()), "bar.1000".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar.2000".to_string()), "bar.2000".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar.-2000".to_string()), "bar.-2000".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar.1".to_string()), "bar.1".to_string());
  }

  #[test]
  fn test_prefix_has_suffix() {
    let mut uniquer = NameUniquer::new(".".to_string());

    assert_eq!(uniquer.get_unique_name(&"foo.11.0".to_string()), "foo.11.0".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.11".to_string()), "foo.11".to_string());
  }

  #[test]
  fn test_sanitize() {
    let mut uniquer = NameUniquer::new("_".to_string());

    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo_1".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.54".to_string()), "foo.54".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo_54".to_string()), "foo_54".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo_54.1".to_string()), "foo_54.1".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo".to_string()), "foo_2".to_string());

    // Invalid characters will be replaced with '_'.
    assert_eq!(uniquer.get_unique_name(&"bar<1000".to_string()), "bar_1000".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar<2000".to_string()), "bar_2000".to_string());
    assert_eq!(uniquer.get_unique_name(&"bar_1".to_string()), "bar_1".to_string());

    // Separator is only recognized in the middle of the prefix.
    // the leading '.' is replaced with '_'.
    assert_eq!(uniquer.get_unique_name(&".10".to_string()), "_10".to_string());
    assert_eq!(uniquer.get_unique_name(&".10".to_string()), "_10_1".to_string());
    assert_eq!(uniquer.get_unique_name(&"foobar_".to_string()), "foobar_".to_string());
    assert_eq!(uniquer.get_unique_name(&"foobar_".to_string()), "foobar__1".to_string());
  }

  #[test]
  fn test_keep_names_in_random_order() {
    let mut uniquer = NameUniquer::new(".".to_string());

    assert_eq!(uniquer.get_unique_name(&"foo.11".to_string()), "foo.11".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.10".to_string()), "foo.10".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.1".to_string()), "foo.1".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.12".to_string()), "foo.12".to_string());
    assert_eq!(uniquer.get_unique_name(&"foo.3".to_string()), "foo.3".to_string());
  }

  #[test]
  fn test_avoid_keywords() {
    let mut uniquer = NameUniquer::new(".".to_string());

    assert_eq!(uniquer.get_unique_name(&"f32".to_string()), "f32_".to_string());
    assert_eq!(uniquer.get_unique_name(&"s64".to_string()), "s64_".to_string());
    assert_eq!(uniquer.get_unique_name(&"pred".to_string()), "pred_".to_string());

    // Name prefix __blitz_ is preserved.
    assert_ne!(uniquer.get_unique_name(&"__blitz_".to_string()).find("__blitz_"), None);
    // Other form of __ prefixes is not preserved to avoid using name prefixes
    // reserved by backends.
    assert_eq!(uniquer.get_unique_name(&"__abx".to_string()).find("__"), None);

    // Though a primitive type, "tuple" is not a keyword.
    assert_eq!(uniquer.get_unique_name(&"tuple".to_string()), "tuple".to_string());

    // Keywords are not capitalized.
    assert_eq!(uniquer.get_unique_name(&"F32".to_string()), "F32".to_string());
    assert_eq!(uniquer.get_unique_name(&"S32".to_string()), "S32".to_string());
    assert_eq!(uniquer.get_unique_name(&"Pred".to_string()), "Pred".to_string());
  }

  #[test]
  fn test_detect_separator() {
    let mut uniquer = NameUniquer::new("__".to_string());

    assert_eq!(uniquer.get_unique_name(&"a__1".to_string()), "a__1".to_string());
    assert_eq!(uniquer.get_unique_name(&"a".to_string()), "a".to_string());
    assert_eq!(uniquer.get_unique_name(&"a".to_string()), "a__2".to_string());
  }
}