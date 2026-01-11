//! Tests for signed arbitrary-width types (SB1-SB64).

mod common;

use bitpiece::*;
use common::expect_panic;

// =============================================================================
// Construction tests
// =============================================================================

#[test]
fn sb_type_new_valid_values() {
    // SB1: -1 to 0 (1 bit signed: sign bit only)
    assert_eq!(SB1::new(0).get(), 0);
    assert_eq!(SB1::new(-1).get(), -1);

    // SB3: -4 to 3
    assert_eq!(SB3::new(0).get(), 0);
    assert_eq!(SB3::new(3).get(), 3);
    assert_eq!(SB3::new(-1).get(), -1);
    assert_eq!(SB3::new(-4).get(), -4);

    // SB8: -128 to 127
    assert_eq!(SB8::new(0).get(), 0);
    assert_eq!(SB8::new(127).get(), 127);
    assert_eq!(SB8::new(-1).get(), -1);
    assert_eq!(SB8::new(-128).get(), -128);

    // SB16: -32768 to 32767
    assert_eq!(SB16::new(0).get(), 0);
    assert_eq!(SB16::new(32767).get(), 32767);
    assert_eq!(SB16::new(-32768).get(), -32768);

    // SB32: i32 range
    assert_eq!(SB32::new(0).get(), 0);
    assert_eq!(SB32::new(i32::MAX).get(), i32::MAX);
    assert_eq!(SB32::new(i32::MIN).get(), i32::MIN);

    // SB64: i64 range
    assert_eq!(SB64::new(0).get(), 0);
    assert_eq!(SB64::new(i64::MAX).get(), i64::MAX);
    assert_eq!(SB64::new(i64::MIN).get(), i64::MIN);
}

#[test]
fn sb_type_new_panics_on_overflow() {
    expect_panic(|| {
        let _ = SB3::new(4); // max is 3
    });
    expect_panic(|| {
        let _ = SB3::new(-5); // min is -4
    });
    // SB8 uses i8 storage, so we can't test values outside i8 range directly
    // Instead test SB9 which uses i16 storage
    expect_panic(|| {
        let _ = SB9::new(256); // max is 255
    });
    expect_panic(|| {
        let _ = SB9::new(-257); // min is -256
    });
}

#[test]
fn sb_type_try_new_valid_values() {
    assert_eq!(SB3::try_new(0), Some(SB3::new(0)));
    assert_eq!(SB3::try_new(3), Some(SB3::new(3)));
    assert_eq!(SB3::try_new(-4), Some(SB3::new(-4)));
    assert_eq!(SB8::try_new(127), Some(SB8::new(127)));
    assert_eq!(SB8::try_new(-128), Some(SB8::new(-128)));
}

#[test]
fn sb_type_try_new_invalid_values() {
    assert_eq!(SB3::try_new(4), None);
    assert_eq!(SB3::try_new(-5), None);
    assert_eq!(SB3::try_new(16), None);
    // SB8 uses i8 storage, so we can't test values outside i8 range directly
    // Instead test SB9 which uses i16 storage
    assert_eq!(SB9::try_new(256), None);
    assert_eq!(SB9::try_new(-257), None);
}

#[test]
fn sb_type_from_bits_valid() {
    // SB3: bits 0b000 = 0, 0b001 = 1, 0b011 = 3, 0b100 = -4, 0b111 = -1
    assert_eq!(SB3::from_bits(0b000).get(), 0);
    assert_eq!(SB3::from_bits(0b001).get(), 1);
    assert_eq!(SB3::from_bits(0b011).get(), 3);
    assert_eq!(SB3::from_bits(0b100).get(), -4);
    assert_eq!(SB3::from_bits(0b111).get(), -1);

    // SB8
    assert_eq!(SB8::from_bits(0x00).get(), 0);
    assert_eq!(SB8::from_bits(0x7F).get(), 127);
    assert_eq!(SB8::from_bits(0x80).get(), -128);
    assert_eq!(SB8::from_bits(0xFF).get(), -1);
}

#[test]
fn sb_type_from_bits_panics_on_overflow() {
    expect_panic(|| {
        let _ = SB3::from_bits(0b1000); // 8 doesn't fit in 3 bits
    });
    expect_panic(|| {
        let _ = SB3::from_bits(0b10001011);
    });
}

#[test]
fn sb_type_try_from_bits() {
    assert_eq!(SB3::try_from_bits(0b000).map(|x| x.get()), Some(0));
    assert_eq!(SB3::try_from_bits(0b001).map(|x| x.get()), Some(1));
    assert_eq!(SB3::try_from_bits(0b011).map(|x| x.get()), Some(3));
    assert_eq!(SB3::try_from_bits(0b100).map(|x| x.get()), Some(-4));
    assert_eq!(SB3::try_from_bits(0b111).map(|x| x.get()), Some(-1));
    assert_eq!(SB3::try_from_bits(0b1000), None);
    assert_eq!(SB3::try_from_bits(0b10001011), None);
}

