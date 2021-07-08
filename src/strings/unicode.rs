pub fn is_line_terminator(c: u32) -> bool {
  c == 0x000A || c == 0x000D || c == 0x2028 || c == 0x2029
}
