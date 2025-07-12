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

#[bitpiece(16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitPieceMixed {
    a_u3: B3,
    b_s5: SB5,
    c_s8: SB8,
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

    assert_eq!(BitPieceMixed::from_bits(0b10000000_01111_111).a_u3().get(), 7);
    assert_eq!(BitPieceMixed::from_bits(0b10000000_01111_111).b_s5().get(), 15);
    assert_eq!(BitPieceMixed::from_bits(0b10000000_10000_111).b_s5().get(), -16);
    assert_eq!(BitPieceMixed::from_bits(0b10000000_00000_111).c_s8().get(), -128);
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

    let mut value = BitPieceMixed::zeroes();
    value.set_b_s5(SB5::from_bits(0b01111));
    assert_eq!(value.b_s5().get(), 15);
    value.set_b_s5(SB5::from_bits(0b010000));
    assert_eq!(value.b_s5().get(), -16);
    value.set_c_s8(SB8::from_bits(5));
    assert_eq!(value.c_s8().get(), 5);
    value.set_c_s8(SB8::from_bits(-128));
    assert_eq!(value.c_s8().get(), -128);

}

#[test]
fn test_b_types_max_value() {
    assert_eq!(B3::MAX.get(), 0b111);
    assert_eq!(B13::MAX.get(), 0b1111111111111);
    assert_eq!(B8::MAX.get(), 0b11111111);
    assert_eq!(B16::MAX.get(), 0b1111111111111111);
}

#[test]
fn test_b_types_enforce_length() {
    assert!(B3::new(0b000).is_some());
    assert!(B3::new(0b111).is_some());
    assert!(B3::new(0b1000).is_none());
    assert!(B3::new(0b10000011).is_none());
}

#[test]
fn test_sb_types_min_max_value() {
    assert_eq!(SB3::MAX.get(),  3);
    assert_eq!(SB3::MIN.get(), -4);
    assert_eq!(SB16::MAX.get(),  32767);
    assert_eq!(SB16::MIN.get(), -32768);
    assert_eq!(SB64::MAX.get(),  9_223_372_036_854_775_807);
    assert_eq!(SB64::MIN.get(), -9_223_372_036_854_775_808);
}

#[test]
fn test_sb_types_enforce_length() {
    assert!(SB3::new(3).is_some());
    assert!(SB3::new(-4).is_some());
    assert!(SB3::new(4).is_none());
    assert!(SB3::new(-64).is_none());
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
            y: B3::new(0b010).unwrap(),
        },
    };
    let value = BitPieceComplex::from_fields(fields);

    assert_eq!(value.a().x(), true);
    assert_eq!(value.a().y(), BitPieceEnum::Variant1);
    assert_eq!(value.a().storage, 0b011);
    assert_eq!(value.b().x(), 0b1111100);
    assert_eq!(value.b().y(), B3::new(0b010).unwrap());
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
    assert_eq!(B1::try_from_bits(0), Some(B1::new(0).unwrap()));
    assert_eq!(B1::try_from_bits(1), Some(B1::new(1).unwrap()));
    assert_eq!(B1::try_from_bits(2), None);

    assert_eq!(B6::try_from_bits(17), Some(B6::new(17).unwrap()));
    assert_eq!(B6::try_from_bits(2), Some(B6::new(2).unwrap()));
    assert_eq!(B6::try_from_bits(241), None);
}

#[test]
fn sb_types_try_from_bits() {
    assert_eq!(SB6::try_from_bits(31), Some(SB6::new(31).unwrap()));
    assert_eq!(SB6::try_from_bits(23), Some(SB6::new(23).unwrap()));
    assert_eq!(SB6::try_from_bits(-32), Some(SB6::new(-32).unwrap()));
    assert_eq!(SB6::try_from_bits(120), None);
    assert_eq!(SB6::try_from_bits(-64), None);
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

pub fn expect_panic<F: FnOnce() + std::panic::UnwindSafe>(f: F) {
    let result = std::panic::catch_unwind(f);
    result.expect_err("expected the code to panic");
}
