#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp;

const NPOS: usize = 99999999999; // TODO

// Represent a constant reference to a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringRef {
  data: String,
  length: usize
}

impl StringRef {
  // Construct an empty string ref.
  pub fn new() -> Self {
    StringRef { data: String::new(), length: 0 }
  }

  // Construct a string ref from a string.
  pub fn new_from_string(data: &str) -> Self {
    let string = data;
    StringRef { data: String::from(string), length: string.len() }
  }

  // Construct a string ref from a pointer and length.
  pub fn new_from_string_and_length(data: &str, length: usize) -> Self {
    let string = data;
    StringRef { data: String::from(string), length: length }
  }

  // Iterators
  pub fn begin(&self) -> char {
    self.data().chars().next().unwrap()
  }

  pub fn end(&self) -> char {
    self.data().chars().last().unwrap()
  }

  pub fn bytes_begin() {}

  pub fn bytes_end() {}

  pub fn bytes() {}

  // String operations

  // Get a pointer to the start of the string (which may not be null terminated).
  pub fn data(&self) -> String {
    self.data.clone()
  }

  // Check if the string is empty.
  pub fn empty(&self) -> bool {
    self.data.is_empty()
  }

  // Get the string size.
  pub fn size(&self) -> usize {
    self.data.len()
  }

  // Get the first character in the string.
  pub fn front(&self) -> char {
    self.data.chars().nth(0).unwrap()
  }

  // Get the last character in the string.
  pub fn back(&self) -> char {
    self.data.chars().last().unwrap()
  }

  // Allocate copy in allocator and return StringRef to it.
  pub fn copy(&self) -> StringRef {
    StringRef { data: self.data.clone(), length: self.length }
  }

  // Check for string equality.
  pub fn equals(&self, rhs: StringRef) -> bool {
    self.data.eq(&rhs.data)
  }

  // Check for string equality, ignoring case.
  pub fn equals_insensitive(&self, rhs: StringRef) -> bool {
    self.data.eq_ignore_ascii_case(&rhs.data)
  }

  // Compare two strings; the result is -1, 0, or 1 if this string 
  // is lexicographically less than, equal to, or greater than the rhs.
  pub fn compare(&self, rhs: &str) -> i64 {
    if self.data().as_str().cmp(rhs) == cmp::Ordering::Equal {
      return 0;
    } else if self.data().as_str().cmp(rhs) == cmp::Ordering::Less {
        return -1;
    } else {
      return 1;
    }
  }

  // Compare two strings, ignoring case.
  pub fn compare_insensitive(&self, rhs: &str) -> i64 {
    let rhs_lower = rhs.to_ascii_lowercase();
    if self.data().as_str().to_ascii_lowercase().cmp(&rhs_lower) == cmp::Ordering::Equal {
      return 0;
    } else if self.data().as_str().to_ascii_lowercase().cmp(&rhs_lower) == cmp::Ordering::Less {
        return -1;
    } else {
      return 1;
    }
  }

  // Compare two strings, treating sequences of digits as numbers.
  pub fn compare_numeric() {}

  pub fn edit_distance() {}

  pub fn edit_distance_insensitive() {}

  pub fn str() {}

  pub fn at(&self, index: usize) -> char {
    self.data.chars().nth(index).unwrap()
  }

  // Check if this string starts with the given prefix.
  pub fn starts_with(&self, prefix: &str) -> bool {
    self.data.starts_with(prefix)
  }

  // Check if this string starts with the given prefix, ignoring case.
  pub fn starts_with_insensitive(&self, prefix: &str) -> bool {
    let original = self.data.to_ascii_lowercase();
    let other = prefix.to_ascii_lowercase();
    return original.starts_with(other.as_str());
  }

  // Check if this string ends with the given suffix.
  pub fn ends_with(&self, suffix: &str) -> bool {
    self.data.ends_with(suffix)
  }

  // Check if this string ends with the given suffix, ignoring case.
  pub fn ends_with_insensitive(&self, suffix: &str) -> bool {
    let original = self.data.to_ascii_lowercase();
    let other = suffix.to_ascii_lowercase();
    return original.ends_with(other.as_str());
  }

  // Search for the first character c in the string.
  pub fn find(&self, c: char, from: usize) -> usize {
    let str = self.data().clone();
    let find_begin = cmp::min(from, self.length);
    if find_begin < self.length {
      let (abandoned, remained) = str.split_at(find_begin);
      let pos = remained.find(c);
      if pos != None {
        return pos.unwrap() + abandoned.len();
      } else {
        return NPOS;
      }
    }
    NPOS
  }

  // Search for the first string str in the string.
  pub fn find_str(&self, str: StringRef, from: usize) -> usize {
    let s = self.data().clone();
    let find_begin = cmp::min(from, self.length);
    if find_begin < self.length {
      let (abandoned, remained) = s.split_at(find_begin);
      let remained_str = String::from(remained);
      let result = remained_str.match_indices(str.data().as_str()).next();
      if result != None {
        return result.unwrap().0 + abandoned.len();
      } else {
        return NPOS;
      }
    }
    NPOS
  }

