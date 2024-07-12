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

macro_rules! define_b_types {
    {$($bit_len: literal => $ident: ident),+ $(,)?} => {
        $(
            #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub struct $ident(pub <BitLength<$bit_len> as AssociatedStorage>::Storage);
            impl BitPiece for $ident {
                const BITS: usize = $bit_len;

                type Bits = <BitLength<$bit_len> as AssociatedStorage>::Storage;

                fn from_bits(bits: Self::Bits) -> Self {
                    Self(bits)
                }

                fn to_bits(self) -> Self::Bits {
                    self.0
                }
            }
            impl core::fmt::Display for $ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    core::fmt::Display::fmt(&self.0, f)
                }
            }
            impl core::fmt::Debug for $ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    core::fmt::Debug::fmt(&self.0, f)
                }
            }
        )+
    };
}
define_b_types! {
    1 => B1,
    2 => B2,
    3 => B3,
    4 => B4,
    5 => B5,
    6 => B6,
    7 => B7,
    8 => B8,
    9 => B9,
    10 => B10,
    11 => B11,
    12 => B12,
    13 => B13,
    14 => B14,
    15 => B15,
    16 => B16,
    17 => B17,
    18 => B18,
    19 => B19,
    20 => B20,
    21 => B21,
    22 => B22,
    23 => B23,
    24 => B24,
    25 => B25,
    26 => B26,
    27 => B27,
    28 => B28,
    29 => B29,
    30 => B30,
    31 => B31,
    32 => B32,
    33 => B33,
    34 => B34,
    35 => B35,
    36 => B36,
    37 => B37,
    38 => B38,
    39 => B39,
    40 => B40,
    41 => B41,
    42 => B42,
    43 => B43,
    44 => B44,
    45 => B45,
    46 => B46,
    47 => B47,
    48 => B48,
    49 => B49,
    50 => B50,
    51 => B51,
    52 => B52,
    53 => B53,
    54 => B54,
    55 => B55,
    56 => B56,
    57 => B57,
    58 => B58,
    59 => B59,
    60 => B60,
    61 => B61,
    62 => B62,
    63 => B63,
    64 => B64,
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
