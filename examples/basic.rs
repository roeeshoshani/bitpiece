use std::{fmt::Display, str::FromStr};

use bitpiece::*;

#[bitpiece]
#[derive(Debug, Clone, Copy)]
struct Shit {
    a: Nibble,
    b: Nibble,
}

#[bitpiece]
#[derive(Debug, Clone, Copy)]
struct Nibble {
    x: B2,
    y: B2,
}

struct NibbleMut<'a, S: BitStorage> {
    storage: &'a mut S,
    start_bit_index: usize,
}
impl<'a, S: BitStorage> NibbleMut<'a, S> {
    pub fn set_x(&mut self, new_x: B2) {
        let v = self.storage.to_u64();
        *self.storage = S::from_u64(v | new_x.to_bits() as u64).unwrap();
    }
}

#[inline(never)]
fn read_val<T: FromStr>() -> T {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().ok().unwrap()
}

#[inline(never)]
fn print_val<T: Display>(val: T) {
    println!("{}", val);
}

fn main() {
    let val: u8 = read_val();
    let mut shit = Shit::from_bits(val);
    shit.set_b(Nibble::from_bits(1));
    print_val(shit.to_bits());
}
