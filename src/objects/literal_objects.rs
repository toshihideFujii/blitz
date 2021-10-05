const LITERAL_TYPE_OFFSET: u64 = 0;
const DESCRIPTION_START_INDEX: u64 = 1;

// ObjectBoilerplateDescription is a list of properties consisting of name
// value pairs.
struct ObjectBoilerplateDescription {}

impl ObjectBoilerplateDescription {
  pub fn name() {}

  pub fn value() {}

  pub fn set_key_value() {}

  // The number of boilerplate properties.
  pub fn size() {}

  // Number of boilerplate properties and properties with computed names.
  pub fn backing_store_size() {}
  pub fn set_backing_store_size() {}

  fn has_number_of_properties() {}
}

struct ArrayBoilerplateDescription {}

impl ArrayBoilerplateDescription {
  pub fn elements_kind() {}
  pub fn set_elements_kind() {}

  pub fn is_empty() {}
}

struct RegExpBoilerplateDescription {}

pub enum ValueKind {
  Data,
  Getter,
  Setter,
}

struct ClassBoilerplate {}
