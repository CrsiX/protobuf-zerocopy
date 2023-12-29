use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProtobufZeroError {
    InvalidWireType,
    EmptyBuffer,
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
        }
    }
}
