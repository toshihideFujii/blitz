
// Consolidated utilities for logging information during compilation, usually
// based on the options specified in the DebugOptions proto.
//
// Most functions here take an HloModule and read the DebugOptions from the
// module's config.

use std::{collections::HashMap, sync::LazyLock};

use common::blitz_data::DebugOptions;
use hlo::{hlo_module::HloModule, hlo_proto::HloSnapshot};


// Argument used when calling DumpHloModuleIfEnabled before optimizations are
// performed on an HloModule.
const _BEFORE_OPTIMIZATIONS_DUMP_NAME: &str = "before_optimizations";
const _AFTER_OPTIMIZATIONS_DUMP_NAME: &str = "after_optimizations";
const _NON_DEFAULT_DEBUG_OPTIONS_DUMP_SUFFIX: &str = "debug_options";

// Maps a module's unique ID to a timestamp indicating when we've first dumped
// this module during the compilation pipeline and when we first started
// compiling this module.  This lets us keep the filenames ordered nicely.
//
// Entries added here leak forever; we have no way to GC them when a module
// dies.  But we only add an entry if dumping is enabled for this module, and
// dumping a module leaks buffer space in stdout or bytes on disk *way* faster
// than this hashtable leaks memory.
static _MODULE_ID_TO_TIMESTAMP: LazyLock<HashMap<i64, i64>>
  = LazyLock::new(HashMap::new);

// Creates dir if doesn't exist (analogue of `mkdir -p`), tries to get around
// race conditions by trying again on collision.
pub fn create_dir_needed(_dir: &String) -> Result<(), String> {
  unimplemented!()
}

// Get a timestamp which we can use as a filename prefix specific to this
// module.
pub fn timestamp_for(_module: &HloModule) -> String {
  unimplemented!()
}

// Create the filename we will use to dump in DumpToFileInDir.
pub fn filename_for(
  unique_id: i64,
  module_name: &String,
  prefix: &String,
  suffix: &String) -> String
{
  let mut filename = String::new();
  if !prefix.is_empty() {
    filename.push_str(&prefix);
    filename.push_str(".");
  }
  filename.push_str("module_");
  filename.push_str(&unique_id.to_string());
  if !module_name.is_empty() {
    filename.push_str(".");
    filename.push_str(&module_name);
  }
  filename.push_str(".");
  filename.push_str(&suffix);
  if !module_name.is_empty() && filename.len() > 255 {
    return filename_for(unique_id, &"".to_string(), prefix, suffix);
  }
  filename
}

// Writes the given string to a file in the xla_dump_to directory specified by
// module's DebugOptions.
// If module doesn't have an xla_dump_to directory, does nothing.
pub fn dump_to_file_in_dir(
  _module: &HloModule,
  _file_predix: &String,
  _file_suffix: &String,
  _contents: &String)
{
  unimplemented!()    
}

// Like DumpToFileInDir, except if module doesn't have an xla_dump_to directory
// specified, or if that directory is equal to "-", writes to stdout instead.
pub fn dump_to_file_in_dir_or_stdout(
  _module: &HloModule,
  _file_predix: &String,
  _file_suffix: &String,
  _contents: &String)
{
  unimplemented!()    
}

// Dumps the given HLO module if dumping is enabled for the module. Exactly
// where and in what formats it's dumped is determined by the module's config.
// Returns the full file paths of all dumps of the module, or an empty vector if
// nothing was dumped.
pub fn dump_hlo_module_if_enabled(
  _module: &HloModule, _name: &String) -> Vec<String>
{
  unimplemented!()    
}

// Dumps the given HloSnapshot to the module's xla_dump_dir, if this is enabled.
// Prefer the first overload below, as this will give filenames that are
// consistent with the other methods here.  The second overload (which doesn't
// take an HloModule) is useful in the cases when you're dumping an HloSnapshot
// and simply don't have an HloModule.
pub fn dump_hlo_snapshot_if_enabled(_module: &HloModule, _snapshot: &HloSnapshot) {
  unimplemented!()
}

// Returns true if DumpToFileInDirOrStdout and DumpHloModuleIfEnabled will write
// to stdout, rather than to a file on disk.
// This is useful if you want to do something different when writing to stdout.
// For example, maybe you have (almost-)duplicate data that you wouldn't mind
// writing to two files, but you don't want to print twice.
pub fn dumping_to_stdout(_opts: &DebugOptions) {
  unimplemented!()
}

// Returns the non-default debug options as a string. The default debug options
// are received from DefaultDebugOptionsIgnoringFlags().
// TODO: move this to xla/debug_options_flags.cc
pub fn get_non_default_debug_options(_debug_options: &DebugOptions) -> String {
  unimplemented!()
}