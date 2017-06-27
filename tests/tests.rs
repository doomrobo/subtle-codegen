extern crate subtle;
#[macro_use]
extern crate subtle_codegen;

use subtle::CTEq;

#[derive(Copy, Clone, CTEq)]
struct Foo {
    a: u8,
    b: u8,
}

#[derive(Copy, Clone, CTEq)]
struct Bar (u8, u8);

#[derive(Copy, Clone, CTEq)]
struct Baz {
    a: Foo,
    b: Bar,
    c: [i16; 4],
}

#[test]
fn test_struct_eq() {
    let f = Foo { a: 10, b: 11 };
    let g = f.clone();

    assert_eq!(f.ct_eq(&g), 1u8);
}

#[test]
fn test_struct_neq() {
    let f = Foo { a: 10, b: 11 };
    let g = Foo { a: 10, b: 12 };

    assert_eq!(f.ct_eq(&g), 0u8);
}

#[test]
fn test_tuple_eq() {
    let f = Bar(10, 11);
    let g = f.clone();

    assert_eq!(f.ct_eq(&g), 1u8);
}

#[test]
fn test_tuple_neq() {
    let f = Bar(10, 11);
    let g = Bar(10, 12);

    assert_eq!(f.ct_eq(&g), 0u8);
}

#[test]
fn test_nested_eq() {
    let f = Baz {
        a: Foo {
            a: 10,
            b: 20,
        },
        b: Bar(30, 40),
        c: [1, 2, 3, 4],
    };
    let g = f.clone();

    assert_eq!(f.ct_eq(&g), 1u8);
}

#[test]
fn test_nested_neq() {
    let f = Baz {
        a: Foo {
            a: 10,
            b: 20,
        },
        b: Bar(30, 40),
        c: [1, 2, 3, 4],
    };
    let mut g = f.clone();
    g.c[0] = 2;

    assert_eq!(f.ct_eq(&g), 0u8);
}
