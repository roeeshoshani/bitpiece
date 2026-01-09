use crate::*;

macro_rules! define_b_type {
    { $bit_len: literal, $ident: ident, $storage: ty, $mut_ref_ty_name: ident } => {
        /// a type used to represent a field with a specific amount of bits.
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ident($storage);
        impl BitPiece for $ident {
            const BITS: usize = $bit_len;
            const ZEROES: Self = Self(0);
            const ONES: Self = Self(
                if $bit_len == <$storage>::BITS {
                    // if the bit length is equal to the amount of bits in our storage type, avoid the overflow
                    // which will happen when shifting, and just returns the maximum value of the underlying
                    // storage type.
                    <$storage>::MAX
                } else {
                    ((1 as $storage) << $bit_len).wrapping_sub(1)
                }
            );
            const MIN: Self = Self::ZEROES;
            const MAX: Self = Self::ONES;
            type Bits = $storage;
            type Converter = Self;
            fn try_from_bits(bits: Self::Bits) -> Option<Self> {
                <Self as BitPiece>::Converter::try_from_bits(bits)
            }
            fn from_bits(bits: Self::Bits) -> Self {
                <Self as BitPiece>::Converter::from_bits(bits)
            }
            fn to_bits(self) -> Self::Bits {
                <Self as BitPiece>::Converter::to_bits(self)
            }
        }

        impl BitPieceHasMutRef for $ident {
            type MutRef<'s> = $mut_ref_ty_name<'s>;
        }
        impl BitPieceHasFields for $ident {
            type Fields = Self;
            fn from_fields(fields: Self::Fields) -> Self {
                <Self as BitPiece>::Converter::from_fields(fields)
            }
            fn to_fields(self) -> Self::Fields {
                <Self as BitPiece>::Converter::to_fields(self)
            }
        }
        impl $ident {
            pub const fn from_fields(fields: $ident) -> $ident {
                fields
            }
            pub const fn to_fields(x: $ident) -> $ident {
                x
            }
            pub const fn try_from_bits(bits: $storage) -> Option<$ident> {
                Self::try_new(bits)
            }
            pub const fn from_bits(bits: $storage) -> $ident {
                Self::new(bits)
            }
            pub const fn to_bits(x: $ident) -> $storage {
                x.0
            }
            pub const fn const_eq(a: Self, b: Self) -> bool {
                a.0 == b.0
            }
        }
        impl $ident {
            /// the max allowed value for this type.
            pub const MAX: Self = Self::ONES;

            /// creates a new instance of this bitfield type with the given value.
            ///
            /// this function panics if the value does not fit within the bit length of this type.
            pub const fn new(value: $storage) -> Self {
                Self::try_new(value).unwrap()
            }

            /// creates a new instance of this bitfield type with the given value.
            ///
            /// if the value does not fit within the bit length of this type, returns `None`.
            pub const fn try_new(value: $storage) -> Option<Self> {
                if value <= Self::ONES.0 {
                    Some(Self(value))
                } else {
                    None
                }
            }

            /// creates a new instance of this bitfield type with the given value, without checking that the value
            /// fits within the bit length of this type.
            ///
            /// # safety
            /// the provided value must fit within the bit length of this type.
            pub const unsafe fn new_unchecked(value: $storage) -> Self {
                Self(value)
            }

            /// returns the inner value.
            pub const fn get(&self) -> $storage {
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
        bitpiece_check_full_impl! { $ident }
        bitpiece_define_mut_ref_type! { $ident, $mut_ref_ty_name, pub }
    };
}
macro_rules! define_b_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste!{
                define_b_type! {
                    $bit_len,
                    [<B $bit_len>],
                    <BitLength<$bit_len> as AssociatedStorage>::Storage,
                    [<B $bit_len MutRef>]
                }
            }
        )+
    };
}
define_b_types! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
}
