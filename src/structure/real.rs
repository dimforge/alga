use num::{Float, Num, FromPrimitive};
use std::ops::{Neg, AddAssign, MulAssign, SubAssign, DivAssign};
use cmp::ApproxEq;
use structure::FieldApprox;

#[allow(missing_docs)]

/// Trait shared by all approximate reals.
///
/// Approximate reals are approximate fields aquipped with funtinos that are commonly used on
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
    fn floor(self) -> Self {
        self.floor()
    }

    fn ceil(self) -> Self {
        self.ceil()
    }

    fn round(self) -> Self {
        self.round()
    }

    fn trunc(self) -> Self {
        self.trunc()
    }

    fn fract(self) -> Self {
        self.fract()
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn signum(self) -> Self {
        self.signum()
    }

    fn is_sign_positive(self) -> bool {
        self.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }

    fn recip(self) -> Self {
        self.recip()
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn exp(self) -> Self {
        self.exp()
    }

    fn exp2(self) -> Self {
        self.exp2()
    }

    fn ln(self) -> Self {
        self.ln()
    }

    fn log(self, base: Self) -> Self {
        self.log(base)
    }

    fn log2(self) -> Self {
        self.log2()
    }

    fn log10(self) -> Self {
        self.log10()
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn abs_sub(self, other: Self) -> Self {
        self.abs_sub(other)
    }

    fn cbrt(self) -> Self {
        self.cbrt()
    }

    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn tan(self) -> Self {
        self.tan()
    }

    fn asin(self) -> Self {
        self.asin()
    }

    fn acos(self) -> Self {
        self.acos()
    }

    fn atan(self) -> Self {
        self.atan()
    }

    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }

    fn exp_m1(self) -> Self {
        self.exp_m1()
    }

    fn ln_1p(self) -> Self {
        self.ln_1p()
    }

    fn sinh(self) -> Self {
        self.sinh()
    }

    fn cosh(self) -> Self {
        self.cosh()
    }

    fn tanh(self) -> Self {
        self.tanh()
    }

    fn asinh(self) -> Self {
        self.asinh()
    }

    fn acosh(self) -> Self {
        self.acosh()
    }

    fn atanh(self) -> Self {
        self.atanh()
    }
}
