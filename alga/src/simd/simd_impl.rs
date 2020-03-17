#![allow(missing_docs)]
#![allow(non_camel_case_types)] // For the simd type aliases.
//! Traits for SIMD values.

use crate::general::*;
use crate::simd::{
    SimdBool, SimdComplexField, SimdPartialOrd, SimdRealField, SimdSigned, SimdValue,
};
#[cfg(feature = "decimal")]
use decimal::d128;
use num::{FromPrimitive, Num, One, Zero};
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

/// An Simd structure that implements all the relevant traits from `alga`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Simd<N: SimdValue>(pub N);

impl<N: SimdValue> fmt::Display for Simd<N>
where
    N::Element: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if N::lanes() == 1 {
            return self.extract(0).fmt(f);
        }

        write!(f, "({}", self.extract(0))?;

        for i in 1..N::lanes() {
            write!(f, ", {}", self.extract(i))?;
        }

        write!(f, ")")
    }
}

impl<N: SimdValue> SimdValue for Simd<N> {
    type Element = N::Element;

    #[inline(always)]
    fn lanes() -> usize {
        N::lanes()
    }

    #[inline(always)]
    fn splat(val: Self::Element) -> Self {
        Simd(N::splat(val))
    }

    #[inline(always)]
    fn extract(self, i: usize) -> Self::Element {
        self.0.extract(i)
    }

    #[inline(always)]
    unsafe fn extract_unchecked(self, i: usize) -> Self::Element {
        self.0.extract_unchecked(i)
    }

    #[inline(always)]
    fn replace(self, i: usize, val: Self::Element) -> Self {
        Simd(self.0.replace(i, val))
    }

    #[inline(always)]
    unsafe fn replace_unchecked(self, i: usize, val: Self::Element) -> Self {
        Simd(self.0.replace_unchecked(i, val))
    }
}

macro_rules! impl_simd_value_for_scalar(
    ($($t: ty),*) => {$(
        impl SimdValue for $t {
            type Element = $t;

            #[inline(always)]
            fn lanes() -> usize {
                1
            }

            #[inline(always)]
            fn splat(val: Self::Element) -> Self {
                val
            }

            #[inline(always)]
            fn extract(self, _: usize) -> Self::Element {
                self
            }

            #[inline(always)]
            unsafe fn extract_unchecked(self, _: usize) -> Self::Element {
                self
            }

            #[inline(always)]
            fn replace(self, _: usize, val: Self::Element) -> Self {
                val
            }

            #[inline(always)]
            unsafe fn replace_unchecked(self, _: usize, val: Self::Element) -> Self {
                val
            }
        }
    )*}
);

impl_simd_value_for_scalar!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
#[cfg(feature = "decimal")]
impl_simd_value_for_scalar!(d128);

macro_rules! impl_simd_bool(
    ($($t: ty;)*) => {$(
        impl SimdBool for $t {
            #[inline(always)]
            fn and(self) -> bool {
                self.and()
            }

            #[inline(always)]
            fn or(self) -> bool {
                self.or()
            }

            #[inline(always)]
            fn xor(self) -> bool {
                self.xor()
            }

            #[inline(always)]
            fn all(self) -> bool {
                self.all()
            }

            #[inline(always)]
            fn any(self) -> bool {
                self.any()
            }

            #[inline(always)]
            fn none(self) -> bool {
                self.none()
            }
        }
    )*}
);

macro_rules! impl_scalar_subset_of_simd(
    ($($t: ty),*) => {$(
        impl<N2: SimdValue> SubsetOf<Simd<N2>> for $t
            where N2::Element: SupersetOf<$t> + PartialEq {
            #[inline(always)]
            fn to_superset(&self) -> Simd<N2> {
                Simd(N2::splat(N2::Element::from_subset(self)))
            }

            #[inline(always)]
            unsafe fn from_superset_unchecked(element: &Simd<N2>) -> $t {
                element.extract(0).to_subset_unchecked()
            }

            #[inline(always)]
            fn is_in_subset(c: &Simd<N2>) -> bool {
                let elt0 = c.extract(0);
                elt0.is_in_subset() &&
                (1..N2::lanes()).all(|i| c.extract(i) == elt0)
            }
        }
    )*}
);

