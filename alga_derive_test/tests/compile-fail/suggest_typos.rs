extern crate alga;
#[macro_use]
extern crate alga_derive;

#[derive(Alga)]//~ ERROR proc-macro derive panicked
//~^ HELP Invalid Alga trait provided. Did you mean `Semigroup`?
#[alga_traits(Senigoop(Additive))]
struct W;
