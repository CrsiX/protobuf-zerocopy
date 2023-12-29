pub trait NumFromBytes<const N: usize> {
    fn from_be_bytes(bytes: [u8; N]) -> Self;
}

macro_rules! impl_num_from_bytes {
    ($($num:ty),+) => {$(
        impl NumFromBytes<{ ::std::mem::size_of::<$num>() }> for $num {
            fn from_be_bytes(bytes: [u8; { ::std::mem::size_of::<$num>() }]) -> Self {
                Self::from_be_bytes(bytes)
            }
        }
    )+};
}

impl_num_from_bytes!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
