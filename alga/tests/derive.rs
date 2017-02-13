extern crate alga;
#[macro_use]
extern crate alga_derive;

use alga::general::{Inverse, Additive, Multiplicative, AbstractMagma};

#[derive(Alga, Clone, PartialEq)]
#[alga_traits(Quasigroup = "Additive", Semigroup = "Multiplicative")]
struct W(i64);

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
