use bitpiece::*;

#[bitpiece(2)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BitPieceEnum {
    Variant0 = 0,
    Variant1 = 1,
    Variant2 = 2,
    Variant3 = 3,
}

#[bitpiece(3)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitPieceA {
    x: bool,
    y: BitPieceEnum,
}

#[bitpiece(35)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitPieceB {
    x: u32,
    y: B3,
}

#[bitpiece(38)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitPieceComplex {
    a: BitPieceA,
    b: BitPieceB,
}

#[test]
fn bit_extraction() {
    assert_eq!(BitPieceA::from_bits(0b001).x(), true);
    assert_eq!(BitPieceA::from_bits(0b111).x(), true);
    assert_eq!(BitPieceA::from_bits(0b000).x(), false);
    assert_eq!(BitPieceA::from_bits(0b110).x(), false);

    assert_eq!(BitPieceA::from_bits(0b110).y(), BitPieceEnum::Variant3);
    assert_eq!(BitPieceA::from_bits(0b101).y(), BitPieceEnum::Variant2);
}

#[test]
fn bit_modification() {
    let mut value = BitPieceA::zeroes();
    assert_eq!(value.x(), false);
    assert_eq!(value.y(), BitPieceEnum::Variant0);
    assert_eq!(value.storage, 0);

    value.set_x(true);
    assert_eq!(value.x(), true);
    assert_eq!(value.y(), BitPieceEnum::Variant0);
    assert_eq!(value.storage, 0b001);

    value.set_y(BitPieceEnum::Variant3);
    assert_eq!(value.x(), true);
    assert_eq!(value.y(), BitPieceEnum::Variant3);
    assert_eq!(value.storage, 0b111);

    value.set_x(false);
    assert_eq!(value.x(), false);
    assert_eq!(value.y(), BitPieceEnum::Variant3);
    assert_eq!(value.storage, 0b110);
}

#[test]
fn test_b_types_max_value() {
    assert_eq!(B3::MAX.get(), 0b111);
    assert_eq!(B13::MAX.get(), 0b1111111111111);
    assert_eq!(B8::MAX.get(), u8::MAX);
    assert_eq!(B16::MAX.get(), u16::MAX);
}

#[test]
fn test_b_types_enforce_length() {
    assert!(B3::try_new(0b000).is_some());
    assert!(B3::try_new(0b111).is_some());
    assert!(B3::try_new(0b1000).is_none());
    assert!(B3::try_new(0b10000011).is_none());
}

#[test]
fn test_b_types_enforce_length_in_from_bits() {
    expect_panic(|| {
        let _ = B3::from_bits(0b1000);
    });
    expect_panic(|| {
        let _ = B3::from_bits(0b10001011);
    });
}

#[test]
fn from_to_fields() {
    let fields = BitPieceComplexFields {
        a: BitPieceAFields {
            x: true,
            y: BitPieceEnum::Variant1,
        },
        b: BitPieceBFields {
            x: 0b1111100,
            y: B3::new(0b010),
        },
    };
    let value = BitPieceComplex::from_fields(fields);

    assert_eq!(value.a().x(), true);
    assert_eq!(value.a().y(), BitPieceEnum::Variant1);
    assert_eq!(value.a().storage, 0b011);
    assert_eq!(value.b().x(), 0b1111100);
    assert_eq!(value.b().y(), B3::new(0b010));
    assert_eq!(value.b().storage, 0b01000000000000000000000000001111100);
    assert_eq!(value.storage, 0b01000000000000000000000000001111100011);

    assert_eq!(value.to_fields(), fields);
}

#[test]
fn zeroed() {
    let zeroed = BitPieceComplex::zeroes();
    assert_eq!(zeroed.storage, 0);
}

