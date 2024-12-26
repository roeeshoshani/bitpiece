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
    let mut value = BitPieceA::zeroed();
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
fn from_to_fields() {
    let fields = BitPieceComplexFields {
        a: BitPieceAFields {
            x: true,
            y: BitPieceEnum::Variant1,
        },
        b: BitPieceBFields {
            x: 0b1111100,
            y: B3(0b010),
        },
    };
    let value = BitPieceComplex::from_fields(fields);

    assert_eq!(value.a().x(), true);
    assert_eq!(value.a().y(), BitPieceEnum::Variant1);
    assert_eq!(value.a().storage, 0b011);
    assert_eq!(value.b().x(), 0b1111100);
    assert_eq!(value.b().y(), B3(0b010));
    assert_eq!(value.b().storage, 0b01000000000000000000000000001111100);
    assert_eq!(value.storage, 0b01000000000000000000000000001111100011);

    assert_eq!(value.to_fields(), fields);
}

#[test]
fn zeroed() {
    let zeroed = BitPieceComplex::zeroed();
    assert_eq!(zeroed.storage, 0);
}
