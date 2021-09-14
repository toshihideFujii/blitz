use crate::common::globals::OptimizationTier;
use crate::flags::flag_definitions;

#[derive(PartialEq, Clone, Copy)]
pub enum CodeKind {
  BytecodeHandler,
  ForTesting,
  Builtin,
  RegExp,
  WasmFunction,
  WasmToCApiFunction,
  WasmToJsFunction,
  JsToWasmFunction,
  JsToJsFunction,
  CWasmEntry,
  InterpretedFunction,
  Baseline,
  TurboProp,
  TurboFan,
}

pub fn code_kind_to_string(kind: CodeKind) -> String {
  match kind {
    CodeKind::BytecodeHandler => "ByteCodeHandler".to_string(),
    CodeKind::ForTesting => "ForTesting".to_string(),
    CodeKind::Builtin => "Builtin".to_string(),
    CodeKind::RegExp => "RegExp".to_string(),
    CodeKind::WasmFunction => "WasmFunction".to_string(),
    CodeKind::WasmToCApiFunction => "WasmToCApiFunction".to_string(),
    CodeKind::WasmToJsFunction => "WasmToJSFunction".to_string(),
    CodeKind::JsToWasmFunction => "JsToWasmFunction".to_string(),
    CodeKind::JsToJsFunction => "JsToJsFunction".to_string(),
    CodeKind::CWasmEntry => "CWasmEntry".to_string(),
    CodeKind::InterpretedFunction => "InterpretedFunction".to_string(),
    CodeKind::Baseline => "Baseline".to_string(),
    CodeKind::TurboProp => "TurboProp".to_string(),
    CodeKind::TurboFan => "TurboFan".to_string(),
    _ => panic!("Unsupported CodeKind."),
  }
}

pub fn code_kind_to_marker(kind: CodeKind) -> String {
  match kind {
    CodeKind::InterpretedFunction => "~".to_string(),
    CodeKind::Baseline => "^".to_string(),
    CodeKind::TurboProp => "+".to_string(),
    CodeKind::TurboFan => "*".to_string(),
    _ => "".to_string(),
  }
}

pub fn code_kind_is_interpreted_js_function(kind: CodeKind) -> bool {
  kind == CodeKind::InterpretedFunction
}

pub fn code_kind_is_baselined_js_function(kind: CodeKind) -> bool {
  kind == CodeKind::Baseline
}

pub fn code_kind_is_unoptimized_js_function(kind: CodeKind) -> bool {
  kind == CodeKind::InterpretedFunction || kind == CodeKind::Baseline
}

pub fn code_kind_is_optimized_js_function(kind: CodeKind) -> bool {
  kind == CodeKind::TurboProp || kind == CodeKind::TurboFan
}

pub fn code_kind_is_js_function(kind: CodeKind) -> bool {
  code_kind_is_unoptimized_js_function(kind)
    || code_kind_is_optimized_js_function(kind)
}

pub fn code_kind_is_builtin_or_js_function(kind: CodeKind) -> bool {
  kind == CodeKind::Builtin || code_kind_is_js_function(kind)
}

pub fn code_kind_can_deoptimize(kind: CodeKind) -> bool {
  code_kind_is_optimized_js_function(kind)
}

pub fn code_kind_can_osr(kind: CodeKind) -> bool {
  kind == CodeKind::TurboFan || kind == CodeKind::TurboProp
}

pub fn code_kind_is_optimized_and_can_tier_up(kind: CodeKind) -> bool {
  flag_definitions::FLAG_TURBOPROP_AS_TOPTIER == false
    && kind == CodeKind::TurboProp
}

pub fn code_kind_can_tier_up(kind: CodeKind) -> bool {
  code_kind_is_unoptimized_js_function(kind)
    || code_kind_is_optimized_and_can_tier_up(kind)
}

pub fn code_kind_is_stored_in_optimized_code_cache(kind: CodeKind) -> bool {
  kind == CodeKind::TurboFan || kind == CodeKind::TurboProp
}

pub fn get_tier_for_code_kind(kind: CodeKind) -> OptimizationTier {
  if kind == CodeKind::TurboFan {
    return OptimizationTier::TopTier;
  }
  if kind == CodeKind::TurboProp {
    if flag_definitions::FLAG_TURBOPROP_AS_TOPTIER == true {
      return OptimizationTier::TopTier;
    } else {
      return OptimizationTier::MidTier;
    }
  }
  return OptimizationTier::None;
}

pub fn code_kind_for_top_tier() -> CodeKind {
  if flag_definitions::FLAG_TURBOPROP_AS_TOPTIER == false {
    return CodeKind::TurboProp;
  }
  return CodeKind::TurboFan;
}

pub fn code_kind_for_osr() -> CodeKind {
  if flag_definitions::FLAG_TURBOPROP == false {
    return CodeKind::TurboProp;
  }
  return CodeKind::TurboFan;
}
