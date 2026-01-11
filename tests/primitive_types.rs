//! Tests for primitive types (bool, u8-u64, i8-i64) implementing BitPiece.

use bitpiece::*;

// =============================================================================
// bool tests
// =============================================================================

#[test]
fn bool_bits_constant() {
    assert_eq!(<bool as BitPiece>::BITS, 1);
}

#[test]
fn bool_zeroes_ones() {
    assert_eq!(<bool as BitPiece>::ZEROES, false);
    assert_eq!(<bool as BitPiece>::ONES, true);
}

#[test]
fn bool_min_max() {
    assert_eq!(<bool as BitPiece>::MIN, false);
    assert_eq!(<bool as BitPiece>::MAX, true);
}

#[test]
fn bool_from_bits() {
    assert_eq!(bool::from_bits(0), false);
    assert_eq!(bool::from_bits(1), true);
    // Any non-zero value is true
    assert_eq!(bool::from_bits(2), true);
    assert_eq!(bool::from_bits(255), true);
}

#[test]
fn bool_try_from_bits() {
    assert_eq!(bool::try_from_bits(0), Some(false));
    assert_eq!(bool::try_from_bits(1), Some(true));
    // Any non-zero value is true
    assert_eq!(bool::try_from_bits(255), Some(true));
}

#[test]
fn bool_to_bits() {
    assert_eq!(false.to_bits(), 0);
    assert_eq!(true.to_bits(), 1);
}

#[test]
fn bool_roundtrip() {
    assert_eq!(bool::from_bits(false.to_bits()), false);
    assert_eq!(bool::from_bits(true.to_bits()), true);
}

#[test]
fn bool_converter_const_eq() {
    assert!(BitPieceBoolConverter::const_eq(true, true));
    assert!(BitPieceBoolConverter::const_eq(false, false));
    assert!(!BitPieceBoolConverter::const_eq(true, false));
}

// =============================================================================
// u8 tests
// =============================================================================

#[test]
fn u8_bits_constant() {
    assert_eq!(<u8 as BitPiece>::BITS, 8);
}

#[test]
fn u8_zeroes_ones() {
    assert_eq!(<u8 as BitPiece>::ZEROES, 0);
    assert_eq!(<u8 as BitPiece>::ONES, 255);
}

#[test]
fn u8_min_max() {
    assert_eq!(<u8 as BitPiece>::MIN, u8::MIN);
    assert_eq!(<u8 as BitPiece>::MAX, u8::MAX);
}

#[test]
fn u8_from_to_bits() {
    assert_eq!(u8::from_bits(0), 0);
    assert_eq!(u8::from_bits(255), 255);
    assert_eq!(u8::from_bits(42), 42);

    assert_eq!((0u8).to_bits(), 0);
    assert_eq!((255u8).to_bits(), 255);
    assert_eq!((42u8).to_bits(), 42);
}

#[test]
fn u8_try_from_bits() {
    assert_eq!(u8::try_from_bits(0), Some(0));
    assert_eq!(u8::try_from_bits(255), Some(255));
}

#[test]
fn u8_roundtrip() {
    for v in [0u8, 1, 127, 128, 255] {
        assert_eq!(u8::from_bits(v.to_bits()), v);
    }
}

// =============================================================================
// u16 tests
// =============================================================================

#[test]
fn u16_bits_constant() {
    assert_eq!(<u16 as BitPiece>::BITS, 16);
}

#[test]
fn u16_zeroes_ones() {
    assert_eq!(<u16 as BitPiece>::ZEROES, 0);
    assert_eq!(<u16 as BitPiece>::ONES, 65535);
}

#[test]
fn u16_min_max() {
    assert_eq!(<u16 as BitPiece>::MIN, u16::MIN);
    assert_eq!(<u16 as BitPiece>::MAX, u16::MAX);
}