#[test]
fn sb_type_to_bits() {
    // Positive values
    assert_eq!(SB3::new(0).to_bits(), 0b000);
    assert_eq!(SB3::new(1).to_bits(), 0b001);
    assert_eq!(SB3::new(3).to_bits(), 0b011);

    // Negative values (two's complement)
    assert_eq!(SB3::new(-1).to_bits(), 0b111);
    assert_eq!(SB3::new(-2).to_bits(), 0b110);
    assert_eq!(SB3::new(-4).to_bits(), 0b100);

    // SB8
    assert_eq!(SB8::new(0).to_bits(), 0x00);
    assert_eq!(SB8::new(127).to_bits(), 0x7F);
    assert_eq!(SB8::new(-1).to_bits(), 0xFF);
    assert_eq!(SB8::new(-128).to_bits(), 0x80);
}

// =============================================================================
// Constants tests
// =============================================================================

#[test]
fn sb_type_bits_constant() {
    assert_eq!(SB1::BITS, 1);
    assert_eq!(SB3::BITS, 3);
    assert_eq!(SB8::BITS, 8);
    assert_eq!(SB16::BITS, 16);
    assert_eq!(SB32::BITS, 32);
    assert_eq!(SB64::BITS, 64);
}

#[test]
fn sb_type_zeroes_constant() {
    // ZEROES is 0 for signed types
    assert_eq!(SB1::ZEROES.get(), 0);
    assert_eq!(SB3::ZEROES.get(), 0);
    assert_eq!(SB8::ZEROES.get(), 0);
    assert_eq!(SB16::ZEROES.get(), 0);
    assert_eq!(SB32::ZEROES.get(), 0);
    assert_eq!(SB64::ZEROES.get(), 0);
}

#[test]
fn sb_type_ones_constant() {
    // ONES is -1 for signed types (all bits set = -1 in two's complement)
    assert_eq!(SB1::ONES.get(), -1);
    assert_eq!(SB3::ONES.get(), -1);
    assert_eq!(SB8::ONES.get(), -1);
    assert_eq!(SB16::ONES.get(), -1);
    assert_eq!(SB32::ONES.get(), -1);
    assert_eq!(SB64::ONES.get(), -1);
}

#[test]
fn sb_type_min_constant() {
    // MIN is the most negative value
    assert_eq!(SB1::MIN.get(), -1); // 1-bit signed: only -1 and 0
    assert_eq!(SB3::MIN.get(), -4);
    assert_eq!(SB8::MIN.get(), i8::MIN);
    assert_eq!(SB16::MIN.get(), i16::MIN);
    assert_eq!(SB32::MIN.get(), i32::MIN);
    assert_eq!(SB64::MIN.get(), i64::MIN);
}

#[test]
fn sb_type_max_constant() {
    // MAX is the most positive value
    assert_eq!(SB1::MAX.get(), 0); // 1-bit signed: only -1 and 0
    assert_eq!(SB3::MAX.get(), 3);
    assert_eq!(SB8::MAX.get(), i8::MAX);
    assert_eq!(SB16::MAX.get(), i16::MAX);
    assert_eq!(SB32::MAX.get(), i32::MAX);
    assert_eq!(SB64::MAX.get(), i64::MAX);
}

// =============================================================================
// Odd bit widths tests
// =============================================================================

#[test]
fn sb_type_odd_widths() {
    // SB5: -16 to 15
    assert_eq!(SB5::MIN.get(), -16);
    assert_eq!(SB5::MAX.get(), 15);
    assert_eq!(SB5::new(15).get(), 15);
    assert_eq!(SB5::new(-16).get(), -16);
    assert!(SB5::try_new(16).is_none());
    assert!(SB5::try_new(-17).is_none());

    // SB13: -4096 to 4095
    assert_eq!(SB13::MIN.get(), -4096);
    assert_eq!(SB13::MAX.get(), 4095);
    assert_eq!(SB13::new(4095).get(), 4095);
    assert_eq!(SB13::new(-4096).get(), -4096);
    assert!(SB13::try_new(4096).is_none());
    assert!(SB13::try_new(-4097).is_none());

    // SB7: -64 to 63
    assert_eq!(SB7::MIN.get(), -64);
    assert_eq!(SB7::MAX.get(), 63);
}

// =============================================================================
// Sign extension tests
// =============================================================================

#[test]
fn sb_type_sign_extension() {
    // When converting from bits, negative values should be sign-extended
    // SB5 with value -1 (0b11111) should become -1 when sign-extended
    let val = SB5::from_bits(0b11111);
    assert_eq!(val.get(), -1);

    // SB5 with value -16 (0b10000) should become -16
    let val = SB5::from_bits(0b10000);
    assert_eq!(val.get(), -16);

    // Positive values should not be affected
    let val = SB5::from_bits(0b01111);
    assert_eq!(val.get(), 15);
}

