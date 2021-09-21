#[derive(PartialEq, Copy, Clone)]
pub enum FunctionKind {
  NormalFunction,
  Module,
  AsyncModule,
  BaseConstructor,
  DefaultBaseConstructor,
  DefaultDerivedConstructor,
  DerivedConstructor,
  GetterFunction,
  StaticGetterFunction,
  SetterFunction,
  StaticSetterFunction,
  ArrowFunction,
  AsyncArrowFunction,
  AsyncFunction,
  AsyncConciseMethod,
  StaticAsyncConciseMethod,
  AsyncConciseGeneratorMethod,
  StaticAsyncConciseGeneratorMethod,
  AsyncGeneratorFunction,
  GeneratorFunction,
  ConciseGeneratorMethod,
  StaticConciseGeneratorMethod,
  ConciseMethod,
  StaticConciseMethod,
  ClassMembersInitializerFunction,
  ClassStaticInitializerFunction,
  Invalid,
}

const FUNCTION_KIND_BIT_SIZE: u64 = 5;

pub fn is_arrow_function(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::ArrowFunction => true,
    FunctionKind::AsyncArrowFunction => true,
    _ => false,
  }
}

pub fn is_module(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::Module => true,
    FunctionKind::AsyncModule => true,
    _ => false,
  }
}

pub fn is_async_module(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::AsyncModule => true,
    _ => false,
  }
}

pub fn is_async_generator_function(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::AsyncConciseGeneratorMethod => true,
    FunctionKind::StaticAsyncConciseGeneratorMethod => true,
    FunctionKind::AsyncGeneratorFunction => true,
    _ => false,
  }
}

pub fn is_generator_function(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::AsyncConciseGeneratorMethod => true,
    FunctionKind::StaticAsyncConciseGeneratorMethod => true,
    FunctionKind::AsyncGeneratorFunction => true,
    FunctionKind::GeneratorFunction => true,
    FunctionKind::ConciseGeneratorMethod => true,
    FunctionKind::StaticConciseGeneratorMethod => true,
    _ => false,
  }
}

pub fn is_async_function(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::AsyncArrowFunction => true,
    FunctionKind::AsyncFunction => true,
    FunctionKind::AsyncConciseMethod => true,
    FunctionKind::StaticAsyncConciseMethod => true,
    FunctionKind::AsyncConciseGeneratorMethod => true,
    FunctionKind::StaticAsyncConciseGeneratorMethod => true,
    FunctionKind::AsyncGeneratorFunction => true,
    _ => false,
  }
}

pub fn is_resumable_function(kind: FunctionKind) -> bool {
  if is_generator_function(kind) || is_async_function(kind) || is_module(kind) {
    return true;
  } else {
    return false;
  }
}

pub fn is_concise_method(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::AsyncConciseMethod => true,
    FunctionKind::StaticAsyncConciseMethod => true,
    FunctionKind::AsyncConciseGeneratorMethod => true,
    FunctionKind::StaticAsyncConciseGeneratorMethod => true,
    FunctionKind::ConciseGeneratorMethod => true,
    FunctionKind::StaticConciseGeneratorMethod => true,
    FunctionKind::ConciseMethod => true,
    FunctionKind::StaticConciseMethod => true,
    FunctionKind::ClassMembersInitializerFunction => true,
    FunctionKind::ClassStaticInitializerFunction => true,
    _ => false,
  }
}

pub fn is_strict_function_without_prototype(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::GetterFunction => true,
    FunctionKind::StaticGetterFunction => true,
    FunctionKind::SetterFunction => true,
    FunctionKind::StaticSetterFunction => true,
    FunctionKind::ArrowFunction => true,
    FunctionKind::AsyncArrowFunction => true,
    FunctionKind::AsyncConciseMethod => true,
    FunctionKind::StaticAsyncConciseMethod => true,
    FunctionKind::AsyncConciseGeneratorMethod => true,
    FunctionKind::StaticAsyncConciseGeneratorMethod => true,
    FunctionKind::ConciseGeneratorMethod => true,
    FunctionKind::StaticConciseGeneratorMethod => true,
    FunctionKind::ConciseMethod => true,
    FunctionKind::StaticConciseMethod => true,
    FunctionKind::ClassMembersInitializerFunction => true,
    FunctionKind::ClassStaticInitializerFunction => true,
    _ => false,
  }
}

pub fn is_getter_function(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::GetterFunction => true,
    FunctionKind::StaticGetterFunction => true,
    _ => false,
  }
}

pub fn is_setter_function(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::SetterFunction => true,
    FunctionKind::StaticSetterFunction => true,
    _ => false,
  }
}

pub fn is_accessor_function(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::GetterFunction => true,
    FunctionKind::StaticGetterFunction => true,
    FunctionKind::SetterFunction => true,
    FunctionKind::StaticSetterFunction => true,
    _ => false,
  }
}

