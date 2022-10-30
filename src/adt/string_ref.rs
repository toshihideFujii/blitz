#![allow(dead_code)]

// Represent a constant reference to a string.
struct StringRef {
  // The start of the string, in an external buffer.
  data_: String
}

impl StringRef {
  // Construct an empty string ref.
  pub fn new() -> Self {
    StringRef { data_: String::new() }
  }

  pub fn begin() {}

  pub fn end() {}

  pub fn bytes_begin() {}

  pub fn bytes_end() {}

  pub fn bytes() {}

  // String operations

  // Get a pointer to the start of the string (which may not be null terminated).
  pub fn data(&self) -> String {
    self.data_.clone()
  }

  // Check if the string is empty.
  pub fn empty(&self) -> bool {
    self.data_.is_empty()
  }

  // Get the string size.
  pub fn size(&self) -> usize {
    self.data_.len()
  }

  // Get the first character in the string.
  pub fn front(&self) -> char {
    self.data_.chars().nth(0).unwrap()
  }

  // Get the last character in the string.
  pub fn back(&self) -> char {
    self.data_.chars().last().unwrap()
  }

  // Allocate copy in allocator and return StringRef to it.
  pub fn copy(&self) -> StringRef {
    StringRef { data_: self.data_.clone() }
  }

  // Check for string equality.
  pub fn equals(&self, rhs: StringRef) -> bool {
    self.data_.eq(&rhs.data_)
  }

  // Check for string equality, ignoring case.
  pub fn equals_insensitive(&self, rhs: StringRef) -> bool {
    self.data_.eq_ignore_ascii_case(&rhs.data_)
  }

  // Compare two strings; the result is -1, 0, or 1 if this string 
  // is lexicographically less than, equal to, or greater than the rhs.
  pub fn compare(&self, rhs: StringRef) -> i64 {
    if self.data_.len() == rhs.data_.len() {
      return  0;
    }
    return if self.data_.len() < rhs.data_.len() { -1 } else { 1 };
  }

  // Compare two strings, ignoring case.
  pub fn compare_insensitive() {}

  // Compare two strings, treating sequences of digits as numbers.
  pub fn compare_numeric() {}

  pub fn edit_distance() {}

  pub fn edit_distance_insensitive() {}

  pub fn str() {}

  pub fn at(&self, index: usize) -> char {
    self.data_.chars().nth(index).unwrap()
  }

  // Check if this string starts with the given prefix.
  pub fn starts_with(&self, prefix: StringRef) -> bool {
    self.data_.starts_with(prefix.data_.as_str())
  }

  // Check if this string starts with the given prefix, ignoring case.
  pub fn starts_with_insensitive(&self, prefix: StringRef) -> bool {
    let original = self.data_.to_ascii_lowercase();
    let other = prefix.data_.to_ascii_lowercase();
    return original.starts_with(other.as_str());
  }

  // Check if this string ends with the given suffix.
  pub fn ends_with(&self, suffix: StringRef) -> bool {
    self.data_.ends_with(suffix.data_.as_str())
  }

  // Check if this string ends with the given suffix, ignoring case.
  pub fn ends_with_insensitive(&self, suffix: StringRef) -> bool {
    let original = self.data_.to_ascii_lowercase();
    let other = suffix.data_.to_ascii_lowercase();
    return original.ends_with(other.as_str());
  }

  // Search for the first character c in the string.
  pub fn find(&self, c: char) -> usize {
    self.data_.find(c).unwrap()
  }

  // Search for the first character c in the string, ignoring case.
  pub fn find_insensitive(&self, c: char) -> usize {
    let original = self.data_.to_ascii_lowercase();
    let c_lower = c.to_ascii_lowercase();
    return original.find(c_lower).unwrap();
  }

  pub fn find_if() {}

  pub fn find_if_not() {}

  // Search for the last character c in the string.
  pub fn rfind(&self, c: char) -> usize {
    self.data_.rfind(c).unwrap()
  }

  // Search for the last character c in the string, ignoring case.
  pub fn rfind_insensitive(&self, c: char) -> usize {
    let original = self.data_.to_ascii_lowercase();
    let c_lower = c.to_ascii_lowercase();
    return original.rfind(c_lower).unwrap();
  }

  pub fn find_first_of() {}

  pub fn find_first_not_of() {}

  pub fn find_last_of() {}

  pub fn find_last_not_of() {}

  // Return true if the given string is a substring of this,
  // and false otherwise.
  pub fn contains(&self, other: StringRef) -> bool {
    self.data_.contains(other.data_.as_str())
  }

  pub fn contains_insensitive(&self, other: StringRef) -> bool {
    let original = self.data_.to_ascii_lowercase();
    let other_lower = other.data_.to_ascii_lowercase();
    return original.contains(other_lower.as_str());
  }

  // Return the number of occurrences of c in the string.
  pub fn count(&self, c: char) -> usize {
    let mut words = self.data_.chars();
    let mut count: usize = 0;
    for word in words.next() {
      if word == c {
        count+=1;
      }
    }
    return count;
  }

  pub fn get_as_integer() {}

  pub fn consume_integer() {}

  pub fn get_as_double() {}

  pub fn lower() {}

  pub fn upper() {}

  pub fn substr() {}

  pub fn take_front() {}

  pub fn take_back() {}

  pub fn take_while() {}

  pub fn take_until() {}

  pub fn drop_front() {}

  pub fn drop_back() {}

  pub fn drop_while() {}

  pub fn drop_until() {}

  pub fn consume_front() {}

  pub fn consume_front_insensitive() {}

  pub fn consume_back() {}

  pub fn consume_back_insensitive() {}

  pub fn slice() {}

  pub fn split() {}

  pub fn rsplit() {}

  pub fn ltrim() {}

  pub fn rtrim() {}

  pub fn trim() {}

  pub fn detect_eol() {}
}

struct StringLiteral {}