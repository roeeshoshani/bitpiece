use crate::*;

pub struct BitPieceBoolConverter;
impl BitPieceBoolConverter {
    pub const fn from_fields(fields: bool) -> bool {
        fields
    }
    pub const fn to_fields(x: bool) -> bool {
        x
    }
    pub const fn try_from_bits(bits: u8) -> Option<bool> {
        Some(bits != 0)
    }
    pub const fn from_bits(bits: u8) -> bool {
        bits != 0
    }
    pub const fn to_bits(x: bool) -> u8 {
        if x {
            1
        } else {
            0
        }
    }
    pub const fn const_eq(a: bool, b: bool) -> bool {
        a == b
    }
}

impl BitPiece for bool {
    const BITS: usize = 1;
    const ZEROES: Self = false;
    const ONES: Self = true;
    type Bits = u8;
    type Converter = BitPieceBoolConverter;
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
impl BitPieceHasMutRef for bool {
    type MutRef<'s> = BitPieceBoolMutRef<'s>;
}
impl BitPieceHasFields for bool {
    type Fields = bool;
    fn from_fields(fields: Self::Fields) -> Self {
        <Self as BitPiece>::Converter::from_fields(fields)
    }
    fn to_fields(self) -> Self::Fields {
        <Self as BitPiece>::Converter::to_fields(self)
    }
}
bitpiece_check_full_impl! { bool }
bitpiece_define_mut_ref_type! {bool, BitPieceBoolMutRef }
