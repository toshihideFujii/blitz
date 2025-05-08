
use crate::blitz_data::PaddingConfig;
pub type DimensionVector = Vec<i64>;

pub fn nan_with_sign_and_payload<T>(_sign: bool, _nan_payload: u64) -> T {
  unimplemented!()
}

pub fn append_status(_prior: i64, _context: String) -> Result<(), String> {
  unimplemented!()
}

pub fn make_no_padding_config(_rank: i64) -> PaddingConfig {
  unimplemented!()
}

pub fn product(xs: &Vec<i64>) -> i64 {
  let mut result = 1;
  for i in xs {
    result = result * i;
  }
  result
}

// Splits the lines of the original, replaces leading whitespace with the prefix
// given by "indentation", and returns the string joined by newlines again. As a
// side effect, any additional trailing whitespace is removed.
//
// Note: even different amounts of leading whitespace on different lines will be
// uniformly replaced with "indentation".
pub fn reindent(_original: &String, _indentation: &String) -> String {
  "".to_string()
}

// Formats the container as a comma-separated string. StrAppend must support
// appending the elements of the container. Prefix is prepended and suffix is
// appended to the returned string.
pub fn comma_separated_string<T>(
  container: &Vec<T>, prefix: &'static str, suffix: &'static str)  -> String
  where T: std::fmt::Display + PartialEq
{
  let mut comma_separated = prefix.to_string();
  for entry in container {
    comma_separated.push_str(&entry.to_string());
    if *container.last().unwrap() != *entry {
      comma_separated.push_str(", ");
    }
  }
  comma_separated.push_str(&suffix);
  comma_separated
}

// Formats the container in the mathematical notation for a vector, e.g. (1, 3,
// 7). StrAppend must support appending the elements of container.
pub fn vector_string<T>(container: &Vec<T>) -> String
  where T: std::fmt::Display + PartialEq
{
  comma_separated_string(container, "(", ")")
}

// Removes illegal characters from filenames.
pub fn sanitize_filename(filename: &'static str) -> String {
  let mut result = filename.to_string();
  result = result.replace("/", "_");
  result = result.replace("\\", "_");
  result = result.replace("[", "_");
  result = result.replace("]", "_");
  result = result.replace(" ", "_");
  result
}

pub fn human_readable_num_ops(
  flops: f64, nanoseconds: f64, op_prefix: String) -> String
{
  if nanoseconds == 0.0 {
    let mut str = "NaN ".to_string();
    str.push_str(&op_prefix);
    str.push_str("OP/s");
    return str;
  }
  let nano_flops = flops / nanoseconds;
  //let mut throughput = ((nano_flops * 1e9) as i64).to_string();
  let mut throughput = nano_flops.to_string();

  // Use the more common "G(FLOPS)", rather than "B(FLOPS)"
  //if throughput.ends_with("B") || throughput.ends_with("b") {
    //throughput.pop();
    //throughput.push_str("G");
  //}
  throughput.push_str("G");

  throughput.push_str(&op_prefix);
  throughput.push_str("OP/s");
  throughput
}

// Given a number of flops executed in an amount of time, produces a string that
// represents the throughput;
// e.g. HumanReadableNumFlops(1e9, 1e9) => 1.00GFLOP/s.
pub fn human_readable_num_flops(flops: f64, nanoseconds: f64) -> String {
  human_readable_num_ops(flops, nanoseconds, "FL".to_string())
}

// Return ceiling(log2(n)) for positive integer n.  panic iff n == 0.
pub fn log_2_ceiling(x: usize) -> u32 {
  x.ilog2()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comma_separated_string() {
    assert_eq!(comma_separated_string::<String>(
      &vec![], "", ""), "".to_string());
    assert_eq!(comma_separated_string::<String>(
      &vec!["hello world".to_string()], "", ""), "hello world".to_string());
    assert_eq!(comma_separated_string::<i64>(
      &vec![1, 57, 2], "foo", "bar"), "foo1, 57, 2bar".to_string());
  }

  #[test]
  fn test_vector_string() {
    let empty_list: Vec<i64> = vec![];
    assert_eq!(vector_string(&empty_list), "()".to_string());
    let float_vector = vec![5.5];
    assert_eq!(vector_string(&float_vector), "(5.5)".to_string());
    let string_vector = vec!["a", "b"];
    assert_eq!(vector_string(&string_vector), "(a, b)".to_string());
    let int_vector = vec![1, 57, 2];
    assert_eq!(vector_string(&int_vector), "(1, 57, 2)".to_string());
  }

  #[test]
  fn test_sanitize_file_name() {
    assert_eq!(sanitize_filename(""), "".to_string());
    assert_eq!(sanitize_filename("abc"), "abc".to_string());
    assert_eq!(sanitize_filename("/\\[]"), "____".to_string());
    assert_eq!(sanitize_filename("/A\\B[C]"), "_A_B_C_".to_string());
  }

  #[test]
  fn test_human_readable_num_flops_example() {
    assert_eq!(human_readable_num_flops(1e9, 1e9), "1GFLOP/s".to_string());
  }
}