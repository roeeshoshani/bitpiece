//! Tests for BitPiece trait generic usage.

use bitpiece::*;

// =============================================================================
// Generic function tests
// =============================================================================

fn get_bits_count<T: BitPiece>() -> usize {
    T::BITS
}

#[test]
fn generic_bits_count() {
    assert_eq!(get_bits_count::<bool>(), 1);
    assert_eq!(get_bits_count::<u8>(), 8);
    assert_eq!(get_bits_count::<u16>(), 16);
    assert_eq!(get_bits_count::<u32>(), 32);
    assert_eq!(get_bits_count::<u64>(), 64);
    assert_eq!(get_bits_count::<i8>(), 8);
    assert_eq!(get_bits_count::<i16>(), 16);
    assert_eq!(get_bits_count::<i32>(), 32);
    assert_eq!(get_bits_count::<i64>(), 64);
    assert_eq!(get_bits_count::<B3>(), 3);
    assert_eq!(get_bits_count::<B13>(), 13);
    assert_eq!(get_bits_count::<SB5>(), 5);
    assert_eq!(get_bits_count::<SB17>(), 17);
}

fn get_zeroes<T: BitPiece>() -> T {
    T::ZEROES
}

#[test]
fn generic_zeroes() {
    assert_eq!(get_zeroes::<bool>(), false);
    assert_eq!(get_zeroes::<u8>(), 0);
    assert_eq!(get_zeroes::<i8>(), 0);
    assert_eq!(get_zeroes::<B8>(), B8::new(0));
    assert_eq!(get_zeroes::<SB8>(), SB8::new(0));
}

fn get_ones<T: BitPiece>() -> T {
    T::ONES
}

#[test]
fn generic_ones() {
    assert_eq!(get_ones::<bool>(), true);
    assert_eq!(get_ones::<u8>(), 255);
    assert_eq!(get_ones::<i8>(), -1);
    assert_eq!(get_ones::<B8>(), B8::new(255));
    assert_eq!(get_ones::<SB8>(), SB8::new(-1));
}

fn get_min<T: BitPiece>() -> T {
    T::MIN
}

#[test]
fn generic_min() {
    assert_eq!(get_min::<bool>(), false);
    assert_eq!(get_min::<u8>(), 0);
    assert_eq!(get_min::<i8>(), i8::MIN);
    assert_eq!(get_min::<B8>(), B8::new(0));
    assert_eq!(get_min::<SB8>(), SB8::new(i8::MIN));
}

fn get_max<T: BitPiece>() -> T {
    T::MAX
}

#[test]
fn generic_max() {
    assert_eq!(get_max::<bool>(), true);
    assert_eq!(get_max::<u8>(), u8::MAX);
    assert_eq!(get_max::<i8>(), i8::MAX);
    assert_eq!(get_max::<B8>(), B8::new(u8::MAX));
    assert_eq!(get_max::<SB8>(), SB8::new(i8::MAX));
}

// =============================================================================
// Generic conversion tests
// =============================================================================

fn roundtrip<T: BitPiece>(value: T) -> T {
    T::from_bits(value.to_bits())
}

#[test]
fn generic_roundtrip() {
    assert_eq!(roundtrip(true), true);
    assert_eq!(roundtrip(false), false);
    assert_eq!(roundtrip(42u8), 42u8);
    assert_eq!(roundtrip(-42i8), -42i8);
    assert_eq!(roundtrip(B8::new(100)), B8::new(100));
    assert_eq!(roundtrip(SB8::new(-50)), SB8::new(-50));
}

fn try_roundtrip<T: BitPiece>(value: T) -> Option<T> {
    T::try_from_bits(value.to_bits())
}

#[test]
fn generic_try_roundtrip() {
    assert_eq!(try_roundtrip(true), Some(true));
    assert_eq!(try_roundtrip(42u8), Some(42u8));
    assert_eq!(try_roundtrip(B8::new(100)), Some(B8::new(100)));
}

// =============================================================================
// Generic with bounds tests
// =============================================================================

fn print_bitpiece_info<T: BitPiece + core::fmt::Debug>(value: T) -> String
where
    T::Bits: core::fmt::Debug,
{
    format!(
        "bits={}, value={:?}, raw={:?}",
        T::BITS,
        value,
        value.to_bits()
    )
}

#[test]
fn generic_with_debug_bound() {
    let info = print_bitpiece_info(B8::new(42));
    assert!(info.contains("bits=8"));
    assert!(info.contains("42"));
}

// =============================================================================
// BitPieceHasMutRef trait tests
// =============================================================================

