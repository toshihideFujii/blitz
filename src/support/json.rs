
#![allow(dead_code)]

/*
JSON values, parsing and serialization
*/

// An Object is a JSON object, which maps sstrings to heterogenous
// JSON values. It simulates DenseMap<ObjectKey, Value>.
// ObjectKey is a maybe-owned string.
struct Object {}

impl Object {
  pub fn new() {}

  pub fn begin() {}

  pub fn end() {}

  pub fn empty() {}

  pub fn size() {}

  pub fn clear() {}

  pub fn insert() {}

  pub fn try_emplace() {}

  pub fn erase() {}

  pub fn find() {}

  pub fn at() {}

  pub fn get() {}

  pub fn get_null() {}

  pub fn get_boolean() {}

  pub fn get_number() {}

  pub fn get_integer() {}

  pub fn get_string() {}

  pub fn get_object() {}

  pub fn get_array() {}

  pub fn equal() {}
}

// An Array is a JSON array, which contains heterogeneous JSON values.
// It simulates std::vector<Value>.
struct Array {}

impl Array {
  pub fn new() {}

  pub fn at() {}

  pub fn front() {}

  pub fn back() {}

  pub fn data() {}

  pub fn begin() {}

  pub fn end() {}

  pub fn empty() {}

  pub fn size() {}

  pub fn reserve() {}

  pub fn clear() {}

  pub fn push_back() {}

  pub fn emplace_back() {}

  pub fn pop_back() {}

  pub fn insert() {}

  pub fn emplace() {}

  pub fn equal() {}

  pub fn not_equal() {}
}

// A Value is an JSON value of unknown type.
// They can be copied, but should generally be moved.
struct Value {}

enum Kind {
  Null,
  Boolean,
  Number,
  String,
  Array,
  Object
}

impl Value {
  pub fn new() {}
}

// ObjectKey is a used to capture keys in object.
struct ObjectKey {}

impl ObjectKey {
  pub fn new() {}

  pub fn assign() {}

  pub fn str() {}
}

// A 'cursor' marking a position within a Value.
struct Path {}

impl Path {
  pub fn report() {}

  pub fn index() {}

  pub fn field() {}
}