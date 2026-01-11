//! Tests for struct bitfields.

mod common;

use bitpiece::*;
use common::expect_panic;

// =============================================================================
// Basic struct definition tests
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct BasicStruct {
    a: B3,
    b: B5,
}
bitpiece_check_full_impl! {BasicStruct, true}

#[test]
fn basic_struct_bit_len() {
    assert_eq!(BASIC_STRUCT_BIT_LEN, 8);
    assert_eq!(BasicStruct::BITS, 8);
}

#[test]
fn basic_struct_storage_type() {
    let _: BasicStructStorageTy = 0u8;
}

#[test]
fn basic_struct_field_constants() {
    assert_eq!(BasicStruct::A_OFFSET, 0);
    assert_eq!(BasicStruct::A_LEN, 3);
    assert_eq!(BasicStruct::B_OFFSET, 3);
    assert_eq!(BasicStruct::B_LEN, 5);
}

#[test]
fn basic_struct_from_bits() {
    let val = BasicStruct::from_bits(0b11111_010);
    assert_eq!(val.a(), B3::new(0b010));
    assert_eq!(val.b(), B5::new(0b11111));
}

#[test]
fn basic_struct_to_bits() {
    let val = BasicStruct::from_bits(0b11111_010);
    assert_eq!(val.to_bits(), 0b11111_010);
}

#[test]
fn basic_struct_try_from_bits() {
    let val = BasicStruct::try_from_bits(0b11111_010);
    assert!(val.is_some());
    assert_eq!(val.unwrap().a(), B3::new(0b010));
}

// =============================================================================
// Field getter tests
// =============================================================================

#[test]
fn struct_field_getters() {
    let val = BasicStruct::from_bits(0b10101_011);
    assert_eq!(val.a(), B3::new(0b011));
    assert_eq!(val.b(), B5::new(0b10101));
}

#[test]
fn struct_field_getters_boundary_values() {
    // All zeros
    let val = BasicStruct::from_bits(0b00000_000);
    assert_eq!(val.a(), B3::new(0));
    assert_eq!(val.b(), B5::new(0));

    // All ones
    let val = BasicStruct::from_bits(0b11111_111);
    assert_eq!(val.a(), B3::new(7));
    assert_eq!(val.b(), B5::new(31));
}

// =============================================================================
// Field setter tests (set_*)
// =============================================================================

#[test]
fn struct_field_setters() {
    let mut val = BasicStruct::ZEROES;
    assert_eq!(val.a(), B3::new(0));
    assert_eq!(val.b(), B5::new(0));

    val.set_a(B3::new(5));
    assert_eq!(val.a(), B3::new(5));
    assert_eq!(val.b(), B5::new(0));

    val.set_b(B5::new(20));
    assert_eq!(val.a(), B3::new(5));
    assert_eq!(val.b(), B5::new(20));
}

#[test]
fn struct_field_setters_overwrite() {
    let mut val = BasicStruct::from_bits(0xFF);
    assert_eq!(val.a(), B3::new(7));
    assert_eq!(val.b(), B5::new(31));

    val.set_a(B3::new(0));
    assert_eq!(val.a(), B3::new(0));
    assert_eq!(val.b(), B5::new(31)); // b should be unchanged
}

// =============================================================================
// Builder pattern tests (with_*)
// =============================================================================

#[test]
fn struct_with_methods() {
    let val = BasicStruct::ZEROES.with_a(B3::new(5)).with_b(B5::new(20));
    assert_eq!(val.a(), B3::new(5));
    assert_eq!(val.b(), B5::new(20));
}

#[test]
fn struct_with_methods_chaining() {
    let val = BasicStruct::ZEROES
        .with_a(B3::new(1))
        .with_b(B5::new(2))
        .with_a(B3::new(3)) // overwrite a
        .with_b(B5::new(4)); // overwrite b
    assert_eq!(val.a(), B3::new(3));
    assert_eq!(val.b(), B5::new(4));
}

#[test]
fn struct_with_methods_immutable() {
    let original = BasicStruct::ZEROES;
    let modified = original.with_a(B3::new(5));

    // Original should be unchanged
    assert_eq!(original.a(), B3::new(0));
    assert_eq!(modified.a(), B3::new(5));
}

// =============================================================================
// Noshift getter tests
// =============================================================================

#[test]
fn struct_noshift_getters() {
    let val = BasicStruct::from_bits(0b10101_011);

    // a is at offset 0, so noshift should be same as regular
    assert_eq!(val.a_noshift(), 0b011);

    // b is at offset 3, so noshift should keep the bits at their position
    assert_eq!(val.b_noshift(), 0b10101_000);
}

