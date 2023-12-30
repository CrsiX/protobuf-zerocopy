use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum ProtobufZeroError {
    InvalidWireType,
    EmptyBuffer,
    ShortBuffer,
    ConversionU128Error,
}

impl Display for ProtobufZeroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtobufZeroError::InvalidWireType => {
                write!(f, "Failed to parse WireType from protobuf")
            }
            ProtobufZeroError::EmptyBuffer => {
                write!(f, "Can not read beyond end of buffer")
            }
            ProtobufZeroError::ShortBuffer => {
                write!(f, "Expected more bytes but the buffer was too short")
            }
            ProtobufZeroError::ConversionU128Error => {
                write!(f, "Converting u128 to another type failed")
            }
        }
    }
}
