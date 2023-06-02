#![allow(dead_code)]

struct InlineOrder {}
impl InlineOrder {
  pub fn new() {}
  pub fn size() {}
  pub fn push() {}
  pub fn pop() {}
  pub fn erase_if() {}
  pub fn empty() {}
}

pub fn get_default_inline_order() {}
pub fn get_inline_order() {}

struct PluginInlineOrderAnalysis {}
impl PluginInlineOrderAnalysis {
  pub fn new() {}
  pub fn run() {}
  pub fn get_result() {}
  pub fn is_registered() {}
  pub fn unregister() {}
}