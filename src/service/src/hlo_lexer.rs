#![allow(dead_code)]

use common::{
  blitz_data::PrimitiveType,
  primitive_util::{is_primitive_type_name, string_to_primitive_type},
  util::nan_with_sign_and_payload
};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokKind {
  // Markers
  Eof,
  Error,
  // Tokens with no info
  Equal,
  Comma,
  Colon,
  Asterisk,
  QuestionMark,
  Octothorp,
  Plus,
  Tilde,
  Lsquare,
  Rsquare,
  Lbrace,
  Rbrace,
  Lparen,
  Rparen,
  Dots,
  Arrow,
  Leq,
  // Keywords
  HloModule,
  Entry,
  Root,
  True,
  False,
  Maximal,
  Replicated,
  Manual,
  LastTileDimReplicate,
  ShardAs,
  ShardLike,
  Unknown,
  Inf,
  NegInf,
  // Typed tokens
  PrimitiveType,
  Name,
  AttributeName,
  DimLabels,
  DxD,
  Pad,
  Ident,
  String,
  Int,
  Decimal,
}

const EOF: i64 = -1;
const ERROR: i64 = -2;

pub fn tok_kind_to_string(_kind: &TokKind) -> String {
  unimplemented!()
}

// Informaton about the current token.
#[derive(Debug, Clone)]
struct TokenState {
  token_start: usize, //Option<char>,
  current_kind: TokKind,
  str_val: String,
  i64_val: i64,
  decimal_val: f64,
  primitive_type_val: PrimitiveType
}

impl TokenState {
  fn default() -> Self {
    TokenState {
      token_start: 0,
      current_kind: TokKind::Eof,
      str_val: "".to_string(),
      i64_val: -1,
      decimal_val: -1.0,
      primitive_type_val: PrimitiveType::Token
    }
  }
}

struct LineNoCacheTy {
  last_query: char,
  line_no_of_query: u64,
}

// Lexer for the HloModule::to_string() format text.
// This class is meant to be used by HloParser. You shouldn't need to use
// it directly.
pub struct HloLexer {
  buf: String,
  current_ptr: usize,
  token_state: TokenState,
}

impl HloLexer {
  pub fn new(buf: String) -> Self {
    HloLexer {
      buf: buf,
      current_ptr: 0,
      token_state: TokenState::default()
    }
  }

  pub fn lex(&mut self) -> TokKind {
    self.token_state.current_kind = self.lex_token();
    self.token_state.current_kind.clone()
  }

  pub fn get_kind(&self) -> TokKind {
    self.token_state.current_kind.clone()
  }

  pub fn get_str_val(&self) -> String {
    match self.get_kind() {
      TokKind::Name => return self.token_state.str_val.clone(),
      TokKind::AttributeName => return self.token_state.str_val.clone(),
      TokKind::DimLabels => return self.token_state.str_val.clone(),
      TokKind::DxD => return self.token_state.str_val.clone(),
      TokKind::Pad => return self.token_state.str_val.clone(),
      TokKind::String => return self.token_state.str_val.clone(),
      TokKind::Ident => return self.token_state.str_val.clone(),
      _ => unreachable!("This token does not have string value.")
    }
  }

  pub fn get_i64_val(&self) -> i64 {
    assert!(self.get_kind() == TokKind::Int);
    self.token_state.i64_val
  }

  pub fn get_decimal_val(&self) -> f64 {
    assert!(self.get_kind() == TokKind::Decimal);
    self.token_state.decimal_val
  }

  pub fn get_primitive_type_val(&self) -> PrimitiveType {
    assert!(self.get_kind() == TokKind::PrimitiveType);
    self.token_state.primitive_type_val.clone()
  }

  // Returns the location of the current token.
  pub fn get_loc(&self) -> usize {
    self.token_state.token_start
  }

  pub fn get_line_and_column() {}
  pub fn get_line() {}

  // Looks ahead one token and returns it. Lexer state is unchanged.
  pub fn look_ahead(&mut self) -> TokKind {
    if self.get_kind() == TokKind::Eof || self.get_kind() == TokKind::Error {
      return self.get_kind();
    }
    let old_current_ptr = self.current_ptr;
    let old_token_state = self.token_state.clone();
    self.lex();
    let kind = self.get_kind();
    self.token_state = old_token_state;
    self.current_ptr = old_current_ptr;
    kind
  }

