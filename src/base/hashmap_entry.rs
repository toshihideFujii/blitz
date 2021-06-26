struct TemplateHashMapEntry<Key, Value> {
  key: Key,
  value: Value,
  hash: u32, // The full hash value for key
  exists_: bool,
}

// HashMap entries are (key, value, hash) triplets, with a boolean indicating
// if they are an empty entry.
// Some clients may not need to use the value slot, in which case they should use NoHashMapValue.
impl<Key, Value> TemplateHashMapEntry<Key, Value> {
  fn exists(self) -> bool {
    self.exists_
  }
  fn clear(&mut self) {
    self.exists_ = false
  }
}
