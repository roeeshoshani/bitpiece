use core::num::TryFromIntError;

use crate::*;

/// a type which can be used as the internal storage of a bitpiece.
pub trait BitStorage: BitPiece {
    const ZEROES: Self;
    const ONES: Self;

    /// the signed version of this storage integer type.
    type Signed;

    fn to_u64(self) -> u64;
    fn from_u64(value: u64) -> Result<Self, TryFromIntError>;
}

impl BitStorage for u64 {
    const ZEROES: Self = 0;
    const ONES: Self = u64::MAX;

    type Signed = i64;

    fn to_u64(self) -> u64 {
        self
    }

    fn from_u64(value: u64) -> Result<Self, TryFromIntError> {
        Ok(value)
    }
}

macro_rules! impl_bit_storage_for_small_unsigned_int_types {
    { $($bit_len: literal),+ } => {
        $(
            paste::paste! {
                impl BitStorage for [<u $bit_len>] {
                    const ZEROES: Self = 0;
                    const ONES: Self = Self::MAX;
                    type Signed = [<i $bit_len>];
                    fn to_u64(self) -> u64 {
                        self as u64
                    }
                    fn from_u64(value: u64) -> Result<Self, TryFromIntError> {
                        value.try_into()
                    }
                }
            }
        )+
    };
}
impl_bit_storage_for_small_unsigned_int_types! { 8, 16, 32 }

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
            paste::paste! {
                impl ExactAssociatedStorage for BitLength<$bit_length> {
                    type Storage = [<u $bit_length>];
                }
            }
        )+
    }
}
impl_exact_associated_storage! { 8, 16, 32, 64 }

/// calculate the bit length of the smallest type required to store that amount of bits. for example for bits lengths `1..=8` this
/// will return `8`.
const fn exact_associated_storage_bit_length(bit_length: usize) -> usize {
    if bit_length == 0 {
        panic!("bit length can't be 0");
    }
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
