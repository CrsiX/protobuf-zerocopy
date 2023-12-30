use crate::wire_type::WireType;
pub mod wire_type;

#[cfg(test)]
mod tests;

use thiserror::Error;

use crate::wire_type::WireTypeError;

#[derive(Error, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ProtobufZeroError {
    #[error("{}", .0)]
    InvalidWireType(#[from] WireTypeError),
    #[error("Unexpected end of buffer")]
    ShortBuffer,
    #[error("The varint was to large")]
    ConversionU128Error,
}

/// Read a full var int (u128) from a buffer, modifying the input slice
pub fn decode_var_int_u128(buffer: &mut &[u8]) -> Result<u128, ProtobufZeroError> {
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

    *buffer = &*&buffer[offset..];

    Ok(result)
}

/// Reads a WireType and its tag ID (field number) from a buffer, modifying the input slice
#[inline]
pub fn decode_tag(buffer: &mut &[u8]) -> Result<(WireType, u32), ProtobufZeroError> {
    let full_var_int = decode_var_int_u128(buffer)?;
    let wire_type: WireType = ((full_var_int.clone() & 7) as u8).try_into()?;
    Ok((wire_type, (full_var_int >> 3) as u32))
}

/// Read a var int from a buffer, modifying the input slice
#[inline]
pub fn decode_var_int<T: TryFrom<u128>>(buffer: &mut &[u8]) -> Result<T, ProtobufZeroError> {
    let value = decode_var_int_u128(buffer)?;
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
    let result = decode_var_int_u128(buffer)?;
    Ok((result >> 1) as i64 ^ -((result & 1) as i64))
}

/// Read a var length signed int to i32, modifying the input slice
#[inline]
pub fn decode_var_signed_i32(buffer: &mut &[u8]) -> Result<i32, ProtobufZeroError> {
    let result = decode_var_int_u128(buffer)?;
    Ok((result >> 1) as i32 ^ -((result & 1) as i32))
}

/// Read a fixed size 64 bit number from a buffer, advancing the input slice
fn decode_fixed_64<T: NumBytes<8>>(buffer: &mut &[u8]) -> Result<T, ProtobufZeroError> {
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
    Ok(T::from_le_bytes(array))
}

/// Read a fixed size 32 bit number from a buffer, advancing the input slice
fn decode_fixed_32<T: NumBytes<4>>(buffer: &mut &[u8]) -> Result<T, ProtobufZeroError> {
    let slice = match buffer.get(..4) {
        None => return Err(ProtobufZeroError::ShortBuffer),
        Some(v) => v,
    };
    if (slice.len()) != 4 {
        return Err(ProtobufZeroError::ShortBuffer);
    }
    *buffer = &*&buffer[4..];
    let array = [slice[0], slice[1], slice[2], slice[3]];
    Ok(T::from_le_bytes(array))
}

pub trait NumBytes<const N: usize> {
    fn from_le_bytes(bytes: [u8; N]) -> Self;
    fn into_le_bytes(self) -> [u8; N];
}
macro_rules! impl_num_bytes {
    ($($num:ty),+) => {$(
        impl NumBytes<{ ::std::mem::size_of::<$num>() }> for $num {
            fn from_le_bytes(bytes: [u8; { ::std::mem::size_of::<$num>() }]) -> Self {
                Self::from_le_bytes(bytes)
            }
            fn into_le_bytes(self) -> [u8; { ::std::mem::size_of::<$num>() }] {
                self.to_le_bytes()
            }
        }
    )+};
}
impl_num_bytes!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
