use std::{fmt::Display, str::FromStr};

use bitpiece::{bitpiece, BitPiece};

#[bitpiece]
#[derive(Debug, Clone, Copy)]
struct Shit {
    n1: Nibble,
    n2: Nibble,
}

#[bitpiece]
#[derive(Debug, Clone, Copy)]
struct Nibble {
    b1: bool,
    b2: bool,
    b3: bool,
    b4: bool,
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
    let shit = Shit::from_bits(val);
    print_val(shit.n2().b2());
}
