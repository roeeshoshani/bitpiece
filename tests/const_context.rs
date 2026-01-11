//! Tests for const context usage.

use bitpiece::*;

// =============================================================================
// Const struct definition
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct Config {
    mode: B2,
    speed: B3,
    enabled: bool,
    reserved: B2,
}
bitpiece_check_full_impl! {Config, true}

// =============================================================================
// Const construction tests
// =============================================================================

const DEFAULT_CONFIG: Config = Config::from_bits(0b00_1_101_01);

#[test]
fn const_from_bits() {
    assert_eq!(DEFAULT_CONFIG.mode(), B2::new(1));
    assert_eq!(DEFAULT_CONFIG.speed(), B3::new(5));
    assert_eq!(DEFAULT_CONFIG.enabled(), true);
    assert_eq!(DEFAULT_CONFIG.reserved(), B2::new(0));
}

const ZEROES_CONFIG: Config = Config::ZEROES;
const ONES_CONFIG: Config = Config::ONES;
const MIN_CONFIG: Config = Config::MIN;
const MAX_CONFIG: Config = Config::MAX;

#[test]
fn const_constants() {
    assert_eq!(ZEROES_CONFIG.storage, 0);
    assert_eq!(ONES_CONFIG.storage, 0xFF);
    assert_eq!(MIN_CONFIG.storage, 0);
    assert_eq!(MAX_CONFIG.storage, 0xFF);
}

// =============================================================================
// Const field access tests
// =============================================================================

const DEFAULT_MODE: B2 = DEFAULT_CONFIG.mode();
const DEFAULT_SPEED: B3 = DEFAULT_CONFIG.speed();
const IS_ENABLED: bool = DEFAULT_CONFIG.enabled();

#[test]
fn const_field_access() {
    assert_eq!(DEFAULT_MODE.get(), 1);
    assert_eq!(DEFAULT_SPEED.get(), 5);
    assert_eq!(IS_ENABLED, true);
}

// =============================================================================
// Const modification tests (with_*)
// =============================================================================

const DISABLED_CONFIG: Config = DEFAULT_CONFIG.with_enabled(false);
const FAST_CONFIG: Config = DEFAULT_CONFIG.with_speed(B3::new(7));
const MODIFIED_CONFIG: Config = DEFAULT_CONFIG
    .with_mode(B2::new(3))
    .with_speed(B3::new(0))
    .with_enabled(false);

#[test]
fn const_with_methods() {
    assert_eq!(DISABLED_CONFIG.enabled(), false);
    assert_eq!(DISABLED_CONFIG.mode(), B2::new(1)); // unchanged

    assert_eq!(FAST_CONFIG.speed(), B3::new(7));
    assert_eq!(FAST_CONFIG.mode(), B2::new(1)); // unchanged

    assert_eq!(MODIFIED_CONFIG.mode(), B2::new(3));
    assert_eq!(MODIFIED_CONFIG.speed(), B3::new(0));
    assert_eq!(MODIFIED_CONFIG.enabled(), false);
}

// =============================================================================
// Const assertions
// =============================================================================

const _: () = assert!(DEFAULT_MODE.get() == 1);
const _: () = assert!(DEFAULT_SPEED.get() == 5);
const _: () = assert!(IS_ENABLED == true);
const _: () = assert!(DISABLED_CONFIG.enabled() == false);

// =============================================================================
// Const functions
// =============================================================================

const fn create_config(mode: u8, speed: u8, enabled: bool) -> Config {
    Config::ZEROES
        .with_mode(B2::new(mode))
        .with_speed(B3::new(speed))
        .with_enabled(enabled)
}

const CUSTOM_CONFIG: Config = create_config(2, 5, true);

#[test]
fn const_function() {
    assert_eq!(CUSTOM_CONFIG.mode(), B2::new(2));
    assert_eq!(CUSTOM_CONFIG.speed(), B3::new(5));
    assert_eq!(CUSTOM_CONFIG.enabled(), true);
}

// =============================================================================
// Const to_bits
// =============================================================================

const CONFIG_BITS: u8 = DEFAULT_CONFIG.to_bits();

#[test]
fn const_to_bits() {
    assert_eq!(CONFIG_BITS, 0b00_1_101_01);
}

// =============================================================================
// Const try_from_bits
// =============================================================================

const MAYBE_CONFIG: Option<Config> = Config::try_from_bits(0xFF);

#[test]
fn const_try_from_bits() {
    assert!(MAYBE_CONFIG.is_some());
    assert_eq!(MAYBE_CONFIG.unwrap().storage, 0xFF);
}

