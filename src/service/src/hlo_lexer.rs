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
  FileLocations,
  FileNames,
  FunctionNames,
  StackFrames,
  True,
  False,
  Maximal,
  Replicated,
  Manual,
  LastTileDimReplicate,
  ShardAs,
  ShardLike,
  Unknown,
  Unreduced,
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

impl TokKind {
  pub fn to_string(&self) -> String {
    match self {
      TokKind::Eof => return "Eof".to_string(),
      TokKind::Error => return "Error".to_string(),
      TokKind::Equal => return "Equal".to_string(),
      TokKind::Comma => return "Comma".to_string(),
      TokKind::Colon => return "Colon".to_string(),
      TokKind::Asterisk => return "Asterisk".to_string(),
      TokKind::QuestionMark => return "QuestionMark".to_string(),
      TokKind::Octothorp => return "Octothorp".to_string(),
      TokKind::Plus => return "Plus".to_string(),
      TokKind::Tilde => return "Tilde".to_string(),
      TokKind::Lsquare => return "Lsquare".to_string(),
      TokKind::Rsquare => return "Rsquare".to_string(),
      TokKind::Lbrace => return "Lbrace".to_string(),
      TokKind::Rbrace => return "Rbrace".to_string(),
      TokKind::Lparen => return "Lparen".to_string(),
      TokKind::Rparen => return "Rparen".to_string(),
      TokKind::Dots => return "Dots".to_string(),
      TokKind::Arrow => return "Arrow".to_string(),
      TokKind::Leq => return "Leq".to_string(),
      TokKind::HloModule => return "HloModule".to_string(),
      TokKind::Entry => return "Entry".to_string(),
      TokKind::Root => return "Root".to_string(),
      TokKind::FileLocations => return "FileLocations".to_string(),
      TokKind::FileNames => return "FileNames".to_string(),
      TokKind:: FunctionNames => return "FunctionNames".to_string(),
      TokKind::StackFrames => return "StackFrames".to_string(),
      TokKind::True => return "True".to_string(),
      TokKind::False => return "false".to_string(),
      TokKind::Maximal => return "Maximal".to_string(),
      TokKind::Replicated => return "Replicated".to_string(),
      TokKind::Manual => return "Manual".to_string(),
      TokKind::LastTileDimReplicate => return "LastTileDimReplicate".to_string(),
      TokKind::ShardAs => return "shardAs".to_string(),
      TokKind::ShardLike => return "ShardLike".to_string(),
      TokKind::Unknown => return "Unknown".to_string(),
      TokKind::Unreduced => return "Unreduced".to_string(),
      TokKind::Inf => return "Inf".to_string(),
      TokKind::NegInf => return "NegInf".to_string(),
      TokKind::PrimitiveType => return "PrimitiveType".to_string(),
      TokKind::Name => return "Name".to_string(),
      TokKind::AttributeName => return "AttributeName".to_string(),
      TokKind::DimLabels => return "DimLabels".to_string(),
      TokKind::DxD => return "DxD".to_string(),
      TokKind::Pad => return "Pad".to_string(),
      TokKind::Ident => return "Ident".to_string(),
      TokKind::String => return "String".to_string(),
      TokKind::Int => return "Int".to_string(),
      TokKind::Decimal => return "Decimal".to_string()
    }
  }
}

const EOF: i64 = -1;
const ERROR: i64 = -2;

pub fn tok_kind_to_string(_kind: &TokKind) -> String {
  unimplemented!()
}

