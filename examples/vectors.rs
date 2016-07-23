extern crate algebra;

use algebra::ops::{Additive, Inverse};
use algebra::structure::MagmaApprox;

struct Vec2<Scalar: FieldApprox> {
    x: Scalar,
    y: Scalar,
}

impl<Scalar: FieldApprox> ApproxEq for Vec<Scalar> {
    type Eps = Scalar::Eps;

    fn default_epsilon(_: Option<Self>) -> Self::Eps {
        Scalar::default_epsilon(None)
    }

    fn approx_eq_eps(&self, b: &Self, epsilon: &Self::Eps) -> bool {
        self.x.approx_eq_eps(b.x, epsilon) &&
        self.y.approx_eq_eps(b.y, epsilon)
    }
}

impl<Scalar: FieldApprox> MagmaApprox<Additive> for Vec2<Scalar> {
    fn approx(mut self, lhs: Self) -> Self {
        self.x = self.x.approx(lhs.x);
        self.y = self.y.approx(lhs.y);
        self
    }
}

impl<Scalar: FieldApprox> Inverse<Additive> for Vec2<Scalar> {
    fn inv(mut self) -> Self {
        self.x = self.x.inv();
        self.y = self.y.inv();
    }
}

impl<Scalar: FieldApprox> Identity<Additive> for Vec2<Scalar> {
    fn id() -> Self {
        Vec2 {
            x: Scalar::id(),
            y: Scalar::id(),
        }
    }
}

impl<Scalar: FieldApprox> QuasigroupApprox<Additive> for Vec2<Scalar> {}
impl<Scalar: FieldApprox> LoopApprox<Additive> for Vec2<Scalar> {}

impl<Scalar: FieldApprox> SemigroupApprox<Additive> for Vec2<Scalar> {}
impl<Scalar: FieldApprox> MonoidApprox<Additive> for Vec2<Scalar> {}

impl<Scalar: FieldApprox> GroupApprox<Additive> for Vec2<Scalar> {}
impl<Scalar: FieldApprox> AbelianGroupApprox<Additive> for Vec2<Scalar> {}

impl<Scalar: FieldApprox> Module<Scalar> for Vec2<Scalar> {}
impl<Scalar: FieldApprox> Vector<Scalar> for Vec2<Scalar> {}

impl<Scalar: FieldApprox> MagmaApprox<Multiplicative> for Vec2<Scalar> {
    fn approx(mut self, lhs: Self) -> Self {
        self.x = self.x.approx(lhs.x);
        self.y = self.y.approx(lhs.y);
        self
    }
}

impl<Scalar: FieldApprox> Identity<Multiplicative> for Vec2<Scalar> {
    fn id() -> Self {
        Vec2 {
            x: Scalar::id(),
            y: Scalar::id(),
        }
    }
}

impl<Scalar: FieldApprox> SemigroupApprox<Multiplicative> for Vec2<Scalar> {}
impl<Scalar: FieldApprox> MonoidApprox<Multiplicative> for Vec2<Scalar> {}
//TODO: Is this valid? And does it mean that these vectors are rings themselves?.


fn main() {
    let vec = Ma::new(Vec2::new(2f32, 3.));
    let vec2 = Ma::new(Vec2::new(5., 7.));
    let vec3 = Ma::new(Vec2::new(11., 17.));
    let vec4 = (vec * vec2) + (vec * vec3);
    let vec5 = vec * (vec2 + vec3);
    if vec4.approx_eq(vec5) {
        println!("Works.");
    }
}
