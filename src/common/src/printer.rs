#![allow(dead_code)]

pub trait Printer {
  fn append(&mut self, _string: &String);
}

pub struct StringPrinter {
  result: String
}

impl StringPrinter {
  pub fn new() -> Self {
    StringPrinter { result: "".to_string() }
  }

  pub fn to_string(&self) -> String {
    self.result.clone()
  }
}

impl Printer for StringPrinter {
  fn append(&mut self, string: &String) {
    self.result.push_str(string);
  }
}

pub struct CordPrinter {}

pub fn apeend_join(_printer: &mut dyn Printer, _separator: String) {}

pub fn appenf_cat() {}