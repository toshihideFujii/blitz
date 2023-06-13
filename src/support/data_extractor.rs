#![allow(dead_code)]

use std::mem::{size_of};
use crate::adt::string_ref::StringRef;
use super::swap_byte_order::{
  IS_LITTLE_ENDIAN_HOST,
  swap_byte_order_u8,
  swap_byte_order_u16,
  swap_byte_order_u32
};

// A class representing a position in a DataExtractor, as well as
// any error encountered during extraction.
pub struct Cursor {
  offset: u64
}

impl Cursor {
  pub fn new(offset: u64) -> Self {
    Cursor { offset: offset }
  }

  // Return the current position of this cursor.
  pub fn tell(&self) -> u64 {
    self.offset
  }

  // Set the cursor to the new offset.
  pub fn seek(&mut self, new_offset: u64) {
    self.offset = new_offset
  }

  pub fn take_error() {}
}

pub struct DataExtractor {
  data: StringRef,
  is_little_endian: bool,
  address_size: u8
}

impl DataExtractor {
  pub fn new(data: StringRef, is_little_endian: bool, address_size: u8) -> Self {
    DataExtractor {
      data: data,
      is_little_endian: is_little_endian,
      address_size: address_size
    }
  }

  // Get the data pointed to by this extractor.
  pub fn get_data(&self) -> StringRef {
    self.data.clone()
  }

  // Get the endianness for this extractor.
  pub fn is_little_endian(&self) -> bool {
    self.is_little_endian
  }

  // Get the address size for the extractor.
  pub fn get_address_size(&self) -> u8 {
    self.address_size
  }

  // Set the address size for this extractor.
  pub fn set_address_size(&mut self, size: u8) {
    self.address_size = size
  }

  // Extract a C string from offset_ptr.
  // Returns a StringRef for the C String from the data at the offset
  // poiinted to by offset_ptr.
  pub fn get_c_str_ref(&self, offset_ptr: &mut u64) -> StringRef {
    let start = offset_ptr.clone();
    let pos = self.data.find('\0', start as usize);
    if pos != StringRef::NPOS {
      *offset_ptr += pos as u64 + 1;
      return self.data.substr(start as usize, pos - start as usize);
    }
    StringRef::new()
  }

  // Extract a C string from offset_ptr.
  pub fn get_c_str(&self, offset_ptr: &mut u64) -> String {
    self.get_c_str_ref(offset_ptr).data()
  }

  pub fn get_fixed_length_string(&self, _offset_ptr: &mut u64, _length: u64) {}

  // Extract a fixed number of bytes from the specified offset.
  // Returns a StringRef for the bytes from the data at the offset
  // pointed to by offset_ptr.
  pub fn get_bytes(&self, offset_ptr: u64, length: u64) -> (StringRef, u64) {
    if self.prepare_read(offset_ptr, length) {
      return (StringRef::new(), offset_ptr);
    }
    let result_str =
      self.data.substr(offset_ptr as usize, length as usize);
    let result_offset = offset_ptr + length;
    (result_str, result_offset)
  }

  // Extract an unsigned integer of size byte_size from offset_ptr.
  pub fn get_unsigned(&self, offset_ptr: &mut u64, byte_size: u32) -> u64 {
    match byte_size {
      1 => return self.get_u8(offset_ptr) as u64,
      2 => return self.get_u16(offset_ptr) as u64,
      4 => return self.get_u32(offset_ptr) as u64,
      8 => return self.get_u64(offset_ptr),
      _ => panic!("get_unsigned() unhandled case.")
    };
  }

  // Extract an signed integer of size byte_size from offset_ptr.
  pub fn get_signed(&self, offset_ptr: &mut u64, byte_size: u32) -> i64 {
    match byte_size {
      1 => return self.get_u8(offset_ptr) as i64,
      2 => return self.get_u16(offset_ptr) as i64,
      4 => return self.get_u32(offset_ptr) as i64,
      8 => return self.get_u64(offset_ptr) as i64,
      _ => panic!("get_signed() unhandled case.")
    };
  }

  // Extract an pointer from offset_ptr.
  pub fn get_address(&self, offset_ptr: &mut u64) -> u64 {
    self.get_unsigned(offset_ptr, self.address_size as u32)
  }

