use crate::*;

macro_rules! impl_bitpiece_for_unsigned_int_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste::paste! {
                pub struct [<BitPieceU $bit_len Converter>];
                impl [<BitPieceU $bit_len Converter>] {
                    pub const fn from_fields(fields: [<u $bit_len>]) -> [<u $bit_len>] {
                        fields
                    }
                    pub const fn to_fields(x: [<u $bit_len>]) -> [<u $bit_len>] {
                        x
                    }
                    pub const fn try_from_bits(bits: [<u $bit_len>]) -> Option<[<u $bit_len>]> {
                        Some(bits)
                    }
                    pub const fn from_bits(bits: [<u $bit_len>]) -> [<u $bit_len>] {
                        bits
                    }
                    pub const fn to_bits(x: [<u $bit_len>]) -> [<u $bit_len>] {
                        x
                    }
                    pub const fn const_eq(a: [<u $bit_len>], b: [<u $bit_len>]) -> bool {
                        a == b
                    }
                    pub const fn to_storage_mut_ref(x: &mut [<u $bit_len>]) -> BitPieceStorageMutRef<'_> {
                        BitPieceStorageMutRef::[<U $bit_len>](x)
                    }
                }
                impl BitPiece for [<u $bit_len>] {
                    const BITS: usize = $bit_len;
                    const ZEROES: Self = 0;
                    const ONES: Self = !0;
                    const MIN: Self = [<u $bit_len>]::MIN;
                    const MAX: Self = [<u $bit_len>]::MAX;
                    type Bits = Self;
                    type Converter = [<BitPieceU $bit_len Converter>];
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
                impl BitPieceHasMutRef for [<u $bit_len>] {
                    type MutRef<'s> = [<BitPieceU $bit_len MutRef>]<'s>;
                }
                impl BitPieceHasFields for [<u $bit_len>] {
                    type Fields = Self;
                    fn from_fields(fields: Self::Fields) -> Self {
                        <Self as BitPiece>::Converter::from_fields(fields)
                    }
                    fn to_fields(self) -> Self::Fields {
                        <Self as BitPiece>::Converter::to_fields(self)
                    }
                }
                bitpiece_check_full_impl! { [<u $bit_len>], true }
                bitpiece_define_mut_ref_type! { [<u $bit_len>], [<BitPieceU $bit_len MutRef>], pub }
            }
        )+
    };
}
impl_bitpiece_for_unsigned_int_types! { 8, 16, 32, 64 }

macro_rules! impl_bitpiece_for_signed_int_types {
    { $($bit_len: literal),+ $(,)? } => {
        $(
            paste::paste! {
                pub struct [<BitPieceI $bit_len Converter>];
                impl [<BitPieceI $bit_len Converter>] {
                    pub const fn from_fields(fields: [<i $bit_len>]) -> [<i $bit_len>] {
                        fields
                    }
                    pub const fn to_fields(x: [<i $bit_len>]) -> [<i $bit_len>] {
                        x
                    }
                    pub const fn try_from_bits(bits: [<u $bit_len>]) -> Option<[<i $bit_len>]> {
                        Some(bits as [<i $bit_len>])
                    }
                    pub const fn from_bits(bits: [<u $bit_len>]) -> [<i $bit_len>] {
                        bits as [<i $bit_len>]
                    }
                    pub const fn to_bits(x: [<i $bit_len>]) -> [<u $bit_len>] {
                        x as [<u $bit_len>]
                    }
                    pub const fn const_eq(a: [<i $bit_len>], b: [<i $bit_len>]) -> bool {
                        a == b
                    }
                }
                impl BitPiece for [<i $bit_len>] {
                    const BITS: usize = $bit_len;
                    const ZEROES: Self = 0;
                    const ONES: Self = !0;
                    const MIN: Self = [<i $bit_len>]::MIN;
                    const MAX: Self = [<i $bit_len>]::MAX;
                    type Bits = [<u $bit_len>];
                    type Converter = [<BitPieceI $bit_len Converter>];
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
                impl BitPieceHasMutRef for [<i $bit_len>] {
                    type MutRef<'s> = [<BitPieceI $bit_len MutRef>]<'s>;
                }
                impl BitPieceHasFields for [<i $bit_len>] {
                    type Fields = Self;
                    fn from_fields(fields: Self::Fields) -> Self {
                        <Self as BitPiece>::Converter::from_fields(fields)
                    }
                    fn to_fields(self) -> Self::Fields {
                        <Self as BitPiece>::Converter::to_fields(self)
                    }
                }
                bitpiece_check_full_impl! { [<i $bit_len>], true }
                bitpiece_define_mut_ref_type! { [<i $bit_len>], [<BitPieceI $bit_len MutRef>], pub }
            }
        )+
    };
}
impl_bitpiece_for_signed_int_types! { 8, 16, 32, 64 }
