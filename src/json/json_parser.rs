

pub enum ParseElementResult {
    ElementFound,
    ElementNotFound
}

struct JsonString {
    start_: i32,
    index_: u32,
    length_: u32,
    needs_conversion_: bool,
    internalize_: bool,
    has_escape_: bool,
    is_index_: bool
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
            is_index_: false
        }
    }

    pub fn internalize(self) -> bool {
        return self.internalize_
    }

    pub fn needs_conversion(self) -> bool {
        return self.needs_conversion_
    }

    pub fn has_escape(self) -> bool {
        return self.has_escape_
    }

    pub fn start(self) -> i32 {
        return self.start_
    }

    pub fn length(self) -> u32 {
        self.length_
    }

    pub fn index(self) -> u32 {
        return self.index_
    }

    pub fn is_index(self) -> bool {
        return self.is_index_
    }
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
    FlaseLiteral,
    NullLiteral,
    WhiteSpace,
    Colon,
    Conma,
    Illegal,
    Eos
}

// A simple json parser.
#[derive(Copy, Clone)]
struct JsonParser {
    type_: u32,
    index_: u32,
    max_index: u32,
    elements_: u32,
    cursor_: u32,
    next_: JsonToken,
}

impl JsonParser {
    pub fn advance(&mut self) {
        self.cursor_ += 1
    }

    pub fn current_character(self) -> u32 {
        return self.cursor_
    }

    pub fn next_character(&mut self) -> u32 {
        self.advance();
        return self.current_character()
    }

    pub fn advance_to_non_decimal() {

    }

    pub fn peek(self) -> JsonToken {
        return self.next_
    }

    pub fn consume(&mut self, _token: JsonToken) {
        return self.advance()
    }

    pub fn expect(&mut self, _token: JsonToken) {
        if self.peek() == _token {
            return self.advance()
        } else {
            return self.report_unexpected_token(self.peek())
        }
    }

    // Mark that a parsing error has happened at the current token.
    pub fn report_unexpected_token(self, _token: JsonToken) {

    }
}