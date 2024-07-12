#[bitfield_piece(3)]
struct Inner {
    a: bool,
    b: bool,
    c: bool,
}
// =>
struct Inner {
    bits: u8,
}

struct InnerMut<'a> {}

#[bitfield(u8)]
struct Outer {
    inner1: Inner,
    Inner2: Inner,
    x: bool,
    y: bool,
}

// =>

struct Outer {
    bits: u8,
}

impl Outer {
    pub const fn from_bits(bits: u8) -> Self {
        Self { bits }
    }
    pub const fn bits(&self) -> u8 {
        self.bits
    }
    pub const fn inner1() -> Inner {}
}

pub trait Bitfield {
    type Bits: Clone + Copy;
    fn from_bits(bits: Self::Bits) -> Self;
    fn to_bits(&self) -> Self::Bits;
    fn extract_bits()
}

pub trait BitStorage: Clone + Copy {
    fn to_u64() -> u64;
}