fn tok_kind_to_int(kind: &TokKind) -> i64 {
  match kind {
    TokKind::Eof => return 0,
    TokKind::Error => return 1,
    TokKind::Equal => return 2,
    TokKind::Comma => return 3,
    TokKind::Colon => return 4,
    TokKind::Asterisk => return 5,
    TokKind::QuestionMark => return 6,
    TokKind::Octothorp => return 7,
    TokKind::Plus => return 8,
    TokKind::Tilde => return 9,
    TokKind::Lsquare => return 10,
    TokKind::Rsquare => return 11,
    TokKind::Lbrace => return 12,
    TokKind::Rbrace => return 13,
    TokKind::Lparen => return 14,
    TokKind::Rparen => return 15,
    TokKind::Dots => return 16,
    TokKind::Arrow => return 17,
    TokKind::Leq => return 18,
    TokKind::HloModule => return 19,
    TokKind::Entry => return 20,
    TokKind::Root => return 21,
    TokKind::FileLocations => return 22,
    TokKind::FileNames => return 23,
    TokKind:: FunctionNames => return 24,
    TokKind::StackFrames => return 25,
    TokKind::True => return 26,
    TokKind::False => return 27,
    TokKind::Maximal => return 28,
    TokKind::Replicated => return 29,
    TokKind::Manual => return 30,
    TokKind::LastTileDimReplicate => return 31,
    TokKind::ShardAs => return 32,
    TokKind::ShardLike => return 33,
    TokKind::Unknown => return 34,
    TokKind::Unreduced => return 35,
    TokKind::Inf => return 36,
    TokKind::NegInf => return 37,
    TokKind::PrimitiveType => return 38,
    TokKind::Name => return 39,
    TokKind::AttributeName => return 40,
    TokKind::DimLabels => return 41,
    TokKind::DxD => return 42,
    TokKind::Pad => return 43,
    TokKind::Ident => return 44,
    TokKind::String => return 45,
    TokKind::Int => return 46,
    TokKind::Decimal => return 47
  }
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
  pub last_query: Option<usize>,
  pub line_no_of_query: usize,
}

impl LineNoCacheTy {
  pub fn default() -> Self {
    LineNoCacheTy { last_query: None, line_no_of_query: 0 }
  }
}

// Lexer for the HloModule::to_string() format text.
// This class is meant to be used by HloParser. You shouldn't need to use
// it directly.
pub struct HloLexer {
  buf: String,
  current_ptr: usize,
  token_state: TokenState,
  line_no_cache: LineNoCacheTy
}

impl HloLexer {
  pub fn new(buf: String) -> Self {
    HloLexer {
      buf: buf,
      current_ptr: 0,
      token_state: TokenState::default(),
      line_no_cache: LineNoCacheTy::default(),
    }
  }

