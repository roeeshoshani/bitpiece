//! Tests for unsigned arbitrary-width types (B1-B64).

mod common;

use bitpiece::*;
use common::expect_panic;

// =============================================================================
// Construction tests
// =============================================================================

#[test]
fn b_type_new_valid_values() {
    // B1: 0-1
    assert_eq!(B1::new(0).get(), 0);
    assert_eq!(B1::new(1).get(), 1);

    // B3: 0-7
    assert_eq!(B3::new(0).get(), 0);
    assert_eq!(B3::new(5).get(), 5);
    assert_eq!(B3::new(7).get(), 7);

    // B8: 0-255
    assert_eq!(B8::new(0).get(), 0);
    assert_eq!(B8::new(128).get(), 128);
    assert_eq!(B8::new(255).get(), 255);

    // B16: 0-65535
    assert_eq!(B16::new(0).get(), 0);
    assert_eq!(B16::new(32768).get(), 32768);
    assert_eq!(B16::new(65535).get(), 65535);

    // B32: 0-4294967295
    assert_eq!(B32::new(0).get(), 0);
    assert_eq!(B32::new(2147483648).get(), 2147483648);
    assert_eq!(B32::new(4294967295).get(), 4294967295);

    // B64: 0-u64::MAX
    assert_eq!(B64::new(0).get(), 0);
    assert_eq!(B64::new(u64::MAX / 2).get(), u64::MAX / 2);
    assert_eq!(B64::new(u64::MAX).get(), u64::MAX);
}

#[test]
fn b_type_new_panics_on_overflow() {
    expect_panic(|| {
        let _ = B1::new(2);
    });
    expect_panic(|| {
        let _ = B3::new(8);
    });
    expect_panic(|| {
        let _ = B7::new(128);
    });
}

#[test]
fn b_type_try_new_valid_values() {
    assert_eq!(B1::try_new(0), Some(B1::new(0)));
    assert_eq!(B1::try_new(1), Some(B1::new(1)));
    assert_eq!(B3::try_new(7), Some(B3::new(7)));
    assert_eq!(B8::try_new(255), Some(B8::new(255)));
}

#[test]
fn b_type_try_new_invalid_values() {
    assert_eq!(B1::try_new(2), None);
    assert_eq!(B3::try_new(8), None);
    assert_eq!(B7::try_new(128), None);
    // B8 uses u8 storage, so we can't test values > 255 directly
    // Instead test B9 which uses u16 storage
    assert_eq!(B9::try_new(512), None);
}

#[test]
fn b_type_from_bits_valid() {
    assert_eq!(B3::from_bits(0b101), B3::new(5));
    assert_eq!(B8::from_bits(0xFF), B8::new(255));
    assert_eq!(B16::from_bits(0xABCD), B16::new(0xABCD));
}

#[test]
fn b_type_from_bits_panics_on_overflow() {
    expect_panic(|| {
        let _ = B3::from_bits(0b1000);
    });
    expect_panic(|| {
        let _ = B3::from_bits(0b10001011);
    });
}

#[test]
fn b_type_try_from_bits() {
    assert_eq!(B1::try_from_bits(0), Some(B1::new(0)));
    assert_eq!(B1::try_from_bits(1), Some(B1::new(1)));
    assert_eq!(B1::try_from_bits(2), None);

    assert_eq!(B6::try_from_bits(17), Some(B6::new(17)));
    assert_eq!(B6::try_from_bits(63), Some(B6::new(63)));
    assert_eq!(B6::try_from_bits(64), None);
    assert_eq!(B6::try_from_bits(241), None);
}

#[test]
fn b_type_to_bits() {
    assert_eq!(B3::new(5).to_bits(), 5);
    assert_eq!(B8::new(200).to_bits(), 200);
    assert_eq!(B16::new(0xABCD).to_bits(), 0xABCD);
}

// =============================================================================
// Constants tests
// =============================================================================

#[test]
fn b_type_bits_constant() {
    assert_eq!(B1::BITS, 1);
    assert_eq!(B3::BITS, 3);
    assert_eq!(B8::BITS, 8);
    assert_eq!(B16::BITS, 16);
    assert_eq!(B32::BITS, 32);
    assert_eq!(B64::BITS, 64);
}

#[test]
fn b_type_zeroes_constant() {
    assert_eq!(B1::ZEROES.get(), 0);
    assert_eq!(B3::ZEROES.get(), 0);
    assert_eq!(B8::ZEROES.get(), 0);
    assert_eq!(B16::ZEROES.get(), 0);
    assert_eq!(B32::ZEROES.get(), 0);
    assert_eq!(B64::ZEROES.get(), 0);
}

#[test]
fn b_type_ones_constant() {
    assert_eq!(B1::ONES.get(), 1);
    assert_eq!(B3::ONES.get(), 0b111);
    assert_eq!(B8::ONES.get(), 0xFF);
    assert_eq!(B16::ONES.get(), 0xFFFF);
    assert_eq!(B32::ONES.get(), 0xFFFFFFFF);
    assert_eq!(B64::ONES.get(), u64::MAX);
}