#[test]
fn bit_extraction_noshift() {
    assert_eq!(BitPieceA::from_bits(0b001).x_noshift(), 0b001);
    assert_eq!(BitPieceA::from_bits(0b111).x_noshift(), 0b001);
    assert_eq!(BitPieceA::from_bits(0b000).x_noshift(), 0b000);
    assert_eq!(BitPieceA::from_bits(0b110).x_noshift(), 0b000);

    assert_eq!(BitPieceA::from_bits(0b110).y_noshift(), 0b110,);
    assert_eq!(BitPieceA::from_bits(0b101).y_noshift(), 0b100,);
}

#[bitpiece]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NonExhaustiveEnum {
    Variant0 = 0,
    Variant77 = 77,
    Variant120 = 120,
}

#[test]
fn valid_variants_of_non_exhastive_enum() {
    assert_eq!(NonExhaustiveEnum::from_bits(0), NonExhaustiveEnum::Variant0);
    assert_eq!(
        NonExhaustiveEnum::from_bits(77),
        NonExhaustiveEnum::Variant77
    );
    assert_eq!(
        NonExhaustiveEnum::from_bits(120),
        NonExhaustiveEnum::Variant120
    );
}

#[test]
fn invalid_variant_of_non_exhastive_enum() {
    expect_panic(|| {
        let _ = NonExhaustiveEnum::from_bits(55);
    });
    expect_panic(|| {
        let _ = NonExhaustiveEnum::from_bits(127);
    });
    expect_panic(|| {
        let _ = NonExhaustiveEnum::from_bits(20);
    });
}

#[test]
fn non_exhastive_enum_try_from_bits() {
    assert_eq!(
        NonExhaustiveEnum::try_from_bits(120),
        Some(NonExhaustiveEnum::Variant120)
    );
    assert_eq!(NonExhaustiveEnum::try_from_bits(93), None);
}

#[test]
fn exhastive_enum_try_from_bits() {
    assert_eq!(BitPieceEnum::try_from_bits(0), Some(BitPieceEnum::Variant0));
    assert_eq!(BitPieceEnum::try_from_bits(1), Some(BitPieceEnum::Variant1));
    assert_eq!(BitPieceEnum::try_from_bits(2), Some(BitPieceEnum::Variant2));
    assert_eq!(BitPieceEnum::try_from_bits(3), Some(BitPieceEnum::Variant3));
}

#[test]
fn enum_try_from_bits_out_of_range() {
    assert_eq!(BitPieceEnum::try_from_bits(4), None);
    assert_eq!(BitPieceEnum::try_from_bits(255), None);

    assert_eq!(NonExhaustiveEnum::try_from_bits(194), None);
    assert_eq!(BitPieceEnum::try_from_bits(255), None);
}

#[test]
fn b_types_try_from_bits() {
    assert_eq!(B1::try_from_bits(0), Some(B1::new(0)));
    assert_eq!(B1::try_from_bits(1), Some(B1::new(1)));
    assert_eq!(B1::try_from_bits(2), None);

    assert_eq!(B6::try_from_bits(17), Some(B6::new(17)));
    assert_eq!(B6::try_from_bits(2), Some(B6::new(2)));
    assert_eq!(B6::try_from_bits(241), None);
}

#[bitpiece(16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NonExhaustiveEnumExplicitBitLen {
    Variant0 = 0,
    Variant77 = 77,
    Variant120 = 120,
}

#[test]
fn non_exhaustive_enum_explicit_bit_len() {
    // make sure that the regular stuff works as expected
    assert_eq!(
        NonExhaustiveEnumExplicitBitLen::from_bits(77),
        NonExhaustiveEnumExplicitBitLen::Variant77
    );

    // make sure that we can pass 16 bit values
    assert_eq!(NonExhaustiveEnumExplicitBitLen::try_from_bits(1500), None);
}

#[bitpiece(12)]
struct NonExhaustiveEnumContainer1 {
    a: B2,
    b: NonExhaustiveEnum,
    c: B3,
}

#[bitpiece(27)]
struct NonExhaustiveEnumContainer2 {
    a: u8,
    b: NonExhaustiveEnumContainer1,
    c: B7,
}

