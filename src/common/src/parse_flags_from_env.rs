#![allow(dead_code)]

pub fn parse_flags_from_env_and_die_if_unknown(_envvar: String) {}

pub fn parse_flags_from_env_and_ignore_unknown() {}

pub fn die_if_env_has_unknown_flags_left(_envvar: String) -> bool {
  false
}

pub fn reset_flags_from_env_for_testing() {}