#[test]
fn b_type_min_constant() {
    // For unsigned types, MIN == ZEROES
    assert_eq!(B1::MIN.get(), 0);
    assert_eq!(B3::MIN.get(), 0);
    assert_eq!(B8::MIN.get(), 0);
    assert_eq!(B64::MIN.get(), 0);
}

#[test]
fn b_type_max_constant() {
    // For unsigned types, MAX == ONES
    assert_eq!(B1::MAX.get(), 1);
    assert_eq!(B3::MAX.get(), 0b111);
    assert_eq!(B8::MAX.get(), u8::MAX);
    assert_eq!(B16::MAX.get(), u16::MAX);
    assert_eq!(B32::MAX.get(), u32::MAX);
    assert_eq!(B64::MAX.get(), u64::MAX);
}

// =============================================================================
// Odd bit widths tests
// =============================================================================

#[test]
fn b_type_odd_widths() {
    // B5: 0-31
    assert_eq!(B5::MAX.get(), 31);
    assert_eq!(B5::new(31).get(), 31);
    assert!(B5::try_new(32).is_none());

    // B13: 0-8191
    assert_eq!(B13::MAX.get(), 8191);
    assert_eq!(B13::new(8191).get(), 8191);
    assert!(B13::try_new(8192).is_none());

    // B27: 0-134217727
    assert_eq!(B27::MAX.get(), 134217727);
    assert_eq!(B27::new(134217727).get(), 134217727);
    assert!(B27::try_new(134217728).is_none());

    // B63: 0-(2^63-1)
    assert_eq!(B63::MAX.get(), (1u64 << 63) - 1);
    assert_eq!(B63::new((1u64 << 63) - 1).get(), (1u64 << 63) - 1);
    assert!(B63::try_new(1u64 << 63).is_none());
}

// =============================================================================
// Display and Debug tests
// =============================================================================

#[test]
fn b_type_display() {
    assert_eq!(format!("{}", B3::new(5)), "5");
    assert_eq!(format!("{}", B8::new(255)), "255");
    assert_eq!(format!("{}", B16::new(1000)), "1000");
}

#[test]
fn b_type_debug() {
    assert_eq!(format!("{:?}", B3::new(5)), "5");
    assert_eq!(format!("{:?}", B8::new(255)), "255");
}

// =============================================================================
// Comparison and ordering tests
// =============================================================================

#[test]
fn b_type_equality() {
    assert_eq!(B3::new(5), B3::new(5));
    assert_ne!(B3::new(5), B3::new(6));
}

#[test]
fn b_type_ordering() {
    assert!(B3::new(3) < B3::new(5));
    assert!(B3::new(7) > B3::new(2));
    assert!(B3::new(4) <= B3::new(4));
    assert!(B3::new(4) >= B3::new(4));
}

#[test]
fn b_type_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(B3::new(1));
    set.insert(B3::new(2));
    set.insert(B3::new(1)); // duplicate
    assert_eq!(set.len(), 2);
}

// =============================================================================
// Default trait tests
// =============================================================================

#[test]
fn b_type_default() {
    assert_eq!(B3::default(), B3::new(0));
    assert_eq!(B8::default(), B8::new(0));
    assert_eq!(B64::default(), B64::new(0));
}

// =============================================================================
// Clone and Copy tests
// =============================================================================

#[test]
fn b_type_clone_copy() {
    let a = B8::new(42);
    let b = a; // Copy
    let c = a.clone(); // Clone
    assert_eq!(a, b);
    assert_eq!(a, c);
}

// =============================================================================
// const_eq tests
// =============================================================================

#[test]
fn b_type_const_eq() {
    assert!(B3::const_eq(B3::new(5), B3::new(5)));
    assert!(!B3::const_eq(B3::new(5), B3::new(6)));
}

// =============================================================================
// Storage type tests
// =============================================================================

#[test]
fn b_type_storage_types() {
    // B1-B8 should use u8
    let _: u8 = B1::new(1).to_bits();
    let _: u8 = B8::new(255).to_bits();

    // B9-B16 should use u16
    let _: u16 = B9::new(511).to_bits();
    let _: u16 = B16::new(65535).to_bits();

    // B17-B32 should use u32
    let _: u32 = B17::new(131071).to_bits();
    let _: u32 = B32::new(u32::MAX).to_bits();

    // B33-B64 should use u64
    let _: u64 = B33::new((1u64 << 33) - 1).to_bits();
    let _: u64 = B64::new(u64::MAX).to_bits();
}

// =============================================================================
// Unsafe new_unchecked tests
// =============================================================================

#[test]
fn b_type_new_unchecked() {
    // Safe usage
    let val = unsafe { B3::new_unchecked(5) };
    assert_eq!(val.get(), 5);

    // Note: We don't test invalid usage as it's undefined behavior
}

// =============================================================================
// BitPiece trait tests
// =============================================================================