#[test]
fn non_exhaustive_enum_container() {
    assert_eq!(
        NonExhaustiveEnumContainer2::from_bits(0).b().b(),
        NonExhaustiveEnum::Variant0
    );
    assert_eq!(
        NonExhaustiveEnumContainer2::try_from_bits(0)
            .unwrap()
            .b()
            .b(),
        NonExhaustiveEnum::Variant0
    );

    assert_eq!(
        NonExhaustiveEnumContainer2::from_bits(77 << 10).b().b(),
        NonExhaustiveEnum::Variant77
    );
    assert_eq!(
        NonExhaustiveEnumContainer2::try_from_bits(77 << 10)
            .unwrap()
            .b()
            .b(),
        NonExhaustiveEnum::Variant77
    );

    assert_eq!(NonExhaustiveEnumContainer2::try_from_bits(60 << 10), None);
    assert_eq!(NonExhaustiveEnumContainer2::try_from_bits(25 << 10), None);
    expect_panic(|| {
        let _ = NonExhaustiveEnumContainer2::from_bits(25 << 10);
    });
    expect_panic(|| {
        let _ = NonExhaustiveEnumContainer2::from_bits(26 << 10);
    });
}

#[bitpiece(12)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StructWithSigned {
    a: B3,
    b: i8,
    c: bool,
}

#[bitpiece(51)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StructWithSigned2 {
    a: i16,
    b: bool,
    c: i32,
    d: B2,
}

#[test]
fn signed_i8_extraction() {
    // Test with a negative i8 value.
    // Bit layout (LSB to MSB): [a: B3 (3), b: i8 (8), c: bool (1)]
    // a = 0b101 (5)
    // b = 0b11111111 (-1 in two's complement for i8)
    // c = true (1)
    let raw_val1 = 0b1_11111111_101;
    let value1 = StructWithSigned::from_bits(raw_val1);
    assert_eq!(value1.a(), B3::new(5));
    assert_eq!(value1.b(), -1i8);
    assert_eq!(value1.c(), true);

    // Test with a positive i8 value.
    // a = 0b010 (2)
    // b = 0b00000111 (7 in two's complement for i8)
    // c = false (0)
    let raw_val2 = 0b0_00000111_010;
    let value2 = StructWithSigned::from_bits(raw_val2);
    assert_eq!(value2.a(), B3::new(2));
    assert_eq!(value2.b(), 7i8);
    assert_eq!(value2.c(), false);

    // Test with i8 minimum value.
    // a = 0b111 (7)
    // b = 0b10000000 (-128 in two's complement for i8)
    // c = true (1)
    let raw_val3 = 0b1_10000000_111;
    let value3 = StructWithSigned::from_bits(raw_val3);
    assert_eq!(value3.a(), B3::new(7));
    assert_eq!(value3.b(), -128i8);
    assert_eq!(value3.c(), true);
}

#[test]
fn signed_i8_modification() {
    let mut value = StructWithSigned::zeroes();
    assert_eq!(value.storage, 0);
    assert_eq!(value.a(), B3::new(0));
    assert_eq!(value.b(), 0i8);
    assert_eq!(value.c(), false);

    // Set a negative value
    value.set_b(-1);
    assert_eq!(value.b(), -1i8);
    assert_eq!(value.c(), false);
    assert_eq!(value.a(), B3::new(0));
    // Storage should be the i8 value (-1, which is 0xFF) shifted by 3 (for field `a`)
    assert_eq!(value.storage, (0b11111111u16) << 3);

    // Set other fields
    value.set_a(B3::new(0b101));
    value.set_c(true);
    assert_eq!(value.b(), -1i8);
    assert_eq!(value.a(), B3::new(5));
    assert_eq!(value.c(), true);
    assert_eq!(value.storage, 0b1_11111111_101);

    // Change the signed value back to positive
    value.set_b(42);
    assert_eq!(value.b(), 42i8);
    assert_eq!(value.a(), B3::new(5));
    assert_eq!(value.c(), true);
    assert_eq!(value.storage, 0b1_00101010_101);
}

