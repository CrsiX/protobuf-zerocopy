use super::*;

macro_rules! run_test {
    ($buffer_len:expr, $buffer:expr, $offset:expr, $expected:expr, $function:ident) => {
        let buffer: [u8; $buffer_len] = $buffer;
        let mut slice = $buffer.as_slice();
        let backup = slice.get($offset..).unwrap();
        let result = $function(&mut slice);
        assert_eq!(result, $expected);
        assert_eq!(slice, backup);
    };
}

#[test]
fn test_var_ints() {
    run_test!(
        0,
        [],
        0,
        Err::<u128, errors::ProtobufZeroError>(errors::ProtobufZeroError::ShortBuffer),
        decode_var_int
    );
    run_test!(1, [0x07], 1, Ok(7), decode_var_int);
    run_test!(1, [0x1a], 1, Ok(26), decode_var_int);
    run_test!(1, [0x00], 1, Ok(0), decode_var_int);
    run_test!(1, [0x7f], 1, Ok(127), decode_var_int);
    run_test!(1, [0x2a], 1, Ok(42), decode_var_int);
    run_test!(2, [0x80, 0x01], 2, Ok(128), decode_var_int);
    run_test!(2, [0x81, 0x01], 2, Ok(129), decode_var_int);
    run_test!(2, [0x83, 0x01], 2, Ok(131), decode_var_int);
    run_test!(2, [0x80, 0x02], 2, Ok(256), decode_var_int);
    run_test!(2, [0xff, 0x03], 2, Ok(511), decode_var_int);
    run_test!(2, [0xff, 0x1f], 2, Ok(4095), decode_var_int);
    run_test!(2, [0x81, 0x77], 2, Ok(15233), decode_var_int);
    run_test!(3, [0x81, 0x77, 0x01], 2, Ok(15233), decode_var_int);
    run_test!(2, [0x80, 0x7f], 2, Ok(16256), decode_var_int);
    run_test!(2, [0x8f, 0x7f], 2, Ok(16271), decode_var_int);
    run_test!(3, [0x80, 0x80, 0x01], 3, Ok(16384), decode_var_int);
    run_test!(
        6,
        [0x80, 0x80, 0x01, 0x01, 0x00, 0xff],
        3,
        Ok(16384),
        decode_var_int
    );
    run_test!(
        4,
        [0x81, 0xfb, 0xf1, 0x77],
        4,
        Ok(251428225),
        decode_var_int
    );
    run_test!(
        6,
        [0xc9, 0x98, 0xe6, 0xe9, 0x9b, 0x05],
        6,
        Ok(179268324425),
        decode_var_int
    );
    run_test!(
        7,
        [0xe7, 0xaf, 0xb4, 0xd8, 0xfd, 0xff, 0x3f],
        7,
        Ok(281474356811751),
        decode_var_int
    );
    run_test!(
        7,
        [0xf3, 0xfe, 0xef, 0xb3, 0xd3, 0xfc, 0x59],
        7,
        Ok(395709135978355),
        decode_var_int
    );
    run_test!(
        7,
        [0xdf, 0xb4, 0xd8, 0xbd, 0xfe, 0xff, 0x6c],
        7,
        Ok(479386662214239),
        decode_var_int
    );
    run_test!(
        8,
        [0xb7, 0xf3, 0xa7, 0xbd, 0xbf, 0x9d, 0xa5, 0x46],
        8,
        Ok(39570237932829111),
        decode_var_int
    );
    run_test!(
        8,
        [0x82, 0x80, 0x87, 0x86, 0x80, 0x80, 0x80, 0x40],
        8,
        Ok(36028797031661570),
        decode_var_int
    );
    run_test!(
        9,
        [0xc7, 0xe3, 0x81, 0xa3, 0xf8, 0xde, 0xaf, 0xc5, 0x01],
        9,
        Ok(111111111111111111),
        decode_var_int
    );
    run_test!(
        9,
        [0xe0, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x40],
        9,
        Ok(4611686018427388000),
        decode_var_int
    );
    run_test!(
        10,
        [0xc0, 0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        10,
        Ok(9223372036854776000),
        decode_var_int
    );
    run_test!(
        10,
        [0xc1, 0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        10,
        Ok(9223372036854776001),
        decode_var_int
    );
    run_test!(
        1,
        [0xf7],
        0,
        Err::<u128, errors::ProtobufZeroError>(errors::ProtobufZeroError::ShortBuffer),
        decode_var_int
    );
    run_test!(
        3,
        [0xf7, 0xf1, 0xb1],
        0,
        Err::<u128, errors::ProtobufZeroError>(errors::ProtobufZeroError::ShortBuffer),
        decode_var_int
    );
}

#[test]
fn test_wire_tags() {
    run_test!(
        0,
        [],
        0,
        Err::<(WireType, u64), errors::ProtobufZeroError>(errors::ProtobufZeroError::ShortBuffer),
        decode_tag
    );
    run_test!(1, [0x22], 1, Ok((WireType::LengthDelimited, 4)), decode_tag);
    run_test!(
        1,
        [0x1f],
        0,
        Err::<(WireType, u64), errors::ProtobufZeroError>(
            errors::ProtobufZeroError::InvalidWireType
        ),
        decode_tag
    );
    run_test!(
        2,
        [0x22, 0x61],
        1,
        Ok((WireType::LengthDelimited, 4)),
        decode_tag
    );
    run_test!(1, [0x00], 1, Ok((WireType::VarInt, 0)), decode_tag);
    run_test!(1, [0x32], 1, Ok((WireType::LengthDelimited, 6)), decode_tag);
    run_test!(1, [0x12], 1, Ok((WireType::LengthDelimited, 2)), decode_tag);
    run_test!(1, [0x50], 1, Ok((WireType::VarInt, 10)), decode_tag);
    run_test!(2, [0x50, 0x82], 1, Ok((WireType::VarInt, 10)), decode_tag);
    run_test!(
        5,
        [0xa8, 0xd1, 0xf9, 0xd6, 0x03],
        5,
        Ok((WireType::VarInt, 123456789)),
        decode_tag
    );
}

#[test]
fn test_var_signed_i64() {
    run_test!(1, [0x00], 1, Ok(0), decode_var_signed_i64);
    run_test!(1, [0x02], 1, Ok(1), decode_var_signed_i64);
    run_test!(1, [0x04], 1, Ok(2), decode_var_signed_i64);
    run_test!(1, [0x28], 1, Ok(20), decode_var_signed_i64);
    run_test!(1, [0x01], 1, Ok(-1), decode_var_signed_i64);
    run_test!(1, [0x03], 1, Ok(-2), decode_var_signed_i64);
    run_test!(1, [0x05], 1, Ok(-3), decode_var_signed_i64);
    run_test!(1, [0x17], 1, Ok(-12), decode_var_signed_i64);
    run_test!(2, [0x89, 0x01], 2, Ok(-69), decode_var_signed_i64);
    run_test!(2, [0x81, 0x02], 2, Ok(-129), decode_var_signed_i64);
    run_test!(2, [0xc5, 0x06], 2, Ok(-419), decode_var_signed_i64);
    run_test!(2, [0xc8, 0x06], 2, Ok(420), decode_var_signed_i64);
    run_test!(
        7,
        [0x06, 0x82, 0x81, 0xf3, 0x91, 0x82, 0x23],
        1,
        Ok(3),
        decode_var_signed_i64
    );
}

#[test]
fn test_var_lengths() {
    run_test!(
        0,
        [],
        0,
        Err::<&[u8], errors::ProtobufZeroError>(errors::ProtobufZeroError::EmptyBuffer),
        decode_var_length
    );
    run_test!(
        9,
        [0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67],
        9,
        Ok([0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67].as_slice()),
        decode_var_length
    );
}
