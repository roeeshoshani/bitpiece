# bitpiece

A powerful Rust crate for working with bitfields. Define compact, type-safe bitfield structures with automatic bit packing and extraction.

[![Crates.io](https://img.shields.io/crates/v/bitpiece.svg)](https://crates.io/crates/bitpiece)
[![Documentation](https://docs.rs/bitpiece/badge.svg)](https://docs.rs/bitpiece)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Const-compatible**: All operations work in `const` contexts
- **`no_std` compatible**: Works in embedded and bare-metal environments
- **Type-safe**: Strong typing prevents mixing up different bitfield types
- **Flexible bit widths**: Support for arbitrary bit widths from 1 to 64 bits
- **Signed and unsigned**: Both signed (`SB*`) and unsigned (`B*`) arbitrary-width types
- **Nested bitfields**: Compose complex structures from simpler bitfield types
- **Enum support**: Use enums as bitfield members with automatic bit width calculation
- **Zero-cost abstractions**: Compiles down to efficient bit manipulation operations

## Quick Start

```rust
use bitpiece::*;

// Define a 2-bit enum
#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]
enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

// Define an 8-bit struct containing multiple fields
#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct StatusByte {
    enabled: bool,      // 1 bit
    priority: Priority, // 2 bits
    count: B5,          // 5 bits (unsigned, 0-31)
}

fn main() {
    // Create from raw bits
    let status = StatusByte::from_bits(0b10101_01_1);
    
    assert_eq!(status.enabled(), true);
    assert_eq!(status.priority(), Priority::Medium);
    assert_eq!(status.count(), B5::new(21));
    
    // Modify fields
    let updated = status
        .with_priority(Priority::Critical)
        .with_count(B5::new(7));
    
    assert_eq!(updated.to_bits(), 0b00111_11_1);
}
```

## Table of Contents

- [The `#[bitpiece]` Attribute](#the-bitpiece-attribute)
- [Built-in Types](#built-in-types)
- [Defining Bitfield Structs](#defining-bitfield-structs)
- [Defining Bitfield Enums](#defining-bitfield-enums)
- [Generated Methods and Types](#generated-methods-and-types)
- [Opt-in Features](#opt-in-features)
- [Attributes and Derives](#attributes-and-derives)
- [Working with Fields](#working-with-fields)
- [Nested Bitfields](#nested-bitfields)
- [Signed Types](#signed-types)
- [Const Context Usage](#const-context-usage)
- [The BitPiece Trait](#the-bitpiece-trait)
- [Error Handling](#error-handling)

## The `#[bitpiece]` Attribute

The `#[bitpiece]` attribute macro is the main entry point for defining bitfield types. It can be applied to structs and enums.

### Syntax

```rust
#[bitpiece]                    // Auto-calculate bit length, basic features
#[bitpiece(all)]               // Auto-calculate bit length, all features
#[bitpiece(32)]                // Explicit 32-bit length, basic features
#[bitpiece(32, all)]           // Explicit 32-bit length, all features
#[bitpiece(get, set)]          // Auto-calculate, specific features only
#[bitpiece(16, get, set, with)] // Explicit length with specific features
```

### Arguments

1. **Bit length** (optional): An integer specifying the exact bit length. If omitted, the bit length is calculated automatically from the fields (for structs) or variant values (for enums).

2. **Feature flags** (optional): Control which methods and types are generated. See [Opt-in Features](#opt-in-features) for details.

## Built-in Types

### Unsigned Arbitrary-Width Types (`B1` - `B64`)

Types for unsigned integers of specific bit widths:

```rust
use bitpiece::*;

let three_bits: B3 = B3::new(0b101);  // 3-bit value (0-7)
let five_bits: B5 = B5::new(31);       // 5-bit value (0-31)

assert_eq!(three_bits.get(), 5);
assert_eq!(B3::MAX.get(), 7);

// Validation
assert!(B3::try_new(7).is_some());   // Valid: fits in 3 bits
assert!(B3::try_new(8).is_none());   // Invalid: requires 4 bits
```

### Signed Arbitrary-Width Types (`SB1` - `SB64`)

Types for signed integers of specific bit widths using two's complement:

```rust
use bitpiece::*;

let signed: SB5 = SB5::new(-10);  // 5-bit signed value (-16 to 15)

assert_eq!(signed.get(), -10);
assert_eq!(SB5::MIN.get(), -16);
assert_eq!(SB5::MAX.get(), 15);

// Validation
assert!(SB3::try_new(3).is_some());   // Valid: fits in 3 bits
assert!(SB3::try_new(-4).is_some());  // Valid: minimum for SB3
assert!(SB3::try_new(4).is_none());   // Invalid: too large
assert!(SB3::try_new(-5).is_none());  // Invalid: too small
```

### Standard Integer Types

All standard Rust integer types implement `BitPiece`:

- Unsigned: `u8`, `u16`, `u32`, `u64`
- Signed: `i8`, `i16`, `i32`, `i64`

```rust
use bitpiece::*;

#[bitpiece(48, all)]
struct MixedTypes {
    byte: u8,      // 8 bits
    word: u16,     // 16 bits
    flags: B8,     // 8 bits
    signed: i16,   // 16 bits
}
```

### Boolean Type

`bool` is a 1-bit type:

```rust
use bitpiece::*;

#[bitpiece(3, all)]
struct Flags {
    read: bool,    // 1 bit
    write: bool,   // 1 bit
    execute: bool, // 1 bit
}
```

## Defining Bitfield Structs

Structs are the primary way to define composite bitfields. Fields are packed in order from least significant bit (LSB) to most significant bit (MSB).

```rust
use bitpiece::*;

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    opcode: B4,    // Bits 0-3 (LSB)
    reg_a: B3,     // Bits 4-6
    reg_b: B3,     // Bits 7-9
    immediate: B6, // Bits 10-15 (MSB)
}

// Bit layout:
// [immediate: 6 bits][reg_b: 3 bits][reg_a: 3 bits][opcode: 4 bits]
// MSB                                                          LSB
```

### Field Ordering

Fields are packed starting from bit 0:

```rust
#[bitpiece(8, all)]
struct Example {
    a: B2,  // Bits 0-1
    b: B3,  // Bits 2-4
    c: B3,  // Bits 5-7
}

let val = Example::from_bits(0b111_010_01);
assert_eq!(val.a(), B2::new(0b01));
assert_eq!(val.b(), B3::new(0b010));
assert_eq!(val.c(), B3::new(0b111));
```

## Defining Bitfield Enums

Enums can be used as bitfield types. The bit width is automatically calculated from the variant values, or can be specified explicitly.

### Exhaustive Enums

When all possible bit patterns map to valid variants:

```rust
use bitpiece::*;

#[bitpiece(2, all)]  // 2 bits = 4 possible values
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

// All 2-bit values (0-3) are valid
let dir = Direction::from_bits(2);
assert_eq!(dir, Direction::South);
```

### Non-Exhaustive Enums

When not all bit patterns are valid variants:

```rust
use bitpiece::*;

#[bitpiece(all)]  // Auto-calculated: 7 bits needed for value 100
#[derive(Debug, PartialEq, Eq)]
enum ErrorCode {
    Success = 0,
    NotFound = 10,
    PermissionDenied = 50,
    InternalError = 100,
}

// Valid variant
assert_eq!(ErrorCode::from_bits(10), ErrorCode::NotFound);

// Invalid bit pattern panics in from_bits
// Use try_from_bits for safe conversion
assert!(ErrorCode::try_from_bits(25).is_none());
assert!(ErrorCode::try_from_bits(50).is_some());
```

### Explicit Bit Length for Enums

You can specify a larger bit length than required:

```rust
use bitpiece::*;

#[bitpiece(16, all)]  // Use 16 bits even though values fit in fewer
#[derive(Debug, PartialEq, Eq)]
enum Command {
    Nop = 0,
    Load = 1,
    Store = 2,
}

// Can accept 16-bit values
assert!(Command::try_from_bits(1000).is_none());
```

## Generated Methods and Types

When you apply `#[bitpiece]` to a struct, several methods and types are generated.

### Generated Constants

```rust
#[bitpiece(16, all)]
struct MyStruct { /* ... */ }

// Generated constants:
const MY_STRUCT_BIT_LEN: usize = 16;
type MyStructStorageTy = u16;  // Smallest type that fits
```

### Field Constants

For each field, offset and length constants are generated:

```rust
#[bitpiece(8, all)]
struct Example {
    a: B3,
    b: B5,
}

// Generated:
// Example::A_OFFSET = 0
// Example::A_LEN = 3
// Example::B_OFFSET = 3
// Example::B_LEN = 5
```

### Core Methods

```rust
impl MyStruct {
    // Create from raw bits (panics if invalid for non-exhaustive types)
    pub const fn from_bits(bits: StorageTy) -> Self;
    
    // Try to create from raw bits (returns None if invalid)
    pub const fn try_from_bits(bits: StorageTy) -> Option<Self>;
    
    // Convert to raw bits
    pub const fn to_bits(self) -> StorageTy;
}
```

### Associated Constants

```rust
impl BitPiece for MyStruct {
    const BITS: usize;   // Total bit length
    const ZEROES: Self;  // All bits set to 0 (for structs: each field's ZEROES)
    const ONES: Self;    // All bits set to 1 (for structs: each field's ONES)
    const MIN: Self;     // The minimum value (for structs: each field's MIN)
    const MAX: Self;     // The maximum value (for structs: each field's MAX)
}
```

**Important distinction between `ONES`/`ZEROES` and `MAX`/`MIN`:**

- `ZEROES`: All bits are 0. For unsigned types, this equals `MIN`. For signed types, this is 0 (not the minimum).
- `ONES`: All bits are 1. For unsigned types, this equals `MAX`. For signed types like `i8`, this represents `-1` (not the maximum).
- `MIN`: The minimum representable value. For `i8`, this is `-128`.
- `MAX`: The maximum representable value. For `i8`, this is `127`.

```rust
use bitpiece::*;

// For unsigned types: ZEROES == MIN, ONES == MAX
assert_eq!(B8::ZEROES.get(), 0);
assert_eq!(B8::ONES.get(), 255);
assert_eq!(B8::MIN.get(), 0);
assert_eq!(B8::MAX.get(), 255);

// For signed types: ONES != MAX, ZEROES != MIN
assert_eq!(SB8::ZEROES.get(), 0);    // All bits 0 = 0
assert_eq!(SB8::ONES.get(), -1);     // All bits 1 = -1 in two's complement
assert_eq!(SB8::MIN.get(), -128);    // Minimum value
assert_eq!(SB8::MAX.get(), 127);     // Maximum value
```

**Non-exhaustive enums:** For enums where not all bit patterns are valid variants, `ZEROES` and `ONES` represent the closest valid variant to the all-zeros or all-ones bit pattern (i.e., `MIN` and `MAX` respectively). If an enum has no variant with value 0, `ZEROES` will be the variant with the smallest value, not a value with all bits set to zero.

```rust
#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum Sparse {
    A = 10,
    B = 50,
    C = 100,
}

// No variant has value 0, so ZEROES is the minimum variant
assert_eq!(Sparse::ZEROES, Sparse::A);  // Value 10, not 0
assert_eq!(Sparse::MIN, Sparse::A);
assert_eq!(Sparse::ONES, Sparse::C);    // Maximum variant
assert_eq!(Sparse::MAX, Sparse::C);
```

## Opt-in Features

Control which methods and types are generated using feature flags.

### Feature Flags

| Flag | Description |
|------|-------------|
| `get` | Field getter methods: `field_name()` |
| `set` | Field setter methods: `set_field_name(value)` |
| `with` | Builder-style methods: `with_field_name(value)` |
| `get_noshift` | Raw bit access: `field_name_noshift()` |
| `get_mut` | Mutable field references: `field_name_mut()` |
| `const_eq` | Const equality comparison |
| `fields_struct` | Generate `TypeNameFields` struct |
| `mut_struct` | Generate `TypeNameMutRef` type |
| `mut_struct_field_get` | Getter methods on MutRef |
| `mut_struct_field_set` | Setter methods on MutRef |
| `mut_struct_field_get_noshift` | Noshift getters on MutRef |
| `mut_struct_field_mut` | Nested mutable references on MutRef |

### Presets

| Preset | Includes |
|--------|----------|
| `basic` | `get`, `set`, `with` (default if no flags specified) |
| `all` | All features |
| `mut_struct_all` | All `mut_struct*` features |

### Examples

```rust
// Only getters
#[bitpiece(8, get)]
struct ReadOnly { /* ... */ }

// Getters and setters, no builder pattern
#[bitpiece(8, get, set)]
struct Mutable { /* ... */ }

// Everything
#[bitpiece(8, all)]
struct Full { /* ... */ }

// Custom combination
#[bitpiece(8, get, with, fields_struct)]
struct Custom { /* ... */ }
```

## Attributes and Derives

When you apply `#[bitpiece]` to a type, any attributes you place on the type (such as `#[derive(...)]`) are applied to both the main generated type and the generated fields struct (if `fields_struct` is enabled).

### Automatic Clone and Copy

The `Clone` and `Copy` traits are **automatically derived** on all bitpiece types. You do not need to (and should not) manually derive these traits:

```rust
use bitpiece::*;

// Clone and Copy are automatically derived - don't include them!
#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]  // No Clone, Copy needed
struct MyStruct {
    a: B4,
    b: B4,
}

// Same for enums
#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]  // No Clone, Copy needed
enum MyEnum {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
```

This automatic derivation ensures that all bitpiece types satisfy the `Copy` bound required by the `BitPiece` trait.

### Deriving Additional Traits

You can derive additional traits like `Debug`, `PartialEq`, `Eq`, `Hash`, or even third-party traits like `serde::Serialize` and `serde::Deserialize`:

```rust
use bitpiece::*;

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq, Hash)]
struct Packet {
    version: B4,
    flags: B4,
    length: u8,
}

// With serde (requires serde feature/dependency)
// #[bitpiece(8, all)]
// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// struct Config {
//     mode: B4,
//     level: B4,
// }
```

These attributes are applied to both the main `Packet` type and the `PacketFields` struct, allowing you to serialize/deserialize both types consistently.

## Working with Fields

### Getting Field Values

```rust
#[bitpiece(8, all)]
struct Packet {
    version: B2,
    flags: B3,
    length: B3,
}

let packet = Packet::from_bits(0b101_110_01);

// Get individual fields
let version = packet.version();  // B2
let flags = packet.flags();      // B3
let length = packet.length();    // B3

assert_eq!(version.get(), 1);
assert_eq!(flags.get(), 6);
assert_eq!(length.get(), 5);
```

### Setting Field Values (Immutable)

The `with_*` methods return a new instance with the field modified:

```rust
let packet = Packet::ZEROES;

// Chain modifications
let updated = packet
    .with_version(B2::new(2))
    .with_flags(B3::new(7))
    .with_length(B3::new(4));

// Original unchanged
assert_eq!(packet.to_bits(), 0);
```

### Setting Field Values (Mutable)

The `set_*` methods modify the instance in place:

```rust
let mut packet = Packet::ZEROES;

packet.set_version(B2::new(2));
packet.set_flags(B3::new(7));
packet.set_length(B3::new(4));
```

### Raw Bit Access (Noshift)

Get field bits at their original position without shifting:

```rust
#[bitpiece(8, all)]
struct Example {
    a: B3,  // Bits 0-2
    b: B5,  // Bits 3-7
}

let val = Example::from_bits(0b11111_010);

// Normal getter: shifts to bit 0
assert_eq!(val.b().get(), 0b11111);

// Noshift: keeps original position
assert_eq!(val.b_noshift(), 0b11111_000);
```

### Mutable References

Get a mutable reference to a field within the bitfield:

```rust
#[bitpiece(8, all)]
struct Container {
    inner: B4,
    outer: B4,
}

let mut container = Container::ZEROES;
{
    let mut inner_ref = container.inner_mut();
    inner_ref.set(B4::new(15));
}
assert_eq!(container.inner().get(), 15);
```

## Nested Bitfields

Bitfield types can be nested within other bitfields:

```rust
use bitpiece::*;

#[bitpiece(4, all)]
#[derive(Debug, PartialEq, Eq)]
struct Inner {
    x: B2,
    y: B2,
}

#[bitpiece(12, all)]
#[derive(Debug, PartialEq, Eq)]
struct Outer {
    a: Inner,    // 4 bits
    b: Inner,    // 4 bits
    c: B4,       // 4 bits
}

let outer = Outer::from_bits(0b1010_0110_0011);

// Access nested fields
assert_eq!(outer.a().x(), B2::new(3));
assert_eq!(outer.a().y(), B2::new(0));
assert_eq!(outer.b().x(), B2::new(2));
assert_eq!(outer.b().y(), B2::new(1));
assert_eq!(outer.c(), B4::new(10));
```

### Deep Nesting

```rust
#[bitpiece(8, all)]
struct Level1 {
    data: B4,
    flags: B4,
}

#[bitpiece(16, all)]
struct Level2 {
    l1_a: Level1,
    l1_b: Level1,
}

#[bitpiece(32, all)]
struct Level3 {
    l2: Level2,
    extra: u16,
}

let l3 = Level3::from_bits(0x12345678);
let nested_data = l3.l2().l1_a().data();
```

## Signed Types

### Using Standard Signed Integers

```rust
#[bitpiece(24, all)]
struct SignedExample {
    small: i8,   // 8-bit signed
    large: i16,  // 16-bit signed
}

let val = SignedExample::from_fields(SignedExampleFields {
    small: -50,
    large: -1000,
});

assert_eq!(val.small(), -50i8);
assert_eq!(val.large(), -1000i16);
```

### Using Arbitrary-Width Signed Types

```rust
#[bitpiece(16, all)]
struct CustomSigned {
    a: SB5,   // 5-bit signed (-16 to 15)
    b: bool,
    c: SB7,   // 7-bit signed (-64 to 63)
    d: B3,
}

let val = CustomSigned::from_bits(0b101_0101010_1_11111);

assert_eq!(val.a(), SB5::new(-1));
assert_eq!(val.b(), true);
assert_eq!(val.c(), SB7::new(42));
assert_eq!(val.d(), B3::new(5));
```

## Const Context Usage

All operations work in `const` contexts:

```rust
use bitpiece::*;

#[bitpiece(8, all)]
struct Config {
    mode: B2,
    speed: B3,
    enabled: bool,
    reserved: B2,
}

// Const construction
const DEFAULT_CONFIG: Config = Config::from_bits(0b00_1_101_01);

// Const field access
const DEFAULT_MODE: B2 = DEFAULT_CONFIG.mode();
const DEFAULT_SPEED: B3 = DEFAULT_CONFIG.speed();
const IS_ENABLED: bool = DEFAULT_CONFIG.enabled();

// Const modification
const DISABLED_CONFIG: Config = DEFAULT_CONFIG.with_enabled(false);

// Const assertions
const _: () = assert!(DEFAULT_MODE.get() == 1);
const _: () = assert!(DEFAULT_SPEED.get() == 5);
const _: () = assert!(IS_ENABLED == true);
```

### Const Functions

```rust
const fn create_packet(version: u8, flags: u8) -> Packet {
    Packet::ZEROES
        .with_version(B2::new(version))
        .with_flags(B3::new(flags))
}

const PACKET: Packet = create_packet(2, 5);
```

## The BitPiece Trait

All bitfield types implement the `BitPiece` trait:

```rust
pub trait BitPiece: Copy {
    /// The length in bits of this type
    const BITS: usize;
    
    /// A value with all bits set to 0 (see note below for enums)
    const ZEROES: Self;
    
    /// A value with all bits set to 1 (see note below for enums)
    const ONES: Self;
    
    /// The minimum representable value
    const MIN: Self;
    
    /// The maximum representable value
    const MAX: Self;
    
    /// The storage type used internally
    type Bits: BitStorage;
    
    /// Try to create from raw bits
    fn try_from_bits(bits: Self::Bits) -> Option<Self>;
    
    /// Create from raw bits (may panic)
    fn from_bits(bits: Self::Bits) -> Self;
    
    /// Convert to raw bits
    fn to_bits(self) -> Self::Bits;
}
```

### Using the Trait Generically

```rust
use bitpiece::*;

fn print_bitpiece_info<T: BitPiece + core::fmt::Debug>(value: T) {
    println!("Bits: {}", T::BITS);
    println!("Value: {:?}", value);
    println!("Raw: {:?}", value.to_bits());
}
```

## Error Handling

### Safe Conversion with `try_from_bits`

```rust
use bitpiece::*;

#[bitpiece(all)]
#[derive(Debug, PartialEq, Eq)]
enum Status {
    Ok = 0,
    Error = 1,
    Pending = 2,
}

// Safe conversion
match Status::try_from_bits(1) {
    Some(status) => println!("Status: {:?}", status),
    None => println!("Invalid status code"),
}

// For exhaustive enums, try_from_bits still validates range
assert!(Status::try_from_bits(3).is_none());
```

### Validation for B* and SB* Types

```rust
// B types validate that value fits in bit width
assert!(B4::try_new(15).is_some());  // Max for 4 bits
assert!(B4::try_new(16).is_none());  // Too large

// SB types validate signed range
assert!(SB4::try_new(7).is_some());   // Max for 4-bit signed
assert!(SB4::try_new(-8).is_some());  // Min for 4-bit signed
assert!(SB4::try_new(8).is_none());   // Too large
assert!(SB4::try_new(-9).is_none());  // Too small
```

### Panicking Constructors

The `new` and `from_bits` methods panic on invalid input:

```rust
// These will panic:
// let _ = B3::new(8);           // Value doesn't fit
// let _ = Status::from_bits(5); // Invalid variant
```

## Fields Struct

When `fields_struct` is enabled, a companion struct is generated for convenient construction:

```rust
#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: B2,
    flags: B3,
    length: B3,
}

// Generated: PacketFields struct
let fields = PacketFields {
    version: B2::new(1),
    flags: B3::new(5),
    length: B3::new(7),
};

let packet = Packet::from_fields(fields);

// Convert back to fields
let extracted: PacketFields = packet.to_fields();
assert_eq!(fields, extracted);

// From/Into implementations
let packet2: Packet = fields.into();
let fields2: PacketFields = packet2.into();
```

### Nested Fields

For nested bitfields, the fields struct uses the direct bitfield type (not its `*Fields` type):

```rust
#[bitpiece(4, all)]
struct Inner {
    x: B2,
    y: B2,
}

#[bitpiece(8, all)]
struct Outer {
    a: Inner,
    b: B4,
}

// OuterFields uses Inner directly for field 'a'
let fields = OuterFields {
    a: Inner::from_bits(0b1001),  // or use InnerFields and convert
    b: B4::new(15),
};

let outer = Outer::from_fields(fields);

// You can also construct the inner type from its fields and convert:
let fields2 = OuterFields {
    a: InnerFields {
        x: B2::new(1),
        y: B2::new(2),
    }.into(),  // Convert InnerFields to Inner
    b: B4::new(15),
};
```

## Storage Types

The crate automatically selects the smallest storage type that fits the bit length:

| Bit Length | Storage Type |
|------------|--------------|
| 1-8        | `u8`         |
| 9-16       | `u16`        |
| 17-32      | `u32`        |
| 33-64      | `u64`        |

Access the storage directly:

```rust
#[bitpiece(12, all)]
struct Example {
    a: B6,
    b: B6,
}

let val = Example::from_bits(0xABC);

// Direct storage access
assert_eq!(val.storage, 0xABC);

// Storage type is u16 for 12 bits
let storage: u16 = val.storage;
```

## License

MIT License - see [LICENSE](LICENSE) for details.
