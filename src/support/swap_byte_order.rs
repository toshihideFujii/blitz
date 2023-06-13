#![allow(dead_code)]

pub const IS_BIG_ENDIAN_HOST: bool = false;
pub const IS_LITTLE_ENDIAN_HOST: bool = !IS_BIG_ENDIAN_HOST;

pub fn get_swapped_bytes_u8(value: &mut u8) -> u8 {
  value.swap_bytes()
}

pub fn swap_byte_order_u8(value: &mut u8) -> u8 {
  get_swapped_bytes_u8(value)
}

pub fn get_swapped_bytes_u16(value: &mut u16) -> u16 {
  value.swap_bytes()
}
  
pub fn swap_byte_order_u16(value: &mut u16) -> u16 {
  get_swapped_bytes_u16(value)
}

pub fn get_swapped_bytes_u32(value: &mut u32) -> u32 {
  value.swap_bytes()
}
    
pub fn swap_byte_order_u32(value: &mut u32) -> u32 {
  get_swapped_bytes_u32(value)
}