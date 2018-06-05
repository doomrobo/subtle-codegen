# subtle-codegen
Procedural macros for the Rust `subtle` crate. All that this has at the moment is a custom derive
for the `subtle::ConstantTimeEq` trait.

## Example

```rust
extern crate subtle;
#[macro_use]
extern crate subtle_codegen;

use subtle::ConstantTimeEq;

#[derive(Copy, Clone, ConstantTimeEq)]
struct Foo {
    a: u8,
    b: u8,
}

fn main() {
    let f = Foo { a: 10, b: 11 };
    let g = f.clone();

    assert_eq!(f.ct_eq(&g), 1u8);
}
```

## TODO

 * Check that this implementation is actually constant-time
