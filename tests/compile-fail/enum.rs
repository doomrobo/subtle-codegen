extern crate subtle;
#[macro_use]
extern crate subtle_codegen;

use subtle::CTEq;

#[derive(CTEq)]
//~^ ERROR proc-macro derive panicked
//~^^ HELP CTEq can only be derived on struct, but Bad is an enum
enum Bad {
    A,
    B
}

fn main() {}