  pub fn lex(&mut self, skip_mask: i64) -> TokKind {
    self.token_state.current_kind = self.lex_token(skip_mask);
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

  // Returns the line and column of a location in the buffer.
  pub fn get_line_and_column(&mut self, location: usize) -> (usize, usize) {
    let mut line_no = 1;
    let start = 0;
    let mut ptr = 0;
    if self.line_no_cache.last_query.is_some() && 
      self.can_dereference(self.line_no_cache.last_query.unwrap()) &&
      self.line_no_cache.last_query.unwrap() <= location
    {
      ptr = self.line_no_cache.last_query.unwrap();
      line_no = self.line_no_cache.line_no_of_query;
    }
    for i in 0..location {
      debug_assert!(i < self.buf.len());
      if self.buf.chars().nth(i) == Some('\n') {
        line_no += 1;
      }
      ptr = i;
    }
    // Update the line number cache.
    self.line_no_cache.last_query = Some(ptr);
    self.line_no_cache.line_no_of_query = line_no;
    let mut line_offset =
      self.string_from_pointers(start, ptr).rfind('\n');
    if line_offset.is_none() {
      line_offset = Some(0);
    }
    (line_no, ptr - start - line_offset.unwrap())
  }

  // Returns the whole line given the location.
  pub fn get_line(&self, loc: usize) -> String {
    if !self.can_dereference(loc) {
      return "LINE OUT OF RANGE".to_string();
    }
    let line_start = self.string_from_pointers(
      0, loc + 1).rfind('\n');
    let mut start = 0;
    if line_start.is_some() {
      start = line_start.unwrap() + 1;
    }
    let line_end = self.string_from_pointers(
      loc, self.buf.len()).find('\n');
    let mut end = self.buf.len();
    if line_end.is_none() {
      end = loc + line_end.unwrap();
    }
    self.string_from_pointers(start, end)
  }

  // Looks ahead one token and returns it. Lexer state is unchanged.
  pub fn look_ahead(&mut self) -> TokKind {
    if self.get_kind() == TokKind::Eof || self.get_kind() == TokKind::Error {
      return self.get_kind();
    }
    let old_current_ptr = self.current_ptr;
    let old_token_state = self.token_state.clone();
    self.lex(0);
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
    assert!(self.can_dereference(end - 1));
    self.buf[begin..end].to_string()
  }

  // Returns true if the given ptr is dereferenceable within the range of the
  // current buffer.
  fn can_dereference(&self, ptr: usize) -> bool {
    self.buf.char_indices().nth(ptr) != None
  }

  fn lex_token(&mut self, skip_mask: i64) -> TokKind {
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
        '0' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '1' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '2' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '3' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '4' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '5' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '6' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '7' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '8' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '9' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
        '-' => return self.lex_number_or_pattern_wrap(&current_char, skip_mask),
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
        _ => {
          // [a-zA-Z_]
          if current_char.as_ref().unwrap().is_alphabetic() ||
            current_char.as_ref().unwrap() == &'_'
          {
            return self.lex_identifier();
          }
          return TokKind::Error
        }
      }
    }
  }

  // Lex a shape, name, keyword, attribute name, the dim labels pattern, and
  // other identifiers.
  //
  // shape    ::= ([a-zA-Z0-9_]*[0-9]*)\[([0-9,]*)\](?:\s*{([0-9,]*)})?
  // name     ::= [a-zA-Z_][a-zA-Z0-9_.-]*:
  // keyword  ::= HloModule, ENTRY, ...
  // attribute_name ::= condition, body, dimensions, ...
  // dim_labels_pattern ::= [0-9bf?]{2,}_[0-9io?]{2,}->[0-9bf?]{2,}
  // identifiers ::= other cases that match [a-zA-Z_][a-zA-Z0-9_.-]*
  fn lex_identifier(&mut self) -> TokKind {
    while self.peek_current_char().is_some() &&
      is_identifier_char(self.peek_current_char().as_ref().unwrap())
    {
      self.current_ptr += 1;
    }

    // If followed by ':', it's a name.
    if self.peek_current_char().is_some() &&
      self.peek_current_char().unwrap() == ':'
    {
      let str = self.token_state.str_val.as_str();
      let str_val =
        String::from(&str[self.token_state.token_start..self.current_ptr]);
      self.token_state.str_val = str_val;
      self.current_ptr += 1; // skip ':'
      return TokKind::Name;
    }

    // If followed by '=', it's a attribute name.
    if self.peek_current_char().is_some() &&
      self.peek_current_char().unwrap() == '='
    {
      //let str = self.token_state.str_val.as_str();
      //let str_val =
        //String::from(&str[self.token_state.token_start..self.current_ptr]);
      let str_val = String::from(&self.buf[self.token_state.token_start..self.current_ptr]);
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
        payload = self.lex_nan_payload(&consumable);
        if payload.is_none() {
          return TokKind::Error;
        }
      }
      self.token_state.decimal_val = 
        nan_with_sign_and_payload::<f64>(false, payload.unwrap() as u64);
      return TokKind::Decimal;
    }

    // See if this is a keyword.
    if identifier.as_str() == "HloModule" {
      return TokKind::HloModule;
    }
    if identifier.as_str() == "ENTRY" {
      return TokKind::Entry;
    }
    if identifier.as_str() == "ROOT" {
      return TokKind::Root;
    }
    if identifier.as_str() == "FileLocations" {
      return TokKind::FileLocations;
    }
    if identifier.as_str() == "FileNames" {
      return TokKind::FileNames;
    }
    if identifier.as_str() == "FunctionNames" {
      return TokKind::FunctionNames;
    }
    if identifier.as_str() == "StackFrames" {
      return TokKind::StackFrames;
    }
    if identifier.as_str() == "maximal" {
      return TokKind::Maximal;
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
      while is_identifier_char(self.peek_current_char().as_ref().unwrap()) {
        self.current_ptr += 1;
      }
      let str = self.token_state.str_val.as_str();
      let str_val = String::from(&str[name_start..self.current_ptr]);
      self.token_state.str_val = str_val;
      return TokKind::Name;
    }
    TokKind::Error
  }

  fn lex_shape(&mut self) -> TokKind {
    unimplemented!()
  }

  fn lex_constant(&mut self) -> TokKind {
    unimplemented!()
  }

  // Lex integer and floating-point values, -inf, and patterns for dim labels,
  // dxd (e.g. 1x2x3), and pad.
  //
  // fp with exp ::= [-]?([0-9]+|[0-9]+[.][0-9]*|[0-9]*[.][0-9]+)([eE][+-]?[0-9]+)
  // fp without exp ::= [-]?([0-9]+[.][0-9]*|[0-9]*[.][0-9]+)
  // dim_labels_pattern ::= [0-9bf?]{2,}_[0-9io?]{2,}->[0-9bf?]{2,}
  // dxd_pattern ::= [0-9]+(x[0-9]+)+
  // pad_pattern ::=
  //   [-]?[0-9]+_[-]?[0-9]+(_[0-9]+)?(x[-]?[0-9]+_[-]?[0-9]+(_[0-9]+)?)*
  // int ::=  [-]?[0-9]+
  // negative inf ::= '-inf'
  fn lex_number_or_pattern(&mut self, skip_mask: i64) -> TokKind {
    let consumable = self.string_from_pointers(
      self.token_state.token_start, self.buf.len());

    if (skip_mask & (1 << tok_kind_to_int(&TokKind::Decimal))) != 0 &&
      consume_float_pattern(&consumable)
    {
      self.current_ptr = consumable.len();
      return TokKind::Decimal;
    }
    if (skip_mask & (1 << tok_kind_to_int(&TokKind::DimLabels))) != 0 &&
      consume_dim_labels_pattern(&consumable)
    {
      self.current_ptr = consumable.len();
      return TokKind::DimLabels;
    }
    if (skip_mask & (1 << tok_kind_to_int(&TokKind::DxD))) != 0 &&
      consume_dxd_pattern(&consumable)
    {
      self.current_ptr = consumable.len();
      return TokKind::DxD;
    }
    if (skip_mask & (1 << tok_kind_to_int(&TokKind::Pad))) != 0 &&
      consume_pad_pattern(&consumable) {
      self.current_ptr = consumable.len();
      return TokKind::Pad;
    }
    if self.lex_i64() != TokKind::Error {
      return TokKind::Int;
    }
    if consume_negative_inf_pattern(&consumable) {
      self.current_ptr = consumable.len();
      return TokKind::NegInf;
    }
    if consume_negative_nan_pattern(&consumable) {
      self.current_ptr = consumable.len();
      let mut payload = None;
      if self.peek_current_char() == Some('(') {
        payload = self.lex_nan_payload(&consumable);
        if payload.is_none() {
          return TokKind::Error;
        }
      }
      self.token_state.decimal_val = nan_with_sign_and_payload::<f64>(
        false, payload.unwrap() as u64);
      return TokKind::Decimal;
    }
    TokKind::Error
  }

  fn lex_number_or_pattern_wrap(
    &mut self, current_char: &Option<char>, skip_mask: i64) -> TokKind
  {
    if current_char.unwrap() == '-' && self.peek_current_char() == Some('>') {
      self.current_ptr += 1;
      return TokKind::Arrow;
    }
    let tmp = self.lex_number_or_pattern(skip_mask);
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

  fn lex_i64(&mut self) -> TokKind {
    // This effectively matches the RE2 pattern R"([-]?\d+)".
    let mut pos = self.token_state.token_start;
    let mut pos_char = self.buf.chars().nth(pos);
    let end = self.buf.len();
    let mut has_digits = false;
    let mut i64_val = 0;

    if pos_char == Some('-') {
      pos += 1;
      // Lexing negative integer:
      while pos < end {
        pos_char = self.buf.chars().nth(pos);
        if !pos_char.unwrap().is_ascii_digit() {
          break;
        }
        has_digits = true;
        if i64_val < i64::MIN / 10 {
          return TokKind::Error;
        }
        let c = pos_char.unwrap().to_digit(10).unwrap() as i64;
        i64_val *= 10;
        if i64_val < i64::MIN + c {
          return TokKind::Error;
        }
        i64_val -= c;
        pos += 1;
      }
    } else {
      let mut u64_val = 0;
      // Lexing non-negative integer:
      while pos < end {
        pos_char = self.buf.chars().nth(pos);
        if !pos_char.unwrap().is_ascii_digit() {
          break;
        }
        has_digits = true;
        if u64_val > u64::MAX / 10 {
          return TokKind::Error;
        }
        let c = pos_char.unwrap().to_digit(10).unwrap() as u64;
        u64_val *= 10;
        if u64_val > u64::MAX - c {
          return TokKind::Error;
        }
        u64_val += c;
        pos += 1;
      }
      i64_val = u64_val as i64;
    }
    if !has_digits {
      return TokKind::Error;
    }

    self.token_state.i64_val = i64_val;
    self.current_ptr = pos;
    return TokKind::Int;
  }

  fn lex_nan_payload(&mut self, _consumable: &String) -> Option<i64> { Some(1) }
}

