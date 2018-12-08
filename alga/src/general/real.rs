use num::{Bounded, FromPrimitive, Num, Signed};
use std::any::Any;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};
use std::{f32, f64};

use approx::{RelativeEq, UlpsEq};

use general::{Field, Lattice, SubsetOf, SupersetOf};

#[cfg(not(feature = "std"))]
use libm::F32Ext;
#[cfg(not(feature = "std"))]
use libm::F64Ext;
#[cfg(not(feature = "std"))]
use num;
//#[cfg(feature = "decimal")]
//use decimal::d128;

#[allow(missing_docs)]

/// Trait shared by all reals.
///
/// Reals are equipped with functions that are commonly used on reals. The results of those
/// functions only have to be approximately equal to the actual theoretical values.
// FIXME: SubsetOf should be removed when specialization will be supported by rustc. This will
// allow a blanket impl: impl<T: Clone> SubsetOf<T> for T { ... }
// NOTE: make all types debuggable/'static/Any ? This seems essential for any kind of generic programming.
pub trait CloneReal:
    SubsetOf<Self>
    + SupersetOf<f64>
    + Field
    + Clone
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
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
    fn trunc(&self) -> Self;
    fn fract(&self) -> Self;
    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
    fn is_sign_positive(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
    fn mul_add(&self, a: &Self, b: &Self) -> Self;
    fn recip(&self) -> Self;
    fn powi(&self, n: i32) -> Self;
    fn powf(&self, n: &Self) -> Self;
    fn sqrt(&self) -> Self;
    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn ln(&self) -> Self;
    fn log(&self, base: &Self) -> Self;
    fn log2(&self) -> Self;
    fn log10(&self) -> Self;
    fn max(&self, other: &Self) -> Self;
    fn min(&self, other: &Self) -> Self;
    fn cbrt(&self) -> Self;
    fn hypot(&self, other: &Self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn atan2(&self, other: &Self) -> Self;
    fn sin_cos(&self) -> (Self, Self);
    fn exp_m1(&self) -> Self;
    fn ln_1p(&self) -> Self;
    fn sinh(&self) -> Self;
    fn cosh(&self) -> Self;
    fn tanh(&self) -> Self;
    fn asinh(&self) -> Self;
    fn acosh(&self) -> Self;
    fn atanh(&self) -> Self;

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

#[allow(missing_docs)]

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
    ($($T:ty, $M:ident, $libm: ident);*) => ($(
        impl Real for $T {
            #[inline]
            fn floor(self) -> Self {
                $libm::floor(self)
            }

            #[inline]
            fn ceil(self) -> Self {
                $libm::ceil(self)
            }

            #[inline]
            fn round(self) -> Self {
                $libm::round(self)
            }

            #[inline]
            fn trunc(self) -> Self {
                $libm::trunc(self)
            }

            #[inline]
            fn fract(self) -> Self {
                $libm::fract(self)
            }

            #[inline]
            fn abs(self) -> Self {
                $libm::abs(self)
            }

            #[inline]
            fn signum(self) -> Self {
                Signed::signum(&self)
            }

            #[inline]
            fn is_sign_positive(self) -> bool {
                $M::is_sign_positive(self)
            }

            #[inline]
            fn is_sign_negative(self) -> bool {
                $M::is_sign_negative(self)
            }

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                $libm::mul_add(self, a, b)
            }

            #[inline]
            fn recip(self) -> Self {
                $M::recip(self)
            }

            #[cfg(feature = "std")]
            #[inline]
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }

            #[cfg(not(feature = "std"))]
            #[inline]
            fn powi(self, n: i32) -> Self {
                // FIXME: is there a more efficient solution?
                num::pow(self, n as usize)
            }

            #[inline]
            fn powf(self, n: Self) -> Self {
                $libm::powf(self, n)
            }

            #[inline]
            fn sqrt(self) -> Self {
                $libm::sqrt(self)
            }

            #[inline]
            fn exp(self) -> Self {
                $libm::exp(self)
            }

            #[inline]
            fn exp2(self) -> Self {
                $libm::exp2(self)
            }

            #[inline]
            fn ln(self) -> Self {
                $libm::ln(self)
            }

            #[inline]
            fn log(self, base: Self) -> Self {
                $libm::log(self, base)
            }

            #[inline]
            fn log2(self) -> Self {
                $libm::log2(self)
            }

            #[inline]
            fn log10(self) -> Self {
                $libm::log10(self)
            }

            #[inline]
            fn max(self, other: Self) -> Self {
                $M::max(self, other)
            }

            #[inline]
            fn min(self, other: Self) -> Self {
                $M::min(self, other)
            }

            #[inline]
            fn cbrt(self) -> Self {
                $libm::cbrt(self)
            }

            #[inline]
            fn hypot(self, other: Self) -> Self {
                $libm::hypot(self, other)
            }

            #[inline]
            fn sin(self) -> Self {
                $libm::sin(self)
            }

            #[inline]
            fn cos(self) -> Self {
                $libm::cos(self)
            }

            #[inline]
            fn tan(self) -> Self {
                $libm::tan(self)
            }

            #[inline]
            fn asin(self) -> Self {
                $libm::asin(self)
            }

            #[inline]
            fn acos(self) -> Self {
                $libm::acos(self)
            }

            #[inline]
            fn atan(self) -> Self {
                $libm::atan(self)
            }

            #[inline]
            fn atan2(self, other: Self) -> Self {
                $libm::atan2(self, other)
            }

            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                $libm::sin_cos(self)
            }

            #[inline]
            fn exp_m1(self) -> Self {
                $libm::exp_m1(self)
            }

            #[inline]
            fn ln_1p(self) -> Self {
                $libm::ln_1p(self)
            }

            #[inline]
            fn sinh(self) -> Self {
                $libm::sinh(self)
            }

            #[inline]
            fn cosh(self) -> Self {
                $libm::cosh(self)
            }

            #[inline]
            fn tanh(self) -> Self {
                $libm::tanh(self)
            }

            #[inline]
            fn asinh(self) -> Self {
                $libm::asinh(self)
            }

            #[inline]
            fn acosh(self) -> Self {
                $libm::acosh(self)
            }

            #[inline]
            fn atanh(self) -> Self {
                $libm::atanh(self)
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

#[cfg(not(feature = "std"))]
impl_real!(f32,f32,F32Ext; f64,f64,F64Ext);
#[cfg(feature = "std")]
impl_real!(f32,f32,f32; f64,f64,f64);
//#[cfg(feature = "decimal")]
//impl_real!(d128, d128, d128);

impl<T> CloneReal for T where T: Real {
    fn floor(&self) -> Self {Real::floor(*self)}
    fn ceil(&self) -> Self {Real::ceil(*self)}
    fn round(&self) -> Self {Real::round(*self)}
    fn trunc(&self) -> Self {Real::trunc(*self)}
    fn fract(&self) -> Self {Real::fract(*self)}
    fn abs(&self) -> Self {Real::abs(*self)}
    fn signum(&self) -> Self {Real::signum(*self)}
    fn is_sign_positive(&self) -> bool {Real::is_sign_positive(*self)}
    fn is_sign_negative(&self) -> bool {Real::is_sign_negative(*self)}
    fn mul_add(&self, a: &Self, b: &Self) -> Self {Real::mul_add(*self, *a, *b)}
    fn recip(&self) -> Self {Real::recip(*self)}
    fn powi(&self, n: i32) -> Self {Real::powi(*self, n)}
    fn powf(&self, n: &Self) -> Self {Real::powf(*self, *n)}
    fn sqrt(&self) -> Self {Real::sqrt(*self)}
    fn exp(&self) -> Self {Real::exp(*self)}
    fn exp2(&self) -> Self {Real::exp2(*self)}
    fn ln(&self) -> Self {Real::ln(*self)}
    fn log(&self, base: &Self) -> Self {Real::log(*self, *base)}
    fn log2(&self) -> Self {Real::log2(*self)}
    fn log10(&self) -> Self {Real::log10(*self)}
    fn max(&self, other: &Self) -> Self {Real::max(*self, *other)}
    fn min(&self, other: &Self) -> Self {Real::min(*self, *other)}
    fn cbrt(&self) -> Self {Real::cbrt(*self)}
    fn hypot(&self, other: &Self) -> Self {Real::hypot(*self, *other)}
    fn sin(&self) -> Self {Real::sin(*self)}
    fn cos(&self) -> Self {Real::cos(*self)}
    fn tan(&self) -> Self {Real::tan(*self)}
    fn asin(&self) -> Self {Real::asin(*self)}
    fn acos(&self) -> Self {Real::acos(*self)}
    fn atan(&self) -> Self {Real::atan(*self)}
    fn atan2(&self, other: &Self) -> Self {Real::atan2(*self, *other)}
    fn sin_cos(&self) -> (Self, Self) {Real::sin_cos(*self)}
    fn exp_m1(&self) -> Self {Real::exp_m1(*self)}
    fn ln_1p(&self) -> Self {Real::ln_1p(*self)}
    fn sinh(&self) -> Self {Real::sinh(*self)}
    fn cosh(&self) -> Self {Real::cosh(*self)}
    fn tanh(&self) -> Self {Real::tanh(*self)}
    fn asinh(&self) -> Self {Real::asinh(*self)}
    fn acosh(&self) -> Self {Real::acosh(*self)}
    fn atanh(&self) -> Self {Real::atanh(*self)}

    fn pi() -> Self {Real::pi()}
    fn two_pi() -> Self {Real::two_pi()}
    fn frac_pi_2() -> Self {Real::frac_pi_2()}
    fn frac_pi_3() -> Self {Real::frac_pi_3()}
    fn frac_pi_4() -> Self {Real::frac_pi_4()}
    fn frac_pi_6() -> Self {Real::frac_pi_6()}
    fn frac_pi_8() -> Self {Real::frac_pi_8()}
    fn frac_1_pi() -> Self {Real::frac_1_pi()}
    fn frac_2_pi() -> Self {Real::frac_2_pi()}
    fn frac_2_sqrt_pi() -> Self {Real::frac_2_sqrt_pi()}

    fn e() -> Self {Real::e()}
    fn log2_e() -> Self {Real::log2_e()}
    fn log10_e() -> Self {Real::log10_e()}
    fn ln_2() -> Self {Real::ln_2()}
    fn ln_10() -> Self {Real::ln_10()}
}
