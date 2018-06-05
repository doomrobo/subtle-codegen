extern crate subtle;
#[macro_use]
extern crate subtle_codegen;

#[derive(ConstantTimeEq)]
//~^ ERROR proc-macro derive panicked
//~^^ HELP ConstantTimeEq can only be derived on struct, but Bad is an enum
enum Bad {
    A,
    B
}

fn main() {}
