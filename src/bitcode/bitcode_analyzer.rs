#![allow(dead_code)]

// This header defines interfaces to analyze Blitz bitcode files/streams.

enum CurStreamTypeType {
  UnknownBitStream,
  BlitzIRBitstream,
  SerializedASTBitstream,
  SerializedDiagnosticsBitstream,
  BlitzBitstreamRemarks
}

struct BCDumpOptions {}

struct BitcodeAnalyzer {}
impl BitcodeAnalyzer {
  pub fn new() {}
  pub fn analyze() {}
  pub fn print_stats() {}
  fn parse_block() {}
  fn decode_metadata_strings_blob() {}
}