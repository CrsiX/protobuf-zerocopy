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
    run_test!(0, [], 0, None::<u64>, decode_var_int);
    run_test!(0, [], 0, None::<i32>, decode_var_int);
    run_test!(0, [], 0, None::<i128>, decode_var_int);
    run_test!(1, [0x07], 1, Some(7), decode_var_int);
    run_test!(1, [0x00], 1, Some(0), decode_var_int);
    run_test!(2, [0x00, 0x00], 1, Some(0), decode_var_int);
    run_test!(2, [0x07, 0x01], 1, Some(7), decode_var_int);
    run_test!(2, [0xf7, 0x01], 2, Some(118), decode_var_int);
    // run_test!(
    //     10,
    //     [0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
    //     10,
    //     Some(0xfffffffffffffffe),
    //     decode_var_int
    // );
}

#[test]
fn test_wire_tags() {
    run_test!(0, [], 0, None, decode_tag);
    run_test!(1, [0x1f], 0, None, decode_tag);
    run_test!(
        1,
        [0x22],
        1,
        Some((WireType::LengthDelimited, 4)),
        decode_tag
    );
    run_test!(
        2,
        [0x22, 0x61],
        1,
        Some((WireType::LengthDelimited, 4)),
        decode_tag
    );
    run_test!(
        1,
        [0x32],
        1,
        Some((WireType::LengthDelimited, 6)),
        decode_tag
    );
    run_test!(
        1,
        [0x12],
        1,
        Some((WireType::LengthDelimited, 2)),
        decode_tag
    );
    run_test!(1, [0x50], 1, Some((WireType::VarInt, 10)), decode_tag);
    run_test!(2, [0x50, 0x82], 1, Some((WireType::VarInt, 10)), decode_tag);
}

#[test]
fn test_wire_tag_4() {
    let buffer: [u8; 1] = [0x07];
    let mut slice = buffer.as_slice();
    let backup = slice;
    let result = decode_tag(&mut slice);
    assert_eq!(result, None);
    assert_eq!(slice, backup);
}

#[test]
fn test_var_length_1() {
    let buffer: [u8; 0] = [];
    let mut slice = buffer.as_slice();
    let result = decode_var_length(&mut slice);
    assert_eq!(result, None);
}

#[test]
fn test_var_length_2() {
    let buffer: [u8; 9] = [0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
    let mut slice = buffer.as_slice();
    let result = decode_var_length(&mut slice);
    assert_eq!(
        result,
        Some([0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67].as_slice())
    );
}
