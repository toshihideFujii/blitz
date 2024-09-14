use std::collections::HashMap;

pub fn parse_blitz_backend_extra_options(
  extra_options_map: &mut HashMap<String, String>,
  comma_separated_values: String)
{
  let extra_options_parts: Vec<&str> = comma_separated_values.split(',').collect();

  // The flag contains a comma-separated list of options; some options
  // have arguments following "=", some don't.
  for part in extra_options_parts {
    let eq_ops = part.find('=');
    if eq_ops.is_none() {
      extra_options_map.insert(part.to_string(), "".to_string());
    } else {
      let mut value = "".to_string();
      if eq_ops.unwrap() < part.len() {
        value = part.split_at(eq_ops.unwrap()+1).1.to_string();
      }
      extra_options_map.insert(part.split_at(eq_ops.unwrap()).0.to_string(), value);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_blitz_backend_extra_options() {
    let mut test_map: HashMap<String, String> = HashMap::new();
    let test_string = "aa=bb,cc,dd=,ee=ff=gg".to_string();

    parse_blitz_backend_extra_options(
      &mut test_map, test_string);
    
    assert_eq!(test_map.len(), 4);
    assert_eq!(test_map.get("aa"), Some(&"bb".to_string()));
    assert_eq!(test_map.get("cc"), Some(&"".to_string()));
    assert_eq!(test_map.get("dd"), Some(&"".to_string()));
    assert_eq!(test_map.get("ee"), Some(&"ff=gg".to_string()));
  }
}