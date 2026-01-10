#![no_std]

mod check;
mod impls;
mod mut_ref;
mod storage;
mod utils;
pub use impls::*;
pub use mut_ref::*;
pub use storage::*;
pub use utils::*;

pub use bitpiece_macros::bitpiece;
pub use const_for::const_for;
pub use paste::paste;

pub trait BitPiece: Copy {
    /// the length in bits of this type.
    const BITS: usize;

    /// a value with all zero bits.
    const ZEROES: Self;

    /// a value with all one bits.
    const ONES: Self;

    /// the minimum value.
    const MIN: Self;

    /// the maximum value.
    const MAX: Self;

    /// the storage type used internally to store the bits of this bitpiece.
    type Bits: BitStorage;

    /// a converter types which implements all const conversion functions (e.g `from_bits`).
    ///
    /// traits do not support const functions, so const function must be implemented directly on some type, without using a trait.
    /// but, we can't implement methods directly on foreign types such as `u32` or `bool`, so we must implement them on another type.
    /// this type is the converter type.
    ///
    /// for non-foreign types, this will point to the trait's `Self` type.
    /// for foreign types, this will point to a type-specific converter type.
    type Converter;

    fn try_from_bits(bits: Self::Bits) -> Option<Self>;
    fn from_bits(bits: Self::Bits) -> Self;
    fn to_bits(self) -> Self::Bits;
}

pub trait BitPieceHasMutRef: BitPiece {
    /// the type used to represent a mutable reference to this type inside another bitpiece.
    type MutRef<'s>: BitPieceMutRef<'s>;
}

pub trait BitPieceHasFields: BitPiece {
    /// the type which represents the expanded view of this bitpiece.
    type Fields;
    fn from_fields(fields: Self::Fields) -> Self;
    fn to_fields(self) -> Self::Fields;
}
