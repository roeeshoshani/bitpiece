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
#[should_panic]
fn test_b_types_enforce_length_in_from_bits() {
    let _ = B3::from_bits(0b10000011);
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
    Variant194 = 194,
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
    assert_eq!(
        NonExhaustiveEnum::from_bits(194),
        NonExhaustiveEnum::Variant194
    );
}

#[should_panic]
#[test]
fn invalid_variant_of_non_exhastive_enum() {
    let _ = NonExhaustiveEnum::from_bits(55);
}