// =============================================================================
// Const const_eq
// =============================================================================

const ARE_EQUAL: bool = Config::const_eq(DEFAULT_CONFIG, DEFAULT_CONFIG);
const ARE_NOT_EQUAL: bool = Config::const_eq(DEFAULT_CONFIG, DISABLED_CONFIG);

#[test]
fn const_const_eq() {
    assert!(ARE_EQUAL);
    assert!(!ARE_NOT_EQUAL);
}

// =============================================================================
// Const B type operations
// =============================================================================

const B3_VAL: B3 = B3::new(5);
const B3_BITS: u8 = B3::to_bits(B3_VAL);
const B3_GET: u8 = B3_VAL.get();
const B3_FROM: B3 = B3::from_bits(5);
const B3_TRY: Option<B3> = B3::try_new(5);
const B3_TRY_INVALID: Option<B3> = B3::try_new(8);
const B3_EQ: bool = B3::const_eq(B3_VAL, B3_FROM);

#[test]
fn const_b_type_operations() {
    assert_eq!(B3_BITS, 5);
    assert_eq!(B3_GET, 5);
    assert_eq!(B3_FROM, B3::new(5));
    assert!(B3_TRY.is_some());
    assert!(B3_TRY_INVALID.is_none());
    assert!(B3_EQ);
}

// =============================================================================
// Const SB type operations
// =============================================================================

const SB5_VAL: SB5 = SB5::new(-10);
const SB5_BITS: u8 = SB5::to_bits(SB5_VAL);
const SB5_GET: i8 = SB5_VAL.get();
const SB5_FROM: SB5 = SB5::from_bits(SB5_BITS);
const SB5_TRY: Option<SB5> = SB5::try_new(-10);
const SB5_TRY_INVALID: Option<SB5> = SB5::try_new(-20);
const SB5_EQ: bool = SB5::const_eq(SB5_VAL, SB5_FROM);

#[test]
fn const_sb_type_operations() {
    assert_eq!(SB5_GET, -10);
    assert_eq!(SB5_FROM, SB5::new(-10));
    assert!(SB5_TRY.is_some());
    assert!(SB5_TRY_INVALID.is_none());
    assert!(SB5_EQ);
}

// =============================================================================
// Const enum operations
// =============================================================================

#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]
enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}
bitpiece_check_full_impl! {Priority, true}

const PRIORITY_VAL: Priority = Priority::High;
const PRIORITY_BITS: u8 = PRIORITY_VAL.to_bits();
const PRIORITY_FROM: Priority = Priority::from_bits(2);
const PRIORITY_TRY: Option<Priority> = Priority::try_from_bits(2);
const PRIORITY_TRY_INVALID: Option<Priority> = Priority::try_from_bits(5);
const PRIORITY_EQ: bool = Priority::const_eq(PRIORITY_VAL, PRIORITY_FROM);

#[test]
fn const_enum_operations() {
    assert_eq!(PRIORITY_BITS, 2);
    assert_eq!(PRIORITY_FROM, Priority::High);
    assert!(PRIORITY_TRY.is_some());
    assert!(PRIORITY_TRY_INVALID.is_none());
    assert!(PRIORITY_EQ);
}

// =============================================================================
// Const nested struct operations
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

const INNER_VAL: Inner = Inner::from_bits(0b1001);
const OUTER_VAL: Outer = Outer::ZEROES.with_inner(INNER_VAL).with_extra(B4::new(15));
const OUTER_INNER: Inner = OUTER_VAL.inner();
const OUTER_INNER_X: B2 = OUTER_INNER.x();

#[test]
fn const_nested_operations() {
    assert_eq!(INNER_VAL.x(), B2::new(1));
    assert_eq!(INNER_VAL.y(), B2::new(2));
    assert_eq!(OUTER_VAL.extra(), B4::new(15));
    assert_eq!(OUTER_INNER_X, B2::new(1));
}

// =============================================================================
// Const from_fields/to_fields
// =============================================================================

const CONFIG_FIELDS: ConfigFields = ConfigFields {
    mode: B2::new(2),
    speed: B3::new(4),
    enabled: true,
    reserved: B2::new(1),
};
const CONFIG_FROM_FIELDS: Config = Config::from_fields(CONFIG_FIELDS);
const CONFIG_TO_FIELDS: ConfigFields = CONFIG_FROM_FIELDS.to_fields();