impl_scalar_subset_of_simd!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
#[cfg(feature = "decimal")]
impl_scalar_subset_of_simd!(d128);

macro_rules! impl_simd_value(
    ($($t: ty, $elt: ty;)*) => ($(
        impl SimdValue for $t {
            type Element = $elt;

            #[inline(always)]
            fn lanes() -> usize {
                <$t>::lanes()
            }

            #[inline(always)]
            fn splat(val: Self::Element) -> Self {
                <$t>::splat(val)
            }

            #[inline(always)]
            fn extract(self, i: usize) -> Self::Element {
                self.extract(i)
            }

            #[inline(always)]
            unsafe fn extract_unchecked(self, i: usize) -> Self::Element {
                self.extract_unchecked(i)
            }

            #[inline(always)]
            fn replace(self, i: usize, val: Self::Element) -> Self {
                self.replace(i, val)
            }

            #[inline(always)]
            unsafe fn replace_unchecked(self, i: usize, val: Self::Element) -> Self {
                self.replace_unchecked(i, val)
            }
        }
    )*)
);

macro_rules! impl_uint_simd(
    ($($t: ty, $elt: ty, $bool: ty;)*) => ($(
        impl_simd_value!($t, $elt;);

        impl SubsetOf<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn to_superset(&self) -> Self {
                *self
            }

            #[inline(always)]
            fn from_superset(element: &Self) -> Option<Self> {
                Some(*element)
            }

            #[inline(always)]
            unsafe fn from_superset_unchecked(element: &Self) -> Self {
                *element
            }

            #[inline(always)]
            fn is_in_subset(_: &Self) -> bool {
                true
            }
        }

        impl Num for Simd<$t> {
            type FromStrRadixErr = <$elt as Num>::FromStrRadixErr;

            #[inline(always)]
            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                <$elt>::from_str_radix(str, radix).map(Self::splat)
            }
        }

        impl FromPrimitive for Simd<$t> {
            #[inline(always)]
            fn from_i64(n: i64) -> Option<Self> {
                <$elt>::from_i64(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u64(n: u64) -> Option<Self> {
                <$elt>::from_u64(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_isize(n: isize) -> Option<Self>  {
                <$elt>::from_isize(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i8(n: i8) -> Option<Self>  {
                <$elt>::from_i8(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i16(n: i16) -> Option<Self>  {
                <$elt>::from_i16(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_i32(n: i32) -> Option<Self>  {
                <$elt>::from_i32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_usize(n: usize) -> Option<Self>  {
                <$elt>::from_usize(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u8(n: u8) -> Option<Self>  {
                <$elt>::from_u8(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u16(n: u16) -> Option<Self>  {
                <$elt>::from_u16(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_u32(n: u32) -> Option<Self>  {
                <$elt>::from_u32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_f32(n: f32) -> Option<Self>  {
                <$elt>::from_f32(n).map(Self::splat)
            }

            #[inline(always)]
            fn from_f64(n: f64) -> Option<Self>  {
                <$elt>::from_f64(n).map(Self::splat)
            }
        }


        impl Zero for Simd<$t> {
            #[inline(always)]
            fn zero() -> Self {
                Simd(<$t>::splat(<$elt>::zero()))
            }

            #[inline(always)]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }

        impl One for Simd<$t> {
            #[inline(always)]
            fn one() -> Self {
                Simd(<$t>::splat(<$elt>::one()))
            }
        }

        impl Add<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl Sub<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl Mul<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                Self(self.0 * rhs.0)
            }
        }

        impl Div<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                Self(self.0 / rhs.0)
            }
        }

        impl Rem<Simd<$t>> for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn rem(self, rhs: Self) -> Self {
                Self(self.0 % rhs.0)
            }
        }

        impl AddAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl SubAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl DivAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0
            }
        }

        impl MulAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0
            }
        }

        impl RemAssign<Simd<$t>> for Simd<$t> {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0
            }
        }

        impl SimdPartialOrd for Simd<$t> {
            type SimdBool = $bool;

            #[inline(always)]
            fn simd_gt(self, other: Self) -> Self::SimdBool {
                self.0.gt(other.0)
            }

            #[inline(always)]
            fn simd_lt(self, other: Self) -> Self::SimdBool {
                self.0.lt(other.0)
            }

            #[inline(always)]
            fn simd_ge(self, other: Self) -> Self::SimdBool {
                self.0.ge(other.0)
            }

            #[inline(always)]
            fn simd_le(self, other: Self) -> Self::SimdBool {
                self.0.le(other.0)
            }

            #[inline(always)]
            fn simd_eq(self, other: Self) -> Self::SimdBool {
                self.0.eq(other.0)
            }

            #[inline(always)]
            fn simd_ne(self, other: Self) -> Self::SimdBool {
                self.0.ne(other.0)
            }

            #[inline(always)]
            fn simd_max(self, other: Self) -> Self {
                Simd(self.0.max(other.0))
            }
            #[inline(always)]
            fn simd_min(self, other: Self) -> Self {
                Simd(self.0.min(other.0))
            }

            #[inline(always)]
            fn simd_clamp(self, min: Self, max: Self) -> Self {
                self.simd_max(min).simd_min(max)
            }
        }

        impl MeetSemilattice for Simd<$t> {
            #[inline(always)]
            fn meet(&self, other: &Self) -> Self {
                Simd(self.0.min(other.0))
            }
        }

        impl JoinSemilattice for Simd<$t> {
            #[inline(always)]
            fn join(&self, other: &Self) -> Self {
                Simd(self.0.max(other.0))
            }
        }

        impl AbstractMagma<Additive> for Simd<$t> {
            #[inline(always)]
            fn operate(&self, right: &Self) -> Self {
                Simd(self.0 + right.0)
            }
        }

        impl AbstractMagma<Multiplicative> for Simd<$t> {
            #[inline(always)]
            fn operate(&self, right: &Self) -> Self {
                Simd(self.0 * right.0)
            }
        }

        impl AbstractSemigroup<Additive> for Simd<$t> {}
        impl AbstractSemigroup<Multiplicative> for Simd<$t> {}

        impl Identity<Additive> for Simd<$t> {
            #[inline(always)]
            fn identity() -> Self {
                Self::splat(<$elt>::zero())
            }
        }

        impl Identity<Multiplicative> for Simd<$t> {
            #[inline(always)]
            fn identity() -> Self {
                Self::splat(<$elt>::one())
            }
        }

        impl AbstractMonoid<Additive> for Simd<$t> {}
        impl AbstractMonoid<Multiplicative> for Simd<$t> {}
    )*)
);

macro_rules! impl_int_simd(
    ($($t: ty, $elt: ty, $bool: ty;)*) => ($(
        impl_uint_simd!($t, $elt, $bool;);

        impl Neg for Simd<$t> {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl TwoSidedInverse<Additive> for Simd<$t> {
            #[inline(always)]
            fn two_sided_inverse(&self) -> Self {
                Simd(-self.0)
            }
        }

        impl AbstractQuasigroup<Additive> for Simd<$t> {}
        impl AbstractLoop<Additive> for Simd<$t> {}
        impl AbstractGroup<Additive> for Simd<$t> {}
        impl AbstractGroupAbelian<Additive> for Simd<$t> {}

        impl AbstractRing<Additive, Multiplicative> for Simd<$t> {}
        impl AbstractRingCommutative<Additive, Multiplicative> for Simd<$t> {}
        impl AbstractModule<Additive, Additive, Multiplicative> for Simd<$t> {
            type AbstractRing = Simd<$t>;

            #[inline(always)]
            fn multiply_by(&self, r: Self) -> Self {
                Simd(self.0 * r.0)
            }
        }

        impl Module for Simd<$t> {
            type Ring = Self;
        }
    )*)
);

macro_rules! impl_float_simd(
    ($($t: ty, $elt: ty, $bool: ty;)*) => ($(
        impl_int_simd!($t, $elt, $bool;);

        // FIXME: this should be part of impl_int_simd
        // but those methods do not seem to be implemented
        // by packed_simd for integers.
        impl SimdSigned for Simd<$t> {
            type SimdBool = $bool;

            #[inline(always)]
            fn simd_abs(&self) -> Self {
                Simd(self.0.abs())
            }

            #[inline(always)]
            fn simd_abs_sub(&self, other: &Self) -> Self {
                Simd((self.0 - other.0).max(Self::zero().0))
            }

            #[inline(always)]
            fn simd_signum(&self) -> Self {
                // NOTE: is there a more efficient way of doing this?
                let zero = Self::zero().0;
                let one = Self::one().0;
                let gt = self.0.gt(zero);
                let lt = self.0.lt(zero);
                Simd(lt.select(-one, gt.select(one, zero)))
            }

            #[inline(always)]
            fn is_simd_positive(&self) -> Self::SimdBool {
                self.simd_gt(Self::zero())
            }

            #[inline(always)]
            fn is_simd_negative(&self) -> Self::SimdBool {
                self.simd_lt(Self::zero())
            }
        }

        impl TwoSidedInverse<Multiplicative> for Simd<$t> {
            #[inline(always)]
            fn two_sided_inverse(&self) -> Self {
                Self::splat(<$elt>::one()) / *self
            }
        }

        impl AbstractQuasigroup<Multiplicative> for Simd<$t> {}
        impl AbstractLoop<Multiplicative> for Simd<$t> {}
        impl AbstractGroup<Multiplicative> for Simd<$t> {}
        impl AbstractGroupAbelian<Multiplicative> for Simd<$t> {}
        impl AbstractField<Additive, Multiplicative> for Simd<$t> {}

        impl SimdRealField for Simd<$t> {
            #[inline(always)]
            fn simd_atan2(self, other: Self) -> Self {
                self.zip_map(other, |a, b| a.atan2(b))
            }

            #[inline(always)]
            fn simd_pi() -> Self {
                Simd(<$t>::PI)
            }

            #[inline(always)]
            fn simd_two_pi() -> Self {
                Simd(<$t>::PI + <$t>::PI)
            }

            #[inline(always)]
            fn simd_frac_pi_2() -> Self {
                Simd(<$t>::FRAC_PI_2)
            }

            #[inline(always)]
            fn simd_frac_pi_3() -> Self {
                Simd(<$t>::FRAC_PI_3)
            }

            #[inline(always)]
            fn simd_frac_pi_4() -> Self {
                Simd(<$t>::FRAC_PI_4)
            }

            #[inline(always)]
            fn simd_frac_pi_6() -> Self {
                Simd(<$t>::FRAC_PI_6)
            }

            #[inline(always)]
            fn simd_frac_pi_8() -> Self {
                Simd(<$t>::FRAC_PI_8)
            }

            #[inline(always)]
            fn simd_frac_1_pi() -> Self {
                Simd(<$t>::FRAC_1_PI)
            }

            #[inline(always)]
            fn simd_frac_2_pi() -> Self {
                Simd(<$t>::FRAC_2_PI)
            }

            #[inline(always)]
            fn simd_frac_2_sqrt_pi() -> Self {
                Simd(<$t>::FRAC_2_SQRT_PI)
            }


            #[inline(always)]
            fn simd_e() -> Self {
                Simd(<$t>::E)
            }

            #[inline(always)]
            fn simd_log2_e() -> Self {
                Simd(<$t>::LOG2_E)
            }

            #[inline(always)]
            fn simd_log10_e() -> Self {
                Simd(<$t>::LOG10_E)
            }

            #[inline(always)]
            fn simd_ln_2() -> Self {
                Simd(<$t>::LN_2)
            }

            #[inline(always)]
            fn simd_ln_10() -> Self {
                Simd(<$t>::LN_10)
            }
        }

        impl SimdComplexField for Simd<$t> {
            type SimdRealField = Self;

            #[inline(always)]
            fn from_simd_real(re: Self::SimdRealField) -> Self {
                re
            }

            #[inline(always)]
            fn simd_real(self) -> Self::SimdRealField {
                self
            }

            #[inline(always)]
            fn simd_imaginary(self) -> Self::SimdRealField {
                Self::zero()
            }

            #[inline(always)]
            fn simd_norm1(self) -> Self::SimdRealField {
                Simd(self.0.abs())
            }

            #[inline(always)]
            fn simd_modulus(self) -> Self::SimdRealField {
                Simd(self.0.abs())
            }

            #[inline(always)]
            fn simd_modulus_squared(self) -> Self::SimdRealField {
                self * self
            }

            #[inline(always)]
            fn simd_argument(self) -> Self::SimdRealField {
                self.map(|e| e.argument())
            }

            #[inline(always)]
            fn simd_to_exp(self) -> (Self, Self) {
                let ge = self.0.ge(Self::one().0);
                let exp = ge.select(Self::one().0, -Self::one().0);
                (Simd(self.0 * exp), Simd(exp))
            }

            #[inline(always)]
            fn simd_recip(self) -> Self {
                Self::one() / self
            }

            #[inline(always)]
            fn simd_conjugate(self) -> Self {
                self
            }

            #[inline(always)]
            fn simd_scale(self, factor: Self::SimdRealField) -> Self {
                Simd(self.0 * factor.0)
            }

            #[inline(always)]
            fn simd_unscale(self, factor: Self::SimdRealField) -> Self {
                Simd(self.0 / factor.0)
            }

            #[inline(always)]
            fn simd_floor(self) -> Self {
                Simd(self.0.map(|e| e.floor()))
            }

            #[inline(always)]
            fn simd_ceil(self) -> Self {
                Simd(self.0.map(|e| e.ceil()))
            }

            #[inline(always)]
            fn simd_round(self) -> Self {
                Simd(self.0.map(|e| e.round()))
            }

            #[inline(always)]
            fn simd_trunc(self) -> Self {
                Simd(self.0.map(|e| e.trunc()))
            }

            #[inline(always)]
            fn simd_fract(self) -> Self {
                Simd(self.0.map(|e| e.fract()))
            }

            #[inline(always)]
            fn simd_abs(self) -> Self {
                Simd(self.0.abs())
            }

            #[inline(always)]
            fn simd_signum(self) -> Self {
                Simd(self.0.map(|e| e.signum()))
            }

            #[inline(always)]
            fn simd_mul_add(self, a: Self, b: Self) -> Self {
                Simd(self.0.mul_add(a.0, b.0))
            }

            #[inline(always)]
            fn simd_powi(self, n: i32) -> Self {
               Simd(self.0.powf(<$t>::splat(n as $elt)))
            }

            #[inline(always)]
            fn simd_powf(self, n: Self) -> Self {
                Simd(self.0.powf(n.0))
            }

            #[inline(always)]
            fn simd_powc(self, n: Self) -> Self {
               Simd(self.0.powf(n.0))
            }

            #[inline(always)]
            fn simd_sqrt(self) -> Self {
                Simd(self.0.sqrt())
            }

            #[inline(always)]
            fn simd_exp(self) -> Self {
                Simd(self.0.exp())
            }

            #[inline(always)]
            fn simd_exp2(self) -> Self {
                Simd(self.0.map(|e| e.exp2()))
            }


            #[inline(always)]
            fn simd_exp_m1(self) -> Self {
                Simd(self.0.map(|e| e.exp_m1()))
            }

            #[inline(always)]
            fn simd_ln_1p(self) -> Self {
                Simd(self.0.map(|e| e.ln_1p()))
            }

            #[inline(always)]
            fn simd_ln(self) -> Self {
                Simd(self.0.ln())
            }

            #[inline(always)]
            fn simd_log(self, base: Self) -> Self {
                Simd(self.0.zip_map(base.0, |e, b| e.log(b)))
            }

            #[inline(always)]
            fn simd_log2(self) -> Self {
                Simd(self.0.map(|e| e.log2()))
            }

            #[inline(always)]
            fn simd_log10(self) -> Self {
                Simd(self.0.map(|e| e.log10()))
            }

            #[inline(always)]
            fn simd_cbrt(self) -> Self {
                Simd(self.0.map(|e| e.cbrt()))
            }

            #[inline(always)]
            fn simd_hypot(self, other: Self) -> Self::SimdRealField {
                Simd(self.0.zip_map(other.0, |e, o| e.hypot(o)))
            }

            #[inline(always)]
            fn simd_sin(self) -> Self {
                Simd(self.0.sin())
            }

            #[inline(always)]
            fn simd_cos(self) -> Self {
                Simd(self.0.cos())
            }

            #[inline(always)]
            fn simd_tan(self) -> Self {
                Simd(self.0.map(|e| e.tan()))
            }

            #[inline(always)]
            fn simd_asin(self) -> Self {
                Simd(self.0.map(|e| e.asin()))
            }

            #[inline(always)]
            fn simd_acos(self) -> Self {
                Simd(self.0.map(|e| e.acos()))
            }

            #[inline(always)]
            fn simd_atan(self) -> Self {
                Simd(self.0.map(|e| e.atan()))
            }

            #[inline(always)]
            fn simd_sin_cos(self) -> (Self, Self) {
                (self.simd_sin(), self.simd_cos())
            }

//            #[inline(always]
//            fn simd_exp_m1(self) -> Self {
//                $libm::exp_m1(self)
//            }
//
//            #[inline(always]
//            fn simd_ln_1p(self) -> Self {
//                $libm::ln_1p(self)
//            }
//
            #[inline(always)]
            fn simd_sinh(self) -> Self {
                Simd(self.0.map(|e| e.sinh()))
            }

            #[inline(always)]
            fn simd_cosh(self) -> Self {
                Simd(self.0.map(|e| e.cosh()))
            }

            #[inline(always)]
            fn simd_tanh(self) -> Self {
                Simd(self.0.map(|e| e.tanh()))
            }

            #[inline(always)]
            fn simd_asinh(self) -> Self {
                Simd(self.0.map(|e| e.asinh()))
            }

            #[inline(always)]
            fn simd_acosh(self) -> Self {
                Simd(self.0.map(|e| e.acosh()))
            }

            #[inline(always)]
            fn simd_atanh(self) -> Self {
                Simd(self.0.map(|e| e.atanh()))
            }
        }
    )*)
);

impl_float_simd!(
    packed_simd::f32x2, f32, packed_simd::m32x2;
    packed_simd::f32x4, f32, packed_simd::m32x4;
    packed_simd::f32x8, f32, packed_simd::m32x8;
    packed_simd::f32x16, f32, packed_simd::m32x16;
    packed_simd::f64x2, f64, packed_simd::m64x2;
    packed_simd::f64x4, f64, packed_simd::m64x4;
    packed_simd::f64x8, f64, packed_simd::m64x8;
);

impl_int_simd!(
    packed_simd::i128x1, i128, m128x1;
    packed_simd::i128x2, i128, m128x2;
    packed_simd::i128x4, i128, m128x4;
    packed_simd::i16x2, i16, m16x2;
    packed_simd::i16x4, i16, m16x4;
    packed_simd::i16x8, i16, m16x8;
    packed_simd::i16x16, i16, m16x16;
    packed_simd::i16x32, i16, m16x32;
    packed_simd::i32x2, i32, m32x2;
    packed_simd::i32x4, i32, m32x4;
    packed_simd::i32x8, i32, m32x8;
    packed_simd::i32x16, i32, m32x16;
    packed_simd::i64x2, i64, m64x2;
    packed_simd::i64x4, i64, m64x4;
    packed_simd::i64x8, i64, m64x8;
    packed_simd::i8x2, i8, m8x2;
    packed_simd::i8x4, i8, m8x4;
    packed_simd::i8x8, i8, m8x8;
    packed_simd::i8x16, i8, m8x16;
    packed_simd::i8x32, i8, m8x32;
    packed_simd::i8x64, i8, m8x64;
    packed_simd::isizex2, isize, msizex2;
    packed_simd::isizex4, isize, msizex4;
    packed_simd::isizex8, isize, msizex8;
);

impl_uint_simd!(
    packed_simd::u128x1, u128, m128x1;
    packed_simd::u128x2, u128, m128x2;
    packed_simd::u128x4, u128, m128x4;
    packed_simd::u16x2, u16, m16x2;
    packed_simd::u16x4, u16, m16x4;
    packed_simd::u16x8, u16, m16x8;
    packed_simd::u16x16, u16, m16x16;
    packed_simd::u16x32, u16, m16x32;
    packed_simd::u32x2, u32, m32x2;
    packed_simd::u32x4, u32, m32x4;
    packed_simd::u32x8, u32, m32x8;
    packed_simd::u32x16, u32, m32x16;
    packed_simd::u64x2, u64, m64x2;
    packed_simd::u64x4, u64, m64x4;
    packed_simd::u64x8, u64, m64x8;
    packed_simd::u8x2, u8, m8x2;
    packed_simd::u8x4, u8, m8x4;
    packed_simd::u8x8, u8, m8x8;
    packed_simd::u8x16, u8, m8x16;
    packed_simd::u8x32, u8, m8x32;
    packed_simd::u8x64, u8, m8x64;
    packed_simd::usizex2, usize, msizex2;
    packed_simd::usizex4, usize, msizex4;
    packed_simd::usizex8, usize, msizex8;
);

impl_simd_value!(
    packed_simd::m128x1, bool;
    packed_simd::m128x2, bool;
    packed_simd::m128x4, bool;
    packed_simd::m16x2, bool;
    packed_simd::m16x4, bool;
    packed_simd::m16x8, bool;
    packed_simd::m16x16, bool;
    packed_simd::m16x32, bool;
    packed_simd::m32x2, bool;
    packed_simd::m32x4, bool;
    packed_simd::m32x8, bool;
    packed_simd::m32x16, bool;
    packed_simd::m64x2, bool;
    packed_simd::m64x4, bool;
    packed_simd::m64x8, bool;
    packed_simd::m8x2, bool;
    packed_simd::m8x4, bool;
    packed_simd::m8x8, bool;
    packed_simd::m8x16, bool;
    packed_simd::m8x32, bool;
    packed_simd::m8x64, bool;
    packed_simd::msizex2, bool;
    packed_simd::msizex4, bool;
    packed_simd::msizex8, bool;
);

impl_simd_bool!(
    packed_simd::m128x1;
    packed_simd::m128x2;
    packed_simd::m128x4;
    packed_simd::m16x2;
    packed_simd::m16x4;
    packed_simd::m16x8;
    packed_simd::m16x16;
    packed_simd::m16x32;
    packed_simd::m32x2;
    packed_simd::m32x4;
    packed_simd::m32x8;
    packed_simd::m32x16;
    packed_simd::m64x2;
    packed_simd::m64x4;
    packed_simd::m64x8;
    packed_simd::m8x2;
    packed_simd::m8x4;
    packed_simd::m8x8;
    packed_simd::m8x16;
    packed_simd::m8x32;
    packed_simd::m8x64;
    packed_simd::msizex2;
    packed_simd::msizex4;
    packed_simd::msizex8;
);

///////////////////////////////////

pub type f32x2 = Simd<packed_simd::f32x2>;
pub type f32x4 = Simd<packed_simd::f32x4>;
pub type f32x8 = Simd<packed_simd::f32x8>;
pub type f32x16 = Simd<packed_simd::f32x16>;
pub type f64x2 = Simd<packed_simd::f64x2>;
pub type f64x4 = Simd<packed_simd::f64x4>;
pub type f64x8 = Simd<packed_simd::f64x8>;
pub type i128x1 = Simd<packed_simd::i128x1>;
pub type i128x2 = Simd<packed_simd::i128x2>;
pub type i128x4 = Simd<packed_simd::i128x4>;
pub type i16x2 = Simd<packed_simd::i16x2>;
pub type i16x4 = Simd<packed_simd::i16x4>;
pub type i16x8 = Simd<packed_simd::i16x8>;
pub type i16x16 = Simd<packed_simd::i16x16>;
pub type i16x32 = Simd<packed_simd::i16x32>;
pub type i32x2 = Simd<packed_simd::i32x2>;
pub type i32x4 = Simd<packed_simd::i32x4>;
pub type i32x8 = Simd<packed_simd::i32x8>;
pub type i32x16 = Simd<packed_simd::i32x16>;
pub type i64x2 = Simd<packed_simd::i64x2>;
pub type i64x4 = Simd<packed_simd::i64x4>;
pub type i64x8 = Simd<packed_simd::i64x8>;
pub type i8x2 = Simd<packed_simd::i8x2>;
pub type i8x4 = Simd<packed_simd::i8x4>;
pub type i8x8 = Simd<packed_simd::i8x8>;
pub type i8x16 = Simd<packed_simd::i8x16>;
pub type i8x32 = Simd<packed_simd::i8x32>;
pub type i8x64 = Simd<packed_simd::i8x64>;
pub type isizex2 = Simd<packed_simd::isizex2>;
pub type isizex4 = Simd<packed_simd::isizex4>;
pub type isizex8 = Simd<packed_simd::isizex8>;
pub type u128x1 = Simd<packed_simd::u128x1>;
pub type u128x2 = Simd<packed_simd::u128x2>;
pub type u128x4 = Simd<packed_simd::u128x4>;
pub type u16x2 = Simd<packed_simd::u16x2>;
pub type u16x4 = Simd<packed_simd::u16x4>;
pub type u16x8 = Simd<packed_simd::u16x8>;
pub type u16x16 = Simd<packed_simd::u16x16>;
pub type u16x32 = Simd<packed_simd::u16x32>;
pub type u32x2 = Simd<packed_simd::u32x2>;
pub type u32x4 = Simd<packed_simd::u32x4>;
pub type u32x8 = Simd<packed_simd::u32x8>;
pub type u32x16 = Simd<packed_simd::u32x16>;
pub type u64x2 = Simd<packed_simd::u64x2>;
pub type u64x4 = Simd<packed_simd::u64x4>;
pub type u64x8 = Simd<packed_simd::u64x8>;
pub type u8x2 = Simd<packed_simd::u8x2>;
pub type u8x4 = Simd<packed_simd::u8x4>;
pub type u8x8 = Simd<packed_simd::u8x8>;
pub type u8x16 = Simd<packed_simd::u8x16>;
pub type u8x32 = Simd<packed_simd::u8x32>;
pub type u8x64 = Simd<packed_simd::u8x64>;
pub type usizex2 = Simd<packed_simd::usizex2>;
pub type usizex4 = Simd<packed_simd::usizex4>;
pub type usizex8 = Simd<packed_simd::usizex8>;

pub use packed_simd::m128x1;
pub use packed_simd::m128x2;
pub use packed_simd::m128x4;
pub use packed_simd::m16x16;
pub use packed_simd::m16x2;
pub use packed_simd::m16x32;
pub use packed_simd::m16x4;
pub use packed_simd::m16x8;
pub use packed_simd::m32x16;
pub use packed_simd::m32x2;
pub use packed_simd::m32x4;
pub use packed_simd::m32x8;
pub use packed_simd::m64x2;
pub use packed_simd::m64x4;
pub use packed_simd::m64x8;
pub use packed_simd::m8x16;
pub use packed_simd::m8x2;
pub use packed_simd::m8x32;
pub use packed_simd::m8x4;
pub use packed_simd::m8x64;
pub use packed_simd::m8x8;
pub use packed_simd::msizex2;
pub use packed_simd::msizex4;
pub use packed_simd::msizex8;