  // Search for the first character c in the string, ignoring case.
  pub fn find_insensitive(&self, c: char, from: usize) -> usize {
    let orig_lower = self.data.to_ascii_lowercase();
    let s = StringRef::new_from_string(&orig_lower);
    let c_lower = c.to_ascii_lowercase();
    s.find(c_lower, from)
  }

  // Search for the first string str in the string, ignoring case.
  pub fn find_str_insensitive(&self, str: StringRef, from: usize) -> usize {
    let orig_lower = self.data.to_ascii_lowercase();
    let s = StringRef::new_from_string(&orig_lower);
    let str_lower = str.data().to_ascii_lowercase();
    let str_lower_ref = StringRef::new_from_string(&str_lower);
    s.find_str(str_lower_ref, from)
  }

  // Search for the first character satisfying the predicate f.
  pub fn find_if<F: Fn(char)->bool>(&self, f: F, from: usize) -> usize {
    let mut s = self.drop_front(from);
    while s.empty() == false {
      if f(s.front()) == true {
        return self.size() - s.size();
      }
      s = s.drop_front(1);
    }
    NPOS
  }

  // Search for the first character not satisfying the predicate f.
  pub fn find_if_not<F: Fn(char)->bool>(&self, f: F, from: usize) -> usize {
    let func = |ch| {
      return !f(ch)
    };
    self.find_if(func, from)
  }

  // Search for the last character c in the string.
  pub fn rfind(&self, c: char, from: usize) -> usize {
    let str = self.data().clone();
    let rfind_begin = cmp::min(from, self.length);
    let (remained, abandoned) = str.split_at(rfind_begin);
    let pos = String::from(remained).rfind(c);
    if pos != None {
      return pos.unwrap() + abandoned.len();
    } else {
      return NPOS;
    }
  }

  // Search for the last string str in the string.
  pub fn rfind_str(&self, str: StringRef) -> usize {
    let s = self.data().clone();
    let result = s.rmatch_indices(str.data().as_str()).next();
    if result != None {
      return result.unwrap().0;
    } else {
      return NPOS;
    }
  }

  // Search for the last string str in the string.
  pub fn rfind_str_insensitive(&self, str: StringRef) -> usize {
    let orig_lower = self.data.to_ascii_lowercase();
    let s = StringRef::new_from_string(&orig_lower);
    let str_lower = str.data().to_ascii_lowercase();
    let str_lower_ref = StringRef::new_from_string(&str_lower);
    s.rfind_str(str_lower_ref)
  }

  // Search for the last character c in the string, ignoring case.
  pub fn rfind_insensitive(&self, c: char, from: usize) -> usize {
    let orig_lower = self.data.to_ascii_lowercase();
    let s = StringRef::new_from_string(&orig_lower);
    let c_lower = c.to_ascii_lowercase();
    s.rfind(c_lower, from)
  }

  // Find the first character in the sring that is c.
  pub fn find_first_of(&self, c: char, from: usize) -> usize {
    self.find(c, from)
  }

  // Find the first character in the string that is in str.
  pub fn find_first_of_str(&self, str: StringRef, from: usize) -> usize {
    self.find_str(str, from)
  }

  // Find the first character in the string that is not c.
  pub fn find_first_not_of(&self, c: char, from: usize) -> usize {
    let str = self.data().clone();
    let begin = cmp::min(from, self.length);
    if begin < self.length {
      let (abandoned, remained) = str.split_at(begin);
      let mut chars = remained.char_indices();
      loop {
        let val = chars.next();
        if val == None {
          break;
        }
        if val.unwrap().1 != c {
          return val.unwrap().0 + abandoned.len();
        }
      }
    }
    NPOS
  }

  // Find the first character in the string that is not in the string str.
  pub fn find_first_not_of_str(&self, str: StringRef, from: usize) -> usize {
    let s = self.data().clone();
    let begin = cmp::min(from, self.length);
    if begin < self.length {
      let (abandoned, remained) = s.split_at(begin);
      let mut chars = remained.char_indices();
      let str_clone = str.data().clone();
      loop {
        let val = chars.next();
        if val == None {
          break;
        }
        if str_clone.contains(val.unwrap().1) {
          continue;
        } else {
          return val.unwrap().0 + abandoned.len();
        }
      }
    }
    NPOS
  }

  // Find the last character in the string that is c.
  pub fn find_last_of(&self, c: char, from: usize) -> usize {
    self.rfind(c, from)
  }

  // Find the last character in the string that is not c.
  pub fn find_last_not_of(&self, c: char, from: usize) -> usize {
    let str = self.data().clone();
    let rfind_begin = cmp::min(from, self.length);
    let (remained, abandoned) = str.split_at(rfind_begin);
    let mut chars = remained.char_indices().rev();
    loop {
      let val = chars.next();
      if val == None {
        break;
      }
      if val.unwrap().1 != c {
        return val.unwrap().0 + abandoned.len();
      }
    }
    NPOS
  }

