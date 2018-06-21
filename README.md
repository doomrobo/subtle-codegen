# subtle-codegen
Procedural macros for the Rust [`subtle`](https://doc.dalek.rs/subtle/) crate. All that this has at
the moment is a custom derive for the `subtle::ConstantTimeEq` trait.

## Example

```rust
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
```

## Testing

To test, simply run `cargo test`. If you get something like

```
error: tests/compile-fail/enum.rs:3: unexpected error: '3:1: 3:29: multiple matching crates for `subtle_codegen` [E0464]
```

just run `cargo clean` and try again.


## TODO

 * Check that this implementation is actually constant-time
 * Implement some of the other traits exposed by `subtle`
 * Publish this crate