// =============================================================================
// Display and Debug tests
// =============================================================================

#[test]
fn sb_type_display() {
    assert_eq!(format!("{}", SB3::new(-2)), "-2");
    assert_eq!(format!("{}", SB8::new(-128)), "-128");
    assert_eq!(format!("{}", SB16::new(1000)), "1000");
}

#[test]
fn sb_type_debug() {
    assert_eq!(format!("{:?}", SB3::new(-2)), "-2");
    assert_eq!(format!("{:?}", SB8::new(127)), "127");
}

// =============================================================================
// Comparison and ordering tests
// =============================================================================

#[test]
fn sb_type_equality() {
    assert_eq!(SB3::new(-2), SB3::new(-2));
    assert_ne!(SB3::new(-2), SB3::new(2));
}

#[test]
fn sb_type_ordering() {
    assert!(SB3::new(-4) < SB3::new(-1));
    assert!(SB3::new(-1) < SB3::new(0));
    assert!(SB3::new(0) < SB3::new(3));
    assert!(SB3::new(3) > SB3::new(-4));
}

#[test]
fn sb_type_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(SB3::new(-1));
    set.insert(SB3::new(2));
    set.insert(SB3::new(-1)); // duplicate
    assert_eq!(set.len(), 2);
}

// =============================================================================
// Default trait tests
// =============================================================================

#[test]
fn sb_type_default() {
    assert_eq!(SB3::default(), SB3::new(0));
    assert_eq!(SB8::default(), SB8::new(0));
    assert_eq!(SB64::default(), SB64::new(0));
}

// =============================================================================
// Clone and Copy tests
// =============================================================================

#[test]
fn sb_type_clone_copy() {
    let a = SB8::new(-42);
    let b = a; // Copy
    let c = a.clone(); // Clone
    assert_eq!(a, b);
    assert_eq!(a, c);
}

// =============================================================================
// const_eq tests
// =============================================================================

#[test]
fn sb_type_const_eq() {
    assert!(SB3::const_eq(SB3::new(-2), SB3::new(-2)));
    assert!(!SB3::const_eq(SB3::new(-2), SB3::new(2)));
}

// =============================================================================
// Storage type tests
// =============================================================================

#[test]
fn sb_type_storage_types() {
    // SB1-SB8 should use u8 for storage (unsigned)
    let _: u8 = SB1::new(-1).to_bits();
    let _: u8 = SB8::new(-128).to_bits();

    // SB9-SB16 should use u16
    let _: u16 = SB9::new(-256).to_bits();
    let _: u16 = SB16::new(-32768).to_bits();

    // SB17-SB32 should use u32
    let _: u32 = SB17::new(-65536).to_bits();
    let _: u32 = SB32::new(i32::MIN).to_bits();

    // SB33-SB64 should use u64
    let _: u64 = SB33::new(-1).to_bits();
    let _: u64 = SB64::new(i64::MIN).to_bits();
}

// =============================================================================
// Unsafe new_unchecked tests
// =============================================================================

#[test]
fn sb_type_new_unchecked() {
    // Safe usage
    let val = unsafe { SB3::new_unchecked(-2) };
    assert_eq!(val.get(), -2);
}

// =============================================================================
// BitPiece trait tests
// =============================================================================

#[test]
fn sb_type_bitpiece_trait() {
    fn test_bitpiece<T: BitPiece>() {
        let _ = T::BITS;
        let _ = T::ZEROES;
        let _ = T::ONES;
        let _ = T::MIN;
        let _ = T::MAX;
    }

    test_bitpiece::<SB1>();
    test_bitpiece::<SB8>();
    test_bitpiece::<SB16>();
    test_bitpiece::<SB32>();
    test_bitpiece::<SB64>();
}

// =============================================================================
// Roundtrip tests
// =============================================================================

#[test]
fn sb_type_roundtrip() {
    // Test that from_bits(to_bits(x)) == x
    let values = [0i8, 1, -1, 127, -128];
    for &v in &values {
        let b = SB8::new(v);
        assert_eq!(SB8::from_bits(b.to_bits()), b);
    }

    // Test various bit widths
    assert_eq!(SB3::from_bits(SB3::new(-2).to_bits()), SB3::new(-2));
    assert_eq!(
        SB16::from_bits(SB16::new(-1000).to_bits()),
        SB16::new(-1000)
    );
    assert_eq!(
        SB32::from_bits(SB32::new(-123456).to_bits()),
        SB32::new(-123456)
    );
}

// =============================================================================
// Two's complement verification
// =============================================================================