#[test]
fn u16_from_to_bits() {
    assert_eq!(u16::from_bits(0), 0);
    assert_eq!(u16::from_bits(65535), 65535);
    assert_eq!(u16::from_bits(0xABCD), 0xABCD);

    assert_eq!((0u16).to_bits(), 0);
    assert_eq!((65535u16).to_bits(), 65535);
}

#[test]
fn u16_roundtrip() {
    for v in [0u16, 1, 32767, 32768, 65535] {
        assert_eq!(u16::from_bits(v.to_bits()), v);
    }
}

// =============================================================================
// u32 tests
// =============================================================================

#[test]
fn u32_bits_constant() {
    assert_eq!(<u32 as BitPiece>::BITS, 32);
}

#[test]
fn u32_zeroes_ones() {
    assert_eq!(<u32 as BitPiece>::ZEROES, 0);
    assert_eq!(<u32 as BitPiece>::ONES, u32::MAX);
}

#[test]
fn u32_min_max() {
    assert_eq!(<u32 as BitPiece>::MIN, u32::MIN);
    assert_eq!(<u32 as BitPiece>::MAX, u32::MAX);
}

#[test]
fn u32_from_to_bits() {
    assert_eq!(u32::from_bits(0), 0);
    assert_eq!(u32::from_bits(u32::MAX), u32::MAX);
    assert_eq!(u32::from_bits(0xDEADBEEF), 0xDEADBEEF);

    assert_eq!((0u32).to_bits(), 0);
    assert_eq!((u32::MAX).to_bits(), u32::MAX);
}

#[test]
fn u32_roundtrip() {
    for v in [0u32, 1, u32::MAX / 2, u32::MAX] {
        assert_eq!(u32::from_bits(v.to_bits()), v);
    }
}

// =============================================================================
// u64 tests
// =============================================================================

#[test]
fn u64_bits_constant() {
    assert_eq!(<u64 as BitPiece>::BITS, 64);
}

#[test]
fn u64_zeroes_ones() {
    assert_eq!(<u64 as BitPiece>::ZEROES, 0);
    assert_eq!(<u64 as BitPiece>::ONES, u64::MAX);
}

#[test]
fn u64_min_max() {
    assert_eq!(<u64 as BitPiece>::MIN, u64::MIN);
    assert_eq!(<u64 as BitPiece>::MAX, u64::MAX);
}

#[test]
fn u64_from_to_bits() {
    assert_eq!(u64::from_bits(0), 0);
    assert_eq!(u64::from_bits(u64::MAX), u64::MAX);
    assert_eq!(u64::from_bits(0xDEADBEEFCAFEBABE), 0xDEADBEEFCAFEBABE);

    assert_eq!((0u64).to_bits(), 0);
    assert_eq!((u64::MAX).to_bits(), u64::MAX);
}

#[test]
fn u64_roundtrip() {
    for v in [0u64, 1, u64::MAX / 2, u64::MAX] {
        assert_eq!(u64::from_bits(v.to_bits()), v);
    }
}

// =============================================================================
// i8 tests
// =============================================================================

#[test]
fn i8_bits_constant() {
    assert_eq!(<i8 as BitPiece>::BITS, 8);
}

#[test]
fn i8_zeroes_ones() {
    assert_eq!(<i8 as BitPiece>::ZEROES, 0);
    assert_eq!(<i8 as BitPiece>::ONES, -1); // All bits set = -1 in two's complement
}

#[test]
fn i8_min_max() {
    assert_eq!(<i8 as BitPiece>::MIN, i8::MIN);
    assert_eq!(<i8 as BitPiece>::MAX, i8::MAX);
}

#[test]
fn i8_from_to_bits() {
    // Positive values
    assert_eq!(i8::from_bits(0), 0);
    assert_eq!(i8::from_bits(127), 127);

    // Negative values (two's complement)
    assert_eq!(i8::from_bits(0xFF), -1);
    assert_eq!(i8::from_bits(0x80), -128);

    // to_bits
    assert_eq!((0i8).to_bits(), 0);
    assert_eq!((127i8).to_bits(), 127);
    assert_eq!((-1i8).to_bits(), 0xFF);
    assert_eq!((-128i8).to_bits(), 0x80);
}

