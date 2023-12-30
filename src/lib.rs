use crate::errors::ProtobufZeroError;
use crate::num_from_bytes::NumFromBytes;
use crate::wire_type::WireType;

pub mod errors;
pub(crate) mod num_from_bytes;
pub mod wire_type;

#[cfg(test)]
mod tests;

/// Read a full var int (u128) from a buffer, *not* modifying the input slice but instead returning the final offset
pub fn decode_var_int_u128(buffer: &[u8]) -> Result<(u128, usize), ProtobufZeroError> {
    let mut result: u128 = 0;
    let mut offset = 0;

    loop {
        let next = match buffer.get(offset) {
            None => return Err(ProtobufZeroError::ShortBuffer),
            Some(v) => v,
        };
        result += ((*next % 128u8) as u128) << (7 * offset);
        offset += 1;
        if (offset >= 10) || (next >> 7 == 0) {
            break;
        }
    }

    Ok((result, offset))
}

/// Reads a WireType and its tag ID (field number) from a buffer, modifying the input slice
#[inline]
pub fn decode_tag(buffer: &mut &[u8]) -> Result<(WireType, u32), ProtobufZeroError> {
    let (full_var_int, offset) = decode_var_int_u128(buffer)?;
    let wire_type: WireType = ((full_var_int.clone() & 7) as u8).try_into()?;
    *buffer = &*&buffer[offset..];
    Ok((wire_type, (full_var_int >> 3) as u32))
}

/// Read a var int from a buffer, modifying the input slice
#[inline]
pub fn decode_var_int<T: TryFrom<u128>>(buffer: &mut &[u8]) -> Result<T, ProtobufZeroError> {
    let (value, offset) = decode_var_int_u128(buffer)?;
    *buffer = &*&buffer[offset..];
    let result = match value.try_into() {
        Ok(v) => Ok(v),
        Err(_) => Err(ProtobufZeroError::ConversionU128Error),
    };
    result
}

/// Read a var length field from a buffer, modifying the input slice
///
/// If the length can be read but the buffer behind is not long enough, the slice will be advanced anyways
pub fn decode_var_length<'a>(buffer: &mut &'a [u8]) -> Result<&'a [u8], ProtobufZeroError> {
    let len: usize = decode_var_int(buffer)?;
    let slice = buffer.get(..len).ok_or(ProtobufZeroError::ShortBuffer)?;
    *buffer = &*&buffer[len..];
    Ok(slice)
}

/// Read a var length signed int to i64, modifying the input slice
#[inline]
pub fn decode_var_signed_i64(buffer: &mut &[u8]) -> Result<i64, ProtobufZeroError> {
    let (result, offset) = decode_var_int_u128(buffer)?;
    *buffer = &*&buffer[offset..];
    Ok((result >> 1) as i64 ^ -((result & 1) as i64))
}

/// Read a var length signed int to i32, modifying the input slice
#[inline]
pub fn decode_var_signed_i32(buffer: &mut &[u8]) -> Result<i32, ProtobufZeroError> {
    let (result, offset) = decode_var_int_u128(buffer)?;
    *buffer = &*&buffer[offset..];
    Ok((result >> 1) as i32 ^ -((result & 1) as i32))
}

/// Read a 64 bit array from a buffer, modifying the input slice
fn decode_fixed_64(buffer: &mut &[u8]) -> Result<[u8; 8], ProtobufZeroError> {
    let slice = match buffer.get(..8) {
        None => return Err(ProtobufZeroError::ShortBuffer),
        Some(v) => v,
    };
    if (slice.len()) != 8 {
        return Err(ProtobufZeroError::ShortBuffer);
    }
    *buffer = &*&buffer[8..];
    let array = [
        slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
    ];
    Ok(array)
}

/// Read a 32 bit array from a buffer, modifying the input slice
fn decode_fixed_32(buffer: &mut &[u8]) -> Result<[u8; 4], ProtobufZeroError> {
    let slice = match buffer.get(..4) {
        None => return Err(ProtobufZeroError::ShortBuffer),
        Some(v) => v,
    };
    if (slice.len()) != 4 {
        return Err(ProtobufZeroError::ShortBuffer);
    }
    *buffer = &*&buffer[4..];
    let array = [slice[0], slice[1], slice[2], slice[3]];
    Ok(array)
}

/// Read a fixed-length i64 from a buffer, modifying the input slice
#[inline]
pub fn decode_fixed_i64(buffer: &mut &[u8]) -> Result<i64, ProtobufZeroError> {
    let array = decode_fixed_64(buffer)?;
    Ok(i64::from_le_bytes(array))
}

/// Read a fixed-length u64 from a buffer, modifying the input slice
#[inline]
pub fn decode_fixed_u64(buffer: &mut &[u8]) -> Result<u64, ProtobufZeroError> {
    let array = decode_fixed_64(buffer)?;
    Ok(u64::from_le_bytes(array))
}

/// Read a fixed-length f64 from a buffer, modifying the input slice
#[inline]
pub fn decode_fixed_f64(buffer: &mut &[u8]) -> Result<f64, ProtobufZeroError> {
    let array = decode_fixed_64(buffer)?;
    Ok(f64::from_le_bytes(array))
}

/// Read a fixed-length i32 from a buffer, modifying the input slice
#[inline]
pub fn decode_fixed_i32(buffer: &mut &[u8]) -> Result<i32, ProtobufZeroError> {
    let array = decode_fixed_32(buffer)?;
    Ok(i32::from_le_bytes(array))
}

/// Read a fixed-length u32 from a buffer, modifying the input slice
#[inline]
pub fn decode_fixed_u32(buffer: &mut &[u8]) -> Result<u32, ProtobufZeroError> {
    let array = decode_fixed_32(buffer)?;
    Ok(u32::from_le_bytes(array))
}

/// Read a fixed-length f32 from a buffer, modifying the input slice
#[inline]
pub fn decode_fixed_f32(buffer: &mut &[u8]) -> Result<f32, ProtobufZeroError> {
    let array = decode_fixed_32(buffer)?;
    Ok(f32::from_le_bytes(array))
}
