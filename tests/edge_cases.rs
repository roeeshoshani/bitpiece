//! Tests for edge cases and error handling.

mod common;

use bitpiece::*;
use common::expect_panic;

// =============================================================================
// Boundary bit lengths
// =============================================================================

#[bitpiece(1, all)]
#[derive(Debug, PartialEq, Eq)]
struct OneBit {
    bit: bool,
}
bitpiece_check_full_impl! {OneBit, true}

#[test]
fn one_bit_struct() {
    assert_eq!(OneBit::BITS, 1);

    let val = OneBit::from_bits(0);
    assert_eq!(val.bit(), false);

    let val = OneBit::from_bits(1);
    assert_eq!(val.bit(), true);
}

#[bitpiece(64, all)]
#[derive(Debug, PartialEq, Eq)]
struct SixtyFourBit {
    value: B64,
}
bitpiece_check_full_impl! {SixtyFourBit, true}

#[test]
fn sixty_four_bit_struct() {
    assert_eq!(SixtyFourBit::BITS, 64);

    let val = SixtyFourBit::from_bits(u64::MAX);
    assert_eq!(val.value(), B64::new(u64::MAX));

    let val = SixtyFourBit::from_bits(0);
    assert_eq!(val.value(), B64::new(0));
}

// =============================================================================
// Storage type boundaries
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct ExactlyU8 {
    value: B8,
}

#[bitpiece(9, all)]
#[derive(Debug, PartialEq, Eq)]
struct JustOverU8 {
    a: B8,
    b: B1,
}

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct ExactlyU16 {
    value: B16,
}

#[bitpiece(17, all)]
#[derive(Debug, PartialEq, Eq)]
struct JustOverU16 {
    a: B16,
    b: B1,
}

#[bitpiece(32, all)]
#[derive(Debug, PartialEq, Eq)]
struct ExactlyU32 {
    value: B32,
}

#[bitpiece(33, all)]
#[derive(Debug, PartialEq, Eq)]
struct JustOverU32 {
    a: B32,
    b: B1,
}

#[test]
fn storage_type_boundaries() {
    // 8 bits -> u8
    let _: u8 = ExactlyU8::ZEROES.storage;

    // 9 bits -> u16
    let _: u16 = JustOverU8::ZEROES.storage;

    // 16 bits -> u16
    let _: u16 = ExactlyU16::ZEROES.storage;

    // 17 bits -> u32
    let _: u32 = JustOverU16::ZEROES.storage;

    // 32 bits -> u32
    let _: u32 = ExactlyU32::ZEROES.storage;

    // 33 bits -> u64
    let _: u64 = JustOverU32::ZEROES.storage;
}

// =============================================================================
// All zeros and all ones patterns
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct MixedFields {
    a: B4,
    b: SB4,
    c: bool,
    d: B7,
}
bitpiece_check_full_impl! {MixedFields, true}

#[test]
fn all_zeros_pattern() {
    let val = MixedFields::from_bits(0);
    assert_eq!(val.a(), B4::new(0));
    assert_eq!(val.b(), SB4::new(0));
    assert_eq!(val.c(), false);
    assert_eq!(val.d(), B7::new(0));
}

#[test]
fn all_ones_pattern() {
    let val = MixedFields::from_bits(0xFFFF);
    assert_eq!(val.a(), B4::new(15));
    assert_eq!(val.b(), SB4::new(-1)); // All 1s in signed = -1
    assert_eq!(val.c(), true);
    assert_eq!(val.d(), B7::new(127));
}

// =============================================================================
// Alternating bit patterns
// =============================================================================

#[test]
fn alternating_bits_pattern() {
    // Layout: [d: B7 (7)][c: bool (1)][b: SB4 (4)][a: B4 (4)] = 16 bits
    // 0b0101010101010101
    // a = bits 0-3 = 0b0101 = 5
    // b = bits 4-7 = 0b0101 = 5 (signed)
    // c = bit 8 = 0b0 = false
    // d = bits 9-15 = 0b0101010 = 42
    let val = MixedFields::from_bits(0b0101010101010101);
    assert_eq!(val.a(), B4::new(0b0101));
    assert_eq!(val.b(), SB4::new(5)); // 0b0101 in 4-bit signed = 5
    assert_eq!(val.c(), true); // bit 8 is 1
    assert_eq!(val.d(), B7::new(0b0101010));

    // 0b1010101010101010
    // a = bits 0-3 = 0b1010 = 10
    // b = bits 4-7 = 0b1010 = -6 (signed)
    // c = bit 8 = 0b0 = false
    // d = bits 9-15 = 0b1010101 = 85
    let val = MixedFields::from_bits(0b1010101010101010);
    assert_eq!(val.a(), B4::new(0b1010));
    assert_eq!(val.b(), SB4::new(-6)); // 0b1010 in 4-bit signed = -6
    assert_eq!(val.c(), false); // bit 8 is 0
    assert_eq!(val.d(), B7::new(0b1010101));
}

