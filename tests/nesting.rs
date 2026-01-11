//! Tests for nested bitfields.

use bitpiece::*;

// =============================================================================
// Basic nesting tests
// =============================================================================

#[bitpiece(4, all)]
#[derive(Debug, PartialEq, Eq)]
struct Inner {
    x: B2,
    y: B2,
}
bitpiece_check_full_impl! {Inner, true}

#[bitpiece(12, all)]
#[derive(Debug, PartialEq, Eq)]
struct Outer {
    a: Inner,
    b: Inner,
    c: B4,
}
bitpiece_check_full_impl! {Outer, true}

#[test]
fn nested_struct_bit_len() {
    assert_eq!(Inner::BITS, 4);
    assert_eq!(Outer::BITS, 12);
}

#[test]
fn nested_struct_field_constants() {
    assert_eq!(Outer::A_OFFSET, 0);
    assert_eq!(Outer::A_LEN, 4);
    assert_eq!(Outer::B_OFFSET, 4);
    assert_eq!(Outer::B_LEN, 4);
    assert_eq!(Outer::C_OFFSET, 8);
    assert_eq!(Outer::C_LEN, 4);
}

#[test]
fn nested_struct_from_bits() {
    // Layout: [c: 4 bits][b: 4 bits][a: 4 bits]
    // a.x = 0b11, a.y = 0b00 -> a = 0b0011
    // b.x = 0b10, b.y = 0b01 -> b = 0b0110
    // c = 0b1010
    let raw = 0b1010_0110_0011;
    let outer = Outer::from_bits(raw);

    assert_eq!(outer.a().x(), B2::new(3));
    assert_eq!(outer.a().y(), B2::new(0));
    assert_eq!(outer.b().x(), B2::new(2));
    assert_eq!(outer.b().y(), B2::new(1));
    assert_eq!(outer.c(), B4::new(10));
}

#[test]
fn nested_struct_to_bits() {
    let outer = Outer::from_bits(0b1010_0110_0011);
    assert_eq!(outer.to_bits(), 0b1010_0110_0011);
}

#[test]
fn nested_struct_getters() {
    let outer = Outer::from_bits(0b1010_0110_0011);

    // Get nested struct
    let inner_a = outer.a();
    assert_eq!(inner_a.x(), B2::new(3));
    assert_eq!(inner_a.y(), B2::new(0));

    let inner_b = outer.b();
    assert_eq!(inner_b.x(), B2::new(2));
    assert_eq!(inner_b.y(), B2::new(1));
}

#[test]
fn nested_struct_setters() {
    let mut outer = Outer::ZEROES;

    // Set entire nested struct
    let new_inner = Inner::from_bits(0b1111);
    outer.set_a(new_inner);
    assert_eq!(outer.a().x(), B2::new(3));
    assert_eq!(outer.a().y(), B2::new(3));
}

#[test]
fn nested_struct_with_methods() {
    let outer = Outer::ZEROES
        .with_a(Inner::from_bits(0b0011))
        .with_b(Inner::from_bits(0b0110))
        .with_c(B4::new(10));

    assert_eq!(outer.a().x(), B2::new(3));
    assert_eq!(outer.b().y(), B2::new(1));
    assert_eq!(outer.c(), B4::new(10));
}

// =============================================================================
// Deep nesting tests (3 levels)
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct Level1 {
    data: B4,
    flags: B4,
}
bitpiece_check_full_impl! {Level1, true}

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct Level2 {
    l1_a: Level1,
    l1_b: Level1,
}
bitpiece_check_full_impl! {Level2, true}

#[bitpiece(32, all)]
#[derive(Debug, PartialEq, Eq)]
struct Level3 {
    l2: Level2,
    extra: u16,
}
bitpiece_check_full_impl! {Level3, true}

#[test]
fn deep_nesting_bit_len() {
    assert_eq!(Level1::BITS, 8);
    assert_eq!(Level2::BITS, 16);
    assert_eq!(Level3::BITS, 32);
}