  // Lexes a string delimited by matching curly braces.
  // Curlies contained inside double quotes don't count.
  pub fn lex_json_dict(&mut self) -> TokKind {
    if self.get_kind() != TokKind::Lbrace {
      return TokKind::Error;
    }
    let orig =
      self.string_from_pointers(self.token_state.token_start, self.buf.len());
    let mut str = orig.clone();
    if str.is_empty() {
      return TokKind::Error;
    }
    let mut object_depth = 0;
    if str.chars().nth(0) != Some('{') {
      return TokKind::Error;
    }
    object_depth += 1;
    str.remove(0);

    while !str.is_empty() {
      if object_depth == 0 { break; }
      if str.chars().nth(0) == Some('"') {
        let string_pattern = Regex::new(r"(([^\\]|\\.)*)").unwrap(); // TODO
        if string_pattern.captures(&str).is_none() {
          return TokKind::Error;
        }
        continue;
      }
      if str.chars().nth(0) == Some('{') { object_depth += 1; }
      if str.chars().nth(0) == Some('}') { object_depth -= 1; }
      str.remove(0);
    }

    if object_depth != 0 {
      return TokKind::Error;
    }
    // TODO: cur_ptr = str.data
    self.token_state.current_kind = TokKind::String;
    let split = orig.split_at(orig.len() - str.len());
    self.token_state.str_val = split.0.to_string();
    TokKind::String
  }

  // Returns the current character. If it's  neither the end of input buffer
  // nor invalid character, moves the pointer forward.
  fn get_next_char(&mut self) -> Option<char> {
    let current_char = self.peek_current_char();
    if current_char != None {
      self.current_ptr += 1;
    }
    current_char
  }

  // Returns the current character.
  fn peek_current_char(&self) -> Option<char> {
    if self.buf.char_indices().nth(self.current_ptr) == None {
      return None;
    }
    Some(self.buf.char_indices().nth(self.current_ptr).unwrap().1)
  }

  // Creates string with the given begin and end.
  fn string_from_pointers(&self, begin: usize, end: usize) -> String {
    assert!(begin <= end);
    assert!(self.can_dereference(begin));
    assert!(self.can_dereference(end));
    self.buf[begin..end].to_string()
  }

  // Returns true if the given ptr is dereferenceable within the range of the
  // current buffer.
  fn can_dereference(&self, ptr: usize) -> bool {
    self.buf.char_indices().nth(ptr) != None
  }

  fn lex_token(&mut self) -> TokKind {
    loop {
      self.token_state.token_start = self.current_ptr;
      let current_char = self.get_next_char();
      if current_char.is_none() {
        return TokKind::Eof;
      }
      match current_char.unwrap() {
        ' ' => continue,
        '\t' => continue,
        '\n' => continue,
        '\r' => continue,
        '0' => return self.lex_number_or_pattern_wrap(&current_char),
        '1' => return self.lex_number_or_pattern_wrap(&current_char),
        '2' => return self.lex_number_or_pattern_wrap(&current_char),
        '3' => return self.lex_number_or_pattern_wrap(&current_char),
        '4' => return self.lex_number_or_pattern_wrap(&current_char),
        '5' => return self.lex_number_or_pattern_wrap(&current_char),
        '6' => return self.lex_number_or_pattern_wrap(&current_char),
        '7' => return self.lex_number_or_pattern_wrap(&current_char),
        '8' => return self.lex_number_or_pattern_wrap(&current_char),
        '9' => return self.lex_number_or_pattern_wrap(&current_char),
        '-' => return self.lex_number_or_pattern_wrap(&current_char),
        '=' => return TokKind::Equal,
        '<' => {
          if current_char.unwrap() == '<' && self.peek_current_char() == Some('=') {
            self.current_ptr += 1;
            return TokKind::Leq;
          }
        },
        ',' => return TokKind::Comma,
        '%' => return self.lex_percent(),
        ':' => return TokKind::Colon,
        '*' => return TokKind::Asterisk,
        '#' => return TokKind::Octothorp,
        '+' => return TokKind::Plus,
        '~' => return TokKind::Tilde,
        '[' => return TokKind::Lsquare,
        ']' => return TokKind::Rsquare,
        '{' => return TokKind::Lbrace,
        '}' => return TokKind::Rbrace,
        '(' => return TokKind::Lparen,
        ')' => return TokKind::Rparen,
        '/' => {
          if self.peek_current_char() == Some('*') {
            // This is the start of a /*..*/ delimited comment.
            let comment_start = self.current_ptr;
            self.current_ptr += 1;
            loop {
              let current = self.get_next_char();
              if current == Some('*') && self.peek_current_char() == Some('/') {
                // End of comment.
                self.current_ptr += 1;
                break;
              }
              if current == None {
                // Unterminated comment.
                self.current_ptr = comment_start;
                return TokKind::Error;
              }
              // TODO: if current == Error
            }
            continue;
          } else if self.peek_current_char() == Some('/') {
            // This is the start of a '//' delimited comment.
            loop {
              let current = self.peek_current_char();
              if current == None || current == Some('\n') || current == Some('\r') {
                break;
              }
              // TODO: if current == Error
              self.current_ptr += 1;
            }
          }
          continue;
        },
        '.' => {
          if self.peek_current_char() == Some('.') {
            self.current_ptr += 1;
            if self.peek_current_char() == Some('.') {
              self.current_ptr += 1;
              return TokKind::Dots;
            }
          }
          return TokKind::Error;
        },
        '"' => return self.lex_string(),
        _ => return TokKind::Error
      }
    }
  }