// =============================================================================
// Constants tests
// =============================================================================

#[test]
fn struct_zeroes_constant() {
    let val = BasicStruct::ZEROES;
    assert_eq!(val.storage, 0);
    assert_eq!(val.a(), B3::new(0));
    assert_eq!(val.b(), B5::new(0));
}

#[test]
fn struct_ones_constant() {
    let val = BasicStruct::ONES;
    assert_eq!(val.storage, 0xFF);
    assert_eq!(val.a(), B3::new(7));
    assert_eq!(val.b(), B5::new(31));
}

#[test]
fn struct_min_max_constants() {
    // For unsigned fields, MIN == ZEROES and MAX == ONES
    assert_eq!(BasicStruct::MIN.storage, BasicStruct::ZEROES.storage);
    assert_eq!(BasicStruct::MAX.storage, BasicStruct::ONES.storage);
}

// =============================================================================
// Fields struct tests
// =============================================================================

#[test]
fn struct_from_fields() {
    let fields = BasicStructFields {
        a: B3::new(5),
        b: B5::new(20),
    };
    let val = BasicStruct::from_fields(fields);
    assert_eq!(val.a(), B3::new(5));
    assert_eq!(val.b(), B5::new(20));
}

#[test]
fn struct_to_fields() {
    let val = BasicStruct::from_bits(0b10100_101);
    let fields = val.to_fields();
    assert_eq!(fields.a, B3::new(5));
    assert_eq!(fields.b, B5::new(20));
}

#[test]
fn struct_fields_roundtrip() {
    let original_fields = BasicStructFields {
        a: B3::new(3),
        b: B5::new(15),
    };
    let val = BasicStruct::from_fields(original_fields);
    let extracted_fields = val.to_fields();
    assert_eq!(original_fields, extracted_fields);
}

#[test]
fn struct_fields_into_from() {
    let fields = BasicStructFields {
        a: B3::new(5),
        b: B5::new(20),
    };

    // Into<BasicStruct>
    let val: BasicStruct = fields.into();
    assert_eq!(val.a(), B3::new(5));

    // Into<BasicStructFields>
    let fields2: BasicStructFields = val.into();
    assert_eq!(fields2.a, B3::new(5));
}

// =============================================================================
// const_eq tests
// =============================================================================

#[test]
fn struct_const_eq() {
    let a = BasicStruct::from_bits(0b10101_011);
    let b = BasicStruct::from_bits(0b10101_011);
    let c = BasicStruct::from_bits(0b10101_010);

    assert!(BasicStruct::const_eq(a, b));
    assert!(!BasicStruct::const_eq(a, c));
}

// =============================================================================
// Struct with bool field
// =============================================================================

#[bitpiece(4, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithBool {
    flag: bool,
    value: B3,
}
bitpiece_check_full_impl! {StructWithBool, true}

#[test]
fn struct_with_bool_field() {
    let val = StructWithBool::from_bits(0b101_1);
    assert_eq!(val.flag(), true);
    assert_eq!(val.value(), B3::new(5));

    let val = StructWithBool::from_bits(0b101_0);
    assert_eq!(val.flag(), false);
    assert_eq!(val.value(), B3::new(5));
}

#[test]
fn struct_with_bool_setters() {
    let mut val = StructWithBool::ZEROES;
    val.set_flag(true);
    assert_eq!(val.flag(), true);
    assert_eq!(val.storage, 0b000_1);

    val.set_value(B3::new(7));
    assert_eq!(val.value(), B3::new(7));
    assert_eq!(val.storage, 0b111_1);
}

// =============================================================================
// Struct with signed fields
// =============================================================================

#[bitpiece(12, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithSigned {
    a: B3,
    b: i8,
    c: bool,
}
bitpiece_check_full_impl! {StructWithSigned, true}

#[test]
fn struct_with_signed_extraction() {
    // a = 0b101 (5), b = -1 (0xFF), c = true
    let raw = 0b1_11111111_101;
    let val = StructWithSigned::from_bits(raw);
    assert_eq!(val.a(), B3::new(5));
    assert_eq!(val.b(), -1i8);
    assert_eq!(val.c(), true);
}

#[test]
fn struct_with_signed_modification() {
    let mut val = StructWithSigned::ZEROES;
    val.set_b(-50);
    assert_eq!(val.b(), -50i8);

    val.set_b(100);
    assert_eq!(val.b(), 100i8);
}