#[test]
fn deep_nesting_access() {
    let l3 = Level3::from_bits(0x12345678);

    // Access through multiple levels
    let nested_data = l3.l2().l1_a().data();
    let nested_flags = l3.l2().l1_a().flags();

    // Verify the bit extraction is correct
    // 0x12345678 = 0001_0010_0011_0100_0101_0110_0111_1000
    // l2 is bits 0-15: 0x5678
    // l1_a is bits 0-7 of l2: 0x78
    // data is bits 0-3 of l1_a: 0x8
    // flags is bits 4-7 of l1_a: 0x7
    assert_eq!(nested_data, B4::new(0x8));
    assert_eq!(nested_flags, B4::new(0x7));
}

#[test]
fn deep_nesting_modification() {
    let mut l3 = Level3::ZEROES;

    // Modify at the deepest level by replacing the entire nested struct
    let new_l1 = Level1::from_bits(0xAB);
    let new_l2 = l3.l2().with_l1_a(new_l1);
    l3.set_l2(new_l2);

    assert_eq!(l3.l2().l1_a().data(), B4::new(0xB));
    assert_eq!(l3.l2().l1_a().flags(), B4::new(0xA));
}

// =============================================================================
// Nested struct with enum
// =============================================================================

#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]
enum Status {
    Idle = 0,
    Running = 1,
    Paused = 2,
    Stopped = 3,
}
bitpiece_check_full_impl! {Status, true}

#[bitpiece(6, all)]
#[derive(Debug, PartialEq, Eq)]
struct Task {
    id: B4,
    status: Status,
}
bitpiece_check_full_impl! {Task, true}

#[bitpiece(12, all)]
#[derive(Debug, PartialEq, Eq)]
struct TaskPair {
    task1: Task,
    task2: Task,
}
bitpiece_check_full_impl! {TaskPair, true}

#[test]
fn nested_struct_with_enum() {
    // task1: id=5, status=Running (1) -> 0b01_0101
    // task2: id=10, status=Paused (2) -> 0b10_1010
    let raw = 0b10_1010_01_0101;
    let pair = TaskPair::from_bits(raw);

    assert_eq!(pair.task1().id(), B4::new(5));
    assert_eq!(pair.task1().status(), Status::Running);
    assert_eq!(pair.task2().id(), B4::new(10));
    assert_eq!(pair.task2().status(), Status::Paused);
}

#[test]
fn nested_struct_with_enum_modification() {
    let mut pair = TaskPair::ZEROES;

    let task = Task::ZEROES
        .with_id(B4::new(7))
        .with_status(Status::Stopped);
    pair.set_task1(task);

    assert_eq!(pair.task1().id(), B4::new(7));
    assert_eq!(pair.task1().status(), Status::Stopped);
}

// =============================================================================
// Nested fields struct tests
// =============================================================================

#[test]
fn nested_from_fields() {
    let inner_a = Inner::from_fields(InnerFields {
        x: B2::new(1),
        y: B2::new(2),
    });
    let inner_b = Inner::from_fields(InnerFields {
        x: B2::new(3),
        y: B2::new(0),
    });

    let outer_fields = OuterFields {
        a: inner_a,
        b: inner_b,
        c: B4::new(15),
    };
    let outer = Outer::from_fields(outer_fields);

    assert_eq!(outer.a().x(), B2::new(1));
    assert_eq!(outer.a().y(), B2::new(2));
    assert_eq!(outer.b().x(), B2::new(3));
    assert_eq!(outer.b().y(), B2::new(0));
    assert_eq!(outer.c(), B4::new(15));
}

#[test]
fn nested_to_fields() {
    let outer = Outer::from_bits(0b1111_0011_1001);
    let fields = outer.to_fields();

    assert_eq!(fields.a.x(), B2::new(1));
    assert_eq!(fields.a.y(), B2::new(2));
    assert_eq!(fields.b.x(), B2::new(3));
    assert_eq!(fields.b.y(), B2::new(0));
    assert_eq!(fields.c, B4::new(15));
}

