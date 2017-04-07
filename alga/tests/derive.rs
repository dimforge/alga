extern crate alga;
#[macro_use]
extern crate alga_derive;
extern crate quickcheck;
extern crate approx;

use alga::general::{Inverse, Identity, Additive, Multiplicative, AbstractMagma};

use approx::ApproxEq;

use quickcheck::{Arbitrary, Gen};

#[derive(Alga, Clone, PartialEq, Debug)]
#[alga_traits(Field(Additive, Multiplicative))]
#[alga_quickcheck]
struct W(f64);

impl ApproxEq for W {
    type Epsilon = W;
    fn default_epsilon() -> W {
        W(0.0000000001)
    }

    fn default_max_relative() -> W {
        W(0.0000000001)
    }

    fn default_max_ulps() -> u32 {
        40
    }

    fn relative_eq(&self, other: &Self, epsilon: W, max_relative: W) -> bool {
        self.0.relative_eq(&other.0, epsilon.0, max_relative.0)
    }

    fn ulps_eq(&self, other: &Self, epsilon: W, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon.0, max_ulps)
    }
}

impl Arbitrary for W {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        W(f64::arbitrary(g))
    }
    fn shrink(&self) -> Box<Iterator<Item=Self>> {
        Box::new(self.0.shrink().map(W))
    }
}

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
