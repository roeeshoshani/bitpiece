use core::num::TryFromIntError;

pub trait Bitfield {
    type Bits: BitStorage;
    fn from_bits(bits: Self::Bits) -> Self;
    fn to_bits(&self) -> Self::Bits;
}

pub trait BitStorage: Clone + Copy {
    fn to_u64(self) -> u64;
    fn from_u64(value: u64) -> Result<Self, TryFromIntError>;
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