#[test]
fn nested_fields_roundtrip() {
    let original = Outer::from_bits(0b1010_0110_0011);
    let fields = original.to_fields();
    let reconstructed = Outer::from_fields(fields);
    assert_eq!(original, reconstructed);
}

// =============================================================================
// Nested constants tests
// =============================================================================

#[test]
fn nested_zeroes_constant() {
    let outer = Outer::ZEROES;
    assert_eq!(outer.storage, 0);
    assert_eq!(outer.a().storage, 0);
    assert_eq!(outer.b().storage, 0);
    assert_eq!(outer.c(), B4::new(0));
}

#[test]
fn nested_ones_constant() {
    let outer = Outer::ONES;
    assert_eq!(outer.storage, 0xFFF);
    assert_eq!(outer.a().storage, 0xF);
    assert_eq!(outer.b().storage, 0xF);
    assert_eq!(outer.c(), B4::new(15));
}

// =============================================================================
// Nested noshift tests
// =============================================================================

#[test]
fn nested_noshift_getters() {
    let outer = Outer::from_bits(0b1010_0110_0011);

    // a is at offset 0
    assert_eq!(outer.a_noshift(), 0b0011);

    // b is at offset 4
    assert_eq!(outer.b_noshift(), 0b0110_0000);

    // c is at offset 8
    assert_eq!(outer.c_noshift(), 0b1010_0000_0000);
}

// =============================================================================
// Nested mutable reference tests
// =============================================================================

#[test]
fn nested_mut_ref() {
    let mut outer = Outer::ZEROES;

    {
        let mut a_ref = outer.a_mut();
        a_ref.set(Inner::from_bits(0b1111));
    }

    assert_eq!(outer.a().x(), B2::new(3));
    assert_eq!(outer.a().y(), B2::new(3));
}

#[test]
fn nested_mut_ref_deep() {
    let mut outer = Outer::ZEROES;

    {
        let mut a_ref = outer.a_mut();
        // Access nested field through MutRef
        let inner_val = a_ref.get();
        assert_eq!(inner_val.x(), B2::new(0));

        // Set the entire inner struct
        a_ref.set(Inner::from_bits(0b1010));
    }

    assert_eq!(outer.a().x(), B2::new(2));
    assert_eq!(outer.a().y(), B2::new(2));
}

// =============================================================================
// Complex nesting with mixed types
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct ComplexNested {
    inner: Inner,
    byte: u8,
    flag: bool,
    small: B3,
}
bitpiece_check_full_impl! {ComplexNested, true}

#[test]
fn complex_nested_struct() {
    let val = ComplexNested::from_bits(0b101_1_11111111_0011);

    assert_eq!(val.inner().x(), B2::new(3));
    assert_eq!(val.inner().y(), B2::new(0));
    assert_eq!(val.byte(), 0xFF);
    assert_eq!(val.flag(), true);
    assert_eq!(val.small(), B3::new(5));
}

#[test]
fn complex_nested_from_fields() {
    let fields = ComplexNestedFields {
        inner: Inner::from_bits(0b1001),
        byte: 0xAB,
        flag: false,
        small: B3::new(7),
    };
    let val = ComplexNested::from_fields(fields);

    assert_eq!(val.inner().x(), B2::new(1));
    assert_eq!(val.inner().y(), B2::new(2));
    assert_eq!(val.byte(), 0xAB);
    assert_eq!(val.flag(), false);
    assert_eq!(val.small(), B3::new(7));
}

// =============================================================================
// Nesting with signed types
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct SignedInner {
    a: SB4,
    b: SB4,
}
bitpiece_check_full_impl! {SignedInner, true}

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct SignedOuter {
    inner: SignedInner,
    value: i8,
}
bitpiece_check_full_impl! {SignedOuter, true}

