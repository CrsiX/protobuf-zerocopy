use thiserror::Error;

/// All "wire types" that are supported
#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum WireType {
    VarInt = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

impl TryFrom<u8> for WireType {
    type Error = WireTypeError;

    fn try_from(value: u8) -> Result<Self, WireTypeError> {
        match value {
            0 => Ok(WireType::VarInt),
            1 => Ok(WireType::Fixed64),
            2 => Ok(WireType::LengthDelimited),
            5 => Ok(WireType::Fixed32),
            3 | 4 => Err(WireTypeError::Deprecated(value)),
            _ => Err(WireTypeError::Unknown(value)),
        }
    }
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum WireTypeError {
    #[error("Encountered unknown wire type: {}", .0)]
    Unknown(u8),
    #[error("Encountered deprecated wire type: {}", .0)]
    Deprecated(u8),
}
