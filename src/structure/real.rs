use num::{Float, Num, FromPrimitive};
use std::ops::{Neg, AddAssign, MulAssign, SubAssign, DivAssign};
use cmp::ApproxEq;
use structure::FieldApprox;

#[allow(missing_docs)]

/// Trait shared by all approximate reals.
///
/// Approximate reals are approximate fields aquipped with functions that are commonly used on
/// reals. The results of those functions only have to be approximately equal to the actual
/// theoretical values.
pub trait RealApprox: Copy + Num + FieldApprox + FromPrimitive +
                      Neg<Output = Self> + AddAssign + MulAssign + SubAssign + DivAssign +
                      ApproxEq<Eps = Self> + Ord {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn recip(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn abs_sub(self, other: Self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
}

impl<S> RealApprox for S
where S: Float + FieldApprox + FromPrimitive + Neg +
         AddAssign + MulAssign + SubAssign + DivAssign +
         ApproxEq<Eps = S> + Ord {
    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline]
    fn ceil(self) -> Self {
        self.ceil()
    }

    #[inline]
    fn round(self) -> Self {
        self.round()
    }

    #[inline]
    fn trunc(self) -> Self {
        self.trunc()
    }

    #[inline]
    fn fract(self) -> Self {
        self.fract()
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn signum(self) -> Self {
        self.signum()
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        self.is_sign_positive()
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        self.is_sign_negative()
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }

    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }

    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    #[inline]
    fn exp(self) -> Self {
        self.exp()
    }

    #[inline]
    fn exp2(self) -> Self {
        self.exp2()
    }

    #[inline]
    fn ln(self) -> Self {
        self.ln()
    }

    #[inline]
    fn log(self, base: Self) -> Self {
        self.log(base)
    }

    #[inline]
    fn log2(self) -> Self {
        self.log2()
    }

    #[inline]
    fn log10(self) -> Self {
        self.log10()
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline]
    fn abs_sub(self, other: Self) -> Self {
        self.abs_sub(other)
    }

    #[inline]
    fn cbrt(self) -> Self {
        self.cbrt()
    }

    #[inline]
    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }

    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }

    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }

    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }

    #[inline]
    fn asin(self) -> Self {
        self.asin()
    }

    #[inline]
    fn acos(self) -> Self {
        self.acos()
    }

    #[inline]
    fn atan(self) -> Self {
        self.atan()
    }

    #[inline]
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }

    #[inline]
    fn exp_m1(self) -> Self {
        self.exp_m1()
    }

    #[inline]
    fn ln_1p(self) -> Self {
        self.ln_1p()
    }

    #[inline]
    fn sinh(self) -> Self {
        self.sinh()
    }

    #[inline]
    fn cosh(self) -> Self {
        self.cosh()
    }

    #[inline]
    fn tanh(self) -> Self {
        self.tanh()
    }

    #[inline]
    fn asinh(self) -> Self {
        self.asinh()
    }

    #[inline]
    fn acosh(self) -> Self {
        self.acosh()
    }

    #[inline]
    fn atanh(self) -> Self {
        self.atanh()
    }
}
