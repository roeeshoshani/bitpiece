#![no_std]

pub use bitpiece_macros::bitpiece;
use core::{marker::PhantomData, num::TryFromIntError};
use paste::paste;

/// an empty struct used to represent a specific bit length.
/// this is then combined with some traits ([`ExactAssociatedStorage`], [`AssociatedStorage`]) to perform operations on the
/// specified bit length.
pub struct BitLength<const BITS: usize>;

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
                impl ExactAssociatedStorage for BitLength<$bit_length> {
                    type Storage = [<u $bit_length>];
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
            impl AssociatedStorage for BitLength<$bit_length> {
                type Storage = <BitLength< { exact_associated_storage_bit_length($bit_length) } > as ExactAssociatedStorage>::Storage;
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

    /// returns the underlying bits of this type.
    fn to_bits(self) -> Self::Bits;
}
macro_rules! impl_bitpiece_for_small_int_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste! {
                impl BitPiece for [<u $bit_len>] {
                    const BITS: usize = $bit_len;
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
                define_b_type! { $bit_len, [<B $bit_len>], <BitLength<$bit_len> as AssociatedStorage>::Storage }
            }
        )+
    };
}
define_b_types! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
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
impl_bit_storage_for_small_int_types! { u8, u16, u32 }

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
        extract_bits(
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
pub const fn extract_bits(value: u64, offset: usize, len: usize) -> u64 {
    let mask = extract_bits_mask(len);
    (value >> offset) & mask
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