#[test]
fn b_type_bitpiece_trait() {
    fn test_bitpiece<T: BitPiece>() {
        let _ = T::BITS;
        let _ = T::ZEROES;
        let _ = T::ONES;
        let _ = T::MIN;
        let _ = T::MAX;
    }

    test_bitpiece::<B1>();
    test_bitpiece::<B8>();
    test_bitpiece::<B16>();
    test_bitpiece::<B32>();
    test_bitpiece::<B64>();
}

// =============================================================================
// Roundtrip tests
// =============================================================================

#[test]
fn b_type_roundtrip() {
    // Test that from_bits(to_bits(x)) == x
    let values = [0u8, 1, 127, 255];
    for &v in &values {
        let b = B8::new(v);
        assert_eq!(B8::from_bits(b.to_bits()), b);
    }

    // Test various bit widths
    assert_eq!(B3::from_bits(B3::new(5).to_bits()), B3::new(5));
    assert_eq!(B16::from_bits(B16::new(1000).to_bits()), B16::new(1000));
    assert_eq!(B32::from_bits(B32::new(123456).to_bits()), B32::new(123456));
}

// =============================================================================
// Boundary value tests
// =============================================================================

#[test]
fn b_type_boundary_values() {
    // Test at storage type boundaries
    // B8 at u8 boundary
    assert_eq!(B8::new(0).get(), 0);
    assert_eq!(B8::new(255).get(), 255);

    // B16 at u16 boundary
    assert_eq!(B16::new(0).get(), 0);
    assert_eq!(B16::new(65535).get(), 65535);

    // B32 at u32 boundary
    assert_eq!(B32::new(0).get(), 0);
    assert_eq!(B32::new(u32::MAX).get(), u32::MAX);

    // B64 at u64 boundary
    assert_eq!(B64::new(0).get(), 0);
    assert_eq!(B64::new(u64::MAX).get(), u64::MAX);
}

// =============================================================================
// All B types existence test
// =============================================================================

#[test]
fn all_b_types_exist() {
    // Verify all B types from B1 to B64 exist and have correct BITS
    assert_eq!(B1::BITS, 1);
    assert_eq!(B2::BITS, 2);
    assert_eq!(B3::BITS, 3);
    assert_eq!(B4::BITS, 4);
    assert_eq!(B5::BITS, 5);
    assert_eq!(B6::BITS, 6);
    assert_eq!(B7::BITS, 7);
    assert_eq!(B8::BITS, 8);
    assert_eq!(B9::BITS, 9);
    assert_eq!(B10::BITS, 10);
    assert_eq!(B11::BITS, 11);
    assert_eq!(B12::BITS, 12);
    assert_eq!(B13::BITS, 13);
    assert_eq!(B14::BITS, 14);
    assert_eq!(B15::BITS, 15);
    assert_eq!(B16::BITS, 16);
    assert_eq!(B17::BITS, 17);
    assert_eq!(B18::BITS, 18);
    assert_eq!(B19::BITS, 19);
    assert_eq!(B20::BITS, 20);
    assert_eq!(B21::BITS, 21);
    assert_eq!(B22::BITS, 22);
    assert_eq!(B23::BITS, 23);
    assert_eq!(B24::BITS, 24);
    assert_eq!(B25::BITS, 25);
    assert_eq!(B26::BITS, 26);
    assert_eq!(B27::BITS, 27);
    assert_eq!(B28::BITS, 28);
    assert_eq!(B29::BITS, 29);
    assert_eq!(B30::BITS, 30);
    assert_eq!(B31::BITS, 31);
    assert_eq!(B32::BITS, 32);
    assert_eq!(B33::BITS, 33);
    assert_eq!(B34::BITS, 34);
    assert_eq!(B35::BITS, 35);
    assert_eq!(B36::BITS, 36);
    assert_eq!(B37::BITS, 37);
    assert_eq!(B38::BITS, 38);
    assert_eq!(B39::BITS, 39);
    assert_eq!(B40::BITS, 40);
    assert_eq!(B41::BITS, 41);
    assert_eq!(B42::BITS, 42);
    assert_eq!(B43::BITS, 43);
    assert_eq!(B44::BITS, 44);
    assert_eq!(B45::BITS, 45);
    assert_eq!(B46::BITS, 46);
    assert_eq!(B47::BITS, 47);
    assert_eq!(B48::BITS, 48);
    assert_eq!(B49::BITS, 49);
    assert_eq!(B50::BITS, 50);
    assert_eq!(B51::BITS, 51);
    assert_eq!(B52::BITS, 52);
    assert_eq!(B53::BITS, 53);
    assert_eq!(B54::BITS, 54);
    assert_eq!(B55::BITS, 55);
    assert_eq!(B56::BITS, 56);
    assert_eq!(B57::BITS, 57);
    assert_eq!(B58::BITS, 58);
    assert_eq!(B59::BITS, 59);
    assert_eq!(B60::BITS, 60);
    assert_eq!(B61::BITS, 61);
    assert_eq!(B62::BITS, 62);
    assert_eq!(B63::BITS, 63);
    assert_eq!(B64::BITS, 64);
}