fn has_mut_ref<T: BitPieceHasMutRef>() -> bool {
    true
}

#[test]
fn types_have_mut_ref() {
    assert!(has_mut_ref::<bool>());
    assert!(has_mut_ref::<u8>());
    assert!(has_mut_ref::<u16>());
    assert!(has_mut_ref::<u32>());
    assert!(has_mut_ref::<u64>());
    assert!(has_mut_ref::<i8>());
    assert!(has_mut_ref::<i16>());
    assert!(has_mut_ref::<i32>());
    assert!(has_mut_ref::<i64>());
    assert!(has_mut_ref::<B8>());
    assert!(has_mut_ref::<SB8>());
}

// =============================================================================
// BitPieceHasFields trait tests
// =============================================================================

fn has_fields<T: BitPieceHasFields>() -> bool {
    true
}

#[test]
fn types_have_fields() {
    assert!(has_fields::<bool>());
    assert!(has_fields::<u8>());
    assert!(has_fields::<u16>());
    assert!(has_fields::<u32>());
    assert!(has_fields::<u64>());
    assert!(has_fields::<i8>());
    assert!(has_fields::<i16>());
    assert!(has_fields::<i32>());
    assert!(has_fields::<i64>());
    assert!(has_fields::<B8>());
    assert!(has_fields::<SB8>());
}

fn fields_roundtrip<T: BitPieceHasFields>(value: T) -> T
where
    T::Fields: Copy,
{
    T::from_fields(value.to_fields())
}

#[test]
fn generic_fields_roundtrip() {
    assert_eq!(fields_roundtrip(true), true);
    assert_eq!(fields_roundtrip(42u8), 42u8);
    assert_eq!(fields_roundtrip(B8::new(100)), B8::new(100));
}

// =============================================================================
// Custom struct with BitPiece trait
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct CustomStruct {
    a: B4,
    b: B4,
}
bitpiece_check_full_impl! {CustomStruct, true}

#[test]
fn custom_struct_generic_usage() {
    assert_eq!(get_bits_count::<CustomStruct>(), 8);
    assert_eq!(get_zeroes::<CustomStruct>(), CustomStruct::ZEROES);
    assert_eq!(get_ones::<CustomStruct>(), CustomStruct::ONES);

    let val = CustomStruct::from_bits(0xAB);
    assert_eq!(roundtrip(val), val);
    assert_eq!(try_roundtrip(val), Some(val));
}

// =============================================================================
// Custom enum with BitPiece trait
// =============================================================================

