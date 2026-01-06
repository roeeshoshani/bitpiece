use crate::*;

/// a mutable reference to the storage type of some bitpiece.
///
/// when implementing the logic for dealing with a mutable reference to one bitpiece which is embedded inside of another bitpiece,
/// we need to somehow abstract over the storage type of the container bitpiece, so that the contained bitpiece can be embedded into
/// multiple different bitpiece containers with different storage types.
///
/// this would usually be performed using generics and traits, but trait functions can't be called in const contexts, which greatly
/// limits the usability of generics and traits when working in const contexts.
///
/// so, instead of using generic, we just use an enum, since we know all possible types in advance anyway.
///
/// this allows the code to be generic while also allowing it to work in const contexts.
pub enum BitPieceStorageMutRef<'a> {
    U64(&'a mut u64),
    U32(&'a mut u32),
    U16(&'a mut u16),
    U8(&'a mut u8),
}
impl<'a> BitPieceStorageMutRef<'a> {
    #[inline(always)]
    pub const fn get(&self) -> u64 {
        match self {
            BitPieceStorageMutRef::U64(x) => **x as u64,
            BitPieceStorageMutRef::U32(x) => **x as u64,
            BitPieceStorageMutRef::U16(x) => **x as u64,
            BitPieceStorageMutRef::U8(x) => **x as u64,
        }
    }

    #[inline(always)]
    pub const fn set(&mut self, new_value: u64) {
        match self {
            BitPieceStorageMutRef::U64(x) => **x = new_value as u64,
            BitPieceStorageMutRef::U32(x) => **x = new_value as u32,
            BitPieceStorageMutRef::U16(x) => **x = new_value as u16,
            BitPieceStorageMutRef::U8(x) => **x = new_value as u8,
        }
    }
}

/// a convenience type for interacting with the bits of an underlying storage type, starting at a specific bit index.
/// this is useful for implementing mutable references to bitpieces.
pub struct BitsMut<'s> {
    pub storage: BitPieceStorageMutRef<'s>,
    pub start_bit_index: usize,
}
impl<'s> BitsMut<'s> {
    #[inline(always)]
    pub const fn new(storage: BitPieceStorageMutRef<'s>, start_bit_index: usize) -> Self {
        Self {
            storage,
            start_bit_index,
        }
    }

    /// returns `len` bits starting at relative bit index `rel_bit_index`.
    #[inline(always)]
    pub const fn get_bits(&self, rel_bit_index: usize, len: usize) -> u64 {
        extract_bits(
            self.storage.get(),
            self.start_bit_index + rel_bit_index,
            len,
        )
    }

    /// modifies the `len` bits starting at relative bit index `rel_bit_index` to the given `new_value`.
    #[inline(always)]
    pub const fn set_bits(&mut self, rel_bit_index: usize, len: usize, new_value: u64) {
        self.storage.set(modify_bits(
            self.storage.get(),
            self.start_bit_index + rel_bit_index,
            len,
            new_value,
        ));
    }
}

/// a mutable reference to a bitpiece inside another bitpiece.
pub trait BitPieceMutRef<'s> {
    type BitPiece: BitPiece;
    fn new(storage: BitPieceStorageMutRef<'s>, start_bit_index: usize) -> Self;
    fn get(&self) -> Self::BitPiece;
    fn set(&mut self, new_value: Self::BitPiece);
}

#[macro_export]
macro_rules! bitpiece_define_mut_ref_type {
    {$t: ty, $mut_ref_ty_name: ident} => {
        pub struct $mut_ref_ty_name<'s>(pub $crate::BitsMut<'s>);
        impl<'s> $mut_ref_ty_name<'s> {
            pub const fn new(storage: $crate::BitPieceStorageMutRef<'s>, start_bit_index: usize) -> Self {
                Self($crate::BitsMut::new(storage, start_bit_index))
            }

            pub const fn get(&self) -> $t {
                let bits = self.0.get_bits(0, <$t as $crate::BitPiece>::BITS) as <$t as $crate::BitPiece>::Bits;
                <$t as $crate::BitPiece>::Converter::from_bits(bits)
            }

            pub const fn set(&mut self, new_value: $t) {
                let bits = <$t as $crate::BitPiece>::Converter::to_bits(new_value);
                self.0
                    .set_bits(0, <$t as $crate::BitPiece>::BITS, bits as u64);
            }
        }
        impl<'s> $crate::BitPieceMutRef<'s> for $mut_ref_ty_name<'s> {
            type BitPiece = $t;

            fn new(storage: $crate::BitPieceStorageMutRef<'s>, start_bit_index: usize) -> Self {
                Self::new(storage, start_bit_index)
            }

            fn get(&self) -> $t {
                self.get()
            }

            fn set(&mut self, new_value: $t) {
                self.set(new_value)
            }
        }
    };
}
