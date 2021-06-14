

//use std::mem;
use std::i64;

// Returns the smallest power of two which is greater than or equal to |value|.
#[allow(unused_variables)]
pub fn round_up_to_power_of_two_32(value: u32) -> u32 {
    return 0
}

// Same for 64 bit integers. |value| must be <= 2^63
#[allow(unused_variables)]
pub fn round_up_to_power_of_two_64(value: u64) -> u64 {
    return 0
}

#[allow(unused_variables)]
pub fn round_up_to_power_of_two(value: usize) -> usize {
    return 0
}

// Returns the greatest power of two which is less than or
// equal to |value|. If you pass in a |value| that is already
// a power of two, it is returned as is.
pub fn round_down_to_power_of_two_32(value: u32) -> u32 {
    if value > 0x80000000 {
        return 0x80000000
    }
    let mut result: u32 = round_up_to_power_of_two_32(value);
    if result > value {
        result >>= 1
    }
    return result
}

// Precondition: 0 <= shift < 32
pub fn rotate_right_32(value: u32, shift: u32) -> u32 {
    return (value >> shift) | (value << ((32 - shift) & 31))
}

// Precondition: 0 <= shift < 32
pub fn rotate_left_32(value: u32, shift: u32) -> u32 {
    return (value << shift) | (value >> ((32 - shift) & 31))
}

// Precondition: 0 <= shift < 64
pub fn rotate_right_64(value: u64, shift: u64) -> u64 {
    return (value >> shift) | (value << ((64 - shift) & 63))
}

// Precondition: 0 <= shift < 64
pub fn rotate_left_64(value: u64, shift: u64) -> u64 {
    return (value << shift) | (value >> ((64 - shift) & 63))
}

// Multiplies two signed 32-bit values |lhs| and |rhs|,
// extracts the most significant 32 bits of the result, and returns those.
#[allow(unused_variables)]
pub fn signed_mul_high_32(lhs: i32, rhs: i32) -> i32 {
    0
}

// Multiplies two signed 32-bit values |lhs| and |rhs|,
// extracts the most significant 32 bits of the result, and adds the accumulate value |acc|.
#[allow(unused_variables)]
pub fn signed_mul_high_and_add_32(lhs: i32, rhs: i32, acc: i32) -> i32 {
    0
}

// Divides |lhs| by |rhs| and returns the quotient truncated i32.
// If |rhs| is zero, then zero is returned.
// If |lhs| is minint and |rhs| is -1, it returns minint.
pub fn signed_div_32(lhs: i32, rhs: i32) -> i32 {
    if rhs == 0 {
        return 0
    }
    if rhs == -1 {
        if lhs == i32::MIN {
            return lhs
        } else {
            return -lhs
        }
    }
    return lhs / rhs
}

// Divides |lhs| by |rhs| and returns the remainder truncated i32.
// If either |rhs| is zero or |lhs| is minint and |rhs| is -1, it returns zero.
pub fn signed_mod_32(lhs: i32, rhs: i32) -> i32 {
    if rhs == 0 || rhs == -1 {
        return 0
    }
    return lhs % rhs
}

// Divides |lhs| by |rhs| and returns the quotient truncated to u32.
// If |rhs| is zero, then zero is returned.
pub fn unsigned_div_32(lhs: u32, rhs: u32) -> u32 {
    if rhs != 0 {
        return lhs / rhs  
    } else {
        return 0
    }
}

// Divides |lhs| by |rhs| and returns the remainder truncated to u32.
// If |rhs| is zero, then zero is returned.
pub fn unsigned_mod_32(lhs: u32, rhs: u32) -> u32 {
    if rhs != 0 {
        return lhs % rhs
    } else {
        return 0
    }
}

// Adds |lhs| and |rhs|, checks and returns the result.
pub fn signed_saturated_add_64(lhs: i64, rhs: i64) -> i64 {
    if rhs < 0 && lhs < i64::MIN - rhs {
        return i64::MIN
    }
    if rhs >= 0 && lhs > i64::MAX  - rhs {
        return i64::MAX
    }
    return lhs + rhs
}

// Subtracts |lhs| and |rhs|, checks and returns the result.
pub fn signed_saturated_sub_64(lhs: i64, rhs: i64) -> i64 {
    if rhs > 0 && lhs < i64::MIN + rhs {
        return i64::MIN
    }
    if rhs <= 0 && lhs > i64::MAX  + rhs {
        return i64::MAX
    }
    return lhs - rhs
}