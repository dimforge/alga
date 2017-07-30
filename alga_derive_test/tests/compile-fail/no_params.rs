extern crate alga;
#[macro_use]
extern crate alga_derive;

#[derive(Alga)]//~ ERROR proc-macro derive panicked
//~^ HELP One operator is required for `Group` trait. None was provided.
#[alga_traits(Group())]
struct W;
