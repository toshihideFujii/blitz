pub trait Printer {
  fn append(&mut self, _string: &String);
}

// A printer implementation that accumulates printed strings into `std::string`.
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

pub fn append_join<T>(printer: &mut dyn Printer, vec: &Vec<T>, separator: String)
  where T: ToString
{
  if vec.is_empty() { return; }
  let mut counter = 0;
  for value in vec {
    counter += 1;
    printer.append(&value.to_string());
    if counter == vec.len() { break; }
    printer.append(&separator);
  }
}

// Utility function that appends multiple elements to a Printer as if by calling
// printer->Append(absl::StrCat(...)), but does it in-place.
pub fn append_cat_ab(printer: &mut dyn Printer, a: i64, b: i64) {
  printer.append(&a.to_string());
  printer.append(&b.to_string());
}

pub fn append_cat_abc(printer: &mut dyn Printer, a: i64, b: i64, c: u64) {
  printer.append(&a.to_string());
  printer.append(&b.to_string());
  printer.append(&c.to_string());
}

pub fn append_cat_abcd(printer: &mut dyn Printer, a: i64, b: i64, c: u64, d: i64) {
  printer.append(&a.to_string());
  printer.append(&b.to_string());
  printer.append(&c.to_string());
  printer.append(&d.to_string());
}