  // Find the last character in the string that is not in str.
  pub fn find_last_not_of_str(&self, str: StringRef, from: usize) -> usize {
    let s = self.data().clone();
    let begin = cmp::min(from, self.length);
    let (remained, abandoned) = s.split_at(begin);
    let mut chars = remained.char_indices().rev();
    let str_clone = str.data().clone();
    loop {
      let val = chars.next();
      if val == None {
        break;
      }
      if str_clone.contains(val.unwrap().1) {
        continue;
      } else {
        return val.unwrap().0 + abandoned.len();
      }
    }
    NPOS
  }

  // Return true if the given string is a substring of this,
  // and false otherwise.
  pub fn contains(&self, other: StringRef) -> bool {
    self.data.contains(other.data.as_str())
  }

  pub fn contains_insensitive(&self, other: StringRef) -> bool {
    let original = self.data.to_ascii_lowercase();
    let other_lower = other.data.to_ascii_lowercase();
    return original.contains(other_lower.as_str());
  }

  // Return the number of occurrences of c in the string.
  pub fn count_char(&self, c: char) -> usize {
    let mut count = 0;
    for c_ in self.data().chars() {
      if c_ == c {
        count+=1;
      }
    }
    count
  }

  // Return the number of non-overlapped occurrences o str in
  // the string.
  pub fn count_str(&self, str: &str) -> usize {
    let mut count = 0;
    let n = str.len();
    if n == 0 || n > self.length {
      return 0;
    }
    let mut skip = false;
    let mut skip_count = 0;
    // TODO: ugly
    for i in 0..(self.length-n+1) {
      if skip == true && i < skip_count {
        continue;
      }
      if skip == true && i >= skip_count {
        skip = false;
      }
      if self.substr(i, n).data() == str {
        count+=1;
        skip_count = n;
        skip = true;
        println!("{}: match {}",str, i);
      }
    }
    count
  }

  pub fn get_as_integer() {}

  pub fn consume_integer() {}

  pub fn get_as_double() {}

  // Convert the given ASCII string to lowercase.
  pub fn lower(&self) -> String {
    self.data.to_ascii_lowercase()
  }

  // Convert the given ASCII string to uppercase.
  pub fn upper(&self) -> String {
    self.data.to_ascii_uppercase()
  }

  // Return a reference to the substring from [start, start+n).
  pub fn substr(&self, start: usize, n: usize) -> Self { 
    let start_min = cmp::min(start, self.length);
    let length_min = cmp::min(n, self.length-start_min);
    
    //let sub = &self.data[start_min..self.length];
    let sub = &self.data[start_min..(start_min+length_min)];
    StringRef { data: sub.to_string(), length: length_min }
  }

  // Return a StringRef equal to this but with only the first n
  // elements remaining.
  pub fn take_front(&self, n: usize) -> Self {
    if n >= self.size() {
      return StringRef { data: self.data.clone(), length: self.length };
    }
    self.drop_back(self.size()-n)
  }

  // Return a StringRef equal to this but with only the last n
  // elements remaining.
  pub fn take_back(&self, n: usize) -> Self {
    if n >= self.size() {
      return StringRef { data: self.data.clone(), length: self.length };
    }
    self.drop_front(self.size()-n)
  }

  pub fn take_while() {}

  pub fn take_until() {}

  // Return a StringRef equal to this but with the first n elements
  // dropped.
  pub fn drop_front(&self, n: usize) -> Self {
    if self.size() < n {
      panic!("Dropping more elements than exist.");
    }
    self.substr(n, NPOS)
  }

  // Return a StringRef equal to this but with the last n elements
  // dropped.
  pub fn drop_back(&self, n: usize) -> Self {
    if self.size() < n {
      panic!("Dropping more elements than exist.");
    }
    self.substr(0, self.size()-n)
  }

  // Return a StringRef equal to this, but with all characters satisfying
  // the given predicate dropped from the beginning of the string.
  pub fn drop_while<F: Fn(char)->bool>(&self, f: F) -> Self {
    self.substr(self.find_if_not(f, 0), NPOS)
  }

  // Return a StringRef equal to this, but with all characters not
  // satisfying the given predicate dropped from the beginning of the string.
  pub fn drop_until<F: Fn(char)->bool>(&self, f: F) -> Self {
    self.substr(self.find_if(f, 0), NPOS)
  }

  // Returns true if this StringRef has the given prefix and removes
  // that prefix.
  pub fn consume_front(&mut self, prefix: &str) -> bool {
    let n = prefix.len();
    if !self.starts_with(prefix) {
      return false;
    }
    let s = self.drop_front(n);
    self.data = s.data.clone();
    self.length = s.length;
    true
  }

  // Returns true if this StringRef has the given prefix, ignoring case.
  // and removes that prefix.
  pub fn consume_front_insensitive(&mut self, prefix: &str) -> bool {
    let n = prefix.len();
    if !self.starts_with_insensitive(prefix) {
      return false;
    }
    let s = self.drop_front(n);
    self.data = s.data.clone();
    self.length = s.length;
    true
  }

  // Returns true if this StringRef has the given suffix and removes
  // that suffix.
  pub fn consume_back(&mut self, suffix: &str) -> bool {
    let n = suffix.len();
    if !self.ends_with(suffix) {
      return false;
    }
    let s = self.drop_back(n);
    self.data = s.data.clone();
    self.length = s.length;
    true
  }

