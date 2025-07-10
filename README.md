# bitpiece

**A Rust crate for effortlessly defining and manipulating bitfields with procedural macros.**

`bitpiece` takes the complexity out of bit-level data manipulation. It provides a powerful `#[bitpiece]` macro that lets you define structs and enums as compact, typed bitfields, while automatically generating a safe, high-level API to interact with them. It's perfect for working with network protocols, hardware interfaces, or any scenario where data compactness is key.

## Features

  - **Declarative & Simple**: Define complex bitfield layouts using simple Rust structs and enums.
  - **Type-Safe API**: The macro generates getters and setters for each field, so you work with `bool`, `u8`, `enum` types, etc., not raw bit shifts and masks.
  - **Automatic Bit-Length Calculation**: The macro automatically calculates the total number of bits required for your type.
  - **Nestable**: Compose complex bitfields by nesting `bitpiece` types within each other.
  - **Arbitrary-Width Integers**: Use the built-in `B1`-`B64` types (e.g., `B3`, `B7`, `B12`) for fields with non-standard bit lengths.
  - **Compile-Time Validation**: Optionally specify an expected bit length on your structs (e.g., `#[bitpiece(32)]`) to get a compile-time error if it doesn't match the sum of its fields.
  - **Flexible Enums**: Supports both exhaustive and non-exhaustive enums. You can also specify a larger bit-width for an enum than its variants require.
  - **Safe & Unsafe APIs**: Provides both panicking (`from_bits`) and fallible (`try_from_bits`) APIs for creating bitpieces from raw integer values.
  - `#![no_std]` compatible.

## Getting Started

First, add `bitpiece` to your `Cargo.toml`:

```toml
[dependencies]
bitpiece = "0.1.0" # Use the latest version
```

Now, let's define a bitfield for a hypothetical network packet header.

```rust
use bitpiece::*;

// Define a 2-bit enum for the packet's priority.
// The macro automatically infers it needs 2 bits.
#[bitpiece]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

// Define the packet header structure.
// The macro calculates the total size (1 + 2 + 5 = 8 bits).
#[bitpiece(8)] // The `(8)` is optional but validates the size at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PacketHeader {
    is_fragment: bool,
    priority: Priority,
    payload_len: B5, // A 5-bit integer type
}

fn main() {
    // Create a new header from raw bits (e.g., received from a network).
    // Bits: 0b101_10_1 => is_fragment=1, priority=2 (High), payload_len=5
    let mut header = PacketHeader::from_bits(0b101101);

    // Use the generated getter methods to safely access fields.
    assert_eq!(header.is_fragment(), true);
    assert_eq!(header.priority(), Priority::High);
    assert_eq!(header.payload_len().get(), 5); // Use .get() for B-types

    // Use the generated setter methods to modify the header.
    header.set_priority(Priority::Critical);
    header.set_payload_len(B5::new(31).unwrap()); // Set to max value (2^5 - 1)

    assert_eq!(header.priority(), Priority::Critical);
    assert_eq!(header.payload_len().get(), 31);

    // The underlying storage is automatically updated.
    // Bits: 0b11111_11_1
    assert_eq!(header.to_bits(), 0b11111111);

    // You can also construct a bitpiece from its fields directly.
    let from_fields = PacketHeader::from_fields(PacketHeaderFields {
        is_fragment: false,
        priority: Priority::Low,
        payload_len: B5::new(10).unwrap(),
    });

    assert_eq!(from_fields.to_bits(), 0b1010000);
}
```

## More Examples

### Nesting

You can easily build complex structures by nesting `bitpiece` types.

```rust
use bitpiece::*;

#[bitpiece(4)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MacAddressPart {
    a: B1,
    b: B3,
}

#[bitpiece(16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProtocolInfo {
    part1: MacAddressPart,
    part2: MacAddressPart,
    flags: u8, // Standard integer types are also supported
}

fn main() {
    let mut info = ProtocolInfo::zeroes(); // zeroes() is a handy constructor

    info.set_part1(MacAddressPart::from_bits(0b1010));

    assert_eq!(info.part1().b().get(), 0b101);
    assert_eq!(info.to_bits(), 0b00000000_1010);

    // Set a field in a nested bitpiece
    info.part1_mut().set_b(B3::new(0b110).unwrap());

    assert_eq!(info.part1().b().get(), 0b110);
    assert_eq!(info.to_bits(), 0b00000000_1100);
}
```

### Non-Exhaustive Enums

By default, an enum's bit-length is determined by its largest variant. If you try to create an enum from an invalid integer value, it will panic.

Sometimes, however, an enum definition isn't complete, but you still want to handle known variants. For this, `bitpiece` generates a `try_from_bits` method.

```rust
use bitpiece::*;

#[bitpiece] // Bit length is inferred as 7 bits (from 120)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    Read = 0,
    Write = 1,
    Sync = 80,
    Halt = 120,
}

fn main() {
    // try_from_bits returns an Option, which is great for safe parsing.
    let known_code = OpCode::try_from_bits(80);
    assert_eq!(known_code, Some(OpCode::Sync));

    let unknown_code = OpCode::try_from_bits(55);
    assert_eq!(unknown_code, None);

    // In contrast, from_bits will panic on an unknown variant.
    // let panicked = OpCode::from_bits(55); // This would panic!
}
```

### Explicit Bit-Length on Enums

You can give an enum a larger bit-width than it needs. This is useful when a protocol reserves a certain number of bits for an enum, even if not all values are currently used.

```rust
use bitpiece::*;

// This enum's highest value is 2, which only needs 2 bits.
// But we can force it to occupy a full byte.
#[bitpiece(8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MessageType {
    Query = 0,
    Ack = 1,
    Nak = 2,
}

fn main() {
    // The underlying storage type will be u8.
    assert_eq!(MessageType::from_bits(1).to_bits(), 1u8);

    assert_eq!(MessageType::try_from_bits(200), None); // Fails, not a valid variant
}
```

## Generated API

For a struct like `MyPiece { field_a: bool, field_b: B3 }`, the macro generates:

  - `MyPiece::from_bits(u8) -> Self`: Creates an instance from raw bits. Panics if any field gets an invalid value (e.g., for a non-exhaustive enum).
  - `MyPiece::try_from_bits(u8) -> Option<Self>`: Safely creates an instance, returning `None` if any field would be invalid.
  - `my_piece.to_bits() -> u8`: Returns the raw bits as the smallest possible integer storage type.
  - `MyPiece::from_fields(MyPieceFields) -> Self`: Creates an instance from a struct containing all the fields.
  - `my_piece.to_fields() -> MyPieceFields`: Deconstructs the instance into a struct of its fields.
  - `MyPiece::zeroes() -> Self`: A constructor where all bits are 0.
  - `my_piece.field_a() -> bool`: Getter for `field_a`.
  - `my_piece.set_field_a(bool)`: Setter for `field_a`.
  - `my_piece.field_b() -> B3`: Getter for `field_b`.
  - `my_piece.set_field_b(B3)`: Setter for `field_b`.
  - `my_piece.field_a_mut() -> BitPieceMut`: Advanced usage for mutable access, especially for nested pieces.
  - `my_piece.field_b_mut() -> BitPieceMut`: Same as above, but for field_b

License: MIT