#[test]
fn signed_from_to_fields() {
    // Bit layout (LSB to MSB): [a: i16 (16), b: bool (1), c: i32 (32), d: B2 (2)]
    let fields = StructWithSigned2Fields {
        a: -20000, // i16
        b: true,
        c: 1_000_000_000, // i32
        d: B2::new(3),
    };
    let value = StructWithSigned2::from_fields(fields);

    // Check that the values were stored correctly
    assert_eq!(value.a(), -20000i16);
    assert_eq!(value.b(), true);
    assert_eq!(value.c(), 1_000_000_000i32);
    assert_eq!(value.d(), B2::new(3));

    // Check that converting back yields the same fields
    let converted_fields = value.to_fields();
    assert_eq!(fields, converted_fields);
}

// Bit layout (LSB to MSB): [a: SB5 (5), b: bool (1), c: SB7 (7), d: B3 (3)]
#[bitpiece(16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StructWithSb {
    a: SB5,
    b: bool,
    c: SB7,
    d: B3,
}

#[test]
fn sb_type_extraction() {
    // Test with a mix of positive and negative SB* values.
    // a = -1 (0b11111)
    // b = true (1)
    // c = 42 (0b0101010)
    // d = 5 (0b101)
    let raw_val1 = 0b101_0101010_1_11111;
    let value1 = StructWithSb::from_bits(raw_val1);
    assert_eq!(value1.a(), SB5::new(-1));
    assert_eq!(value1.b(), true);
    assert_eq!(value1.c(), SB7::new(42));
    assert_eq!(value1.d(), B3::new(5));

    // Test with another set of values.
    // a = 15 (0b01111) (Max positive for SB5)
    // b = false (0)
    // c = -60 (0b1000100)
    // d = 0 (0b000)
    let raw_val2 = 0b000_1000100_0_01111;
    let value2 = StructWithSb::from_bits(raw_val2);
    assert_eq!(value2.a(), SB5::new(15));
    assert_eq!(value2.b(), false);
    assert_eq!(value2.c(), SB7::new(-60));
    assert_eq!(value2.d(), B3::new(0));

    // Test with minimum values.
    // a = -16 (0b10000) (Min for SB5)
    // b = true (1)
    // c = -64 (0b1000000) (Min for SB7)
    // d = 7 (0b111)
    let raw_val3 = 0b111_1000000_1_10000;
    let value3 = StructWithSb::from_bits(raw_val3);
    assert_eq!(value3.a(), SB5::new(-16));
    assert_eq!(value3.b(), true);
    assert_eq!(value3.c(), SB7::new(-64));
    assert_eq!(value3.d(), B3::new(7));
}

#[test]
fn sb_type_modification() {
    let mut value = StructWithSb::zeroes();
    assert_eq!(value.storage, 0);
    assert_eq!(value.a(), SB5::new(0));
    assert_eq!(value.b(), false);
    assert_eq!(value.c(), SB7::new(0));
    assert_eq!(value.d(), B3::new(0));

    // Set a negative value for 'a'
    value.set_a(SB5::new(-5));
    assert_eq!(value.a(), SB5::new(-5));
    assert_eq!(value.b(), false);
    assert_eq!(value.c(), SB7::new(0));
    assert_eq!(value.d(), B3::new(0));
    assert_eq!(value.storage, 0b000_0000000_0_11011);

    // Set a positive value for 'c'
    value.set_c(SB7::new(21));
    assert_eq!(value.a(), SB5::new(-5));
    assert_eq!(value.c(), SB7::new(21));
    assert_eq!(value.storage, 0b000_0010101_0_11011);

    // Set other fields
    value.set_b(true);
    value.set_d(B3::new(4));
    assert_eq!(value.a(), SB5::new(-5));
    assert_eq!(value.b(), true);
    assert_eq!(value.c(), SB7::new(21));
    assert_eq!(value.d(), B3::new(4));
    assert_eq!(value.storage, 0b100_0010101_1_11011);

    // Change 'a' back to a positive value
    value.set_a(SB5::new(10));
    assert_eq!(value.a(), SB5::new(10));
    assert_eq!(value.storage, 0b100_0010101_1_01010);
}

