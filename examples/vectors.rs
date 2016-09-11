extern crate algebra;

use std::mem::swap;
use std::fmt::{Display, Formatter, Error};

use algebra::ops::{Additive, Multiplicative, Inverse, inv};
use algebra::cmp::ApproxEq;
use algebra::structure::*;
use algebra::wrapper::Wrapper as W;
use algebra::wrapper::id as wrap_id;
use algebra::ident::{Identity, id};

#[derive(PartialEq, Clone)]
struct Vec2<Scalar: Field> {
    x: Scalar,
    y: Scalar,
}

impl<Scalar: Field> Vec2<Scalar> {
    fn new(x: Scalar, y: Scalar) -> Vec2<Scalar> {
        Vec2 {
            x: x,
            y: y,
        }
    }
}

impl<Scalar: Field + Display> Display for Vec2<Scalar> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl<Scalar: Field + ApproxEq> ApproxEq for Vec2<Scalar> {
    type Eps = Scalar::Eps;

    fn default_epsilon() -> Self::Eps {
        Scalar::default_epsilon()
    }

    fn approx_eq_eps(&self, b: &Self, epsilon: &Self::Eps) -> bool {
        self.x.approx_eq_eps(&b.x, epsilon) &&
        self.y.approx_eq_eps(&b.y, epsilon)
    }
}

impl<Scalar: Field> Magma<Additive> for Vec2<Scalar> {
    fn operate(mut self, lhs: Self) -> Self {
        self.x = self.x.op(Additive, lhs.x);
        self.y = self.y.op(Additive, lhs.y);
        self
    }
}

impl<Scalar: Field> Inverse<Additive> for Vec2<Scalar> {
    fn inv(mut self) -> Self {
        self.x = inv(Additive, self.x);
        self.y = inv(Additive, self.y);
        self
    }
}

impl<Scalar: Field> Identity<Additive> for Vec2<Scalar> {
    fn id() -> Self {
        Vec2 {
            x: id(Additive),
            y: id(Additive),
        }
    }
}

impl<Scalar: Field> Quasigroup<Additive> for Vec2<Scalar> {}
impl<Scalar: Field> Loop<Additive> for Vec2<Scalar> {}

impl<Scalar: Field> Semigroup<Additive> for Vec2<Scalar> {}
impl<Scalar: Field> Monoid<Additive> for Vec2<Scalar> {}

impl<Scalar: Field> Group<Additive> for Vec2<Scalar> {}
impl<Scalar: Field> GroupAbelian<Additive> for Vec2<Scalar> {}

impl<Scalar: Field> Module for Vec2<Scalar> {
    type Ring = Scalar;
}

impl<Scalar: Field> VectorSpace for Vec2<Scalar> {
    type Field = Scalar;
}

impl<Scalar: Field> Magma<Multiplicative> for Vec2<Scalar> {
    fn operate(mut self, lhs: Self) -> Self {
        self.x = self.x.op(Multiplicative, lhs.x);
        self.y = self.y.op(Multiplicative, lhs.y);
        self
    }
}

impl<Scalar: Field> Identity<Multiplicative> for Vec2<Scalar> {
    fn id() -> Self {
        Vec2 {
            x: id(Multiplicative),
            y: id(Multiplicative),
        }
    }
}

fn gcd<T: RingCommutative + PartialOrd>(a: T, b: T) -> T {
    let (mut a, mut b) = (W(a), W(b));
    if a < wrap_id(Additive) {
        a = -a;
    }
    if b < wrap_id(Multiplicative) {
        b = -b;
    }
    while a != b {
        if a > b {
            a = a - b.clone();
        } else {
            b = b - a.clone();
        }
    }
    a.0
}

