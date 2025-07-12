//! **A Rust crate for effortlessly defining and manipulating bitfields with procedural macros.**
//!
//! `bitpiece` takes the complexity out of bit-level data manipulation. It provides a powerful `#[bitpiece]` macro that lets you define structs and enums as compact, typed bitfields, while automatically generating a safe, high-level API to interact with them. It's perfect for working with network protocols, hardware interfaces, or any scenario where data compactness is key.
//!
//! # Features
//!
//!   - **Declarative & Simple**: Define complex bitfield layouts using simple Rust structs and enums.
//!   - **Type-Safe API**: The macro generates getters and setters for each field, so you work with `bool`, `u8`, `enum` types, etc., not raw bit shifts and masks.
//!   - **Automatic Bit-Length Calculation**: The macro automatically calculates the total number of bits required for your type.
//!   - **Nestable**: Compose complex bitfields by nesting `bitpiece` types within each other.
//!   - **Arbitrary-Width Integers**: Use the built-in `B1`-`B64` types (e.g., `B3`, `B7`, `B12`) for fields with non-standard bit lengths.
//!   - **Compile-Time Validation**: Optionally specify an expected bit length on your structs (e.g., `#[bitpiece(32)]`) to get a compile-time error if it doesn't match the sum of its fields.
//!   - **Flexible Enums**: Supports both exhaustive and non-exhaustive enums. You can also specify a larger bit-width for an enum than its variants require.
//!   - **Safe & Unsafe APIs**: Provides both panicking (`from_bits`) and fallible (`try_from_bits`) APIs for creating bitpieces from raw integer values.
//!   - `#![no_std]` compatible.
//!
//! # Getting Started
//!
//! First, add `bitpiece` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bitpiece = "0.1.0" # Use the latest version
//! ```
//!
//! Now, let's define a bitfield for a hypothetical network packet header.
//!
//! ```rust
//! use bitpiece::*;
//!
//! // Define a 2-bit enum for the packet's priority.
//! // The macro automatically infers it needs 2 bits.
//! #[bitpiece]
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum Priority {
//!     Low = 0,
//!     Medium = 1,
//!     High = 2,
//!     Critical = 3,
//! }
//!
//! // Define the packet header structure.
//! // The macro calculates the total size (1 + 2 + 5 = 8 bits).
//! #[bitpiece(8)] // The `(8)` is optional but validates the size at compile time.
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! struct PacketHeader {
//!     is_fragment: bool,
//!     priority: Priority,
//!     payload_len: B5, // A 5-bit integer type
//! }
//!
//! fn main() {
//!     // Create a new header from raw bits (e.g., received from a network).
//!     // Bits: 0b101_10_1 => is_fragment=1, priority=2 (High), payload_len=5
//!     let mut header = PacketHeader::from_bits(0b101101);
//!
//!     // Use the generated getter methods to safely access fields.
//!     assert_eq!(header.is_fragment(), true);
//!     assert_eq!(header.priority(), Priority::High);
//!     assert_eq!(header.payload_len().get(), 5); // Use .get() for B-types
//!
//!     // Use the generated setter methods to modify the header.
//!     header.set_priority(Priority::Critical);
//!     header.set_payload_len(B5::new(31).unwrap()); // Set to max value (2^5 - 1)
//!
//!     assert_eq!(header.priority(), Priority::Critical);
//!     assert_eq!(header.payload_len().get(), 31);
//!
//!     // The underlying storage is automatically updated.
//!     // Bits: 0b11111_11_1
//!     assert_eq!(header.to_bits(), 0b11111111);
//!
//!     // You can also construct a bitpiece from its fields directly.
//!     let from_fields = PacketHeader::from_fields(PacketHeaderFields {
//!         is_fragment: false,
//!         priority: Priority::Low,
//!         payload_len: B5::new(10).unwrap(),
//!     });
//!
//!     assert_eq!(from_fields.to_bits(), 0b1010000);
//! }
//! ```
//!
//! # More Examples
//!
//! ## Nesting
//!
//! You can easily build complex structures by nesting `bitpiece` types.
//!
//! ```rust
//! use bitpiece::*;
//!
//! #[bitpiece(4)]
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! struct MacAddressPart {
//!     a: B1,
//!     b: B3,
//! }
//!
//! #[bitpiece(16)]
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! struct ProtocolInfo {
//!     part1: MacAddressPart,
//!     part2: MacAddressPart,
//!     flags: u8, // Standard integer types are also supported
//! }
//!
//! fn main() {
//!     let mut info = ProtocolInfo::zeroes(); // zeroes() is a handy constructor
//!
//!     info.set_part1(MacAddressPart::from_bits(0b1010));
//!
//!     assert_eq!(info.part1().b().get(), 0b101);
//!     assert_eq!(info.to_bits(), 0b00000000_1010);
//!
//!     // Set a field in a nested bitpiece
//!     info.part1_mut().set_b(B3::new(0b110).unwrap());
//!
//!     assert_eq!(info.part1().b().get(), 0b110);
//!     assert_eq!(info.to_bits(), 0b00000000_1100);
//! }
//! ```
//!
//! ## Non-Exhaustive Enums
//!
//! By default, an enum's bit-length is determined by its largest variant. If you try to create an enum from an invalid integer value, it will panic.
//!
//! Sometimes, however, an enum definition isn't complete, but you still want to handle known variants. For this, `bitpiece` generates a `try_from_bits` method.
//!
//! ```rust
//! use bitpiece::*;
//!
//! #[bitpiece] // Bit length is inferred as 7 bits (from 120)
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum OpCode {
//!     Read = 0,
//!     Write = 1,
//!     Sync = 80,
//!     Halt = 120,
//! }
//!
//! fn main() {
//!     // try_from_bits returns an Option, which is great for safe parsing.
//!     let known_code = OpCode::try_from_bits(80);
//!     assert_eq!(known_code, Some(OpCode::Sync));
//!
//!     let unknown_code = OpCode::try_from_bits(55);
//!     assert_eq!(unknown_code, None);
//!
//!     // In contrast, from_bits will panic on an unknown variant.
//!     // let panicked = OpCode::from_bits(55); // This would panic!
//! }
//! ```
//!
//! ## Explicit Bit-Length on Enums
//!
//! You can give an enum a larger bit-width than it needs. This is useful when a protocol reserves a certain number of bits for an enum, even if not all values are currently used.
//!
//! ```rust
//! use bitpiece::*;
//!
//! // This enum's highest value is 2, which only needs 2 bits.
//! // But we can force it to occupy a full byte.
//! #[bitpiece(8)]
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum MessageType {
//!     Query = 0,
//!     Ack = 1,
//!     Nak = 2,
//! }
//!
//! fn main() {
//!     // The underlying storage type will be u8.
//!     assert_eq!(MessageType::from_bits(1).to_bits(), 1u8);
//!
//!     assert_eq!(MessageType::try_from_bits(200), None); // Fails, not a valid variant
//! }
//! ```
//!
//! # Generated API
//!
//! For a struct like `MyPiece { field_a: bool, field_b: B3 }`, the macro generates:
//!
//!   - `MyPiece::from_bits(u8) -> Self`: Creates an instance from raw bits. Panics if any field gets an invalid value (e.g., for a non-exhaustive enum).
//!   - `MyPiece::try_from_bits(u8) -> Option<Self>`: Safely creates an instance, returning `None` if any field would be invalid.
//!   - `my_piece.to_bits() -> u8`: Returns the raw bits as the smallest possible integer storage type.
//!   - `MyPiece::from_fields(MyPieceFields) -> Self`: Creates an instance from a struct containing all the fields.
//!   - `my_piece.to_fields() -> MyPieceFields`: Deconstructs the instance into a struct of its fields.
//!   - `MyPiece::zeroes() -> Self`: A constructor where all bits are 0.
//!   - `my_piece.field_a() -> bool`: Getter for `field_a`.
//!   - `my_piece.set_field_a(bool)`: Setter for `field_a`.
//!   - `my_piece.field_b() -> B3`: Getter for `field_b`.
//!   - `my_piece.set_field_b(B3)`: Setter for `field_b`.
//!   - `my_piece.field_a_mut() -> BitPieceMut`: Advanced usage for mutable access, especially for nested pieces.
//!   - `my_piece.field_b_mut() -> BitPieceMut`: Same as above, but for field_b

#![no_std]

pub use bitpiece_macros::bitpiece;
use core::{marker::PhantomData, num::TryFromIntError};
use paste::paste;

/// an empty struct used to represent a specific bit length.
/// this is then combined with some traits ([`ExactAssociatedStorage`], [`AssociatedStorage`]) to perform operations on the
/// specified bit length.
pub struct BitLength<const BITS: usize, const SIGN: bool>;

/// a trait implemented for [`BitLength`] types that have an exact associated storage type, for example [`u8`] or [`u16`].
pub trait ExactAssociatedStorage {
    /// the exact storage type, for example [`u8`] or [`u16`].
    type Storage: BitStorage;
}

/// a trait implemented for all [`BitLength`] types that are small enough and provides the minimal storage type required for
/// storing that amount of bits. for example for bit lengths `0..8` this will be [`u8`].
pub trait AssociatedStorage {
    /// the storage type required for storing that amount of bits. for example for bit lengths `0..8` this will be [`u8`].
    type Storage: BitStorage;
}

macro_rules! impl_exact_associated_storage {
    { $($bit_length: literal),+ $(,)? } => {
        $(
            paste! {
                impl ExactAssociatedStorage for BitLength<$bit_length,false> {
                    type Storage = [<u $bit_length>];
                }
                impl ExactAssociatedStorage for BitLength<$bit_length,true> {
                    type Storage = [<i $bit_length>];
                }
            }
        )+
    }
}
impl_exact_associated_storage! { 8, 16, 32, 64 }

/// calculate the bit length of the smallest type required to store that amount of bits. for example for bits lengths `0..8` this
/// will return `8`.
const fn exact_associated_storage_bit_length(bit_length: usize) -> usize {
    let power_of_2 = bit_length.next_power_of_two();
    if power_of_2 < 8 {
        8
    } else {
        power_of_2
    }
}
macro_rules! impl_associated_storage {
    { $($bit_length: literal),+ $(,)? } => {
        $(
            impl AssociatedStorage for BitLength<$bit_length,false> {
                type Storage = <BitLength< { exact_associated_storage_bit_length($bit_length) }, false > as ExactAssociatedStorage>::Storage;
            }
            impl AssociatedStorage for BitLength<$bit_length,true> {
                type Storage = <BitLength< { exact_associated_storage_bit_length($bit_length) }, true > as ExactAssociatedStorage>::Storage;
            }
        )+
    };
}
impl_associated_storage! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
}

/// a mutable reference to a bitpiece inside another bitpiece.
pub trait BitPieceMut<'s, S: BitStorage + 's, P: BitPiece> {
    fn new(storage: &'s mut S, start_bit_index: usize) -> Self;
    fn get(&self) -> P;
    fn set(&mut self, new_value: P);
}

/// a bitpiece.
/// this is the core trait of this crate and represents a type with a specified bit length which can be used in a standalone way
/// or inside another bitpiece.
pub trait BitPiece: Clone + Copy {
    /// the length in bits of this type.
    const BITS: usize;

    /// the fields signed-ness
    const SIGNED: bool;

    /// the storage type used internally to store the bits of this bitpiece.
    type Bits: BitStorage;

    /// the type used to represent a mutable reference to this type inside another bitpiece.
    type Mut<'s, S: BitStorage + 's>: BitPieceMut<'s, S, Self>;

    /// the type which represents the expanded view of this bitpiece.
    type Fields;

    /// constructs this type with a value of zero for all fields.
    fn zeroes() -> Self {
        Self::from_bits(Self::Bits::ZEROES)
    }

    /// constructs this type with a value of one for all fields.
    fn ones() -> Self {
        Self::from_bits(Self::Bits::ONES)
    }

    /// constructs this type from the given fields.
    fn from_fields(fields: Self::Fields) -> Self;

    /// return the values of all fields of this bitpiece.
    fn to_fields(self) -> Self::Fields;

    /// constructs this type from the given bits.
    fn from_bits(bits: Self::Bits) -> Self;

    /// tries to construct this type from the given bits, if the given bits represent a valid value of this type.
    fn try_from_bits(bits: Self::Bits) -> Option<Self> {
        Some(Self::from_bits(bits))
    }

    /// returns the underlying bits of this type.
    fn to_bits(self) -> Self::Bits;
}
macro_rules! impl_bitpiece_for_small_int_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste! {
                impl BitPiece for [<u $bit_len>] {
                    const BITS: usize = $bit_len;
                    const SIGNED: bool = false;
                    type Bits = Self;
                    type Fields = Self;
                    type Mut<'s, S: BitStorage + 's> = GenericBitPieceMut<'s, S, Self>;
                    fn from_fields(fields: Self::Fields) -> Self {
                        fields
                    }
                    fn to_fields(self) -> Self::Fields {
                        self
                    }
                    fn from_bits(bits: Self::Bits) -> Self {
                        bits
                    }
                    fn to_bits(self) -> Self::Bits {
                        self
                    }
                }
                impl BitPiece for [<i $bit_len>] {
                    const BITS: usize = $bit_len;
                    const SIGNED: bool = true;
                    type Bits = Self;
                    type Fields = Self;
                    type Mut<'s, S: BitStorage + 's> = GenericBitPieceMut<'s, S, Self>;
                    fn from_fields(fields: Self::Fields) -> Self {
                        fields
                    }
                    fn to_fields(self) -> Self::Fields {
                        self
                    }
                    fn from_bits(bits: Self::Bits) -> Self {
                        bits
                    }
                    fn to_bits(self) -> Self::Bits {
                        self
                    }
                }
            }
        )+
    };
}
impl_bitpiece_for_small_int_types! { 8, 16, 32 ,64 }

/// a generic implementation of the [`BitPieceMut`] trait used for convenience.
pub struct GenericBitPieceMut<'s, S: BitStorage + 's, P: BitPiece> {
    bits: BitsMut<'s, S>,
    phantom: PhantomData<P>,
}

impl<'s, S: BitStorage + 's, P: BitPiece> BitPieceMut<'s, S, P> for GenericBitPieceMut<'s, S, P> {
    fn new(storage: &'s mut S, start_bit_index: usize) -> Self {
        Self {
            bits: BitsMut::new(storage, start_bit_index),
            phantom: PhantomData,
        }
    }

    fn get(&self) -> P {
        let bits = self.bits.get_bits(0, P::BITS);
        let correct_type_bits = P::Bits::from_u64(bits).unwrap();
        P::from_bits(correct_type_bits)
    }

    fn set(&mut self, new_value: P) {
        self.bits.set_bits(0, P::BITS, new_value.to_bits().to_u64())
    }
}

impl BitPiece for bool {
    const BITS: usize = 1;

    const SIGNED: bool = false;

    type Bits = u8;

    type Fields = bool;

    type Mut<'s, S: BitStorage + 's> = GenericBitPieceMut<'s, S, Self>;

    fn zeroes() -> Self {
        return false;
    }

    fn from_fields(fields: Self::Fields) -> Self {
        fields
    }

    fn to_fields(self) -> Self::Fields {
        self
    }

    fn from_bits(bits: Self::Bits) -> Self {
        bits != 0
    }

    fn to_bits(self) -> Self::Bits {
        if self {
            1
        } else {
            0
        }
    }
}

macro_rules! define_b_type {
    { $bit_len: literal, $ident: ident, $storage: ty } => {
        /// a type used to represent a field with a specific amount of bits.
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ident($storage);
        impl BitPiece for $ident {
            const BITS: usize = $bit_len;
            const SIGNED: bool = false;

            type Bits = $storage;

            type Fields = Self;

            type Mut<'s, S: BitStorage + 's> = GenericBitPieceMut<'s, S, Self>;

            fn from_fields(fields: Self::Fields) -> Self {
                fields
            }

            fn to_fields(self) -> Self::Fields {
                self
            }

            fn from_bits(bits: Self::Bits) -> Self {
                Self::new(bits).unwrap()
            }

            fn try_from_bits(bits: Self::Bits) -> Option<Self> {
                Self::new(bits)
            }

            fn to_bits(self) -> Self::Bits {
                self.0
            }
        }
        impl $ident {
            /// the max allowed value for this type.
            pub const MAX: Self = Self(
                if $bit_len == <$storage>::BITS {
                    // if the bit length is equal to the amount of bits in our storage type, avoid the overflow
                    // which will happen when shifting, and just returns the maximum value of the underlying
                    // storage type.
                    <$storage>::MAX
                } else {
                    (1 as $storage).wrapping_shl($bit_len).wrapping_sub(1)
                }
            );

            /// the bit length of this type.
            pub const BIT_LENGTH: usize = $bit_len;

            /// creates a new instance of this bitfield type with the given value.
            ///
            /// if the value does not fit within the bit length of this type, returns `None`.
            pub fn new(value: $storage) -> Option<Self> {
                if value <= Self::MAX.0 {
                    Some(Self(value))
                } else {
                    None
                }
            }

            /// creates a new instance of this bitfield type with the given value, without checking that the value
            /// fits within the bit length of this type.
            ///
            /// # safety
            /// the provided value must fit withing the bit length of this type.
            pub unsafe fn new_unchecked(value: $storage) -> Self {
                Self(value)
            }

            /// returns the inner value.
            pub fn get(&self) -> $storage {
                self.0
            }
        }
        impl core::fmt::Display for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
        impl core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}

macro_rules! define_b_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste!{
                define_b_type! { $bit_len, [<B $bit_len>], <BitLength<$bit_len, false> as AssociatedStorage>::Storage }
            }
        )+
    };
}
define_b_types! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
}