  // Extract a u8 value from offset_ptr.
  // Extract a single u8 from the binary data at the offset pointed to
  // by offset_ptr, and advance the offset on success.
  pub fn get_u8(&self, offset_ptr: &mut u64) -> u8 {
    let mut val: u8 = 0;
    let offset = offset_ptr.clone();
    if !self.prepare_read(offset, size_of::<u8>() as u64) {
      return val;
    }
    val = self.data.at(offset as usize).clone() as u8;
    if IS_LITTLE_ENDIAN_HOST != self.is_little_endian {
      val = swap_byte_order_u8(&mut val);
    }
    // Advance the offset.
    *offset_ptr += size_of::<u8>() as u64;
    val
  }

  // Extract a u16 value from offset_ptr.
  // Extract a single u16 from the binary data at the offset pointed to
  // by offset_ptr, and advance the offset on success.
  pub fn get_u16(&self, offset_ptr: &mut u64) -> u16 {
    let mut val: u16 = 0;
    let offset = offset_ptr.clone();
    if !self.prepare_read(offset, size_of::<u16>() as u64) {
      return val;
    }
    val = self.data.at(offset as usize).clone() as u16;
    if IS_LITTLE_ENDIAN_HOST != self.is_little_endian {
      val = swap_byte_order_u16(&mut val);
    }
    // Advance the offset.
    *offset_ptr += size_of::<u16>() as u64;
    val
  }

  // Extract a 24-bit unsigned value from offset_ptr and return it in
  // a u32.
  pub fn get_u24(&self, _offset_ptr: &mut u64) -> u32 { 0 }

  // Extract u32 value from offset_ptr.
  // Extract a single u32 from the binary data at the offset pointed to
  // by offset_ptr, and update the offset on sucess.
  pub fn get_u32(&self, offset_ptr: &mut u64) -> u32 {
    let mut val: u32 = 0;
    let offset = offset_ptr.clone();
    if !self.prepare_read(offset, size_of::<u32>() as u64) {
      return val;
    }
    val = self.data.at(offset as usize).clone() as u32;
    if IS_LITTLE_ENDIAN_HOST != self.is_little_endian {
      val = swap_byte_order_u32(&mut val);
    }
    // Advance the offset.
    *offset_ptr += size_of::<u32>() as u64;
    val
  }

  // Extract u64 value from offset_ptr.
  // Extract a single u64 from the binary data at the offset pointed to
  // by offset_ptr, and update the offset on sucess.
  pub fn get_u64(&self, _offset_ptr: &mut u64) -> u64 { 0 }

  pub fn get_sleb_128() {}
  pub fn get_uleb_128() {}

  // Advance the cursor position by the given number of bytes.
  pub fn skip(&self, c: &mut Cursor, length: u64) {
    if self.prepare_read(c.offset, length) {
      c.offset += length;
    }
  }

  // Return true if the cursor is at the end of the buffer, regardless
  // of the error state of the cursor.
  pub fn eof(&self, c: &Cursor) -> bool {
    self.size() as u64 == c.offset
  }

  // Test the validity of offset.
  pub fn is_valid_offset(&self, offset: u64) -> bool {
    self.size() as u64 > offset
  }

  // Test the availability of length bytes of data from offset..
  pub fn is_valid_offset_for_data_of_size(&self, offset: u64, length: u64) -> bool {
    offset + length >= offset && self.is_valid_offset(offset + length - 1)
  }

  // Test the availability of enough bytes of data for a pointer from offset.
  pub fn is_valid_offset_for_address(&self, offset: u64) -> bool {
    self.is_valid_offset_for_data_of_size(offset, self.address_size as u64)
  }

  // Return the number of bytes in the underlying buffer.
  pub fn size(&self) -> usize {
    self.data.size()
  }

  // If it is possible to read size bytes at offset, returns true.
  fn prepare_read(&self, offset: u64, size: u64) -> bool {
    if self.is_valid_offset_for_data_of_size(offset, size) {
      return true;
    }
    false
  }

  fn get_leb_128() {}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_offset_overflow() {
    let number_data = StringRef::new_from_string("x80x90xFFxFFx80x00x00x00");
    let de = DataExtractor::new(number_data, false, 8);
    assert_eq!(de.is_valid_offset_for_data_of_size(100, 100), false);
  }

  #[test]
  fn test_unsigned_numbers() {
    let number_data = StringRef::new_from_string("\u{9080}");
    let de = DataExtractor::new(number_data, false, 8);
    
    let mut offset: u64 = 0;
    assert_eq!(de.get_u8(&mut offset), 0x80);
    assert_eq!(offset, 1);

    offset = 0;
    assert_eq!(de.get_u16(&mut offset), 0x8090);
    assert_eq!(offset, 2);

    //offset = 0;
    //assert_eq!(de.get_u32(&mut offset), 0x8090FFFF);
    //assert_eq!(offset, 4);
  }
}