#[test]
fn struct_with_signed_min_max() {
    // MIN should have minimum values for each field
    let min = StructWithSigned::MIN;
    assert_eq!(min.a(), B3::MIN);
    assert_eq!(min.b(), i8::MIN);
    assert_eq!(min.c(), bool::MIN);

    // MAX should have maximum values for each field
    let max = StructWithSigned::MAX;
    assert_eq!(max.a(), B3::MAX);
    assert_eq!(max.b(), i8::MAX);
    assert_eq!(max.c(), bool::MAX);
}

// =============================================================================
// Struct with SB types
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithSb {
    a: SB5,
    b: bool,
    c: SB7,
    d: B3,
}
bitpiece_check_full_impl! {StructWithSb, true}

#[test]
fn struct_with_sb_extraction() {
    // a = -1 (0b11111), b = true, c = 42 (0b0101010), d = 5
    let raw = 0b101_0101010_1_11111;
    let val = StructWithSb::from_bits(raw);
    assert_eq!(val.a(), SB5::new(-1));
    assert_eq!(val.b(), true);
    assert_eq!(val.c(), SB7::new(42));
    assert_eq!(val.d(), B3::new(5));
}

#[test]
fn struct_with_sb_modification() {
    let mut val = StructWithSb::ZEROES;
    val.set_a(SB5::new(-5));
    assert_eq!(val.a(), SB5::new(-5));

    val.set_c(SB7::new(-30));
    assert_eq!(val.c(), SB7::new(-30));
}

#[test]
fn struct_with_sb_from_to_fields() {
    let fields = StructWithSbFields {
        a: SB5::new(-10),
        b: true,
        c: SB7::new(55),
        d: B3::new(6),
    };
    let val = StructWithSb::from_fields(fields);
    assert_eq!(val.to_fields(), fields);
}

// =============================================================================
// Struct with standard integer types
// =============================================================================

#[bitpiece(48, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithIntegers {
    byte: u8,
    word: u16,
    flags: B8,
    signed: i16,
}
bitpiece_check_full_impl! {StructWithIntegers, true}

#[test]
fn struct_with_integers() {
    let fields = StructWithIntegersFields {
        byte: 0xAB,
        word: 0xCDEF,
        flags: B8::new(0x12),
        signed: -1000,
    };
    let val = StructWithIntegers::from_fields(fields);
    assert_eq!(val.byte(), 0xAB);
    assert_eq!(val.word(), 0xCDEF);
    assert_eq!(val.flags(), B8::new(0x12));
    assert_eq!(val.signed(), -1000i16);
}

// =============================================================================
// Large struct (>32 bits)
// =============================================================================

#[bitpiece(35, all)]
#[derive(Debug, PartialEq, Eq)]
struct LargeStruct {
    x: u32,
    y: B3,
}
bitpiece_check_full_impl! {LargeStruct, true}

#[test]
fn large_struct_storage_type() {
    // 35 bits requires u64 storage
    let _: u64 = LargeStruct::ZEROES.storage;
}

#[test]
fn large_struct_operations() {
    let val = LargeStruct::from_bits(0b101_11111111111111111111111111111111);
    assert_eq!(val.x(), 0xFFFFFFFF);
    assert_eq!(val.y(), B3::new(5));
}

// =============================================================================
// Struct with enum field
// =============================================================================

#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
bitpiece_check_full_impl! {Direction, true}

#[bitpiece(5, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithEnum {
    dir: Direction,
    speed: B3,
}
bitpiece_check_full_impl! {StructWithEnum, true}

#[test]
fn struct_with_enum_field() {
    let val = StructWithEnum::from_bits(0b101_10);
    assert_eq!(val.dir(), Direction::South);
    assert_eq!(val.speed(), B3::new(5));
}

#[test]
fn struct_with_enum_setters() {
    let mut val = StructWithEnum::ZEROES;
    val.set_dir(Direction::West);
    assert_eq!(val.dir(), Direction::West);

    let val = val.with_speed(B3::new(7));
    assert_eq!(val.speed(), B3::new(7));
}

// =============================================================================
// Mutable reference tests
// =============================================================================

#[test]
fn struct_field_mut() {
    let mut val = BasicStruct::ZEROES;
    {
        let mut a_ref = val.a_mut();
        assert_eq!(a_ref.get(), B3::new(0));
        a_ref.set(B3::new(5));
    }
    assert_eq!(val.a(), B3::new(5));
}

#[test]
fn struct_field_mut_multiple() {
    let mut val = BasicStruct::ZEROES;

    {
        let mut a_ref = val.a_mut();
        a_ref.set(B3::new(3));
    }
    {
        let mut b_ref = val.b_mut();
        b_ref.set(B5::new(20));
    }

    assert_eq!(val.a(), B3::new(3));
    assert_eq!(val.b(), B5::new(20));
}