// =============================================================================
// Field at maximum offset
// =============================================================================

#[bitpiece(64, all)]
#[derive(Debug, PartialEq, Eq)]
struct FieldAtEnd {
    padding: B63,
    last_bit: bool,
}
bitpiece_check_full_impl! {FieldAtEnd, true}

#[test]
fn field_at_maximum_offset() {
    assert_eq!(FieldAtEnd::LAST_BIT_OFFSET, 63);
    assert_eq!(FieldAtEnd::LAST_BIT_LEN, 1);

    let val = FieldAtEnd::from_bits(1u64 << 63);
    assert_eq!(val.padding(), B63::new(0));
    assert_eq!(val.last_bit(), true);

    let val = FieldAtEnd::from_bits((1u64 << 63) - 1);
    assert_eq!(val.padding(), B63::MAX);
    assert_eq!(val.last_bit(), false);
}

// =============================================================================
// Invalid enum values
// =============================================================================

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum SparseEnum {
    Zero = 0,
    Hundred = 100,
    TwoHundred = 200,
}
bitpiece_check_full_impl! {SparseEnum, false}

#[test]
fn invalid_enum_from_bits_panics() {
    expect_panic(|| {
        let _ = SparseEnum::from_bits(1);
    });
    expect_panic(|| {
        let _ = SparseEnum::from_bits(50);
    });
    expect_panic(|| {
        let _ = SparseEnum::from_bits(150);
    });
    expect_panic(|| {
        let _ = SparseEnum::from_bits(255);
    });
}

#[test]
fn invalid_enum_try_from_bits_returns_none() {
    assert_eq!(SparseEnum::try_from_bits(1), None);
    assert_eq!(SparseEnum::try_from_bits(50), None);
    assert_eq!(SparseEnum::try_from_bits(150), None);
    assert_eq!(SparseEnum::try_from_bits(255), None);
}

// =============================================================================
// Invalid B type values
// =============================================================================

#[test]
fn invalid_b_type_new_panics() {
    expect_panic(|| {
        let _ = B1::new(2);
    });
    expect_panic(|| {
        let _ = B4::new(16);
    });
    // B8 uses u8 storage, so we can't test values > 255 directly
    // Instead test B9 which uses u16 storage
    expect_panic(|| {
        let _ = B9::new(512);
    });
}

#[test]
fn invalid_b_type_from_bits_panics() {
    expect_panic(|| {
        let _ = B1::from_bits(2);
    });
    expect_panic(|| {
        let _ = B4::from_bits(16);
    });
}

#[test]
fn invalid_b_type_try_new_returns_none() {
    assert_eq!(B1::try_new(2), None);
    assert_eq!(B4::try_new(16), None);
    // B8 uses u8 storage, so we can't test values > 255 directly
    // Instead test B9 which uses u16 storage
    assert_eq!(B9::try_new(512), None);
}

#[test]
fn invalid_b_type_try_from_bits_returns_none() {
    assert_eq!(B1::try_from_bits(2), None);
    assert_eq!(B4::try_from_bits(16), None);
}

// =============================================================================
// Invalid SB type values
// =============================================================================

#[test]
fn invalid_sb_type_new_panics() {
    expect_panic(|| {
        let _ = SB4::new(8); // max is 7
    });
    expect_panic(|| {
        let _ = SB4::new(-9); // min is -8
    });
}

#[test]
fn invalid_sb_type_from_bits_panics() {
    expect_panic(|| {
        let _ = SB4::from_bits(16); // doesn't fit in 4 bits
    });
}

#[test]
fn invalid_sb_type_try_new_returns_none() {
    assert_eq!(SB4::try_new(8), None);
    assert_eq!(SB4::try_new(-9), None);
    // SB8 uses i8 storage, so we can't test values outside i8 range directly
    // Instead test SB9 which uses i16 storage
    assert_eq!(SB9::try_new(256), None);
    assert_eq!(SB9::try_new(-257), None);
}

#[test]
fn invalid_sb_type_try_from_bits_returns_none() {
    // SB4 uses u8 storage, so any 4-bit pattern is valid (it gets sign-extended)
    // We need to test with values that don't fit in 4 bits
    assert_eq!(SB4::try_from_bits(16), None); // 16 = 0b10000, doesn't fit in 4 bits
    assert_eq!(SB4::try_from_bits(32), None);
}

// =============================================================================
// Struct with invalid nested enum
// =============================================================================

