//! Tests for enum bitfields.

mod common;

use bitpiece::*;
use common::expect_panic;

// =============================================================================
// Exhaustive enum tests (all bit patterns are valid)
// =============================================================================

#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]
enum ExhaustiveEnum2 {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
bitpiece_check_full_impl! {ExhaustiveEnum2, true}

#[test]
fn exhaustive_enum_bit_len() {
    assert_eq!(EXHAUSTIVE_ENUM_2_BIT_LEN, 2);
    assert_eq!(ExhaustiveEnum2::BITS, 2);
}

#[test]
fn exhaustive_enum_from_bits() {
    assert_eq!(ExhaustiveEnum2::from_bits(0), ExhaustiveEnum2::A);
    assert_eq!(ExhaustiveEnum2::from_bits(1), ExhaustiveEnum2::B);
    assert_eq!(ExhaustiveEnum2::from_bits(2), ExhaustiveEnum2::C);
    assert_eq!(ExhaustiveEnum2::from_bits(3), ExhaustiveEnum2::D);
}

#[test]
fn exhaustive_enum_to_bits() {
    assert_eq!(ExhaustiveEnum2::A.to_bits(), 0);
    assert_eq!(ExhaustiveEnum2::B.to_bits(), 1);
    assert_eq!(ExhaustiveEnum2::C.to_bits(), 2);
    assert_eq!(ExhaustiveEnum2::D.to_bits(), 3);
}

#[test]
fn exhaustive_enum_try_from_bits() {
    assert_eq!(ExhaustiveEnum2::try_from_bits(0), Some(ExhaustiveEnum2::A));
    assert_eq!(ExhaustiveEnum2::try_from_bits(1), Some(ExhaustiveEnum2::B));
    assert_eq!(ExhaustiveEnum2::try_from_bits(2), Some(ExhaustiveEnum2::C));
    assert_eq!(ExhaustiveEnum2::try_from_bits(3), Some(ExhaustiveEnum2::D));
    // Out of range
    assert_eq!(ExhaustiveEnum2::try_from_bits(4), None);
    assert_eq!(ExhaustiveEnum2::try_from_bits(255), None);
}

#[test]
fn exhaustive_enum_constants() {
    // For exhaustive enums, ZEROES/MIN is the variant with value 0
    assert_eq!(ExhaustiveEnum2::ZEROES, ExhaustiveEnum2::A);
    assert_eq!(ExhaustiveEnum2::MIN, ExhaustiveEnum2::A);

    // ONES/MAX is the variant with the highest value
    assert_eq!(ExhaustiveEnum2::ONES, ExhaustiveEnum2::D);
    assert_eq!(ExhaustiveEnum2::MAX, ExhaustiveEnum2::D);
}

// =============================================================================
// Non-exhaustive enum tests (sparse values)
// =============================================================================

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum NonExhaustiveEnum {
    Variant0 = 0,
    Variant77 = 77,
    Variant120 = 120,
}
bitpiece_check_full_impl! {NonExhaustiveEnum, false}

#[test]
fn non_exhaustive_enum_bit_len() {
    // 120 requires 7 bits (2^7 = 128 > 120)
    assert_eq!(NON_EXHAUSTIVE_ENUM_BIT_LEN, 7);
    assert_eq!(NonExhaustiveEnum::BITS, 7);
}

#[test]
fn non_exhaustive_enum_from_bits_valid() {
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
fn non_exhaustive_enum_from_bits_invalid_panics() {
    expect_panic(|| {
        let _ = NonExhaustiveEnum::from_bits(1);
    });
    expect_panic(|| {
        let _ = NonExhaustiveEnum::from_bits(55);
    });
    expect_panic(|| {
        let _ = NonExhaustiveEnum::from_bits(127);
    });
}

#[test]
fn non_exhaustive_enum_try_from_bits() {
    // Valid
    assert_eq!(
        NonExhaustiveEnum::try_from_bits(0),
        Some(NonExhaustiveEnum::Variant0)
    );
    assert_eq!(
        NonExhaustiveEnum::try_from_bits(77),
        Some(NonExhaustiveEnum::Variant77)
    );
    assert_eq!(
        NonExhaustiveEnum::try_from_bits(120),
        Some(NonExhaustiveEnum::Variant120)
    );

    // Invalid
    assert_eq!(NonExhaustiveEnum::try_from_bits(1), None);
    assert_eq!(NonExhaustiveEnum::try_from_bits(50), None);
    assert_eq!(NonExhaustiveEnum::try_from_bits(100), None);
}

#[test]
fn non_exhaustive_enum_constants() {
    // ZEROES/MIN is the variant with the smallest value
    assert_eq!(NonExhaustiveEnum::ZEROES, NonExhaustiveEnum::Variant0);
    assert_eq!(NonExhaustiveEnum::MIN, NonExhaustiveEnum::Variant0);

    // ONES/MAX is the variant with the largest value
    assert_eq!(NonExhaustiveEnum::ONES, NonExhaustiveEnum::Variant120);
    assert_eq!(NonExhaustiveEnum::MAX, NonExhaustiveEnum::Variant120);
}

// =============================================================================
// Non-exhaustive enum without zero variant
// =============================================================================

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum NoZeroVariant {
    A = 10,
    B = 50,
    C = 100,
}
bitpiece_check_full_impl! {NoZeroVariant, false}

#[test]
fn no_zero_variant_constants() {
    // ZEROES should be the minimum variant, not a value with all bits zero
    assert_eq!(NoZeroVariant::ZEROES, NoZeroVariant::A);
    assert_eq!(NoZeroVariant::MIN, NoZeroVariant::A);
    assert_eq!(NoZeroVariant::ONES, NoZeroVariant::C);
    assert_eq!(NoZeroVariant::MAX, NoZeroVariant::C);
}

#[test]
fn no_zero_variant_from_bits() {
    assert_eq!(NoZeroVariant::from_bits(10), NoZeroVariant::A);
    assert_eq!(NoZeroVariant::from_bits(50), NoZeroVariant::B);
    assert_eq!(NoZeroVariant::from_bits(100), NoZeroVariant::C);
}

#[test]
fn no_zero_variant_try_from_bits() {
    assert_eq!(NoZeroVariant::try_from_bits(0), None);
    assert_eq!(NoZeroVariant::try_from_bits(10), Some(NoZeroVariant::A));
}

// =============================================================================
// Explicit bit length enum tests
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
enum ExplicitBitLenEnum {
    Variant0 = 0,
    Variant77 = 77,
    Variant120 = 120,
}
bitpiece_check_full_impl! {ExplicitBitLenEnum, false}

#[test]
fn explicit_bit_len_enum() {
    assert_eq!(EXPLICIT_BIT_LEN_ENUM_BIT_LEN, 16);
    assert_eq!(ExplicitBitLenEnum::BITS, 16);
}

#[test]
fn explicit_bit_len_enum_storage() {
    // 16 bits should use u16 storage
    let _: u16 = ExplicitBitLenEnum::Variant0.to_bits();
}

#[test]
fn explicit_bit_len_enum_try_from_bits_large_values() {
    // Can accept 16-bit values
    assert_eq!(ExplicitBitLenEnum::try_from_bits(1000), None);
    assert_eq!(ExplicitBitLenEnum::try_from_bits(65535), None);
}

// =============================================================================
// Single variant enum
// =============================================================================

// Note: Single variant enum with value 0 would require 0 bits, which is not supported.
// Instead, we test a single variant enum with a non-zero value.
#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum SingleVariant {
    Only = 1,
}
bitpiece_check_full_impl! {SingleVariant, false}

#[test]
fn single_variant_enum() {
    // Single variant with value 1 needs 1 bit
    assert_eq!(SingleVariant::BITS, 1);
    assert_eq!(SingleVariant::from_bits(1), SingleVariant::Only);
    assert_eq!(SingleVariant::Only.to_bits(), 1);
}

#[test]
fn single_variant_constants() {
    assert_eq!(SingleVariant::ZEROES, SingleVariant::Only);
    assert_eq!(SingleVariant::ONES, SingleVariant::Only);
    assert_eq!(SingleVariant::MIN, SingleVariant::Only);
    assert_eq!(SingleVariant::MAX, SingleVariant::Only);
}

// =============================================================================
// Enum with large values
// =============================================================================

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum LargeValueEnum {
    Small = 0,
    Medium = 1000,
    Large = 50000,
}
bitpiece_check_full_impl! {LargeValueEnum, false}

#[test]
fn large_value_enum_bit_len() {
    // 50000 requires 16 bits (2^16 = 65536 > 50000)
    assert_eq!(LargeValueEnum::BITS, 16);
}

#[test]
fn large_value_enum_from_bits() {
    assert_eq!(LargeValueEnum::from_bits(0), LargeValueEnum::Small);
    assert_eq!(LargeValueEnum::from_bits(1000), LargeValueEnum::Medium);
    assert_eq!(LargeValueEnum::from_bits(50000), LargeValueEnum::Large);
}

// =============================================================================
// Enum roundtrip tests
// =============================================================================

#[test]
fn enum_roundtrip() {
    // Exhaustive
    for variant in [
        ExhaustiveEnum2::A,
        ExhaustiveEnum2::B,
        ExhaustiveEnum2::C,
        ExhaustiveEnum2::D,
    ] {
        assert_eq!(ExhaustiveEnum2::from_bits(variant.to_bits()), variant);
    }

    // Non-exhaustive
    for variant in [
        NonExhaustiveEnum::Variant0,
        NonExhaustiveEnum::Variant77,
        NonExhaustiveEnum::Variant120,
    ] {
        assert_eq!(NonExhaustiveEnum::from_bits(variant.to_bits()), variant);
    }
}

// =============================================================================
// Enum Clone and Copy tests
// =============================================================================

#[test]
fn enum_clone_copy() {
    let a = ExhaustiveEnum2::B;
    let b = a; // Copy
    let c = a.clone(); // Clone
    assert_eq!(a, b);
    assert_eq!(a, c);
}

// =============================================================================
// Enum in struct tests
// =============================================================================

#[bitpiece(3, all)]
#[derive(Debug, PartialEq, Eq)]
struct EnumContainer {
    flag: bool,
    dir: ExhaustiveEnum2,
}
bitpiece_check_full_impl! {EnumContainer, true}

#[test]
fn enum_in_struct() {
    let val = EnumContainer::from_bits(0b10_1);
    assert_eq!(val.flag(), true);
    assert_eq!(val.dir(), ExhaustiveEnum2::C);
}

#[test]
fn enum_in_struct_setters() {
    let mut val = EnumContainer::ZEROES;
    val.set_dir(ExhaustiveEnum2::D);
    assert_eq!(val.dir(), ExhaustiveEnum2::D);
}

// =============================================================================
// Enum const_eq tests
// =============================================================================

#[test]
fn enum_const_eq() {
    assert!(ExhaustiveEnum2::const_eq(
        ExhaustiveEnum2::A,
        ExhaustiveEnum2::A
    ));
    assert!(!ExhaustiveEnum2::const_eq(
        ExhaustiveEnum2::A,
        ExhaustiveEnum2::B
    ));
}

// =============================================================================
// Enum from_fields/to_fields tests
// =============================================================================

#[test]
fn enum_from_to_fields() {
    // For enums, Fields type is Self
    let val = ExhaustiveEnum2::B;
    let fields = val.to_fields();
    assert_eq!(fields, ExhaustiveEnum2::B);

    let reconstructed = ExhaustiveEnum2::from_fields(fields);
    assert_eq!(reconstructed, val);
}

// =============================================================================
// Enum with consecutive values starting from non-zero
// =============================================================================

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum ConsecutiveNonZero {
    A = 5,
    B = 6,
    C = 7,
    D = 8,
}
bitpiece_check_full_impl! {ConsecutiveNonZero, false}

#[test]
fn consecutive_non_zero_enum() {
    // 8 requires 4 bits
    assert_eq!(ConsecutiveNonZero::BITS, 4);

    assert_eq!(ConsecutiveNonZero::from_bits(5), ConsecutiveNonZero::A);
    assert_eq!(ConsecutiveNonZero::from_bits(6), ConsecutiveNonZero::B);
    assert_eq!(ConsecutiveNonZero::from_bits(7), ConsecutiveNonZero::C);
    assert_eq!(ConsecutiveNonZero::from_bits(8), ConsecutiveNonZero::D);

    // Values 0-4 and 9+ are invalid
    assert_eq!(ConsecutiveNonZero::try_from_bits(0), None);
    assert_eq!(ConsecutiveNonZero::try_from_bits(4), None);
    assert_eq!(ConsecutiveNonZero::try_from_bits(9), None);
}

// =============================================================================
// Enum with power of 2 values
// =============================================================================

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum PowerOf2Enum {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}
bitpiece_check_full_impl! {PowerOf2Enum, false}

#[test]
fn power_of_2_enum() {
    // 16 requires 5 bits
    assert_eq!(PowerOf2Enum::BITS, 5);

    assert_eq!(PowerOf2Enum::from_bits(1), PowerOf2Enum::One);
    assert_eq!(PowerOf2Enum::from_bits(16), PowerOf2Enum::Sixteen);

    // 0 is not a valid variant
    assert_eq!(PowerOf2Enum::try_from_bits(0), None);
    // 3 is not a valid variant
    assert_eq!(PowerOf2Enum::try_from_bits(3), None);
}

// =============================================================================
// Enum storage type tests
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
enum Enum8Bit {
    A = 0,
    B = 255,
}

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
enum Enum16Bit {
    A = 0,
    B = 65535,
}

#[bitpiece(32, all)]
#[derive(Debug, PartialEq, Eq)]
enum Enum32Bit {
    A = 0,
    B = 4294967295,
}

#[test]
fn enum_storage_types() {
    let _: u8 = Enum8Bit::A.to_bits();
    let _: u16 = Enum16Bit::A.to_bits();
    let _: u32 = Enum32Bit::A.to_bits();
}

// =============================================================================
// Enum derive propagation tests
// =============================================================================

#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum HashableEnum {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

#[test]
fn enum_derives_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(HashableEnum::A);
    set.insert(HashableEnum::B);
    set.insert(HashableEnum::A); // duplicate
    assert_eq!(set.len(), 2);
}

// =============================================================================
// Enum BitPiece trait tests
// =============================================================================

#[test]
fn enum_implements_bitpiece() {
    fn assert_bitpiece<T: BitPiece>() {}

    assert_bitpiece::<ExhaustiveEnum2>();
    assert_bitpiece::<NonExhaustiveEnum>();
    assert_bitpiece::<ExplicitBitLenEnum>();
}

#[test]
fn enum_implements_bitpiece_has_mut_ref() {
    fn assert_has_mut_ref<T: BitPieceHasMutRef>() {}

    assert_has_mut_ref::<ExhaustiveEnum2>();
    assert_has_mut_ref::<NonExhaustiveEnum>();
}

#[test]
fn enum_implements_bitpiece_has_fields() {
    fn assert_has_fields<T: BitPieceHasFields>() {}

    assert_has_fields::<ExhaustiveEnum2>();
    assert_has_fields::<NonExhaustiveEnum>();
}
