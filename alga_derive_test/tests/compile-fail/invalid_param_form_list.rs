extern crate alga;
#[macro_use]
extern crate alga_derive;

#[derive(Alga)]//~ ERROR proc-macro derive panicked
//~^ HELP Operator has to be provided via #[alga_traits(Group(Operator))
#[alga_traits(Group("Operator"))]
struct W;
