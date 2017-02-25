//! Wrappers that attach an algebraic structure with a value type.

use std::ops::{Add, Neg, Sub, Mul, Div};
use std::cmp::{PartialOrd, Ordering};
use std::fmt::{Display, Formatter, Error};
use std::marker::PhantomData;

use approx::ApproxEq;

use general::{Operator, Inverse};
use general::AbstractMagma;
use general::AbstractQuasigroup;

/// Wrapper that allows to use operators on algebraic types.
#[derive(Debug)]
pub struct Wrapper<T, A, M> {
    pub val: T,
    _add: PhantomData<A>,
    _mul: PhantomData<M>,
}

impl<T: Copy, A, M> Copy for Wrapper<T, A, M> { }

impl<T: Clone, A, M> Clone for Wrapper<T, A, M> {
    fn clone(&self) -> Self {
        Wrapper::new(self.val.clone())
    }
}

impl<T: PartialOrd, A, M> PartialOrd for Wrapper<T, A, M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl<T: PartialEq, A, M> PartialEq for Wrapper<T, A, M> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T, A, M> Wrapper<T, A, M> {
    pub fn new(val: T) -> Self {
        Wrapper {
            val: val,
            _add: PhantomData,
            _mul: PhantomData
        }
    }
}

impl<T: Display, A: Operator, M: Operator> Display for Wrapper<T, A, M> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        self.val.fmt(fmt)
    }
}

impl<T: ApproxEq, A, M> ApproxEq for Wrapper<T, A, M> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.val.relative_eq(&other.val, epsilon, max_relative)
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.val.ulps_eq(&other.val, epsilon, max_ulps)
    }
}

impl<T, A: Operator, M> Add<Wrapper<T, A, M>> for Wrapper<T, A, M>
where T: AbstractMagma<A>
{
    type Output = Self;

    #[inline]
    fn add(self, lhs: Self) -> Self {
        Wrapper::new(self.val.operate(&lhs.val))
    }
}

impl<T, A: Operator, M> Neg for Wrapper<T, A, M>
where T: AbstractQuasigroup<A>
{
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self {
        self.val = self.val.inverse();
        self
    }
}

impl<T, A: Operator, M> Sub<Wrapper<T, A, M>> for Wrapper<T, A, M>
where T: AbstractQuasigroup<A>
{
    type Output = Self;

    #[inline]
    fn sub(self, lhs: Self) -> Self {
        self + -lhs
    }
}

impl<T, A, M: Operator> Mul<Wrapper<T, A, M>> for Wrapper<T, A, M>
where T: AbstractMagma<M>
{
    type Output = Self;

    #[inline]
    fn mul(self, lhs: Self) -> Self {
        Wrapper::new(self.val.operate(&lhs.val))
    }
}

impl<T, A, M: Operator> Inverse<M> for Wrapper<T, A, M>
where T: AbstractQuasigroup<M> {
    #[inline]
    fn inverse(&self) -> Self {
        Wrapper::new(self.val.inverse())
    }
}

impl<T, A, M: Operator> Div<Wrapper<T, A, M>> for Wrapper<T, A, M>
where T: AbstractQuasigroup<M> {
    type Output = Self;

    #[inline]
    fn div(self, lhs: Self) -> Self {
        self * lhs.inverse()
    }
}
