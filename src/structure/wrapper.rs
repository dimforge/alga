use std::ops::{Add, Neg, Sub, Mul, Div};

use ops::{Op, Additive, Multiplicative};
use cmp::ApproxEq;

use structure::MagmaApprox;
use structure::QuasigroupApprox;

/// Wrapper that allows to use operators on algebraic types.
pub struct Ma<M: MagmaApprox<O>, O: Op>(pub M, pub O);

impl<M, O> Clone for Ma<M, O>
where M: MagmaApprox<O>, O: Op
{
    fn clone(&self) -> Self {
        Ma(self.0.clone(), O::oper())
    }
}

impl<M: MagmaApprox<O>, O: Op> Ma<M, O> {
    /// Creates new wrapper with specified value.
    pub fn new(value: M) -> Self {
        Ma(value, O::oper())
    }
}

impl<M, O> PartialEq for Ma<M, O>
where M: MagmaApprox<O>, O: Op
{
    fn eq(&self, lhs: &Self) -> bool {
        self.0 == lhs.0
    }
}

impl<M, O> ApproxEq for Ma<M, O>
where M: MagmaApprox<O>, O: Op
{
    type Eps = M::Eps;
    fn default_epsilon() -> Self::Eps {
        M::default_epsilon()
    }

    fn approx_eq_eps(&self, b: &Self, epsilon: &Self::Eps) -> bool {
        M::approx_eq_eps(&self.0, &b.0, epsilon)
    }
}

impl<M, O> Add<Ma<M, O>> for Ma<M, O>
where M: MagmaApprox<Additive> + MagmaApprox<O>, O: Op
{
    type Output = Self;
    fn add(self, lhs: Self) -> Self {
        Ma(self.0.ap(Additive, lhs.0), O::oper())
    }
}

impl<M, O> Neg for Ma<M, O>
where M: QuasigroupApprox<Additive> + MagmaApprox<O>, O: Op
{
    type Output = Self;
    fn neg(mut self) -> Self {
        self.0 = self.0.inv();
        self
    }
}

impl<M, O> Sub<Ma<M, O>> for Ma<M, O>
where M: QuasigroupApprox<Additive> + MagmaApprox<O>, O: Op
{
    type Output = Self;
    fn sub(self, lhs: Self) -> Self {
        self + -lhs
    }
}

impl<M, O> Mul<Ma<M, O>> for Ma<M, O>
where M: MagmaApprox<Multiplicative> + MagmaApprox<O>, O: Op
{
    type Output = Self;
    fn mul(self, lhs: Self) -> Self {
        Ma(self.0.ap(Multiplicative, lhs.0), O::oper())
    }
}

impl<M, O> Div<Ma<M, O>> for Ma<M, O>
where M: QuasigroupApprox<Multiplicative> + MagmaApprox<O>, O: Op
{
    type Output = Self;
    fn div(self, lhs: Self) -> Self {
        Ma(self.0.ap(Multiplicative, lhs.0.inv()), O::oper())
    }
}
