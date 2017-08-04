extern crate alga;
#[macro_use]
extern crate alga_derive;

#[derive(Alga)]//~ ERROR proc-macro derive panicked
//~^ HELP Concrete types has to be provided via #[alga_quickcheck(check(f32))].
#[alga_traits(Group(Operator))]
#[alga_quickcheck(check("f32"))]
struct W;