#[test]
fn sb_type_twos_complement() {
    // Verify two's complement representation
    // -1 should be all 1s
    assert_eq!(SB8::new(-1).to_bits(), 0xFF);
    assert_eq!(SB16::new(-1).to_bits(), 0xFFFF);
    assert_eq!(SB32::new(-1).to_bits(), 0xFFFFFFFF);

    // MIN should have only the sign bit set
    assert_eq!(SB8::new(-128).to_bits(), 0x80);
    assert_eq!(SB16::new(-32768).to_bits(), 0x8000);

    // MAX should have all bits except sign bit set
    assert_eq!(SB8::new(127).to_bits(), 0x7F);
    assert_eq!(SB16::new(32767).to_bits(), 0x7FFF);
}

// =============================================================================
// All SB types existence test
// =============================================================================

#[test]
fn all_sb_types_exist() {
    // Verify all SB types from SB1 to SB64 exist and have correct BITS
    assert_eq!(SB1::BITS, 1);
    assert_eq!(SB2::BITS, 2);
    assert_eq!(SB3::BITS, 3);
    assert_eq!(SB4::BITS, 4);
    assert_eq!(SB5::BITS, 5);
    assert_eq!(SB6::BITS, 6);
    assert_eq!(SB7::BITS, 7);
    assert_eq!(SB8::BITS, 8);
    assert_eq!(SB9::BITS, 9);
    assert_eq!(SB10::BITS, 10);
    assert_eq!(SB11::BITS, 11);
    assert_eq!(SB12::BITS, 12);
    assert_eq!(SB13::BITS, 13);
    assert_eq!(SB14::BITS, 14);
    assert_eq!(SB15::BITS, 15);
    assert_eq!(SB16::BITS, 16);
    assert_eq!(SB17::BITS, 17);
    assert_eq!(SB18::BITS, 18);
    assert_eq!(SB19::BITS, 19);
    assert_eq!(SB20::BITS, 20);
    assert_eq!(SB21::BITS, 21);
    assert_eq!(SB22::BITS, 22);
    assert_eq!(SB23::BITS, 23);
    assert_eq!(SB24::BITS, 24);
    assert_eq!(SB25::BITS, 25);
    assert_eq!(SB26::BITS, 26);
    assert_eq!(SB27::BITS, 27);
    assert_eq!(SB28::BITS, 28);
    assert_eq!(SB29::BITS, 29);
    assert_eq!(SB30::BITS, 30);
    assert_eq!(SB31::BITS, 31);
    assert_eq!(SB32::BITS, 32);
    assert_eq!(SB33::BITS, 33);
    assert_eq!(SB34::BITS, 34);
    assert_eq!(SB35::BITS, 35);
    assert_eq!(SB36::BITS, 36);
    assert_eq!(SB37::BITS, 37);
    assert_eq!(SB38::BITS, 38);
    assert_eq!(SB39::BITS, 39);
    assert_eq!(SB40::BITS, 40);
    assert_eq!(SB41::BITS, 41);
    assert_eq!(SB42::BITS, 42);
    assert_eq!(SB43::BITS, 43);
    assert_eq!(SB44::BITS, 44);
    assert_eq!(SB45::BITS, 45);
    assert_eq!(SB46::BITS, 46);
    assert_eq!(SB47::BITS, 47);
    assert_eq!(SB48::BITS, 48);
    assert_eq!(SB49::BITS, 49);
    assert_eq!(SB50::BITS, 50);
    assert_eq!(SB51::BITS, 51);
    assert_eq!(SB52::BITS, 52);
    assert_eq!(SB53::BITS, 53);
    assert_eq!(SB54::BITS, 54);
    assert_eq!(SB55::BITS, 55);
    assert_eq!(SB56::BITS, 56);
    assert_eq!(SB57::BITS, 57);
    assert_eq!(SB58::BITS, 58);
    assert_eq!(SB59::BITS, 59);
    assert_eq!(SB60::BITS, 60);
    assert_eq!(SB61::BITS, 61);
    assert_eq!(SB62::BITS, 62);
    assert_eq!(SB63::BITS, 63);
    assert_eq!(SB64::BITS, 64);
}

// =============================================================================
// Verify MIN/MAX ranges for various bit widths
// =============================================================================

#[test]
fn sb_type_ranges() {
    // SB2: -2 to 1
    assert_eq!(SB2::MIN.get(), -2);
    assert_eq!(SB2::MAX.get(), 1);

    // SB4: -8 to 7
    assert_eq!(SB4::MIN.get(), -8);
    assert_eq!(SB4::MAX.get(), 7);

    // SB6: -32 to 31
    assert_eq!(SB6::MIN.get(), -32);
    assert_eq!(SB6::MAX.get(), 31);

    // SB10: -512 to 511
    assert_eq!(SB10::MIN.get(), -512);
    assert_eq!(SB10::MAX.get(), 511);
}
