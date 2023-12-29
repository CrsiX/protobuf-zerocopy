use std::fmt::Display;

/// All "wire types" that are supported
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WireType {
    VarInt = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

impl TryFrom<u8> for WireType {
    type Error = crate::errors::ProtobufZeroError;

    fn try_from(value: u8) -> Result<Self, crate::errors::ProtobufZeroError> {
        match value {
            0 => Ok(WireType::VarInt),
            1 => Ok(WireType::Fixed64),
            2 => Ok(WireType::LengthDelimited),
            5 => Ok(WireType::Fixed32),
            _ => Err(crate::errors::ProtobufZeroError::InvalidWireType),
        }
    }
}