#[test]
fn i8_roundtrip() {
    for v in [0i8, 1, -1, 127, -128] {
        assert_eq!(i8::from_bits(v.to_bits()), v);
    }
}

// =============================================================================
// i16 tests
// =============================================================================

#[test]
fn i16_bits_constant() {
    assert_eq!(<i16 as BitPiece>::BITS, 16);
}

#[test]
fn i16_zeroes_ones() {
    assert_eq!(<i16 as BitPiece>::ZEROES, 0);
    assert_eq!(<i16 as BitPiece>::ONES, -1);
}

#[test]
fn i16_min_max() {
    assert_eq!(<i16 as BitPiece>::MIN, i16::MIN);
    assert_eq!(<i16 as BitPiece>::MAX, i16::MAX);
}

#[test]
fn i16_from_to_bits() {
    assert_eq!(i16::from_bits(0), 0);
    assert_eq!(i16::from_bits(0x7FFF), 32767);
    assert_eq!(i16::from_bits(0xFFFF), -1);
    assert_eq!(i16::from_bits(0x8000), -32768);

    assert_eq!((0i16).to_bits(), 0);
    assert_eq!((32767i16).to_bits(), 0x7FFF);
    assert_eq!((-1i16).to_bits(), 0xFFFF);
    assert_eq!((-32768i16).to_bits(), 0x8000);
}

#[test]
fn i16_roundtrip() {
    for v in [0i16, 1, -1, 32767, -32768] {
        assert_eq!(i16::from_bits(v.to_bits()), v);
    }
}

// =============================================================================
// i32 tests
// =============================================================================

#[test]
fn i32_bits_constant() {
    assert_eq!(<i32 as BitPiece>::BITS, 32);
}

#[test]
fn i32_zeroes_ones() {
    assert_eq!(<i32 as BitPiece>::ZEROES, 0);
    assert_eq!(<i32 as BitPiece>::ONES, -1);
}

#[test]
fn i32_min_max() {
    assert_eq!(<i32 as BitPiece>::MIN, i32::MIN);
    assert_eq!(<i32 as BitPiece>::MAX, i32::MAX);
}

#[test]
fn i32_from_to_bits() {
    assert_eq!(i32::from_bits(0), 0);
    assert_eq!(i32::from_bits(0x7FFFFFFF), i32::MAX);
    assert_eq!(i32::from_bits(0xFFFFFFFF), -1);
    assert_eq!(i32::from_bits(0x80000000), i32::MIN);

    assert_eq!((0i32).to_bits(), 0);
    assert_eq!((i32::MAX).to_bits(), 0x7FFFFFFF);
    assert_eq!((-1i32).to_bits(), 0xFFFFFFFF);
    assert_eq!((i32::MIN).to_bits(), 0x80000000);
}

#[test]
fn i32_roundtrip() {
    for v in [0i32, 1, -1, i32::MAX, i32::MIN] {
        assert_eq!(i32::from_bits(v.to_bits()), v);
    }
}

// =============================================================================
// i64 tests
// =============================================================================

#[test]
fn i64_bits_constant() {
    assert_eq!(<i64 as BitPiece>::BITS, 64);
}

#[test]
fn i64_zeroes_ones() {
    assert_eq!(<i64 as BitPiece>::ZEROES, 0);
    assert_eq!(<i64 as BitPiece>::ONES, -1);
}

#[test]
fn i64_min_max() {
    assert_eq!(<i64 as BitPiece>::MIN, i64::MIN);
    assert_eq!(<i64 as BitPiece>::MAX, i64::MAX);
}