  // Returns true if this StringRef has the given suffix, ignoring case,
  // and removes that suffix.
  pub fn consume_back_insensitive(&mut self, suffix: &str) -> bool {
    let n = suffix.len();
    if !self.ends_with_insensitive(suffix) {
      return false;
    }
    let s = self.drop_back(n);
    self.data = s.data.clone();
    self.length = s.length;
    true
  }

  // Return a reference to the substring from [start, end).
  pub fn slice(&self, start: usize, end: usize) -> Self {
    let new_start = cmp::min(start, self.length);
    let new_end = cmp::min(cmp::max(new_start, end), self.length);
    let data = &self.data[new_start..new_end];
    StringRef { data: data.to_string(), length: data.len() }
  }

  // Split into two substrings around the first occurrence of a
  // separator character.
  pub fn split(&self, separator: char) -> (String, String) {
    let original = self.data.clone();
    let val = original.split_once(separator);
    let mut result = (String::new(), String::new());
    if val.is_none() {
      result.0 = self.data.clone();
    } else {
      result.0 = String::from(val.unwrap().0);
      result.1 = String::from(val.unwrap().1);
    }
    result
  }

  // Split into two substrings around the last occurrence of a
  // separator string.
  pub fn rsplit(&self, separator: char) -> (String, String) {
    let original = self.data.clone();
    let val = original.rsplit_once(separator);
    let mut result = (String::new(), String::new());
    if val.is_none() {
      result.0 = self.data.clone();
    } else {
      result.0 = String::from(val.unwrap().0);
      result.1 = String::from(val.unwrap().1);
    }
    result
  }

  // Return string with consecutive " \t\n\v\f\r" characters starting from 
  // the left removed.
  pub fn ltrim(&self) -> Self {
    let data = self.data().trim_start().to_string();
    let length = data.len();
    StringRef { data: data, length: length }
  }

  // Return string with consecutive " \t\n\v\f\r" characters starting from
  // the right removed.
  pub fn rtrim(&self) -> Self {
    let data = self.data().trim_end().to_string();
    let length = data.len();
    StringRef { data: data, length: length }
  }

  // Return string with consecutive " \t\n\v\f\r" characters starting from
  // the left and right removed.
  pub fn trim(&self) -> Self {
    self.ltrim().rtrim()
  }

  // Detect the line ending style of the string.
  pub fn detect_eol(&self) -> Self {
    let pos = self.find('\r', 0);
    if pos == NPOS {
      return StringRef::new_from_string("\n");
    }
    if pos+1 < self.length && self.data().chars().nth(pos+1).unwrap() == '\n' {
      return StringRef::new_from_string("\r\n");
    }
    if pos > 0 && self.data().chars().nth(pos-1).unwrap() == '\n' {
      return StringRef::new_from_string("\n\r");
    }
    StringRef::new_from_string("\r")
  }
}

impl Iterator for StringRef {
  type Item = char;
  fn next(&mut self) -> Option<Self::Item> {
    self.data().chars().next()
  }
}

struct StringLiteral {}

#[cfg(test)]
mod tests {
use std::vec;

use super::*;

  #[test]
  fn test_construction() {
    let new_from_string = StringRef::new_from_string("hello");
    assert_eq!(new_from_string.data(), "hello");
    /* TODO
    let new_from_string_and_length =
      StringRef::new_from_string_and_length("hello world", 5);
    assert_eq!("hello", new_from_string_and_length.data());
    */
  }

  #[test]
  fn test_empty_initializer() {
    let s = StringRef::new();
    assert_eq!(s.empty(), true);
  }

  #[test]
  fn test_iteration() {
    let mut s = StringRef::new_from_string("hello");
    let p = "hello";
    if let Some(it) = s.next() {
      assert_eq!(it, p.chars().next().unwrap());
    }
  }

  #[test]
  fn test_string_ops() {
    assert_eq!(StringRef::new_from_string("hello").size(), 5);
    assert_eq!(StringRef::new_from_string("aab").compare("aad"), -1);
    assert_eq!(StringRef::new_from_string("aab").compare("aab"), 0);
    assert_eq!(StringRef::new_from_string("aab").compare("aaa"), 1);
    assert_eq!(StringRef::new_from_string("aab").compare("aabb"), -1);
    //assert_eq!(StringRef::new_from_string("\xFF").compare("\1"), 1);

    assert_eq!(StringRef::new_from_string("AaB").compare_insensitive("aAd"), -1);
    assert_eq!(StringRef::new_from_string("AaB").compare_insensitive("aab"), 0);
    assert_eq!(StringRef::new_from_string("AaB").compare_insensitive("AAA"), 1);
    assert_eq!(StringRef::new_from_string("AaB").compare_insensitive("aaBb"), -1);
    assert_eq!(StringRef::new_from_string("AaB").compare_insensitive("bb"), -1);
    assert_eq!(StringRef::new_from_string("aaBb").compare_insensitive("Aab"), 1);
    assert_eq!(StringRef::new_from_string("bb").compare_insensitive("Aab"), 1);
    assert_eq!(StringRef::new_from_string("AaB").compare_insensitive("aA"), 1);
  }