#[test]
fn gcd_works() {
    assert_eq!(2, gcd(8, 6));
    assert_eq!(2, gcd(6, 8));
    assert_eq!(3, gcd(15, 6));
    assert_eq!(3, gcd(6, 15));
    assert_eq!(1, gcd(17, 12345));
    assert_eq!(1, gcd(42312, 17));
    assert_eq!(5, gcd(15, -35));
    assert_eq!(5, gcd(-15, 35));
    assert_eq!(5, gcd(-15, -35));
}

#[derive(Clone)]
struct Rational {
    a: i64,
    b: i64,
}

impl Rational {
    fn new(a: i64, b: i64) -> Self {
        assert!(b != 0);
        Rational {
            a: a,
            b: b,
        }
    }

    fn whole(n: i64) -> Self {
        Rational {
            a: n,
            b: 1,
        }
    }
}

impl Display for Rational {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.b == 1 {
            fmt.write_fmt(format_args!("{}", self.a))
        } else {
            fmt.write_fmt(format_args!("{}⁄{}", self.a, self.b))
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, lhs: &Self) -> bool {
        self.a * lhs.b == lhs.a * self.b
    }
}

impl ApproxEq for Rational {
    type Eps = f64;

    fn default_epsilon() -> Self::Eps {
        0.
    }

    fn approx_eq_eps(&self, b: &Self, epsilon: &Self::Eps) -> bool {
        let us = self.a as f64 / self.b as f64;
        let them = b.a as f64 / b.b as f64;
        (us - them).abs() <= *epsilon
    }
}

impl Magma<Additive> for Rational {
    fn operate(mut self, lhs: Self) -> Self {
        self.a = self.a * lhs.b + lhs.a * self.b;
        self.b *= lhs.b;
        let gcd = gcd(self.a, self.b);
        self.a /= gcd;
        self.b /= gcd;
        self
    }
}

impl Inverse<Additive> for Rational {
    fn inv(mut self) -> Self {
        self.a = -self.a;
        self.b = self.b;
        self
    }
}

impl Identity<Additive> for Rational {
    fn id() -> Self {
        Rational {
            a: 0,
            b: 1,
        }
    }
}

impl Quasigroup<Additive> for Rational {}
impl Loop<Additive> for Rational {}

impl Semigroup<Additive> for Rational {}
impl Monoid<Additive> for Rational {}

impl Group<Additive> for Rational {}
impl GroupAbelian<Additive> for Rational {}

impl Magma<Multiplicative> for Rational {
    fn operate(mut self, lhs: Self) -> Self {
        self.a *= lhs.a;
        self.b *= lhs.b;
        let gcd = gcd(self.a, self.b);
        self.a /= gcd;
        self.b /= gcd;
        self
    }
}

impl Inverse<Multiplicative> for Rational {
    fn inv(mut self) -> Self {
        swap(&mut self.a, &mut self.b);
        self
    }
}

impl Identity<Multiplicative> for Rational {
    fn id() -> Self {
        Rational {
            a: 1,
            b: 1,
        }
    }
}

impl Quasigroup<Multiplicative> for Rational {}
impl Loop<Multiplicative> for Rational {}

impl Semigroup<Multiplicative> for Rational {}
impl Monoid<Multiplicative> for Rational {}

impl Group<Multiplicative> for Rational {}
impl GroupAbelian<Multiplicative> for Rational {}

impl Ring for Rational {}
impl RingCommutative for Rational {}
impl Field for Rational {}

fn main() {
    let vec = || W(Vec2::new(Rational::new(1, 2), Rational::whole(3)));
    let vec2 = || W(Vec2::new(Rational::whole(5), Rational::new(11, 7)));
    let vec3 = || W(Vec2::new(Rational::new(7, 11), Rational::whole(17)));

    let vec4 = (vec() * vec2()) + (vec() * vec3());
    let vec5 = vec() * (vec2() + vec3());
    if vec4.approx_eq(&vec5) {
        println!("{} == {}", vec4, vec5);
    } else {
        println!("{} != {}", vec4, vec5);
    }
}