// [a-zA-Z0-9_.-]
fn is_identifier_char(c: &char) -> bool {
  c.is_ascii_alphanumeric() || c == &'-' || c == &'.' || c == &'_'
}

fn consume_float_pattern(consumable: &String) -> bool {
  let float_pattern = Regex::new(
    r"([-]?((\d+|\d+[.]\d*|\d*[.]\d+)([eE][+-]?\d+))|[-]?(\d+[.]\d*|\d*[.]\d+))").unwrap();
  float_pattern.is_match(&consumable)
}

fn consume_dim_labels_pattern(consumable: &String) -> bool {
  let dim_labels_pattern = Regex::new(
    r"([0-9bf?]{2,}_[0-9io?]{2,}->[0-9bf?]{2,})").unwrap();
  dim_labels_pattern.is_match(&consumable)
}

fn consume_dxd_pattern(consumable: &String) -> bool {
  let dxd_pattern = Regex::new(r"([0-9]+(x[0-9]+)+)").unwrap();
  dxd_pattern.is_match(&consumable)
}

fn consume_pad_pattern(consumable: &String) -> bool {
  let pad_pattern = Regex::new(
    r"([-]?[0-9]+_[-]?[0-9]+(_[0-9]+)?(x[-]?[0-9]+_[-]?[0-9]+(_[0-9]+)?)*)").unwrap();
  pad_pattern.is_match(&consumable)
}