pub fn is_default_constructor(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::DefaultBaseConstructor => true,
    FunctionKind::DefaultDerivedConstructor => true,
    _ => false,
  }
}

pub fn is_base_constructor(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::BaseConstructor => true,
    FunctionKind::DefaultBaseConstructor => true,
    _ => false,
  }
}

pub fn is_derived_constructor(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::DefaultDerivedConstructor => true,
    FunctionKind::DerivedConstructor => true,
    _ => false,
  }
}

pub fn is_class_constructor(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::BaseConstructor => true,
    FunctionKind::DefaultBaseConstructor => true,
    FunctionKind::DefaultDerivedConstructor => true,
    FunctionKind::DerivedConstructor => true,
    _ => false,
  }
}

pub fn is_class_members_initializer_function(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::ClassMembersInitializerFunction => true,
    FunctionKind::ClassStaticInitializerFunction => true,
    _ => false,
  }
}

pub fn is_constructable(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::NormalFunction => true,
    FunctionKind::Module => true,
    FunctionKind::AsyncModule => true,
    FunctionKind::BaseConstructor => true,
    FunctionKind::DefaultBaseConstructor => true,
    FunctionKind::DefaultDerivedConstructor => true,
    FunctionKind::DerivedConstructor => true,
    _ => false,
  }
}

pub fn is_static(kind: FunctionKind) -> bool {
  match kind {
    FunctionKind::StaticGetterFunction => true,
    FunctionKind::StaticSetterFunction => true,
    FunctionKind::StaticConciseMethod => true,
    FunctionKind::StaticConciseGeneratorMethod => true,
    FunctionKind::StaticAsyncConciseMethod => true,
    FunctionKind::StaticAsyncConciseGeneratorMethod => true,
    FunctionKind::ClassStaticInitializerFunction => true,
    _ => false,
  }
}

pub fn binds_super(kind: FunctionKind) -> bool {
  if is_concise_method(kind)
    || is_accessor_function(kind)
    || is_class_constructor(kind)
  {
    return true;
  } else {
    return false;
  }
}

pub fn is_await_as_identifier_disallowed(kind: FunctionKind) -> bool {
  if is_async_function(kind)
    || kind == FunctionKind::ClassStaticInitializerFunction
  {
    return true;
  } else {
    return false;
  }
}

pub fn function_kind_to_string(kind: FunctionKind) -> String {
  match kind {
    FunctionKind::NormalFunction => "NormalFunction".to_string(),
    FunctionKind::ArrowFunction => "ArrowFunction".to_string(),
    FunctionKind::GeneratorFunction => "GeneratorFunction".to_string(),
    FunctionKind::ConciseMethod => "ConciseMethod".to_string(),
    FunctionKind::StaticConciseMethod => "StaticConciseMethod".to_string(),
    FunctionKind::DerivedConstructor => "DerivedConstructor".to_string(),
    FunctionKind::BaseConstructor => "BaseConstructor".to_string(),
    FunctionKind::GetterFunction => "GetterFunction".to_string(),
    FunctionKind::StaticGetterFunction => "StaticGetterFunction".to_string(),
    FunctionKind::SetterFunction => "SetterFunction".to_string(),
    FunctionKind::StaticSetterFunction => "StaticSetterFunction".to_string(),
    FunctionKind::AsyncFunction => "AsyncFunction".to_string(),
    FunctionKind::Module => "Module".to_string(),
    FunctionKind::AsyncModule => "AsyncModule".to_string(),
    FunctionKind::ClassMembersInitializerFunction => {
      "ClassMembersInitializerFunction".to_string()
    }
    FunctionKind::ClassStaticInitializerFunction => {
      "ClassStaticInitializerFunction".to_string()
    }
    FunctionKind::DefaultBaseConstructor => {
      "DefaultBaseConstructor".to_string()
    }
    FunctionKind::DefaultDerivedConstructor => {
      "DefaultDerivedConstructor".to_string()
    }
    FunctionKind::AsyncArrowFunction => "AsyncArrowFunction".to_string(),
    FunctionKind::AsyncConciseMethod => "AsyncConciseMethod".to_string(),
    FunctionKind::StaticAsyncConciseMethod => {
      "StaticAsyncConciseMethod".to_string()
    }
    FunctionKind::ConciseGeneratorMethod => {
      "ConciseGeneratorMethod".to_string()
    }
    FunctionKind::StaticConciseGeneratorMethod => {
      "StaticConciseGeneratorMethod".to_string()
    }
    FunctionKind::AsyncConciseGeneratorMethod => {
      "AsyncConciseGeneratorMethod".to_string()
    }
    FunctionKind::StaticAsyncConciseGeneratorMethod => {
      "StaticAsyncConciseGeneratorMethod".to_string()
    }
    FunctionKind::AsyncGeneratorFunction => {
      "AsyncGeneratorFunction".to_string()
    }
    FunctionKind::Invalid => "Invalid".to_string(),
  }
}
