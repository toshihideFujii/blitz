#![allow(dead_code)]

// This module exports ParseFlagsFromEnvAndDieIfUnknown(), which allows other
// modules to parse flags from an environment variable, or (if the first
// non-whitespace in the variable value is not '-'), a file named by that
// environment variable.
//
// The accepted syntax is that flags arguments are of the form --flag=value or
// (for boolean flags) --flag, and are whitespace separated.  The <value> may be
// one of:
//
//  - <non-whitespace, non-nul not starting with single-quote or double-quote>
//    in which case the effective value is the string itself
//  - <single-quote><characters string not containing nul or
//    single-quote><single_quote> in which case the effective value is the
//    string with the single-quotes removed
//  - <double-quote><character string not containing nul or unescaped
//    double-quote><double_quote> in which case the effective value if the
//    string with the double-quotes removed, and escaped sequences of
//    <backslash><char> replaced by <char>.
//
// Flags values inconsistent with the type of the flag will be rejected by the
// flag parser.
//
// Examples:
//
//  - BLITZ_FLAGS="--foo=bar  --wombat='value with a space'"
//  - BLITZ_FLAGS=/tmp/flagfile
//
// where /tmp/flagfile might contain
//
//  --some_flag="This is a string containing a \" and a '."
//  --another_flag=wombats


// Calls tsl::Flags::Parse(argc, argv, flag_list) against any as yet
// unrecognized flags passed in the environment variable `envvar`.
//
// Raises a fatal error if any flags in `envvar` were not recognized, or if flag
// parsing failed.
pub fn parse_flags_from_env_and_die_if_unknown(_envvar: String, _flag_list: &Vec<i64>) {
  unimplemented!()
}

pub fn parse_flags_from_env_and_ignore_unknown() {}

pub fn reset_flags_from_env_for_testing() {}