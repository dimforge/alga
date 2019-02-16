extern crate alga;
#[macro_use]
extern crate alga_derive;

use alga::general::{TwoSidedInverse, Identity, Additive, Multiplicative, AbstractMagma};

#[derive(Alga, Clone, PartialEq, Debug)]
#[alga_traits(Field(Additive, Multiplicative))]
struct W(f64);

impl AbstractMagma<Additive> for W {
    fn operate(&self, right: &Self) -> Self {
        W(self.0 + right.0)
    }
}
impl AbstractMagma<Multiplicative> for W {
    fn operate(&self, right: &Self) -> Self {
        W(self.0 * right.0)
    }
}

impl TwoSidedInverse<Additive> for W {
    fn two_sided_inverse(&self) -> Self {
        W(-self.0)
    }
}

impl TwoSidedInverse<Multiplicative> for W {
    fn two_sided_inverse(&self) -> Self {
        W(1. / self.0)
    }
}

impl Identity<Additive> for W {
    fn identity() -> Self {
        W(0.)
    }
}

impl Identity<Multiplicative> for W {
    fn identity() -> Self {
        W(1.)
    }
}

fn main() {
    let zero = W::id(Additive);
    let one = W::id(Multiplicative);
    assert_eq!(zero.op(Multiplicative, &one), zero);
}
