pub use bitpiece_macros::bitpiece;
use core::num::TryFromIntError;

pub struct BitLength<const BITS: usize>;
pub trait AssociatedStorage {
    type Storage: BitStorage;
}
macro_rules! impl_associated_storage {
    { $([$($bit_length: literal),+ $(,)?] => $storage_ty: ty),+ $(,)? } => {
        $(
            $(
                impl AssociatedStorage for BitLength<$bit_length> {
                    type Storage = $storage_ty;
                }
            )+
        )+
    };
}
impl_associated_storage! {
    [1, 2, 3, 4, 5, 6, 7, 8] => u8,
    [9, 10, 11, 12, 13, 14, 15, 16] => u16,
    [17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32] => u32,
    [33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64] => u64
}

pub trait BitPiece: Clone + Copy {
    const BITS: usize;

    type Bits: BitStorage;

    fn from_bits(bits: Self::Bits) -> Self;

    fn to_bits(self) -> Self::Bits;
}
macro_rules! impl_bitpiece_for_small_int_type {
    { $ty: ty, $bit_len: literal, $storage_ty: ty } => {
        impl BitPiece for $ty {
            const BITS: usize = $bit_len;
            type Bits = $storage_ty;
            fn from_bits(bits: Self::Bits) -> Self {
                bits as $ty
            }
            fn to_bits(self) -> Self::Bits {
                self as $storage_ty
            }
        }
    };
}
macro_rules! impl_bitpiece_for_signed_unsigned_ints {
    { $unsigned: ty, $signed: ty, $bit_len: literal } => {
        impl_bitpiece_for_small_int_type! { $unsigned, $bit_len, $unsigned }
        impl_bitpiece_for_small_int_type! { $signed, $bit_len, $unsigned }
    };
}
impl_bitpiece_for_signed_unsigned_ints! { u8, i8, 8 }
impl_bitpiece_for_signed_unsigned_ints! { u16, i16, 16 }
impl_bitpiece_for_signed_unsigned_ints! { u32, i32, 32 }
impl_bitpiece_for_signed_unsigned_ints! { u64, i64, 64 }
impl BitPiece for bool {
    const BITS: usize = 1;

    type Bits = u8;

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

pub trait BitStorage: Clone + Copy {
    fn to_u64(self) -> u64;
    fn from_u64(value: u64) -> Result<Self, TryFromIntError>;
}

pub enum BitOrder {
    LsbFirst,
    MsbFirst,
}

impl BitStorage for u64 {
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
