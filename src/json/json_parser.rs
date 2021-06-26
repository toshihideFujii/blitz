use std::char;

pub enum ParseElementResult {
  ElementFound,
  ElementNotFound,
}

struct JsonString {
  start_: i32,
  index_: u32,
  length_: u32,
  needs_conversion_: bool,
  internalize_: bool,
  has_escape_: bool,
  is_index_: bool,
}

impl JsonString {
  pub fn new() -> JsonString {
    JsonString {
      start_: 0,
      index_: 0,
      length_: 0,
      needs_conversion_: false,
      internalize_: false,
      has_escape_: false,
      is_index_: false,
    }
  }

  pub fn internalize(self) -> bool {
    return self.internalize_;
  }

  pub fn needs_conversion(self) -> bool {
    return self.needs_conversion_;
  }

  pub fn has_escape(self) -> bool {
    return self.has_escape_;
  }

  pub fn start(self) -> i32 {
    return self.start_;
  }

  pub fn length(self) -> u32 {
    self.length_
  }

  pub fn index(self) -> u32 {
    return self.index_;
  }

  pub fn is_index(self) -> bool {
    return self.is_index_;
  }
}

struct JsonProperty {
  string_: JsonString,
}

#[derive(Copy, Clone, PartialEq)]
pub enum JsonToken {
  Number,
  String,
  Lbrace,
  Rbrace,
  Lbrack,
  Rbrack,
  TrueLiteral,
  FalseLiteral,
  NullLiteral,
  WhiteSpace,
  Colon,
  Conma,
  Illegal,
  Eos,
}

// A simple json parser.
#[derive(Copy, Clone)]
struct JsonParser {
  type_: u32,
  index_: u32,
  max_index: u32,
  elements_: u32,
  next_: JsonToken,
  chars_may_relocate_: bool,
  //original_source_: &str,
  //source_: &str,
  cursor_: u32,
  end_: u32,
  chars_: u32,
}

impl JsonParser {
  pub fn advance(&mut self) {
    self.cursor_ += 1
  }

  pub fn current_character(self) -> u32 {
    return self.cursor_;
  }

  pub fn next_character(&mut self) -> u32 {
    self.advance();
    return self.current_character();
  }

  pub fn advance_to_non_decimal() {}

  pub fn peek(self) -> JsonToken {
    return self.next_;
  }

  pub fn consume(&mut self, _token: JsonToken) {
    return self.advance();
  }

  pub fn expect(&mut self, token: JsonToken) {
    if self.peek() == token {
      return self.advance();
    } else {
      return self.report_unexpected_token(self.peek());
    }
  }

  pub fn expect_next(&mut self, token: JsonToken) {
    self.skip_white_space();
    self.expect(token)
  }

  pub fn check(&mut self, token: JsonToken) -> bool {
    self.skip_white_space();
    if self.next_ != token {
      return false;
    }
    self.advance();
    return true;
  }

  pub fn scan_literal(self) {}

  pub fn skip_white_space(self) {}

  pub fn scan_json_string(self, _needs_internalization: bool) {}

  pub fn scan_json_property_key(self) {}

  pub fn scan_unicode_character(self) -> u32 {
    return 0;
  }

  pub fn make_string(self) -> String {
    return "".to_string();
  }

  pub fn decode_string(self) {}

  pub fn parse_json_number(self) {}

  pub fn parse_json_value(self) {}

  pub fn build_json_object(self) {}
  pub fn build_json_array(self) {}

  pub fn report_unexpected_character(self) {}

  // Mark that a parsing error has happened at the current token.
  pub fn report_unexpected_token(self, _token: JsonToken) {}

  fn is_at_end(&self) -> bool {
    return self.cursor_ == self.end_;
  }

  fn position(&self) -> u32 {
    self.cursor_ - self.chars_
  }
}

fn get_one_char_json_token(code: u8) -> JsonToken {
  let c = code as char;
  match c {
    '"' => JsonToken::String,
    '-' => JsonToken::Number,
    '[' => JsonToken::Lbrack,
    '{' => JsonToken::Lbrace,
    't' => JsonToken::TrueLiteral,
    'f' => JsonToken::FalseLiteral,
    'n' => JsonToken::NullLiteral,
    ' ' => JsonToken::WhiteSpace,
    '\t' => JsonToken::WhiteSpace,
    '\r' => JsonToken::WhiteSpace,
    '\n' => JsonToken::WhiteSpace,
    ':' => JsonToken::Colon,
    ',' => JsonToken::Conma,
    _ => JsonToken::Illegal,
  }
}
