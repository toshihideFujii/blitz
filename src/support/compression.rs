#![allow(dead_code)]

// This file contains basic functions for compression/decompression.

enum DebugCompressionType {
  None,
  Zlib,
  Zstd
}

const ZLIB_NO_COMPRESSION: i32 = 0;
const ZLIB_BEST_SPEED_COMPRESSION: i32 = 1;
const ZLIB_DEFAULT_COMPRESSION: i32 = 6;
const ZLIB_BEST_SIZE_COMPRESSION: i32 = 9;

pub fn zlib_is_available() {}

pub fn zlib_compress() {}

pub fn zlib_decompress() {}

const ZSTD_NO_COMPRESSION: i32 = -5;
const ZSTD_BEST_SPEED_COMPRESSION: i32 = 1;
const ZSTD_DEFAULT_COMPRESSION: i32 = 5;
const ZSTD_BEST_SIZE_COMPRESSION: i32 = 12;

pub fn zstd_is_available() {}

pub fn zstd_compress() {}

pub fn zstd_decompress() {}

enum Format {
  Zlib,
  Zstd
}

pub fn get_reason_if_unsupported() {}