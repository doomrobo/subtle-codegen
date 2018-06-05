extern crate subtle;
#[macro_use]
extern crate subtle_codegen;

use subtle::{Choice, ConstantTimeEq};

#[derive(Debug, ConstantTimeEq)]
struct Foo {
    a: u8,
    b: u8,
}

fn assert_true(c: Choice) {
    assert!(c.unwrap_u8() == 1u8);
}

fn main() {
    let f = Foo { a: 10, b: 11 };
    let g = Foo { a: 10, b: 11 };

    println!("Is {:?} == {:?}?", f, g);
    assert_true(f.ct_eq(&g));
    println!("Yes!");
}