macro_rules! define_sb_type {
    { $bit_len: literal, $ident: ident, $storage: ty } => {
        /// a type used to represent a field with a specific amount of bits.
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ident($storage);
        impl BitPiece for $ident {
            const BITS: usize = $bit_len;

            const SIGNED: bool = true;

            type Bits = $storage;

            type Fields = Self;

            type Mut<'s, S: BitStorage + 's> = GenericBitPieceMut<'s, S, Self>;

            fn from_fields(fields: Self::Fields) -> Self {
                fields
            }

            fn to_fields(self) -> Self::Fields {
                self
            }

            fn from_bits(bits: Self::Bits) -> Self {
                Self::try_from_bits(bits).unwrap()
            }

            fn try_from_bits(bits: Self::Bits) -> Option<Self> {
                // When trying from bits allow using unsigned value
                if bits >= (1 as $storage).wrapping_shl($bit_len-1) && $bit_len!=<$storage>::BITS {
                    Self::new(bits.wrapping_sub((1 as $storage).wrapping_shl($bit_len)))
                } else {
                    Self::new(bits)
                }
            }

            fn to_bits(self) -> Self::Bits {
                self.0
            }
        }
        impl $ident {
            /// the max allowed value for this type.
            pub const MAX: Self = Self(
                if $bit_len == <$storage>::BITS {
                    // if the bit length is equal to the amount of bits in our storage type, avoid the overflow
                    // which will happen when shifting, and just returns the maximum value of the underlying
                    // storage type.
                    <$storage>::MAX
                } else {
                    (1 as $storage).wrapping_shl($bit_len-1).wrapping_sub(1)
                }
            );

            /// the max allowed value for this type.
            pub const MIN: Self = Self(
                (-1 as $storage).wrapping_shl($bit_len-1)
            );

            /// the bit length of this type.
            pub const BIT_LENGTH: usize = $bit_len;

            /// creates a new instance of this bitfield type with the given value.
            ///
            /// if the value does not fit within the bit length of this type, returns `None`.
            pub fn new(value: $storage) -> Option<Self> {
                if value >= Self::MIN.0 && value <= Self::MAX.0 {
                    Some(Self(value))
                } else {
                    None
                }
            }

            /// creates a new instance of this bitfield type with the given value, without checking that the value
            /// fits within the bit length of this type.
            ///
            /// # safety
            /// the provided value must fit withing the bit length of this type.
            pub unsafe fn new_unchecked(value: $storage) -> Self {
                Self(value)
            }

            /// returns the inner value.
            pub fn get(&self) -> $storage {
                self.0
            }
        }
        impl core::fmt::Display for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
        impl core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}


macro_rules! define_sb_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste!{
                define_sb_type! { $bit_len, [<SB $bit_len>], <BitLength<$bit_len, true> as AssociatedStorage>::Storage }
            }
        )+
    };
}
define_sb_types! {
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
}