  // Lex a shape, name, keyword, attribute name, the dim labels pattern, and
  // other identifiers.
  fn lex_identifier(&mut self) -> TokKind {
    while is_identifier_char(self.peek_current_char().unwrap()) {
      self.current_ptr += 1;
    }

    if self.peek_current_char().unwrap() == ':' {
      let str = self.token_state.str_val.as_str();
      let str_val =
        String::from(&str[self.token_state.token_start..self.current_ptr]);
      self.token_state.str_val = str_val;
      self.current_ptr += 1; // skip ':'
      return TokKind::Name;
    }

    if self.peek_current_char().unwrap() == '=' {
      let str = self.token_state.str_val.as_str();
      let str_val =
        String::from(&str[self.token_state.token_start..self.current_ptr]);
      self.token_state.str_val = str_val;
      self.current_ptr += 1; // skip '='
      return TokKind::AttributeName;
    }

    let identifier = self.string_from_pointers(
      self.token_state.token_start, self.current_ptr);

    if is_primitive_type_name(&identifier) {
      let primitive_type = string_to_primitive_type(&identifier);
      if primitive_type.unwrap() != &PrimitiveType::Tuple {
        self.token_state.primitive_type_val = primitive_type.unwrap().clone();
        return TokKind::PrimitiveType;
      }
    }

    if identifier.as_str() == "nan" {
      let mut payload: Option<i64> = None;
      if self.peek_current_char() == Some('(') {
        let consumable =
          self.string_from_pointers(self.current_ptr, self.buf.len());
        payload = self.lex_nan_payload(consumable);
        if payload.is_none() {
          return TokKind::Error;
        }
      }
      self.token_state.decimal_val = 
        nan_with_sign_and_payload::<f64>(false, payload.unwrap() as u64);
      return TokKind::Decimal;
    }

    let consumable =
      self.string_from_pointers(self.token_state.token_start, self.buf.len());
    let dim_labels_pattern =
      Regex::new(r"[0-9bf?]{2,}_[0-9io?]{2,}->[0-9bf?]{2,}").unwrap();
    let captures = dim_labels_pattern.captures(&consumable);
    if captures.is_some() {
      let data = captures.unwrap().get(0).unwrap().as_str().to_string();
      self.token_state.str_val = data;
      return TokKind::DimLabels;
    }

    self.token_state.str_val = identifier;
    TokKind::Ident
  }

  // Lex names after a % character.
  // name ::= [a-zA-Z_][a-zA-Z0-9_.-]
  fn lex_percent(&mut self) -> TokKind {
    let name_start = self.current_ptr;
    let curr_char = self.peek_current_char().unwrap();
    if curr_char.is_ascii_alphabetic() || curr_char == '_' {
      self.current_ptr += 1;
      while is_identifier_char(self.peek_current_char().unwrap()) {
        self.current_ptr += 1;
      }
      let str = self.token_state.str_val.as_str();
      let str_val = String::from(&str[name_start..self.current_ptr]);
      self.token_state.str_val = str_val;
      return TokKind::Name;
    }
    TokKind::Error
  }

  fn lex_shape() {}
  fn lex_constant() {}
  fn lex_number_or_pattern(&mut self) -> TokKind {
    TokKind::Arrow
  }

  fn lex_number_or_pattern_wrap(&mut self, current_char: &Option<char>) -> TokKind {
    if self.peek_current_char() == Some('>') {
      self.current_ptr += 1;
      return TokKind::Arrow;
    }
    let tmp = self.lex_number_or_pattern();
    if tmp == TokKind::Error && current_char == &Some('?') {
      return TokKind::QuestionMark;
    }
    return tmp;
  }

  // Lexes quoted string with escapingcharacters.
  fn lex_string(&mut self) -> TokKind {
    let consumable =
      self.string_from_pointers(self.token_state.token_start, self.buf.len());
    let escaping_pattern = Regex::new(r"(([^\\]|\\.)*)").unwrap(); // TODO
    let data = escaping_pattern.captures(&consumable);
    if data.is_some() {
      // TODO
      return TokKind::String;
    }
    TokKind::Error
  }

  fn lex_nan_payload(&mut self, _consumable: String) -> Option<i64> { Some(1) }
}

// [a-zA-Z0-9_.-]
fn is_identifier_char(c: char) -> bool {
  c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_'
}