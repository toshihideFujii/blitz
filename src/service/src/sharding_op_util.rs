use crate::hlo_lexer::{HloLexer, TokKind};


// Encodes the attributes string for Sharding and auto/manual conversion custom
// ops. This will be used in the opaque field.
pub fn encode_attributes(unspecified_dims: Vec<i64>) -> String {
  if unspecified_dims.is_empty() {
    return  "".to_string();
  }
  let mut result = "unspecified_dims=[".to_string();
  let mut counter = 0;
  for i in &unspecified_dims {
    counter += 1;
    result.push_str(&i.to_string());
    if counter == unspecified_dims.len() { break; }
    result.push_str(",");
  }
  result.push_str("]");
  result
}

// Parses the opaque string of Sharding and auto/manual conversion custom ops.
pub fn parse_attributes(
  opaque: String, unspecified_dims: &mut Vec<i64>) -> Result<(), String>
{
  let mut lexer = HloLexer::new(opaque.clone());
  while lexer.lex(0) != TokKind::Eof {
    if lexer.get_kind() != TokKind::AttributeName {
      let mut err_msg = "Cannot parse sharding op attributes: ".to_string();
      err_msg.push_str(&opaque);
      return Err(err_msg);
    }
    let attr_name = lexer.get_str_val();
    if attr_name == "unspecified_dims".to_string() {
      assert!(lexer.lex(0) == TokKind::Lsquare);
      while lexer.lex(0) == TokKind::Int {
        unspecified_dims.push(lexer.get_i64_val());
        if lexer.lex(0) != TokKind::Comma { break; }
      }
      assert!(lexer.lex(0) == TokKind::Rsquare);
    } else {
      let mut err_msg = "Unknown attribute name in sharding op: ".to_string();
      err_msg.push_str(&attr_name);
      return Err(err_msg);
    }
  }
  Ok(())
}