#[test]
fn nested_signed_types() {
    // inner.a = -3 (0b1101), inner.b = 5 (0b0101), value = -10
    let inner_bits = 0b0101_1101u8; // b=5, a=-3
    let value_bits = (-10i8) as u8;
    let raw = ((value_bits as u16) << 8) | (inner_bits as u16);

    let outer = SignedOuter::from_bits(raw);

    assert_eq!(outer.inner().a(), SB4::new(-3));
    assert_eq!(outer.inner().b(), SB4::new(5));
    assert_eq!(outer.value(), -10i8);
}

#[test]
fn nested_signed_modification() {
    let mut outer = SignedOuter::ZEROES;

    let inner = SignedInner::ZEROES.with_a(SB4::new(-7)).with_b(SB4::new(3));
    outer.set_inner(inner);
    outer.set_value(-50);

    assert_eq!(outer.inner().a(), SB4::new(-7));
    assert_eq!(outer.inner().b(), SB4::new(3));
    assert_eq!(outer.value(), -50i8);
}

// =============================================================================
// Self-similar nesting (same type nested)
// =============================================================================

#[bitpiece(4, all)]
#[derive(Debug, PartialEq, Eq)]
struct Nibble {
    low: B2,
    high: B2,
}
bitpiece_check_full_impl! {Nibble, true}

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct ByteFromNibbles {
    low_nibble: Nibble,
    high_nibble: Nibble,
}
bitpiece_check_full_impl! {ByteFromNibbles, true}

#[test]
fn self_similar_nesting() {
    let byte = ByteFromNibbles::from_bits(0xAB);

    // 0xAB = 0b1010_1011
    // low_nibble = 0b1011 (low=3, high=2)
    // high_nibble = 0b1010 (low=2, high=2)
    assert_eq!(byte.low_nibble().low(), B2::new(3));
    assert_eq!(byte.low_nibble().high(), B2::new(2));
    assert_eq!(byte.high_nibble().low(), B2::new(2));
    assert_eq!(byte.high_nibble().high(), B2::new(2));
}

// =============================================================================
// Nesting with non-exhaustive enum
// =============================================================================

mod common;

use common::expect_panic;

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum SparseStatus {
    Ok = 0,
    Error = 100,
    Unknown = 200,
}
bitpiece_check_full_impl! {SparseStatus, false}

#[bitpiece(12, all)]
#[derive(Debug, PartialEq, Eq)]
struct ContainerWithSparse {
    status: SparseStatus,
    extra: B4,
}
bitpiece_check_full_impl! {ContainerWithSparse, false}

#[test]
fn nested_non_exhaustive_enum_valid() {
    // status = Ok (0), extra = 5
    let val = ContainerWithSparse::from_bits(0b0101_00000000);
    assert_eq!(val.status(), SparseStatus::Ok);
    assert_eq!(val.extra(), B4::new(5));

    // status = Error (100), extra = 10
    let val = ContainerWithSparse::from_bits(0b1010_01100100);
    assert_eq!(val.status(), SparseStatus::Error);
    assert_eq!(val.extra(), B4::new(10));
}

#[test]
fn nested_non_exhaustive_enum_try_from_bits() {
    // Valid
    assert!(ContainerWithSparse::try_from_bits(0b0000_00000000).is_some()); // Ok
    assert!(ContainerWithSparse::try_from_bits(0b0000_01100100).is_some()); // Error
    assert!(ContainerWithSparse::try_from_bits(0b0000_11001000).is_some()); // Unknown

    // Invalid status value
    assert!(ContainerWithSparse::try_from_bits(0b0000_00000001).is_none()); // 1 is not valid
    assert!(ContainerWithSparse::try_from_bits(0b0000_00110010).is_none()); // 50 is not valid
}

#[test]
fn nested_non_exhaustive_enum_from_bits_panics() {
    expect_panic(|| {
        let _ = ContainerWithSparse::from_bits(0b0000_00000001); // 1 is not valid
    });
}
