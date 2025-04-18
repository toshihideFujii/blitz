#![allow(dead_code)]

use std::{fs::File, io::Read};

use crate::{layout::Layout, literal::Literal, shape::Shape};

// Reads packed data from a metadata-less file as requested by a user (who must
// know its internal format). These are yielded as (structured) literal values.
pub struct PackedLiteralReader {
  file: File,
  offset: u64,
}

impl PackedLiteralReader {
  // Ownership of file is passed to this instance -- this instance takes
  // responsibility for closing it.
  pub fn new(file: File) -> Self {
    PackedLiteralReader { file: file, offset: 0 }
  }

  // Yields the next packed literal with shape "shape" as read from the
  // underlying file stream.
  //
  // Layout is optional. If it is not provided, no layout is set on the literal
  // that is produced.
  pub fn read<T>(
    &self,
    _shape: &Shape,
    _layout: &Option<Layout>) -> Result<Literal<T>, String>
    where T: Clone + Default + PartialEq
  {
    unimplemented!()
  }

  // Returns whether the input file has been fully exhausted; i.e. all available
  // packed literals have been read and we're at the end of the file.
  pub fn is_exhausted(&mut self) -> bool {
    // Try to read a single byte from offset_.  If we can't, we've
    // exhausted the data.
    let single_byte: &mut [u8] = &mut [1];
    let result = self.file.read(single_byte);
    result.is_ok()
  }
}