#[bitpiece(12, all)]
#[derive(Debug, PartialEq, Eq)]
struct ContainerWithSparse {
    value: SparseEnum,
    extra: B4,
}
bitpiece_check_full_impl! {ContainerWithSparse, false}

#[test]
fn struct_with_invalid_enum_from_bits_panics() {
    expect_panic(|| {
        // 1 is not a valid SparseEnum value
        let _ = ContainerWithSparse::from_bits(0b0000_00000001);
    });
}

#[test]
fn struct_with_invalid_enum_try_from_bits_returns_none() {
    // 1 is not a valid SparseEnum value
    assert_eq!(ContainerWithSparse::try_from_bits(0b0000_00000001), None);

    // Valid values
    assert!(ContainerWithSparse::try_from_bits(0b0000_00000000).is_some()); // 0
    assert!(ContainerWithSparse::try_from_bits(0b0000_01100100).is_some()); // 100
    assert!(ContainerWithSparse::try_from_bits(0b0000_11001000).is_some()); // 200
}

// =============================================================================
// Deeply nested invalid enum
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct DeepContainer {
    inner: ContainerWithSparse,
    padding: B4,
}
bitpiece_check_full_impl! {DeepContainer, false}

#[test]
fn deeply_nested_invalid_enum_from_bits_panics() {
    expect_panic(|| {
        // Inner enum value 1 is invalid
        let _ = DeepContainer::from_bits(0b0000_0000_00000001);
    });
}

#[test]
fn deeply_nested_invalid_enum_try_from_bits_returns_none() {
    // Inner enum value 1 is invalid
    assert_eq!(DeepContainer::try_from_bits(0b0000_0000_00000001), None);

    // Valid
    assert!(DeepContainer::try_from_bits(0b0000_0000_00000000).is_some());
}

// =============================================================================
// Signed type edge cases
// =============================================================================

#[test]
fn signed_type_min_max_boundaries() {
    // SB8 boundaries
    assert_eq!(SB8::new(-128).get(), -128);
    assert_eq!(SB8::new(127).get(), 127);

    // SB16 boundaries
    assert_eq!(SB16::new(-32768).get(), -32768);
    assert_eq!(SB16::new(32767).get(), 32767);

    // SB32 boundaries
    assert_eq!(SB32::new(i32::MIN).get(), i32::MIN);
    assert_eq!(SB32::new(i32::MAX).get(), i32::MAX);

    // SB64 boundaries
    assert_eq!(SB64::new(i64::MIN).get(), i64::MIN);
    assert_eq!(SB64::new(i64::MAX).get(), i64::MAX);
}

#[test]
fn signed_type_zero_crossing() {
    // Test values around zero
    assert_eq!(SB8::new(-1).get(), -1);
    assert_eq!(SB8::new(0).get(), 0);
    assert_eq!(SB8::new(1).get(), 1);

    // Verify bit representation
    assert_eq!(SB8::new(-1).to_bits(), 0xFF);
    assert_eq!(SB8::new(0).to_bits(), 0x00);
    assert_eq!(SB8::new(1).to_bits(), 0x01);
}

// =============================================================================
// Bit extraction edge cases
// =============================================================================

#[test]
fn extract_bits_at_boundaries() {
    // Extract from position 0
    assert_eq!(extract_bits(0xFFFFFFFFFFFFFFFF, 0, 8), 0xFF);

    // Extract from high position
    assert_eq!(extract_bits(0xFFFFFFFFFFFFFFFF, 56, 8), 0xFF);

    // Extract single bit
    assert_eq!(extract_bits(0b10101010, 0, 1), 0);
    assert_eq!(extract_bits(0b10101010, 1, 1), 1);

    // Extract all 64 bits
    assert_eq!(extract_bits(0xDEADBEEFCAFEBABE, 0, 64), 0xDEADBEEFCAFEBABE);
}

#[test]
fn extract_bits_noshift_at_boundaries() {
    // Extract from position 0 (same as regular extract)
    assert_eq!(extract_bits_noshift(0xFF, 0, 4), 0x0F);

    // Extract from position 4
    assert_eq!(extract_bits_noshift(0xFF, 4, 4), 0xF0);
}

#[test]
fn modify_bits_at_boundaries() {
    // Modify at position 0
    assert_eq!(modify_bits(0x00, 0, 8, 0xFF), 0xFF);

    // Modify at high position
    assert_eq!(modify_bits(0x00, 56, 8, 0xFF), 0xFF00000000000000);

    // Modify single bit
    assert_eq!(modify_bits(0x00, 0, 1, 1), 0x01);
    assert_eq!(modify_bits(0xFF, 0, 1, 0), 0xFE);
}

// =============================================================================
// Roundtrip edge cases
// =============================================================================

#[test]
fn roundtrip_all_zeros() {
    let val = MixedFields::from_bits(0);
    assert_eq!(MixedFields::from_bits(val.to_bits()), val);
}

