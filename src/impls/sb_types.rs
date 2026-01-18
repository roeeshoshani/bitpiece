use crate::*;

macro_rules! sb_type_impl_from_int_type {
    { $sb_type_ident: ident, $sb_type_storage_signed: ty, $uint_type: ty } => {
        impl From<$uint_type> for $sb_type_ident {
            fn from(x: $uint_type) -> Self {
                Self::new(x as $sb_type_storage_signed)
            }
        }
    }
}

macro_rules! sb_type_impl_from_int_types {
    { $sb_type_ident: ident, $sb_type_storage_signed: ty } => {
        sb_type_impl_from_int_type! { $sb_type_ident, $sb_type_storage_signed, u8 }
        sb_type_impl_from_int_type! { $sb_type_ident, $sb_type_storage_signed, u16 }
        sb_type_impl_from_int_type! { $sb_type_ident, $sb_type_storage_signed, u32 }
        sb_type_impl_from_int_type! { $sb_type_ident, $sb_type_storage_signed, u64 }
        sb_type_impl_from_int_type! { $sb_type_ident, $sb_type_storage_signed, i8 }
        sb_type_impl_from_int_type! { $sb_type_ident, $sb_type_storage_signed, i16 }
        sb_type_impl_from_int_type! { $sb_type_ident, $sb_type_storage_signed, i32 }
        sb_type_impl_from_int_type! { $sb_type_ident, $sb_type_storage_signed, i64 }
    }
}

macro_rules! define_sb_type {
    { $bit_len: literal, $ident: ident, $storage: ty, $storage_signed: ty, $mut_ref_ty_name: ident } => {
        /// a type used to represent a field with a specific amount of bits.
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ident($storage_signed);
        impl BitPiece for $ident {
            const BITS: usize = $bit_len;
            const ZEROES: Self = Self(0);
            const ONES: Self = Self::from_bits(Self::STORAGE_MASK);
            const MIN: Self = Self(((1 as $storage) << ($bit_len - 1)).wrapping_neg() as $storage_signed);
            const MAX: Self = Self(((1 as $storage) << ($bit_len - 1)).wrapping_sub(1) as $storage_signed);
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

        sb_type_impl_from_int_types! { $ident, $storage_signed }

        impl BitPieceHasFields for $ident {
            type Fields = Self;
            fn from_fields(fields: Self::Fields) -> Self {
                <Self as BitPiece>::Converter::from_fields(fields)
            }
            fn to_fields(self) -> Self::Fields {
                <Self as BitPiece>::Converter::to_fields(self)
            }
        }

        impl BitPieceHasMutRef for $ident {
            type MutRef<'s> = $mut_ref_ty_name<'s>;
        }

        impl $ident {
            pub const fn from_fields(fields: Self) -> Self {
                fields
            }
            pub const fn to_fields(x: Self) -> Self {
                x
            }
            pub const fn try_from_bits(bits: $storage) -> Option<Self> {
                // extract the sign bit according to the bit length of this type.
                let sign_bit = (bits >> ($bit_len - 1)) & 1;

                // sign extend if needed
                let sign_extended = if sign_bit != 0 {
                    // set all bits above the bit length to 1, which will sign extend it
                    bits | (!Self::STORAGE_MASK)
                } else {
                    bits
                };
                Self::try_new(sign_extended as $storage_signed)
            }
            pub const fn from_bits(bits: $storage) -> Self {
                Self::try_from_bits(bits).unwrap()
            }
            pub const fn to_bits(self) -> $storage {
                (self.0 as $storage) & Self::STORAGE_MASK
            }
            pub const fn const_eq(a: Self, b: Self) -> bool {
                a.0 == b.0
            }
        }
        impl $ident {
            /// a mask of the bit length of this type.
            const STORAGE_MASK: $storage = (
                if $bit_len == <$storage>::BITS {
                    // if the bit length is equal to the amount of bits in our storage type, avoid the overflow
                    // which will happen when shifting, and just returns the maximum value of the underlying
                    // storage type.
                    <$storage>::MAX
                } else {
                    ((1 as $storage) << $bit_len).wrapping_sub(1)
                }
            );

            /// creates a new instance of this bitfield type with the given value.
            ///
            /// this function panics if the value does not fit within the bit length of this type.
            pub const fn new(value: $storage_signed) -> Self {
                Self::try_new(value).unwrap()
            }

            /// creates a new instance of this bitfield type with the given value.
            ///
            /// if the value does not fit within the bit length of this type, returns `None`.
            pub const fn try_new(value: $storage_signed) -> Option<Self> {
                if value <= Self::MAX.0 && value >= Self::MIN.0 {
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
            pub const unsafe fn new_unchecked(value: $storage_signed) -> Self {
                Self(value)
            }

            /// returns the inner value.
            pub const fn get(&self) -> $storage_signed {
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
        bitpiece_check_full_impl! { $ident, true }
        bitpiece_define_mut_ref_type! { $ident, $mut_ref_ty_name, pub }
    };
}
macro_rules! define_sb_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste!{
                define_sb_type! {
                    $bit_len,
                    [<SB $bit_len>],
                    <BitLength<$bit_len> as AssociatedStorage>::Storage,
                    <<BitLength<$bit_len> as AssociatedStorage>::Storage as BitStorage>::Signed,
                    [<SB $bit_len MutRef>]
                }
            }
        )+
    };
}
define_sb_types! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
}