/// a type which can be used as the internal storage of a bitpiece.
pub trait BitStorage: BitPiece {
    const ZEROES: Self;
    const ONES: Self;
    fn to_u64(self) -> u64;
    fn from_u64(value: u64) -> Result<Self, TryFromIntError>;
}

impl BitStorage for u64 {
    const ZEROES: Self = 0;
    const ONES: Self = u64::MAX;

    fn to_u64(self) -> u64 {
        self
    }

    fn from_u64(value: u64) -> Result<Self, TryFromIntError> {
        Ok(value)
    }
}

macro_rules! impl_bit_storage_for_small_int_types {
    { $($ty: ty),+ } => {
        $(
            impl BitStorage for $ty {
                const ZEROES: Self = 0;
                const ONES: Self = Self::MAX;
                fn to_u64(self) -> u64 {
                    self as u64
                }
                fn from_u64(value: u64) -> Result<Self, TryFromIntError> {
                    value.try_into()
                }
            }
        )+
    };
}
impl_bit_storage_for_small_int_types! { u8, u16, u32, i8, i16, i32, i64 }

/// a convenience type for interacting with the bits of an underlying storage type, starting at a specific bit index.
/// this is useful for implementing mutable references.
pub struct BitsMut<'s, S: BitStorage> {
    pub storage: &'s mut S,
    pub start_bit_index: usize,
}
impl<'s, S: BitStorage> BitsMut<'s, S> {
    #[inline(always)]
    pub fn new(storage: &'s mut S, start_bit_index: usize) -> Self {
        Self {
            storage,
            start_bit_index,
        }
    }

