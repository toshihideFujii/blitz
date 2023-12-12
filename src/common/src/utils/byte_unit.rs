#![allow(dead_code)]

enum ByteUnitEnum {
  Byte,
  KB,
  MB,
  GB,
  TB,
  PB,
}

pub struct ByteUnit {
  unit: ByteUnitEnum,
  multiplier: u64,
}

impl ByteUnit {
  pub fn convert_to(&self, d: u64, u: &ByteUnit) -> Result<u64, &str> {
    if self.multiplier > u.multiplier {
      let ratio = self.multiplier / u.multiplier;
      if u64::MAX / ratio < d {
        return Err("Conversion of d exceeds u64::MAX. Try a larger unit.");
      }
      return Ok(d * ratio);
    } else {
      return Ok(d / (u.multiplier / self.multiplier));
    }
  }

  pub fn to_bytes(&self, d: u64) -> u64 {
    d * self.multiplier
  }

  pub fn to_kb(&self, d: u64) -> Result<u64, &str> { self.convert_to(d, &KB) }
  pub fn to_mb(&self, d: u64) -> Result<u64, &str> { self.convert_to(d, &MB) }
  pub fn to_gb(&self, d: u64) -> Result<u64, &str> { self.convert_to(d, &GB) }
  pub fn to_tb(&self, d: u64) -> Result<u64, &str> { self.convert_to(d, &TB) }
  pub fn to_pb(&self, d: u64) -> Result<u64, &str> { self.convert_to(d, &PB) }
}

pub const BYTE: ByteUnit = ByteUnit {
  unit: ByteUnitEnum::Byte,
  multiplier: 1,
};

pub const KB: ByteUnit = ByteUnit {
  unit: ByteUnitEnum::KB,
  multiplier: 1 << 10,
};

pub const MB: ByteUnit = ByteUnit {
  unit: ByteUnitEnum::MB,
  multiplier: 1 << 20,
};

pub const GB: ByteUnit = ByteUnit {
  unit: ByteUnitEnum::GB,
  multiplier: 1 << 30,
};

pub const TB: ByteUnit = ByteUnit {
  unit: ByteUnitEnum::TB,
  multiplier: 1 << 40,
};

pub const PB: ByteUnit = ByteUnit {
  unit: ByteUnitEnum::PB,
  multiplier: 1 << 50,
};