  #[test]
  fn test_substr() {
    let s = StringRef::new_from_string("hello");
    assert_eq!(s.substr(3, NPOS).data(), "lo");
    assert_eq!(s.substr(100, NPOS).data(), "");
    assert_eq!(s.substr(0, 100).data(), "hello");
    assert_eq!(s.substr(4, 10).data(), "o");
  }

  #[test]
  fn test_slice() {
    let s = StringRef::new_from_string("hello");
    assert_eq!(s.slice(2, 3).data(), "l");
    assert_eq!(s.slice(1, 4).data(), "ell");
    assert_eq!(s.slice(2, 100).data(), "llo");
    assert_eq!(s.slice(2, 1).data(), "");
    assert_eq!(s.slice(10, 20).data(), "");
  }

  #[test]
  fn test_split() {
    let s = StringRef::new_from_string("hello");
    let mut result = s.split('X');
    assert_eq!(result.0.as_str(), "hello");
    assert_eq!(result.1.as_str(), "");
    result = s.split('e');
    assert_eq!(result.0.as_str(), "h");
    assert_eq!(result.1.as_str(), "llo");
    result = s.split('h');
    assert_eq!(result.0.as_str(), "");
    assert_eq!(result.1.as_str(), "ello");
    result = s.split('l');
    assert_eq!(result.0.as_str(), "he");
    assert_eq!(result.1.as_str(), "lo");
    result = s.split('o');
    assert_eq!(result.0.as_str(), "hell");
    assert_eq!(result.1.as_str(), "");

    result = s.rsplit('X');
    assert_eq!(result.0.as_str(), "hello");
    assert_eq!(result.1.as_str(), "");
    result = s.rsplit('e');
    assert_eq!(result.0.as_str(), "h");
    assert_eq!(result.1.as_str(), "llo");
    result = s.rsplit('h');
    assert_eq!(result.0.as_str(), "");
    assert_eq!(result.1.as_str(), "ello");
    result = s.rsplit('l');
    assert_eq!(result.0.as_str(), "hel");
    assert_eq!(result.1.as_str(), "o");
    result = s.rsplit('o');
    assert_eq!(result.0.as_str(), "hell");
    assert_eq!(result.1.as_str(), "");
  }

  #[test]
  fn test_trim() {
    let s0 = StringRef::new_from_string("hello");
    let s1 = StringRef::new_from_string(" hello ");
    let s2 = StringRef::new_from_string("  hello  ");
    let s3 = StringRef::new_from_string("\t\n\r  hello  \t\n\r");

    assert_eq!(s0.rtrim(), StringRef::new_from_string("hello"));
    assert_eq!(s1.rtrim(), StringRef::new_from_string(" hello"));
    assert_eq!(s2.rtrim(), StringRef::new_from_string("  hello"));
    assert_eq!(s3.rtrim(), StringRef::new_from_string("\t\n\r  hello"));

    assert_eq!(s0.ltrim(), StringRef::new_from_string("hello"));
    assert_eq!(s1.ltrim(), StringRef::new_from_string("hello "));
    assert_eq!(s2.ltrim(), StringRef::new_from_string("hello  "));
    assert_eq!(s3.ltrim(), StringRef::new_from_string("hello  \t\n\r"));

    assert_eq!(s0.trim(), StringRef::new_from_string("hello"));
    assert_eq!(s1.trim(), StringRef::new_from_string("hello"));
    assert_eq!(s2.trim(), StringRef::new_from_string("hello"));
    assert_eq!(s3.trim(), StringRef::new_from_string("hello"));

    assert_eq!(StringRef::new_from_string("").trim(), StringRef::new_from_string(""));
    assert_eq!(StringRef::new_from_string(" ").trim(), StringRef::new_from_string(""));
    assert_eq!(StringRef::new_from_string_and_length(" \0 ", 3).trim(),
      StringRef::new_from_string_and_length("\0", 1));
    assert_eq!(StringRef::new_from_string_and_length("\0\0", 2).trim(),
      StringRef::new_from_string_and_length("\0\0", 2));
  }

  #[test]
  fn test_starts_with() {
    let s = StringRef::new_from_string("hello");
    assert_eq!(s.starts_with(""), true);
    assert_eq!(s.starts_with("he"), true);
    assert_eq!(s.starts_with("helloworld"), false);
    assert_eq!(s.starts_with("hi"), false);
  }

  #[test]
  fn test_starts_with_insensitive() {
    let s = StringRef::new_from_string("heLLo");
    assert_eq!(s.starts_with_insensitive(""), true);
    assert_eq!(s.starts_with_insensitive("he"), true);
    assert_eq!(s.starts_with_insensitive("hell"), true);
    assert_eq!(s.starts_with_insensitive("HELlo"), true);
    assert_eq!(s.starts_with_insensitive("helloworld"), false);
    assert_eq!(s.starts_with_insensitive("hi"), false);
  }

