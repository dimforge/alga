extern crate alga;
#[macro_use]
extern crate alga_derive;

use alga::general::{Inverse, Identity, Additive, Multiplicative, AbstractMagma};

#[derive(Alga, Clone, PartialEq)]
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

impl Inverse<Additive> for W {
    fn inverse(&self) -> Self {
        W(-self.0)
    }
}

impl Inverse<Multiplicative> for W {
    fn inverse(&self) -> Self {
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
