pub use bitpiece_macros::bitpiece;
use core::num::TryFromIntError;
use paste::paste;

pub struct BitLength<const BITS: usize>;
pub trait ExactAssociatedStorage {
    type Storage: BitStorage;
}
pub trait AssociatedStorage {
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

pub trait BitPiece: Clone + Copy {
    const BITS: usize;

    type Bits: BitStorage;

    fn from_bits(bits: Self::Bits) -> Self;

    fn to_bits(self) -> Self::Bits;
}
macro_rules! impl_bitpiece_for_small_int_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste! {
                impl BitPiece for [<u $bit_len>] {
                    const BITS: usize = $bit_len;
                    type Bits = Self;
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

macro_rules! define_b_type {
    { $bit_len: literal, $ident: ident } => {
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
    };
}
macro_rules! define_b_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste!{
                define_b_type! { $bit_len, [<B $bit_len>] }
            }
        )+
    };
}
define_b_types! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
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