// =============================================================================
// Clone and Copy tests
// =============================================================================

#[test]
fn struct_clone_copy() {
    let a = BasicStruct::from_bits(0b10101_011);
    let b = a; // Copy
    let c = a.clone(); // Clone
    assert_eq!(a, b);
    assert_eq!(a, c);
}

// =============================================================================
// Struct with non-exhaustive enum
// =============================================================================

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum SparseEnum {
    A = 0,
    B = 10,
    C = 50,
}
bitpiece_check_full_impl! {SparseEnum, false}

// SparseEnum needs 6 bits (50 requires 6 bits: 2^6 = 64 > 50)
// Layout: [extra: B2 (2)][flag: bool (1)][val: SparseEnum (6)] = 9 bits
#[bitpiece(9, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithSparseEnum {
    val: SparseEnum,
    flag: bool,
    extra: B2,
}
bitpiece_check_full_impl! {StructWithSparseEnum, false}

#[test]
fn struct_with_sparse_enum_valid() {
    // Layout: [extra: 2][flag: 1][val: 6] = 9 bits
    // val = 10 (0b001010), flag = true (1), extra = 1 (0b01)
    // bits = 0b01_1_001010 = 0b011001010
    let val = StructWithSparseEnum::from_bits(0b01_1_001010);
    assert_eq!(val.val(), SparseEnum::B);
    assert_eq!(val.flag(), true);
    assert_eq!(val.extra(), B2::new(1));
}

#[test]
fn struct_with_sparse_enum_try_from_bits() {
    // Valid
    assert!(StructWithSparseEnum::try_from_bits(0b00_0_000000).is_some()); // A=0
    assert!(StructWithSparseEnum::try_from_bits(0b00_0_001010).is_some()); // B=10
    assert!(StructWithSparseEnum::try_from_bits(0b00_0_110010).is_some()); // C=50

    // Invalid enum value
    assert!(StructWithSparseEnum::try_from_bits(0b00_0_000001).is_none()); // 1 is not valid
    assert!(StructWithSparseEnum::try_from_bits(0b00_0_000101).is_none()); // 5 is not valid
}

#[test]
fn struct_with_sparse_enum_from_bits_panics() {
    expect_panic(|| {
        let _ = StructWithSparseEnum::from_bits(0b00_0_000101); // 5 is not valid
    });
}

// =============================================================================
// Visibility tests
// =============================================================================

mod visibility_test {
    use bitpiece::*;

    #[bitpiece(8, all)]
    #[derive(Debug, PartialEq, Eq)]
    pub struct PublicStruct {
        pub public_field: B4,
        private_field: B4,
    }

    #[test]
    fn public_struct_public_field() {
        let val = PublicStruct::from_bits(0xFF);
        assert_eq!(val.public_field(), B4::new(15));
    }
}

// =============================================================================
// Derive attribute propagation tests
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq, Hash)]
struct DerivedStruct {
    a: B4,
    b: B4,
}

#[test]
fn struct_derives_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(DerivedStruct::from_bits(0x12));
    set.insert(DerivedStruct::from_bits(0x34));
    set.insert(DerivedStruct::from_bits(0x12)); // duplicate
    assert_eq!(set.len(), 2);
}

// =============================================================================
// Auto bit length calculation
// =============================================================================

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
struct AutoLenStruct {
    a: B3,
    b: B5,
    c: bool,
}
bitpiece_check_full_impl! {AutoLenStruct, true}

#[test]
fn auto_len_struct() {
    // Should be 3 + 5 + 1 = 9 bits
    assert_eq!(AutoLenStruct::BITS, 9);
    assert_eq!(AUTO_LEN_STRUCT_BIT_LEN, 9);
}

// =============================================================================
// Storage type selection tests
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct Storage8 {
    a: B8,
}

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct Storage16 {
    a: B16,
}

#[bitpiece(32, all)]
#[derive(Debug, PartialEq, Eq)]
struct Storage32 {
    a: B32,
}

#[bitpiece(64, all)]
#[derive(Debug, PartialEq, Eq)]
struct Storage64 {
    a: B64,
}

#[test]
fn storage_type_selection() {
    let _: u8 = Storage8::ZEROES.storage;
    let _: u16 = Storage16::ZEROES.storage;
    let _: u32 = Storage32::ZEROES.storage;
    let _: u64 = Storage64::ZEROES.storage;
}