  #[test]
  fn test_consume_front() {
    let mut s = StringRef::new_from_string("hello");
    assert_eq!(s.consume_front(""), true);
    assert_eq!(s.data(), "hello");
    assert_eq!(s.consume_front("he"), true);
    assert_eq!(s.data(), "llo");
    assert_eq!(s.consume_front("lloworld"), false);
    assert_eq!(s.data(), "llo");
    assert_eq!(s.consume_front("lol"), false);
    assert_eq!(s.data(), "llo");
    assert_eq!(s.consume_front("llo"), true);
    assert_eq!(s.data(), "");
    assert_eq!(s.consume_front("o"), false);
    assert_eq!(s.data(), "");
  }

  #[test]
  fn test_consume_front_insensitive() {
    let mut s = StringRef::new_from_string("heLLo");
    assert_eq!(s.consume_front_insensitive(""), true);
    assert_eq!(s.data(), "heLLo");
    assert_eq!(s.consume_front("HEl"), false);
    assert_eq!(s.data(), "heLLo");
    assert_eq!(s.consume_front_insensitive("HEl"), true);
    assert_eq!(s.data(), "Lo");
    assert_eq!(s.consume_front_insensitive("loworld"), false);
    assert_eq!(s.data(), "Lo");
    assert_eq!(s.consume_front_insensitive("ol"), false);
    assert_eq!(s.data(), "Lo");
    assert_eq!(s.consume_front_insensitive("lo"), true);
    assert_eq!(s.data(), "");
    assert_eq!(s.consume_front_insensitive("o"), false);
    assert_eq!(s.consume_front_insensitive(""), true);
  }

  #[test]
  fn test_ends_with() {
    let s = StringRef::new_from_string("hello");
    assert_eq!(s.ends_with(""), true);
    assert_eq!(s.ends_with("lo"), true);
    assert_eq!(s.ends_with("helloworld"), false);
    assert_eq!(s.ends_with("worldhello"), false);
    assert_eq!(s.ends_with("so"), false);
  }

  #[test]
  fn test_ends_with_insensitive() {
    let s = StringRef::new_from_string("heLLo");
    assert_eq!(s.ends_with_insensitive(""), true);
    assert_eq!(s.ends_with_insensitive("lo"), true);
    assert_eq!(s.ends_with_insensitive("LO"), true);
    assert_eq!(s.ends_with_insensitive("ELlo"), true);
    assert_eq!(s.ends_with_insensitive("helloworld"), false);
    assert_eq!(s.ends_with_insensitive("hi"), false);
  }

  #[test]
  fn test_consume_back() {
    let mut s = StringRef::new_from_string("hello");
    assert_eq!(s.consume_back(""), true);
    assert_eq!(s.data(), "hello");
    assert_eq!(s.consume_back("lo"), true);
    assert_eq!(s.data(), "hel");
    assert_eq!(s.consume_back("helhel"), false);
    assert_eq!(s.data(), "hel");
    assert_eq!(s.consume_back("hle"), false);
    assert_eq!(s.data(), "hel");
    assert_eq!(s.consume_back("hel"), true);
    assert_eq!(s.data(), "");
    assert_eq!(s.consume_back("h"), false);
    assert_eq!(s.data(), "");
  }

  #[test]
  fn test_consume_back_insensitive() {
    let mut s = StringRef::new_from_string("heLLo");
    assert_eq!(s.consume_back_insensitive(""), true);
    assert_eq!(s.data(), "heLLo");
    assert_eq!(s.consume_back("lO"), false);
    assert_eq!(s.data(), "heLLo");
    assert_eq!(s.consume_back_insensitive("lO"), true);
    assert_eq!(s.data(), "heL");
    assert_eq!(s.consume_back_insensitive("helhel"), false);
    assert_eq!(s.data(), "heL");
    assert_eq!(s.consume_back_insensitive("hle"), false);
    assert_eq!(s.data(), "heL");
    assert_eq!(s.consume_back_insensitive("hEl"), true);
    assert_eq!(s.data(), "");
    assert_eq!(s.consume_back_insensitive("h"), false);
    assert_eq!(s.consume_back_insensitive(""), true);
  }