#[test]
fn roundtrip_all_ones() {
    let val = MixedFields::from_bits(0xFFFF);
    assert_eq!(MixedFields::from_bits(val.to_bits()), val);
}

#[test]
fn roundtrip_random_patterns() {
    let patterns = [0x1234u16, 0xABCD, 0x5555, 0xAAAA, 0x0F0F, 0xF0F0];
    for pattern in patterns {
        let val = MixedFields::from_bits(pattern);
        assert_eq!(MixedFields::from_bits(val.to_bits()), val);
    }
}

// =============================================================================
// Fields struct edge cases
// =============================================================================

#[test]
fn fields_struct_roundtrip() {
    let original = MixedFields::from_bits(0xABCD);
    let fields = original.to_fields();
    let reconstructed = MixedFields::from_fields(fields);
    assert_eq!(original, reconstructed);
}

// =============================================================================
// Const array utilities
// =============================================================================

#[test]
fn const_array_max_u64_single() {
    assert_eq!(const_array_max_u64(&[42]), 42);
}

#[test]
fn const_array_max_u64_multiple() {
    assert_eq!(const_array_max_u64(&[1, 5, 3, 2, 4]), 5);
    assert_eq!(const_array_max_u64(&[100, 50, 200, 150]), 200);
}

#[test]
fn const_array_min_u64_single() {
    assert_eq!(const_array_min_u64(&[42]), 42);
}

#[test]
fn const_array_min_u64_multiple() {
    assert_eq!(const_array_min_u64(&[5, 1, 3, 2, 4]), 1);
    assert_eq!(const_array_min_u64(&[100, 50, 200, 150]), 50);
}

// =============================================================================
// Multiple fields with same type
// =============================================================================

#[bitpiece(12, all)]
#[derive(Debug, PartialEq, Eq)]
struct SameTypeFields {
    a: B4,
    b: B4,
    c: B4,
}
bitpiece_check_full_impl! {SameTypeFields, true}

#[test]
fn same_type_fields_independence() {
    let mut val = SameTypeFields::ZEROES;

    val.set_a(B4::new(1));
    assert_eq!(val.a(), B4::new(1));
    assert_eq!(val.b(), B4::new(0));
    assert_eq!(val.c(), B4::new(0));

    val.set_b(B4::new(2));
    assert_eq!(val.a(), B4::new(1));
    assert_eq!(val.b(), B4::new(2));
    assert_eq!(val.c(), B4::new(0));

    val.set_c(B4::new(3));
    assert_eq!(val.a(), B4::new(1));
    assert_eq!(val.b(), B4::new(2));
    assert_eq!(val.c(), B4::new(3));
}

// =============================================================================
// Enum with maximum value for bit width
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
enum MaxValueEnum {
    Zero = 0,
    Max = 255,
}
bitpiece_check_full_impl! {MaxValueEnum, false}

#[test]
fn enum_with_max_value() {
    assert_eq!(MaxValueEnum::from_bits(0), MaxValueEnum::Zero);
    assert_eq!(MaxValueEnum::from_bits(255), MaxValueEnum::Max);

    // Values in between are invalid
    assert_eq!(MaxValueEnum::try_from_bits(1), None);
    assert_eq!(MaxValueEnum::try_from_bits(128), None);
    assert_eq!(MaxValueEnum::try_from_bits(254), None);
}

// =============================================================================
// Single field struct
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct SingleField {
    value: B8,
}
bitpiece_check_full_impl! {SingleField, true}

#[test]
fn single_field_struct() {
    let val = SingleField::from_bits(0xAB);
    assert_eq!(val.value(), B8::new(0xAB));

    let val = val.with_value(B8::new(0xCD));
    assert_eq!(val.value(), B8::new(0xCD));
}

// =============================================================================
// Many fields struct
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct ManyFields {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
    i: bool,
    j: bool,
    k: bool,
    l: bool,
    m: bool,
    n: bool,
    o: bool,
    p: bool,
}
bitpiece_check_full_impl! {ManyFields, true}

#[test]
fn many_fields_struct() {
    let val = ManyFields::from_bits(0b1010101010101010);

    assert_eq!(val.a(), false);
    assert_eq!(val.b(), true);
    assert_eq!(val.c(), false);
    assert_eq!(val.d(), true);
    // ... pattern continues
    assert_eq!(val.p(), true);
}

#[test]
fn many_fields_independence() {
    let mut val = ManyFields::ZEROES;

    val.set_a(true);
    assert_eq!(val.a(), true);
    assert_eq!(val.b(), false);

    val.set_p(true);
    assert_eq!(val.a(), true);
    assert_eq!(val.p(), true);
    assert_eq!(val.o(), false);
}
