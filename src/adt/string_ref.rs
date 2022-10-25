
#![allow(dead_code)]

// Represent a constant reference to a string.
struct StringRef {}

impl StringRef {
  pub fn begin() {}

  pub fn end() {}

  pub fn bytes_begin() {}

  pub fn bytes_end() {}

  pub fn bytes() {}

  // String operations

  // Get a pointer to the start of the string
  // (which may not be null terminated).
  pub fn data() {}

  // Check if the string is empty.
  pub fn empty() {}

  // Get the string size.
  pub fn size() {}

  // Get the first character in the string.
  pub fn front() {}

  // Get the last character in the string.
  pub fn back() {}

  // Allocate copy in allocator and return StringRef to it.
  pub fn copy() {}

  // Check for string equality.
  pub fn equals() {}

  // Checj for string equality, ignoring case.
  pub fn equals_insensitive() {}

  // Compare two strings.
  pub fn compare() {}

  // Compare two strings, ignoring case.
  pub fn compare_insensitive() {}

  // Compare two strings, treating sequences of digits as numbers.
  pub fn compare_numeric() {}
}