    /// returns `len` bits starting at relative bit index `rel_bit_index`.
    #[inline(always)]
    pub fn get_bits(&self, rel_bit_index: usize, len: usize) -> u64 {
        extract_bits::<false>(
            self.storage.to_u64(),
            self.start_bit_index + rel_bit_index,
            len,
        )
    }

    /// modifies the `len` bits starting at relative bit index `rel_bit_index` to the given `new_value`.
    #[inline(always)]
    pub fn set_bits(&mut self, rel_bit_index: usize, len: usize, new_value: u64) {
        *self.storage = S::from_u64(modify_bits(
            self.storage.to_u64(),
            self.start_bit_index + rel_bit_index,
            len,
            new_value,
        ))
        .unwrap();
    }
}

#[inline(always)]
const fn extract_bits_mask(len: usize) -> u64 {
    (1u64 << len).wrapping_sub(1)
}

#[inline(always)]
const fn extract_bits_shifted_mask(offset: usize, len: usize) -> u64 {
    extract_bits_mask(len) << offset
}

/// extracts some bits from a value
#[inline(always)]
pub const fn extract_bits<const SIGNED: bool>(value: u64, offset: usize, len: usize) -> u64 {
    let mask = extract_bits_mask(len);
    let raw_value = (value >> offset) & mask;
    if SIGNED && len!=64 && raw_value >= (1<<(len-1)) {
        raw_value.wrapping_sub(1<<len)
    } else {
        raw_value
    }
}

/// extracts some bits (mask only, no shift) from a value
#[inline(always)]
pub const fn extract_bits_noshift(value: u64, offset: usize, len: usize) -> u64 {
    let mask = extract_bits_mask(len);
    let shifted_mask = mask << offset;
    value & shifted_mask
}
/// returns a new value with the specified bit range modified to the new value
#[inline(always)]
pub const fn modify_bits(value: u64, offset: usize, len: usize, new_value: u64) -> u64 {
    let shifted_mask = extract_bits_shifted_mask(offset, len);

    let without_original_bits = value & (!shifted_mask);
    let shifted_new_value = new_value << offset;
    without_original_bits | shifted_new_value
}
