use crate::num_from_bytes::NumFromBytes;
use crate::wire_type::WireType;
use std::fs::{read, read_to_string};

pub mod errors;
mod num_from_bytes;
pub mod wire_type;

#[cfg(test)]
mod tests;

/// Reads a WireType and its tag ID (field number) from a buffer, modifying the input slice
pub fn decode_tag(buffer: &mut &[u8]) -> Result<(WireType, u64), errors::ProtobufZeroError> {
    let first = match buffer.get(0) {
        None => return Err(errors::ProtobufZeroError::EmptyBuffer),
        Some(v) => v,
    };
    let wire_type: WireType = first.try_into()?;
}

/// Read a var int from a buffer, modifying the input slice
pub fn decode_var_int<const N: usize, T: NumFromBytes<N>>(buffer: &mut &[u8]) -> Option<T> {
    None
}

/// Read a var length field from a buffer, modifying the input slice
pub fn decode_var_length<'a>(buffer: &mut &'a [u8]) -> Option<&'a [u8]> {
    None
}

/// Read a fixed-length i64 from a buffer, modifying the input slice
pub fn decode_fixed_i64(buffer: &mut &[u8]) -> Option<i64> {
    None
}

/// Read a fixed-length u64 from a buffer, modifying the input slice
pub fn decode_fixed_u64(buffer: &mut &[u8]) -> Option<u64> {
    None
}

/// Read a fixed-length f64 from a buffer, modifying the input slice
pub fn decode_fixed_f64(buffer: &mut &[u8]) -> Option<f64> {
    None
}

/// Read a fixed-length i32 from a buffer, modifying the input slice
pub fn decode_fixed_i32(buffer: &mut &[u8]) -> Option<i32> {
    None
}

/// Read a fixed-length u32 from a buffer, modifying the input slice
pub fn decode_fixed_u32(buffer: &mut &[u8]) -> Option<u32> {
    None
}

/// Read a fixed-length f32 from a buffer, modifying the input slice
pub fn decode_fixed_f32(buffer: &mut &[u8]) -> Option<f32> {
    None
}

/*
message BlobHeader {
   required string type = 1;
   optional bytes indexdata = 2;
   required int32 datasize = 3;
 }
 */