  #[test]
  fn test_find() {
    let s = StringRef::new_from_string("helloHELLO");
    let long_s =
      StringRef::new_from_string("hellx xello hell ello world foo bar hello HELLO");

    #[derive(Debug, PartialEq)]
    struct CharExpectation {
      s_: StringRef,
      c_: char,
      from_: usize,
      pos_: usize,
      insensitive_pos_: usize
    }
    impl CharExpectation {
      pub fn new(s: StringRef, c: char, from: usize, pos: usize, insensitive_pos: usize) -> Self {
        Self { s_: s, c_: c, from_: from, pos_: pos, insensitive_pos_: insensitive_pos }
      }
    }
    let mut char_expectations: Vec<CharExpectation> = vec![
      CharExpectation::new(s.clone(), 'h', 0, 0, 0),
      CharExpectation::new(s.clone(), 'e', 0, 1, 1),
      CharExpectation::new(s.clone(), 'l', 0, 2, 2),
      CharExpectation::new(s.clone(), 'l', 3, 3, 3),
      CharExpectation::new(s.clone(), 'o', 0, 4, 4),
      CharExpectation::new(s.clone(), 'L', 0, 7, 2),
      CharExpectation::new(s.clone(), 'z', 0, NPOS, NPOS)
    ];
    let mut iter = char_expectations.iter_mut();
    loop {
      let c_expect = iter.next();
      if c_expect == None {
        break;
      }
      let val = c_expect.unwrap();
      assert_eq!(val.pos_, val.s_.find(val.c_, val.from_));
      assert_eq!(val.insensitive_pos_, val.s_.find_insensitive(val.c_, val.from_));
      assert_eq!(val.insensitive_pos_, val.s_.find_insensitive(val.c_.to_ascii_uppercase(), val.from_));
    }

    #[derive(Debug, PartialEq)]
    struct StrExpectation {
      s_: StringRef,
      str_: StringRef,
      from_: usize,
      pos_: usize,
      insensitive_pos_: usize
    }
    impl StrExpectation {
      pub fn new(s: StringRef, str: StringRef, from: usize, pos: usize, insensitive_pos: usize) -> Self {
        Self { s_: s, str_: str, from_: from, pos_: pos, insensitive_pos_: insensitive_pos }
      }
    }
    let mut str_expectations: Vec<StrExpectation> = vec![
      StrExpectation::new(s.clone(), StringRef::new_from_string("helloworld"), 0, NPOS, NPOS),
      StrExpectation::new(s.clone(), StringRef::new_from_string("hello"), 0, 0, 0),
      StrExpectation::new(s.clone(), StringRef::new_from_string("ello"), 0, 1, 1),
      StrExpectation::new(s.clone(), StringRef::new_from_string("zz"), 0, NPOS, NPOS),
      StrExpectation::new(s.clone(), StringRef::new_from_string("ll"), 2, 2, 2),
      StrExpectation::new(s.clone(), StringRef::new_from_string("ll"), 3, NPOS, 7),
      StrExpectation::new(s.clone(), StringRef::new_from_string("LL"), 2, 7, 2),
      StrExpectation::new(s.clone(), StringRef::new_from_string("LL"), 3, 7, 7),
      StrExpectation::new(s.clone(), StringRef::new_from_string(""), 0, 0, 0),
      StrExpectation::new(long_s.clone(), StringRef::new_from_string("hello"), 0, 36, 36),
      StrExpectation::new(long_s.clone(), StringRef::new_from_string("foo"), 0, 28, 28),
      StrExpectation::new(long_s.clone(), StringRef::new_from_string("hell"), 2, 12, 12),
      StrExpectation::new(long_s.clone(), StringRef::new_from_string("HELL"), 2, 42, 12),
      StrExpectation::new(long_s.clone(), StringRef::new_from_string(""), 0, 0, 0)
    ];
    let mut iter_str = str_expectations.iter_mut();
    loop {
      let str_expect = iter_str.next();
      if str_expect == None {
        break;
      }
      let val = str_expect.unwrap();
      assert_eq!(val.pos_, val.s_.find_str(val.str_.clone(), val.from_));
      assert_eq!(val.insensitive_pos_, val.s_.find_str_insensitive(val.str_.clone(), val.from_));
      let str_upper = val.str_.data().to_ascii_uppercase();
      let str_upper_ref = StringRef::new_from_string(str_upper.as_str());
      assert_eq!(val.insensitive_pos_, val.s_.find_str_insensitive(str_upper_ref, val.from_));
    }

    assert_eq!(s.rfind('l', NPOS), 3);
    assert_eq!(s.rfind('z', NPOS), NPOS);
    assert_eq!(s.rfind_str(StringRef::new_from_string("helloworld")), NPOS);
    assert_eq!(s.rfind_str(StringRef::new_from_string("hello")), 0);
    assert_eq!(s.rfind_str(StringRef::new_from_string("ello")), 1);
    assert_eq!(s.rfind_str(StringRef::new_from_string("zz")), NPOS);

    assert_eq!(s.rfind_insensitive('l', NPOS), 8);
    assert_eq!(s.rfind_insensitive('L', NPOS), 8);
    assert_eq!(s.rfind_insensitive('z', NPOS), NPOS);
    assert_eq!(s.rfind_str_insensitive(StringRef::new_from_string("HELLOWORLD")), NPOS);
    assert_eq!(s.rfind_str(StringRef::new_from_string("HELLO")), 5);
    assert_eq!(s.rfind_str(StringRef::new_from_string("ELLO")), 6);
    assert_eq!(s.rfind_str(StringRef::new_from_string("ZZ")), NPOS);

    assert_eq!(s.find_first_of('l', 0), 2);
    assert_eq!(s.find_first_of_str(StringRef::new_from_string("el"), 0), 1);
    assert_eq!(s.find_first_of_str(StringRef::new_from_string("xyz"), 0), NPOS);

    let s1 = StringRef::new_from_string("hello");
    assert_eq!(s1.find_first_not_of('h', 0), 1);
    assert_eq!(s1.find_first_not_of_str(StringRef::new_from_string("hel"), 0), 4);
    assert_eq!(s1.find_first_not_of_str(StringRef::new_from_string("hello"), 0), NPOS);

    assert_eq!(s1.find_last_not_of('o', NPOS), 3);
    assert_eq!(s1.find_last_not_of_str(StringRef::new_from_string("lo"), NPOS), 1);
    assert_eq!(s1.find_last_not_of_str(StringRef::new_from_string("helo"), NPOS), NPOS);
  }