fn consume_int_pattern(consumable: &String) -> bool {
  let int_pattern = Regex::new(r"([-]?\d+)").unwrap();
  int_pattern.is_match(&consumable)
}

fn consume_negative_inf_pattern(consumable: &String) -> bool {
  let neg_inf = Regex::new(r"-inf").unwrap();
  neg_inf.is_match(&consumable)
}

fn consume_negative_nan_pattern(consumable: &String) -> bool {
  let neg_nan = Regex::new(r"-nan").unwrap();
  neg_nan.is_match(&consumable)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_non_negative_integer_corner_cases() {
    let values: Vec<i64> = vec![
      0, 1, 0x7fffffff, 0x80000000, 0xffffffff, 0x100000000,
      0x7fffffffffffffff, /* 0x8000000000000000, 0xffffffffffffffff */];
    let mut input = String::new();
    for v in &values {
      input.push_str(&v.to_string());
      if v != values.last().unwrap() { input.push_str(","); }
    }
    let mut lexer = HloLexer::new(input);
    let mut first_value = true;
    for v in &values {
      if first_value {
        first_value = false;
      } else {
        assert_eq!(lexer.lex(0), TokKind::Comma);
      }
      assert_eq!(lexer.lex(0), TokKind::Int);
      assert_eq!(lexer.get_i64_val(), *v);
      println!("Input value was: {:?}", *v);
    }
    assert_eq!(lexer.lex(0), TokKind::Eof);
  }

  #[test]
  fn test_negative_integer_corner_cases() {
    let values = vec![-1, -2, i32::MIN as i64, i64::MIN+1, i64::MIN];
    let mut input = String::new();
    for v in &values {
      input.push_str(&v.to_string());
      if v != values.last().unwrap() { input.push_str(","); }
    }
    let mut lexer = HloLexer::new(input);
    let mut first_value = true;
    for v in &values {
      if first_value {
        first_value = false;
      } else {
        assert_eq!(lexer.lex(0), TokKind::Comma);
      }
      assert_eq!(lexer.lex(0), TokKind::Int);
      assert_eq!(lexer.get_i64_val(), *v);
      println!("Input value was: {:?}", *v);
    }
    assert_eq!(lexer.lex(0), TokKind::Eof);
  }

  #[test]
  fn test_non_negative_integer_overflow_1_error() {
    // 1 more than the maximum uint64_t.
    let mut lexer =
      HloLexer::new("18446744073709551616".to_string()); // 2^64.
    assert_eq!(lexer.lex(0), TokKind::Error);
  }

  #[test]
  fn test_non_negative_integer_overflow_2_error() {
    // No overflow at 20 digits, but overflow at 21 digits.
    let mut lexer =
      HloLexer::new("184467440737095516150".to_string()); // 10 * (2^64 - 1)
    assert_eq!(lexer.lex(0), TokKind::Error);
  }

  #[test]
  fn test_negative_integer_underflow_1_error() {
    // 1 less than the minimum int64_t.
    let mut lexer =
      HloLexer::new("-9223372036854775809".to_string()); // -10 * 2^63-1.
    assert_eq!(lexer.lex(0), TokKind::Error);
  }

  #[test]
  fn test_negative_integer_underflow_2_error() {
    // No underflow at 20 digits, but underflow at 21 digits.
    let mut lexer =
      HloLexer::new("-92233720368547758080".to_string()); // -10 * 2^63.
    assert_eq!(lexer.lex(0), TokKind::Error);
  }

  #[test]
  fn test_negative_but_no_digits_error() {
    let mut lexer = HloLexer::new("-,-1".to_string());
    assert_eq!(lexer.lex(0), TokKind::Error);
  }

  #[test]
  fn test_non_negative_number_without_null_terminating_character() {
    let mut input = "123".to_string();
    // Overwriting the NULL character with a digit intentionally.
    input.push('4');
    let mut lexer = HloLexer::new(input);
    assert_eq!(lexer.lex(0), TokKind::Int);
    // Lexed value should not be 1234. Should not result in a crash.
    // (↑ Not correct in this program. Lexed value should be 1234.)
    assert_eq!(lexer.get_i64_val(), 1234);
  }

  #[test]
  fn test_negative_number_without_null_terminating_character() {
    let mut input = "-123".to_string();
    // Overwriting the NULL character with a digit intentionally.
    input.push('4');
    let mut lexer = HloLexer::new(input);
    assert_eq!(lexer.lex(0), TokKind::Int);
    // Lexed value should not be -1234. Should not result in a crash.
    // (↑ Not correct in this program. Lexed value should be -1234.)
    assert_eq!(lexer.get_i64_val(), -1234);
  }

  #[test]
  fn test_file_locations_keyword() {
    let mut lexer = HloLexer::new("FileLocations".to_string());
    assert_eq!(lexer.lex(0), TokKind::FileLocations);
  }

  #[test]
  fn test_file_names_keyword() {
    let mut lexer = HloLexer::new("FileNames".to_string());
    assert_eq!(lexer.lex(0), TokKind::FileNames);
  }

  #[test]
  fn test_function_names_keyword() {
    let mut lexer = HloLexer::new("FunctionNames".to_string());
    assert_eq!(lexer.lex(0), TokKind::FunctionNames);
  }

  #[test]
  fn test_stack_frame_keyword() {
    let mut lexer = HloLexer::new("StackFrames".to_string());
    assert_eq!(lexer.lex(0), TokKind::StackFrames);
  }
}