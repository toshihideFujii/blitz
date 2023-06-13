#![allow(dead_code)]

// This file replicates the record definition for XRay log entries.
// This should follow the evolution of the log record versions supported
// in the compiler-rt xray project.

// XRay traces all have a header providing some top-matter information
// useful to help tools determine how to interpret the information available
// in the trace.
pub struct XRayFileHeader {
  // Version of the XRay implementation that produced this file.
  version: u16,
  type_: u16,
  // Whether the cpu that produced the timestamp counters (tsc) move at a 
  // constant rate.
  constant_tsc: bool,
  // Whether the cpu that produced the timestamp counters (tsc) do not stop.
  nonstop_tsc: bool,
  cycle_frequency: u64,
  free_form_data: char
}

// Determines the supported types of records that could be seen in XRay traces.
// This may or may not correspond to actual record types in the raw traces (as
// the loader implementation may synthesize this information in the process of
// loading).
#[derive(Debug, Clone, PartialEq)]
pub enum RecordTypes {
  Enter,
  Exit,
  TailExit,
  EnterArg,
  CustomEvent,
  TypedEvent
}

// An XRayRecord is the denormalized view of data associated in a trace.
// These records may not correspond to actual entries in the raw traces,
// but they are the logical representation of records in a higher-level
// event log.
pub struct XRayRecord {
  pub record_type: u16,
  // The cpu where the thread is running. We assume number of cpus <= 65536.
  pub cpu: u16,
  // Identifies the type of record.
  pub type_: RecordTypes,
  // The function id for the record, if this is a function call record.
  pub func_id: i32,
  pub tsc: u64,
  // The thread id for the currently running threead.
  pub tid: u32,
  // The process id for the currently running process.
  pub pid: u32,
  // The function call arguments.
  pub call_args: Vec<u64>,
  // For custom and typed events, we provide the raw data from the trace.
  pub data: String
}