pub fn ascii_alpha_to_lower(c: u32) -> u32 {
    c | 0x20
}

pub fn is_carriage_return(c: u32) -> bool {
    c == 0x000D
}

pub fn is_line_feed(c: u32) -> bool {
    c == 0x000A
}

pub fn is_ascii_identifier(c: u32) -> bool {
    return is_alpha_numeric(c) || c == '$' as u32 || c == '_' as u32;
}

pub fn is_alpha_numeric(c: u32) -> bool {
    is_in_range(c, 'a', 'z') || is_decimal_digit(c)
}

pub fn is_decimal_digit(c: u32) -> bool {
    is_in_range(c, '0', '9')
}

pub fn is_hex_digit(c: u32) -> bool {
    is_decimal_digit(c) || is_in_range(ascii_alpha_to_lower(c), 'a', 'f')
}

pub fn is_octal_digit(c: u32) -> bool {
    is_in_range(c, '0', '7')
}

pub fn is_non_octal_decimal_digit(c: u32) -> bool {
    is_in_range(c, '8', '9')
}

pub fn is_binary_digit(c: u32) -> bool {
    c == '0' as u32 || c == '1' as u32
}

pub fn is_ascii_lower(c: u32) -> bool {
    is_in_range(c, 'a', 'z')
}

pub fn is_ascii_upper(c: u32) -> bool {
    is_in_range(c, 'A', 'Z')
}

pub fn to_ascii_upper(_c: u32) -> u32 {
    0 //c & !(is_ascii_lower(c) << 5)
}

pub fn to_ascii_lower(_c: u32) -> u32 {
    0 //c | (is_ascii_upper(c) << 5)
}

pub fn is_reg_exp_word(c: u32) -> bool {
    is_alpha_numeric(c) || c == '_' as u32
}

fn is_in_range(value: u32, lower_limit: char, higher_limit: char) -> bool {
    (value - lower_limit as u32) <= (higher_limit as u32 - lower_limit as u32)
}
