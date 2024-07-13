use std::{fmt::Display, str::FromStr};

use bitpiece::*;

#[bitpiece]
#[derive(Debug, Clone, Copy)]
enum MyEnum {
    A0,
    A1,
    A2,
    A3,
}

#[bitpiece]
#[derive(Debug, Clone, Copy)]
struct Big {
    shit1: Shit,
    shit2: Shit,
}

#[bitpiece]
#[derive(Debug, Clone, Copy)]
struct Shit {
    nibble1: Nibble,
    nibble2: Nibble,
}

#[bitpiece]
#[derive(Debug, Clone, Copy)]
struct Nibble {
    x: MyEnum,
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
    let val: u16 = read_val();
    let mut big = Big::from_bits(val);
    big.shit2_mut().nibble2_mut().x_mut().set(MyEnum::A2);
    print_val(big.to_bits());
}
