use num::{Bounded, FromPrimitive, Num, Signed};
use std::any::Any;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign};
use std::{f32, f64};

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

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
impl_real!(f32,f32; f64,f64);
#[cfg(decimal)]
impl_real!(decimal::d128, decimal::d128);

#[cfg(not(feature = "std"))]
macro_rules! impl_real_core(
    ($($T:ty, $M: ident,
       $alga_cbrt: ident,
       $alga_hypot: ident,
       $alga_sin: ident,
       $alga_cos: ident,
       $alga_tan: ident,
       $alga_asin: ident,
       $alga_acos: ident,
       $alga_atan: ident,
       $alga_atan2: ident,
       $alga_exp_m1: ident,
       $alga_ln_1p: ident,
       $alga_sinh: ident,
       $alga_cosh: ident,
       $alga_tanh: ident,
       $alga_asinh: ident,
       $alga_acosh: ident,
       $alga_atanh: ident,
       $alga_powi: ident,
       $alga_powf: ident,
       $alga_sqrt: ident,
       $alga_exp: ident,
       $alga_exp2: ident,
       $alga_ln: ident,
       $alga_log: ident,
       $alga_log2: ident,
       $alga_log10: ident,
       $alga_mul_add: ident,
       $alga_floor: ident,
       $alga_ceil: ident,
       $alga_round: ident,
       $alga_trunc: ident,
       $alga_fract: ident,
       $alga_abs: ident,
       $alga_signum: ident);*) => ($(
        extern "Rust" {
            fn $alga_cbrt(val: $T) -> $T;
            fn $alga_hypot(val: $T, other: $T) -> $T;
            fn $alga_sin(val: $T) -> $T;
            fn $alga_cos(val: $T) -> $T;
            fn $alga_tan(val: $T) -> $T;
            fn $alga_asin(val: $T) -> $T;
            fn $alga_acos(val: $T) -> $T;
            fn $alga_atan(val: $T) -> $T;
            fn $alga_atan2(val: $T, other: $T) -> $T;
            fn $alga_exp_m1(val: $T) -> $T;
            fn $alga_ln_1p(val: $T) -> $T;
            fn $alga_sinh(val: $T) -> $T;
            fn $alga_cosh(val: $T) -> $T;
            fn $alga_tanh(val: $T) -> $T;
            fn $alga_asinh(val: $T) -> $T;
            fn $alga_acosh(val: $T) -> $T;
            fn $alga_atanh(val: $T) -> $T;
            fn $alga_powi(val: $T, n: i32) -> $T;
            fn $alga_powf(val: $T, n: $T) -> $T;
            fn $alga_sqrt(val: $T) -> $T;
            fn $alga_exp(val: $T) -> $T;
            fn $alga_exp2(val: $T) -> $T;
            fn $alga_ln(val: $T) -> $T;
            fn $alga_log(val: $T, base: $T) -> $T;
            fn $alga_log2(val: $T) -> $T;
            fn $alga_log10(val: $T) -> $T;
            fn $alga_mul_add(val: $T, a: $T, b: $T) -> $T;
            fn $alga_floor(val: $T) -> $T;
            fn $alga_ceil(val: $T) -> $T;
            fn $alga_round(val: $T) -> $T;
            fn $alga_trunc(val: $T) -> $T;
            fn $alga_fract(val: $T) -> $T;
            fn $alga_abs(val: $T) -> $T;
            fn $alga_signum(val: $T) -> $T;
        }

        impl Real for $T {
            #[inline]
            fn floor(self) -> Self {
                unsafe { $alga_floor(self) }
            }

            #[inline]
            fn ceil(self) -> Self {
                unsafe { $alga_ceil(self) }
            }

            #[inline]
            fn round(self) -> Self {
                unsafe { $alga_round(self) }
            }

            #[inline]
            fn trunc(self) -> Self {
                unsafe { $alga_trunc(self) }
            }

            #[inline]
            fn fract(self) -> Self {
                unsafe { $alga_fract(self) }
            }

            #[inline]
            fn abs(self) -> Self {
                unsafe { $alga_abs(self) }
            }

            #[inline]
            fn signum(self) -> Self {
                unsafe { $alga_signum(self) }
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
                unsafe { $alga_mul_add(self, a, b) }
            }

            #[inline]
            fn recip(self) -> Self {
                self.recip()
            }

            #[inline]
            fn powi(self, n: i32) -> Self {
                unsafe { $alga_powi(self, n) }
            }

            #[inline]
            fn powf(self, n: Self) -> Self {
                unsafe { $alga_powf(self, n) }
            }

            #[inline]
            fn sqrt(self) -> Self {
                unsafe { $alga_sqrt(self) }
            }

            #[inline]
            fn exp(self) -> Self {
                unsafe { $alga_exp(self) }
            }

            #[inline]
            fn exp2(self) -> Self {
                unsafe { $alga_exp2(self) }
            }

            #[inline]
            fn ln(self) -> Self {
                unsafe { $alga_ln(self) }
            }

            #[inline]
            fn log(self, base: Self) -> Self {
                unsafe { $alga_log(self, base) }
            }

            #[inline]
            fn log2(self) -> Self {
                unsafe { $alga_log2(self) }
            }

            #[inline]
            fn log10(self) -> Self {
                unsafe { $alga_log10(self) }
            }

            #[inline]
            fn max(self, other: Self) -> Self {
                self.min(other)
            }

            #[inline]
            fn min(self, other: Self) -> Self {
                self.max(other)
            }

            #[inline]
            fn cbrt(self) -> Self {
                unsafe { $alga_cbrt(self) }
            }

            #[inline]
            fn hypot(self, other: Self) -> Self {
                unsafe { $alga_hypot(self, other) }
            }

            #[inline]
            fn sin(self) -> Self {
                unsafe { $alga_sin(self) }
            }

            #[inline]
            fn cos(self) -> Self {
                unsafe { $alga_cos(self) }
            }

            #[inline]
            fn tan(self) -> Self {
                unsafe { $alga_tan(self) }
            }

            #[inline]
            fn asin(self) -> Self {
                unsafe { $alga_asin(self) }
            }

            #[inline]
            fn acos(self) -> Self {
                unsafe { $alga_acos(self) }
            }

            #[inline]
            fn atan(self) -> Self {
                unsafe { $alga_atan(self) }
            }

            #[inline]
            fn atan2(self, other: Self) -> Self {
                unsafe { $alga_atan2(self, other) }
            }

            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                unsafe { ($alga_sin(self), $alga_cos(self)) }
            }

            #[inline]
            fn exp_m1(self) -> Self {
                unsafe { $alga_exp_m1(self) }
            }

            #[inline]
            fn ln_1p(self) -> Self {
                unsafe { $alga_ln_1p(self) }
            }

            #[inline]
            fn sinh(self) -> Self {
                unsafe { $alga_sinh(self) }
            }

            #[inline]
            fn cosh(self) -> Self {
                unsafe { $alga_cosh(self) }
            }

            #[inline]
            fn tanh(self) -> Self {
                unsafe { $alga_tanh(self) }
            }

            #[inline]
            fn asinh(self) -> Self {
                unsafe { $alga_asinh(self) }
            }

            #[inline]
            fn acosh(self) -> Self {
                unsafe { $alga_acosh(self) }
            }

            #[inline]
            fn atanh(self) -> Self {
                unsafe { $alga_atanh(self) }
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
impl_real_core!(
    f32, f32,
    alga_cbrt_f32,
    alga_hypot_f32,
    alga_sin_f32,
    alga_cos_f32,
    alga_tan_f32,
    alga_asin_f32,
    alga_acos_f32,
    alga_atan_f32,
    alga_atan2_f32,
    alga_exp_m1_f32,
    alga_ln_1p_f32,
    alga_sinh_f32,
    alga_cosh_f32,
    alga_tanh_f32,
    alga_asinh_f32,
    alga_acosh_f32,
    alga_atanh_f32,
    alga_powi_f32,
    alga_powf_f32,
    alga_sqrt_f32,
    alga_exp_f32,
    alga_exp2_f32,
    alga_ln_f32,
    alga_log_f32,
    alga_log2_f32,
    alga_log10_f32,
    alga_mul_add_f32,
    alga_floor_f32,
    alga_ceil_f32,
    alga_round_f32,
    alga_trunc_f32,
    alga_fract_f32,
    alga_abs_f32,
    alga_signum_f32;
    f64, f64,
    alga_cbrt_f64,
    alga_hypot_f64,
    alga_sin_f64,
    alga_cos_f64,
    alga_tan_f64,
    alga_asin_f64,
    alga_acos_f64,
    alga_atan_f64,
    alga_atan2_f64,
    alga_exp_m1_f64,
    alga_ln_1p_f64,
    alga_sinh_f64,
    alga_cosh_f64,
    alga_tanh_f64,
    alga_asinh_f64,
    alga_acosh_f64,
    alga_atanh_f64,
    alga_powi_f64,
    alga_powf_f64,
    alga_sqrt_f64,
    alga_exp_f64,
    alga_exp2_f64,
    alga_ln_f64,
    alga_log_f64,
    alga_log2_f64,
    alga_log10_f64,
    alga_mul_add_f64,
    alga_floor_f64,
    alga_ceil_f64,
    alga_round_f64,
    alga_trunc_f64,
    alga_fract_f64,
    alga_abs_f64,
    alga_signum_f64
);