#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]
enum CustomEnum {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
bitpiece_check_full_impl! {CustomEnum, true}

#[test]
fn custom_enum_generic_usage() {
    assert_eq!(get_bits_count::<CustomEnum>(), 2);
    assert_eq!(get_zeroes::<CustomEnum>(), CustomEnum::A);
    assert_eq!(get_ones::<CustomEnum>(), CustomEnum::D);
    assert_eq!(get_min::<CustomEnum>(), CustomEnum::A);
    assert_eq!(get_max::<CustomEnum>(), CustomEnum::D);

    assert_eq!(roundtrip(CustomEnum::B), CustomEnum::B);
    assert_eq!(try_roundtrip(CustomEnum::C), Some(CustomEnum::C));
}

// =============================================================================
// Associated types tests
// =============================================================================

fn get_storage_size<T: BitPiece>() -> usize {
    core::mem::size_of::<T::Bits>()
}

#[test]
fn generic_storage_size() {
    assert_eq!(get_storage_size::<bool>(), 1);
    assert_eq!(get_storage_size::<u8>(), 1);
    assert_eq!(get_storage_size::<u16>(), 2);
    assert_eq!(get_storage_size::<u32>(), 4);
    assert_eq!(get_storage_size::<u64>(), 8);
    assert_eq!(get_storage_size::<B1>(), 1);
    assert_eq!(get_storage_size::<B8>(), 1);
    assert_eq!(get_storage_size::<B9>(), 2);
    assert_eq!(get_storage_size::<B16>(), 2);
    assert_eq!(get_storage_size::<B17>(), 4);
    assert_eq!(get_storage_size::<B32>(), 4);
    assert_eq!(get_storage_size::<B33>(), 8);
    assert_eq!(get_storage_size::<B64>(), 8);
}

// =============================================================================
// Collection of BitPiece types
// =============================================================================

fn sum_bits<T: BitPiece, const N: usize>(_values: [T; N]) -> usize {
    T::BITS * N
}

#[test]
fn generic_array_operations() {
    let arr = [B8::new(1), B8::new(2), B8::new(3)];
    assert_eq!(sum_bits(arr), 24);

    let arr = [true, false, true, false];
    assert_eq!(sum_bits(arr), 4);
}

// =============================================================================
// Trait object tests (where applicable)
// =============================================================================

// Note: BitPiece requires Copy, so it's not object-safe
// But we can test with concrete types in generic contexts

fn process_bitpiece<T: BitPiece + core::fmt::Debug + PartialEq>(
    value: T,
    expected_bits: usize,
) -> bool {
    T::BITS == expected_bits && roundtrip(value) == value
}

#[test]
fn generic_processing() {
    assert!(process_bitpiece(B8::new(42), 8));
    assert!(process_bitpiece(SB16::new(-100), 16));
    assert!(process_bitpiece(true, 1));
    assert!(process_bitpiece(0xABCDu16, 16));
}

// =============================================================================
// Nested generic usage
// =============================================================================

#[bitpiece(4, all)]
#[derive(Debug, PartialEq, Eq)]
struct Inner {
    x: B2,
    y: B2,
}
bitpiece_check_full_impl! {Inner, true}

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct Outer {
    inner: Inner,
    extra: B4,
}
bitpiece_check_full_impl! {Outer, true}

#[test]
fn nested_generic_usage() {
    assert_eq!(get_bits_count::<Inner>(), 4);
    assert_eq!(get_bits_count::<Outer>(), 8);

    let inner = Inner::from_bits(0b1001);
    assert_eq!(roundtrip(inner), inner);

    let outer = Outer::from_bits(0xAB);
    assert_eq!(roundtrip(outer), outer);
}

// =============================================================================
// Generic with multiple trait bounds
// =============================================================================

fn complex_generic<T>(value: T) -> (usize, T, T)
where
    T: BitPiece + BitPieceHasFields + core::fmt::Debug + PartialEq,
    T::Fields: Copy,
{
    let bits = T::BITS;
    let roundtripped = T::from_bits(value.to_bits());
    let fields_roundtripped = T::from_fields(value.to_fields());
    (bits, roundtripped, fields_roundtripped)
}

#[test]
fn complex_generic_usage() {
    let (bits, rt, frt) = complex_generic(B8::new(42));
    assert_eq!(bits, 8);
    assert_eq!(rt, B8::new(42));
    assert_eq!(frt, B8::new(42));

    let (bits, rt, frt) = complex_generic(CustomStruct::from_bits(0xAB));
    assert_eq!(bits, 8);
    assert_eq!(rt.a(), B4::new(0xB));
    assert_eq!(frt.a(), B4::new(0xB));
}

// =============================================================================
// BitStorage trait tests
// =============================================================================

fn storage_to_u64<T: BitStorage>(value: T) -> u64 {
    value.to_u64()
}

#[test]
fn bit_storage_to_u64() {
    assert_eq!(storage_to_u64(42u8), 42);
    assert_eq!(storage_to_u64(1000u16), 1000);
    assert_eq!(storage_to_u64(100000u32), 100000);
    assert_eq!(storage_to_u64(u64::MAX), u64::MAX);
}

fn storage_from_u64<T: BitStorage>(value: u64) -> Option<T> {
    T::from_u64(value).ok()
}

#[test]
fn bit_storage_from_u64() {
    assert_eq!(storage_from_u64::<u8>(42), Some(42u8));
    assert_eq!(storage_from_u64::<u8>(256), None); // overflow

    assert_eq!(storage_from_u64::<u16>(1000), Some(1000u16));
    assert_eq!(storage_from_u64::<u16>(70000), None); // overflow

    assert_eq!(storage_from_u64::<u32>(100000), Some(100000u32));
    assert_eq!(storage_from_u64::<u64>(u64::MAX), Some(u64::MAX));
}

// =============================================================================
// Const generic patterns
// =============================================================================

fn create_array_of_zeroes<T: BitPiece, const N: usize>() -> [T; N] {
    [T::ZEROES; N]
}

#[test]
fn const_generic_array() {
    let arr: [B8; 4] = create_array_of_zeroes();
    assert_eq!(arr, [B8::new(0); 4]);

    let arr: [bool; 3] = create_array_of_zeroes();
    assert_eq!(arr, [false; 3]);
}

fn create_array_of_ones<T: BitPiece, const N: usize>() -> [T; N] {
    [T::ONES; N]
}

#[test]
fn const_generic_array_ones() {
    let arr: [B8; 4] = create_array_of_ones();
    assert_eq!(arr, [B8::new(255); 4]);

    let arr: [bool; 3] = create_array_of_ones();
    assert_eq!(arr, [true; 3]);
}
