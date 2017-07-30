extern crate alga;
#[macro_use]
extern crate alga_derive;

#[derive(Alga)]//~ ERROR proc-macro derive panicked
//~^ HELP Two operators are required for `Field` trait. Only one was provided.
#[alga_traits(Field(Additive))]
struct W;
