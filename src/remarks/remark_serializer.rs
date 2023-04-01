#![allow(dead_code)]

// This file provides an interface for serializing remarks
// to different formats.

enum SerializerMode {
  Separate,
  Standalone
}

struct RemarkSerializer {}
impl RemarkSerializer {
  pub fn new() {}
}

struct MetaSerializer {}