extern crate alga;
#[macro_use]
extern crate alga_derive;

#[derive(Alga)]//~ ERROR proc-macro derive panicked
//~^ HELP One operator is required for `Semigroup` trait. Too many were provided.
#[alga_traits(Semigroup(Additive, Multiplicative))]
struct W;