#[test]
fn sb_type_from_to_fields() {
    let fields = StructWithSbFields {
        a: SB5::new(-10),
        b: true,
        c: SB7::new(55),
        d: B3::new(6),
    };
    let value = StructWithSb::from_fields(fields);

    // Check that the values were stored correctly
    assert_eq!(value.a(), SB5::new(-10));
    assert_eq!(value.b(), true);
    assert_eq!(value.c(), SB7::new(55));
    assert_eq!(value.d(), B3::new(6));

    // Check that converting back yields the same fields
    let converted_fields = value.to_fields();
    assert_eq!(fields, converted_fields);
}

// Bit layout (LSB to MSB): [a: B2 (2), b: SB3 (3), c: B2 (2)]
#[bitpiece(7)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StructWithSbTryFrom {
    a: B2,
    b: SB3,
    c: B2,
}

#[test]
fn sb_type_try_from_bits() {
    // Valid case
    // a = 1, b = -2 (0b110), c = 2
    let raw_valid = 0b10_110_01;
    let value = StructWithSbTryFrom::try_from_bits(raw_valid).unwrap();
    assert_eq!(value.a(), B2::new(1));
    assert_eq!(value.b(), SB3::new(-2));
    assert_eq!(value.c(), B2::new(2));

    // `try_from_bits` should succeed because SB3 can represent all values from 0..=7
    // when treated as unsigned bits, even though they map to signed values.
    // The check for validity happens inside the non-exhaustive enum or a similar construct,
    // not for the SB type itself within a container's try_from_bits.
    // Let's test the component `try_from_bits` directly.
    assert_eq!(SB3::try_from_bits(0b000).unwrap().get(), 0); // 0
    assert_eq!(SB3::try_from_bits(0b001).unwrap().get(), 1); // 1
    assert_eq!(SB3::try_from_bits(0b011).unwrap().get(), 3); // 3
    assert_eq!(SB3::try_from_bits(0b100).unwrap().get(), -4); // -4
    assert_eq!(SB3::try_from_bits(0b111).unwrap().get(), -1); // -1

    // However, if we had a non-exhaustive enum, an invalid variant would cause None.
    // Let's create a struct that contains one to test the interaction.
}

#[test]
fn test_sb_types_max_value() {
    assert_eq!(SB3::MAX.get(), 0b011);
    assert_eq!(SB13::MAX.get(), 0b0111111111111);
    assert_eq!(SB8::MAX.get(), i8::MAX);
    assert_eq!(SB16::MAX.get(), i16::MAX);
}

#[test]
fn test_sb_types_min_value() {
    assert_eq!(SB3::MIN.get(), -4);
    assert_eq!(SB13::MIN.get(), -4096);
    assert_eq!(SB8::MIN.get(), i8::MIN);
    assert_eq!(SB16::MIN.get(), i16::MIN);
}

#[test]
fn test_sb_types_enforce_length() {
    // SB3 has a max of 3 and a min of -4
    assert!(SB3::try_new(0).is_some());
    assert!(SB3::try_new(3).is_some());
    assert!(SB3::try_new(-2).is_some());
    assert!(SB3::try_new(-4).is_some());

    assert!(SB3::try_new(4).is_none());
    assert!(SB3::try_new(-5).is_none());
    assert!(SB3::try_new(16).is_none());
    assert!(SB3::try_new(-128).is_none());
    assert!(SB3::try_new(127).is_none());

    assert!(SB3::try_from_bits(0b1000).is_none());
    assert!(SB3::try_from_bits(0b10001011).is_none());
}

pub fn expect_panic<F: FnOnce() + std::panic::UnwindSafe>(f: F) {
    let result = std::panic::catch_unwind(f);
    result.expect_err("expected the code to panic");
}