  #[test]
  fn test_count() {
    let s = StringRef::new_from_string("hello");
    assert_eq!(s.count_char('l'), 2);
    assert_eq!(s.count_char('o'), 1);
    assert_eq!(s.count_char('z'), 0);
    assert_eq!(s.count_char('l'), 2);

    assert_eq!(s.count_str("helloworld"), 0);
    assert_eq!(s.count_str("hello"), 1);
    assert_eq!(s.count_str("ello"), 1);
    assert_eq!(s.count_str("zz"), 0);
    assert_eq!(s.count_str(""), 0);

    let overlapp = StringRef::new_from_string("abbabba");
    assert_eq!(overlapp.count_str("abba"), 1);
    let no_overlapp = StringRef::new_from_string("abbaabba");
    assert_eq!(no_overlapp.count_str("abba"), 2);
    let complex = StringRef::new_from_string("abbabbaxyzabbaxyz");
    assert_eq!(complex.count_str("abba"), 2);
  }

  //#[test]
  fn test_edit_distance() {}

  //#[test]
  fn test_edit_distance_insensitive() {}

  #[test]
  fn test_drop() {
    let s = StringRef::new_from_string("StringRefTest::Drop");
    assert_eq!(s.drop_front(5), StringRef::new_from_string("gRefTest::Drop"));
    assert_eq!(s.drop_back(5), StringRef::new_from_string("StringRefTest:"));
    assert_eq!(s.drop_front(0), s);
    assert_eq!(s.drop_back(0), s);
    
    let mut dropped = s.drop_front(s.size());
    assert_eq!(dropped.empty(), true);
    dropped = s.drop_back(s.size());
    assert_eq!(dropped.empty(), true);
  }

  #[test]
  fn test_take() {
    let s = StringRef::new_from_string("StringRefTest::Take");
    assert_eq!(s.take_front(5), StringRef::new_from_string("Strin"));
    assert_eq!(s.take_back(5), StringRef::new_from_string(":Take"));
    assert_eq!(s.take_front(s.size()), s);
    assert_eq!(s.take_back(s.size()), s);

    let mut taken = s.take_front(0);
    assert_eq!(taken.empty(), true);
    taken = s.take_back(0);
    assert_eq!(taken.empty(), true);
  }

  //#[test]
  fn test_find_if() {
    //let punct = StringRef::new_from_string("Test.String");
    //let no_punct = StringRef::new_from_string("ABCDEFG");
  }

  //#[test]
  fn test_take_while_until() {

  }

  #[test]
  fn test_lf_line_editing() {
    let s1 = StringRef::new_from_string("\nDoggo\nPupper");
    let s2 = StringRef::new_from_string("Floofer\n");
    let s3 = StringRef::new_from_string("Woofer");

    assert_eq!(s1.detect_eol(), StringRef::new_from_string("\n"));
    assert_eq!(s2.detect_eol(), StringRef::new_from_string("\n"));
    assert_eq!(s3.detect_eol(), StringRef::new_from_string("\n"));
  }

  #[test]
  fn test_cr_line_editing() {
    let s1 = StringRef::new_from_string("\rDoggo\rPupper");
    let s2 = StringRef::new_from_string("Floofer\r");
    let s3 = StringRef::new_from_string("Woo\rfer\n");

    assert_eq!(s1.detect_eol(), StringRef::new_from_string("\r"));
    assert_eq!(s2.detect_eol(), StringRef::new_from_string("\r"));
    assert_eq!(s3.detect_eol(), StringRef::new_from_string("\r"));
  }

  #[test]
  fn test_crlf_line_editing() {
    let s1 = StringRef::new_from_string("\r\nDoggo\r\nPupper");
    let s2 = StringRef::new_from_string("Floofer\r\n");
    let s3 = StringRef::new_from_string("Woofer\r\nSubWoofer\n");

    assert_eq!(s1.detect_eol(), StringRef::new_from_string("\r\n"));
    assert_eq!(s2.detect_eol(), StringRef::new_from_string("\r\n"));
    assert_eq!(s3.detect_eol(), StringRef::new_from_string("\r\n"));
  }

  #[test]
  fn test_lfcr_line_editing() {
    let s1 = StringRef::new_from_string("\n\rDoggo\n\rPupper");
    let s2 = StringRef::new_from_string("Floofer\n\r");
    let s3 = StringRef::new_from_string("Woofer\n\rSubWoofer\n");

    assert_eq!(s1.detect_eol(), StringRef::new_from_string("\n\r"));
    assert_eq!(s2.detect_eol(), StringRef::new_from_string("\n\r"));
    assert_eq!(s3.detect_eol(), StringRef::new_from_string("\n\r"));
  }
}