#[test]
fn i64_from_to_bits() {
    assert_eq!(i64::from_bits(0), 0);
    assert_eq!(i64::from_bits(0x7FFFFFFFFFFFFFFF), i64::MAX);
    assert_eq!(i64::from_bits(0xFFFFFFFFFFFFFFFF), -1);
    assert_eq!(i64::from_bits(0x8000000000000000), i64::MIN);

    assert_eq!((0i64).to_bits(), 0);
    assert_eq!((i64::MAX).to_bits(), 0x7FFFFFFFFFFFFFFF);
    assert_eq!((-1i64).to_bits(), 0xFFFFFFFFFFFFFFFF);
    assert_eq!((i64::MIN).to_bits(), 0x8000000000000000);
}

#[test]
fn i64_roundtrip() {
    for v in [0i64, 1, -1, i64::MAX, i64::MIN] {
        assert_eq!(i64::from_bits(v.to_bits()), v);
    }
}

// =============================================================================
// BitPiece trait generic tests
// =============================================================================

#[test]
fn primitive_types_implement_bitpiece() {
    fn assert_bitpiece<T: BitPiece>() {}

    assert_bitpiece::<bool>();
    assert_bitpiece::<u8>();
    assert_bitpiece::<u16>();
    assert_bitpiece::<u32>();
    assert_bitpiece::<u64>();
    assert_bitpiece::<i8>();
    assert_bitpiece::<i16>();
    assert_bitpiece::<i32>();
    assert_bitpiece::<i64>();
}

#[test]
fn primitive_types_implement_bitpiece_has_mut_ref() {
    fn assert_has_mut_ref<T: BitPieceHasMutRef>() {}

    assert_has_mut_ref::<bool>();
    assert_has_mut_ref::<u8>();
    assert_has_mut_ref::<u16>();
    assert_has_mut_ref::<u32>();
    assert_has_mut_ref::<u64>();
    assert_has_mut_ref::<i8>();
    assert_has_mut_ref::<i16>();
    assert_has_mut_ref::<i32>();
    assert_has_mut_ref::<i64>();
}

#[test]
fn primitive_types_implement_bitpiece_has_fields() {
    fn assert_has_fields<T: BitPieceHasFields>() {}

    assert_has_fields::<bool>();
    assert_has_fields::<u8>();
    assert_has_fields::<u16>();
    assert_has_fields::<u32>();
    assert_has_fields::<u64>();
    assert_has_fields::<i8>();
    assert_has_fields::<i16>();
    assert_has_fields::<i32>();
    assert_has_fields::<i64>();
}

// =============================================================================
// Converter const_eq tests
// =============================================================================

#[test]
fn unsigned_converter_const_eq() {
    assert!(BitPieceU8Converter::const_eq(42, 42));
    assert!(!BitPieceU8Converter::const_eq(42, 43));

    assert!(BitPieceU16Converter::const_eq(1000, 1000));
    assert!(!BitPieceU16Converter::const_eq(1000, 1001));

    assert!(BitPieceU32Converter::const_eq(100000, 100000));
    assert!(!BitPieceU32Converter::const_eq(100000, 100001));

    assert!(BitPieceU64Converter::const_eq(u64::MAX, u64::MAX));
    assert!(!BitPieceU64Converter::const_eq(u64::MAX, 0));
}

#[test]
fn signed_converter_const_eq() {
    assert!(BitPieceI8Converter::const_eq(-42, -42));
    assert!(!BitPieceI8Converter::const_eq(-42, 42));

    assert!(BitPieceI16Converter::const_eq(-1000, -1000));
    assert!(!BitPieceI16Converter::const_eq(-1000, 1000));

    assert!(BitPieceI32Converter::const_eq(-100000, -100000));
    assert!(!BitPieceI32Converter::const_eq(-100000, 100000));

    assert!(BitPieceI64Converter::const_eq(i64::MIN, i64::MIN));
    assert!(!BitPieceI64Converter::const_eq(i64::MIN, i64::MAX));
}