#[test]
fn const_from_to_fields() {
    assert_eq!(CONFIG_FROM_FIELDS.mode(), B2::new(2));
    assert_eq!(CONFIG_FROM_FIELDS.speed(), B3::new(4));
    assert_eq!(CONFIG_FROM_FIELDS.enabled(), true);
    assert_eq!(CONFIG_FROM_FIELDS.reserved(), B2::new(1));

    assert_eq!(CONFIG_TO_FIELDS.mode, B2::new(2));
    assert_eq!(CONFIG_TO_FIELDS.speed, B3::new(4));
}

// =============================================================================
// Const noshift operations
// =============================================================================

const CONFIG_MODE_NOSHIFT: u8 = DEFAULT_CONFIG.mode_noshift();
const CONFIG_SPEED_NOSHIFT: u8 = DEFAULT_CONFIG.speed_noshift();

#[test]
fn const_noshift() {
    // mode is at offset 0, so noshift == regular
    assert_eq!(CONFIG_MODE_NOSHIFT, 0b01);

    // speed is at offset 2, so noshift keeps bits at position
    assert_eq!(CONFIG_SPEED_NOSHIFT, 0b101_00);
}

// =============================================================================
// Const array of bitfields
// =============================================================================

const CONFIG_ARRAY: [Config; 3] = [
    Config::from_bits(0x00),
    Config::from_bits(0x55),
    Config::from_bits(0xFF),
];

#[test]
fn const_array() {
    assert_eq!(CONFIG_ARRAY[0].storage, 0x00);
    assert_eq!(CONFIG_ARRAY[1].storage, 0x55);
    assert_eq!(CONFIG_ARRAY[2].storage, 0xFF);
}

// =============================================================================
// Const in static context
// =============================================================================

static STATIC_CONFIG: Config = Config::from_bits(0xAB);

#[test]
fn static_config() {
    assert_eq!(STATIC_CONFIG.storage, 0xAB);
}

// =============================================================================
// Const primitive type operations
// =============================================================================

const BOOL_FROM: bool = BitPieceBoolConverter::from_bits(1);
const BOOL_TO: u8 = BitPieceBoolConverter::to_bits(true);
const BOOL_EQ: bool = BitPieceBoolConverter::const_eq(true, true);

const U8_FROM: u8 = BitPieceU8Converter::from_bits(42);
const U8_TO: u8 = BitPieceU8Converter::to_bits(42);
const U8_EQ: bool = BitPieceU8Converter::const_eq(42, 42);

const I8_FROM: i8 = BitPieceI8Converter::from_bits(0xFF);
const I8_TO: u8 = BitPieceI8Converter::to_bits(-1);
const I8_EQ: bool = BitPieceI8Converter::const_eq(-1, -1);

#[test]
fn const_primitive_operations() {
    assert_eq!(BOOL_FROM, true);
    assert_eq!(BOOL_TO, 1);
    assert!(BOOL_EQ);

    assert_eq!(U8_FROM, 42);
    assert_eq!(U8_TO, 42);
    assert!(U8_EQ);

    assert_eq!(I8_FROM, -1);
    assert_eq!(I8_TO, 0xFF);
    assert!(I8_EQ);
}

// =============================================================================
// Const bit length and storage type
// =============================================================================

const CONFIG_BITS_TRAIT: usize = Config::BITS;

#[test]
fn const_bit_len() {
    // CONFIG_BIT_LEN is generated by the macro
    assert_eq!(CONFIG_BIT_LEN, 8);
    assert_eq!(CONFIG_BITS_TRAIT, 8);
}

// =============================================================================
// Complex const expression
// =============================================================================

const fn compute_checksum(config: Config) -> u8 {
    let mode = config.mode().get();
    let speed = config.speed().get();
    let enabled = if config.enabled() { 1 } else { 0 };
    mode ^ speed ^ enabled
}

const CHECKSUM: u8 = compute_checksum(DEFAULT_CONFIG);

#[test]
fn const_complex_expression() {
    // mode=1, speed=5, enabled=1 -> 1 ^ 5 ^ 1 = 5
    assert_eq!(CHECKSUM, 5);
}

// =============================================================================
// Const with generic-like patterns
// =============================================================================

const fn extract_and_modify<const N: usize>(configs: [Config; N], idx: usize) -> Config {
    configs[idx].with_enabled(false)
}

const MODIFIED_FROM_ARRAY: Config = extract_and_modify(CONFIG_ARRAY, 1);

#[test]
fn const_generic_like() {
    assert_eq!(MODIFIED_FROM_ARRAY.enabled(), false);
    // Original value at index 1 was 0x55
    assert_eq!(
        MODIFIED_FROM_ARRAY.storage & !0b00100000,
        0x55 & !0b00100000
    );
}
