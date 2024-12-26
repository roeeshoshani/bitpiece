use bitpiece::*;

#[bitpiece(5)]
#[derive(Debug, Clone, Copy)]
struct BitPieceA {
    x: bool,
    y: B4,
}

#[bitpiece(35)]
#[derive(Debug, Clone, Copy)]
struct BitPieceB {
    x: u32,
    y: B3,
}

#[bitpiece(40)]
#[derive(Debug, Clone, Copy)]
struct BitPieceComplex {
    a: BitPieceA,
    b: BitPieceB,
}

#[test]
fn bit_extraction() {
    assert_eq!(BitPieceA::from_bits(0b00001).x(), true);
    assert_eq!(BitPieceA::from_bits(0b11111).x(), true);
    assert_eq!(BitPieceA::from_bits(0b00000).x(), false);
    assert_eq!(BitPieceA::from_bits(0b11110).x(), false);

    assert_eq!(BitPieceA::from_bits(0b10110).y().0, 0b1011);
    assert_eq!(BitPieceA::from_bits(0b11101).y().0, 0b1110);
}

#[test]
fn bit_modification() {
    let mut value = BitPieceA::zeroed();
    assert_eq!(value.x(), false);
    assert_eq!(value.y().0, 0);
    assert_eq!(value.storage, 0);

    value.set_x(true);
    assert_eq!(value.x(), true);
    assert_eq!(value.y().0, 0);
    assert_eq!(value.storage, 0b00001);

    value.set_y(B4(0b1011));
    assert_eq!(value.x(), true);
    assert_eq!(value.y().0, 0b1011);
    assert_eq!(value.storage, 0b10111);

    value.set_x(false);
    assert_eq!(value.x(), false);
    assert_eq!(value.y().0, 0b1011);
    assert_eq!(value.storage, 0b10110);
}

#[test]
fn zeroed() {
    let zeroed = BitPieceComplex::zeroed();
    assert_eq!(zeroed.storage, 0);
}
