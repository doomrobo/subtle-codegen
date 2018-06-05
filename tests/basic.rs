extern crate subtle;
#[macro_use]
extern crate subtle_codegen;

use subtle::{Choice, ConstantTimeEq};

// --------- Helper functions ---------

fn assert_true(c: Choice) {
    assert_eq!(c.unwrap_u8(), 1u8);
}

fn assert_false(c: Choice) {
    assert_eq!(c.unwrap_u8(), 0u8);
}

// --------- The structs we'll be testing ---------

#[derive(Copy, Clone, ConstantTimeEq)]
struct SimpleStruct {
    a: u8,
    b: u8,
}

#[derive(Copy, Clone, ConstantTimeEq)]
struct TupleStruct(u8, u8);

#[derive(Copy, Clone, ConstantTimeEq)]
struct ComplexStruct {
    a: SimpleStruct,
    b: TupleStruct,
    c: [i16; 4],
}

#[derive(Copy, Clone, ConstantTimeEq)]
struct GenericStruct<'a, 'b, T> where T: 'a + ConstantTimeEq + Sized, 'a : 'b  {
    a: &'a [T],
    b: &'b [u8],
    c: T,
}

// --------- Unit tests ---------

#[test]
fn test_struct_eq() {
    let f = SimpleStruct { a: 10, b: 11 };
    let g = f.clone();

    assert_true(f.ct_eq(&g));
}

#[test]
fn test_struct_neq() {
    let f = SimpleStruct { a: 10, b: 11 };
    let g = SimpleStruct { a: 10, b: 12 };

    assert_false(f.ct_eq(&g));
}

#[test]
fn test_tuple_eq() {
    let f = TupleStruct(10, 11);
    let g = f.clone();

    assert_true(f.ct_eq(&g));
}

#[test]
fn test_tuple_neq() {
    let f = TupleStruct(10, 11);
    let g = TupleStruct(10, 12);

    assert_false(f.ct_eq(&g));
}

#[test]
fn test_nested_eq() {
    let f = ComplexStruct {
        a: SimpleStruct {
            a: 10,
            b: 20,
        },
        b: TupleStruct(30, 40),
        c: [1, 2, 3, 4],
    };
    let g = f.clone();

    assert_true(f.ct_eq(&g));
}

#[test]
fn test_nested_neq() {
    let f = ComplexStruct {
        a: SimpleStruct {
            a: 10,
            b: 20,
        },
        b: TupleStruct(30, 40),
        c: [1, 2, 3, 4],
    };
    let mut g = f.clone();
    g.c[2] = 0;

    assert_false(f.ct_eq(&g));
}

#[test]
fn test_generics_eq() {
    let f = GenericStruct {
        a: &[1, 2, 3],
        b: &[4, 5, 6],
        c: 10u8,
    };
    let g = f.clone();

    assert_true(f.ct_eq(&g));
}

#[test]
fn test_generics_neq() {
    let f = GenericStruct {
        a: &[1, 2, 3],
        b: &[4, 5, 6],
        c: 10u8,
    };
    let g = GenericStruct {
        a: &[1, 2, 3],
        b: &[4, 0, 6],
        c: 10u8,
    };

    assert_false(f.ct_eq(&g));
}
