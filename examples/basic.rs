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
    print_val(shit.b().x());
}
