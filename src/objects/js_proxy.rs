const MAX_ITERATION_LIMIT: u64 = 100 * 1024;

// The JSProxy describes EcmaScript Harmony proxies
struct JSProxy {}

impl JSProxy {
  pub fn new() {}

  pub fn is_revoked() {}

  pub fn revoke() {}

  pub fn get_prototype() {}

  pub fn set_prototype() {}

  pub fn is_extensible() {}

  pub fn is_array() {}

  pub fn prevent_extensions() {}

  pub fn get_own_property_descriptor() {}

  pub fn define_own_property() {}

  pub fn has_property() {}

  pub fn check_has_trap() {}

  pub fn check_delete_trap() {}

  pub fn get_property() {}

  pub fn check_get_set_trap_result() {}

  pub fn set_property() {}

  pub fn delete_property_or_element() {}

  pub fn own_property_keys() {}

  pub fn get_property_attributes() {}
}

const PROXY_INDEX: u64 = 0;
const REVOKE_INDEX: u64 = 1;

// JSProxyRevocableResult is just a JSObject with a specific initial map.
// This initial map adds in-object properties for 'proxy' and 'revoke'.
struct JSProxyRevocableResult {}
