use num::{Bounded, Signed};
use std::{f32, f64};

use approx::{RelativeEq, UlpsEq};

use crate::general::{ComplexField, Lattice, SimdFriendlyComplexField};
use crate::simd::{SimdBool, SimdValue};

#[cfg(not(feature = "std"))]
use num::Float;
//#[cfg(feature = "decimal")]
//use decimal::d128;

#[allow(missing_docs)]
pub trait SimdFriendlyRealField:
    SimdValue + SimdFriendlyComplexField<SimdRealField = Self> + Bounded
{
    type Bool: SimdBool;

    fn gt(self, other: Self) -> Self::Bool;
    fn lt(self, other: Self) -> Self::Bool;
    fn ge(self, other: Self) -> Self::Bool;
    fn le(self, other: Self) -> Self::Bool;
    fn eq(self, other: Self) -> Self::Bool;
    fn neq(self, other: Self) -> Self::Bool;

    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn atan2(self, other: Self) -> Self;

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

/// Trait shared by all reals.
///
/// Reals are equipped with functions that are commonly used on reals. The results of those
/// functions only have to be approximately equal to the actual theoretical values.
// FIXME: SubsetOf should be removed when specialization will be supported by rustc. This will
// allow a blanket impl: impl<T: Clone> SubsetOf<T> for T { ... }
// NOTE: make all types debuggable/'static/Any ? This seems essential for any kind of generic programming.
pub trait RealField:
    SimdFriendlyRealField<Bool = bool>
    + RelativeEq<Epsilon = Self>
    + Lattice
    + UlpsEq<Epsilon = Self>
    + Signed
{
    /// Is the sign of this real number positive?
    fn is_sign_positive(self) -> bool;
    /// Is the sign of this real number negative?
    fn is_sign_negative(self) -> bool;
}

macro_rules! impl_real(
    ($($T:ty, $M:ident, $libm: ident);*) => ($(
        impl RealField for $T {
            #[inline]
            fn is_sign_positive(self) -> bool {
                $M::is_sign_positive(self)
            }

            #[inline]
            fn is_sign_negative(self) -> bool {
                $M::is_sign_negative(self)
            }
        }

        impl SimdFriendlyRealField for $T {
            type Bool = bool;

            #[inline]
            fn gt(self, other: Self) -> Self::Bool {
                self > other
            }

            #[inline]
            fn lt(self, other: Self) -> Self::Bool {
                self < other
            }

            #[inline]
            fn ge(self, other: Self) -> Self::Bool {
                self >= other
            }

            #[inline]
            fn le(self, other: Self) -> Self::Bool {
                self <= other
            }

            #[inline]
            fn eq(self, other: Self) -> Self::Bool {
                self == other
            }

            #[inline]
            fn neq(self, other: Self) -> Self::Bool {
                self != other
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
            fn clamp(self, min: Self, max: Self) -> Self {
                if self < min {
                    min
                } else if self > max {
                    max
                } else {
                    self
                }
            }

            #[inline]
            fn atan2(self, other: Self) -> Self {
                $libm::atan2(self, other)
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
impl_real!(f32,f32,Float; f64,f64,Float);
#[cfg(feature = "std")]
impl_real!(f32,f32,f32; f64,f64,f64);
//#[cfg(feature = "decimal")]
//impl_real!(d128, d128, d128);
