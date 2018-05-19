use core::any::Any;
use core::fmt::{Debug, Display};
use core::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};
use core::{f32, f64};
use num::{Bounded, FromPrimitive, Num, Signed};

use approx::{RelativeEq, UlpsEq};

use general::{Field, Lattice, SubsetOf, SupersetOf};

#[allow(missing_docs)]

/// Trait shared by all reals.
///
/// Reals are equipped with functions that are commonly used on reals. The results of those
/// functions only have to be approximately equal to the actual theoretical values.
// FIXME: SubsetOf should be removed when specialization will be supported by rustc. This will
// allow a blancket impl: impl<T: Clone> SubsetOf<T> for T { ... }
// NOTE: make all types debuggable/'static/Any ? This seems essencial for any kind of generic programming.
pub trait Real:
    SubsetOf<Self>
    + SupersetOf<f64>
    + Field
    + Copy
    + Num
    + FromPrimitive
    + Neg<Output = Self>
    + AddAssign
    + MulAssign
    + SubAssign
    + DivAssign
    + RelativeEq<Epsilon = Self>
    + UlpsEq<Epsilon = Self>
    + Lattice
    + PartialEq
    + Signed
    + Send
    + Sync
    + Any
    + 'static
    + Debug
    + Display
    + Bounded
{
    // NOTE: a real must be bounded because, no matter the chosen representation, being `Copy` implies that it occupies a statically-known size, meaning that it must have min/max values.
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

    fn pi() -> Self;
    fn two_pi() -> Self;
    fn frac_pi_2() -> Self;
    fn frac_pi_3() -> Self;
    fn frac_pi_4() -> Self;
    fn frac_pi_6() -> Self;
    fn frac_pi_8() -> Self;
    fn frac_1_pi() -> Self;
    fn frac_2_pi() -> Self;
    fn frac_2_sqrt_pi() -> Self;

    fn e() -> Self;
    fn log2_e() -> Self;
    fn log10_e() -> Self;
    fn ln_2() -> Self;
    fn ln_10() -> Self;
}

macro_rules! impl_real(
    ($($T:ty, $M:ident);*) => ($(
        impl Real for $T {
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

            /// Archimedes' constant.
            #[inline]
            fn pi() -> Self {
                $M::consts::PI
            }

            /// 2.0 * pi.
            #[inline]
            fn two_pi() -> Self {
                $M::consts::PI + $M::consts::PI
            }

            /// pi / 2.0.
            #[inline]
            fn frac_pi_2() -> Self {
                $M::consts::FRAC_PI_2
            }

            /// pi / 3.0.
            #[inline]
            fn frac_pi_3() -> Self {
                $M::consts::FRAC_PI_3
            }

            /// pi / 4.0.
            #[inline]
            fn frac_pi_4() -> Self {
                $M::consts::FRAC_PI_4
            }

            /// pi / 6.0.
            #[inline]
            fn frac_pi_6() -> Self {
                $M::consts::FRAC_PI_6
            }

            /// pi / 8.0.
            #[inline]
            fn frac_pi_8() -> Self {
                $M::consts::FRAC_PI_8
            }

            /// 1.0 / pi.
            #[inline]
            fn frac_1_pi() -> Self {
                $M::consts::FRAC_1_PI
            }

            /// 2.0 / pi.
            #[inline]
            fn frac_2_pi() -> Self {
                $M::consts::FRAC_2_PI
            }

            /// 2.0 / sqrt(pi).
            #[inline]
            fn frac_2_sqrt_pi() -> Self {
                $M::consts::FRAC_2_SQRT_PI
            }


            /// Euler's number.
            #[inline]
            fn e() -> Self {
                $M::consts::E
            }

            /// log2(e).
            #[inline]
            fn log2_e() -> Self {
                $M::consts::LOG2_E
            }

            /// log10(e).
            #[inline]
            fn log10_e() -> Self {
                $M::consts::LOG10_E
            }

            /// ln(2.0).
            #[inline]
            fn ln_2() -> Self {
                $M::consts::LN_2
            }

            /// ln(10.0).
            #[inline]
            fn ln_10() -> Self {
                $M::consts::LN_10
            }
        }
    )*)
);

impl_real!(f32,f32; f64,f64);
#[cfg(decimal)]
impl_real!(decimal::d128, decimal::d128);
