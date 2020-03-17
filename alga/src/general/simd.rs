#![allow(missing_docs)]
//! Traits for SIMD values.

#[cfg(feature = "simd")]
use crate::general::*;
#[cfg(feature = "decimal")]
use decimal::d128;
#[cfg(feature = "simd")]
use num::{FromPrimitive, Num, One, Zero};
#[cfg(feature = "simd")]
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

/// An Simd structure that implements all the relevant traits from `alga`.
///
/// This is needed to overcome the orphan rules.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg(feature = "simd")]
pub struct Simd<N: SimdValue>(pub N);

#[cfg(feature = "simd")]
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

/// Trait implemented by Simd types as well as scalar types (f32, u32, etc.).
pub trait SimdValue: Copy {
    /// The type of the elements of each lane of this SIMD value.
    type Element: Copy;

    fn lanes() -> usize;
    fn splat(val: Self::Element) -> Self;
    fn extract(self, i: usize) -> Self::Element;
    unsafe fn extract_unchecked(self, i: usize) -> Self::Element;
    fn replace(self, i: usize, val: Self::Element) -> Self;
    unsafe fn replace_unchecked(self, i: usize, val: Self::Element) -> Self;

    #[inline(always)]
    fn map(self, f: impl Fn(Self::Element) -> Self::Element) -> Self {
        let mut result = self;

        for i in 0..Self::lanes() {
            unsafe { result = result.replace_unchecked(i, f(self.extract_unchecked(i))) }
        }

        result
    }

    #[inline(always)]
    fn zip_map(self, b: Self, f: impl Fn(Self::Element, Self::Element) -> Self::Element) -> Self {
        let mut result = self;

        for i in 0..Self::lanes() {
            unsafe {
                let a = self.extract_unchecked(i);
                let b = b.extract_unchecked(i);
                result = result.replace_unchecked(i, f(a, b))
            }
        }

        result
    }
}

#[cfg(feature = "simd")]
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

pub trait SimdBool: Copy {
    fn and(self) -> bool;
    fn or(self) -> bool;
    fn xor(self) -> bool;
    fn all(self) -> bool;
    fn any(self) -> bool;
    fn none(self) -> bool;
}

impl SimdBool for bool {
    #[inline(always)]
    fn and(self) -> bool {
        self
    }

    #[inline(always)]
    fn or(self) -> bool {
        self
    }

    #[inline(always)]
    fn xor(self) -> bool {
        self
    }

    #[inline(always)]
    fn all(self) -> bool {
        self
    }

    #[inline(always)]
    fn any(self) -> bool {
        self
    }

    #[inline(always)]
    fn none(self) -> bool {
        !self
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

#[cfg(feature = "simd")]
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

#[cfg(feature = "simd")]
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

#[cfg(feature = "simd")]
impl_scalar_subset_of_simd!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
#[cfg(all(feature = "decimal", feature = "simd"))]
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

#[cfg(feature = "simd")]
macro_rules! impl_uint_simd(
    ($($t: ty, $elt: ty;)*) => ($(
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

#[cfg(feature = "simd")]
macro_rules! impl_int_simd(
    ($($t: ty, $elt: ty;)*) => ($(
        impl_uint_simd!($t, $elt;);

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

#[cfg(feature = "simd")]
macro_rules! impl_float_simd(
    ($($t: ty, $elt: ty, $bool: ty;)*) => ($(
        impl_int_simd!($t, $elt;);

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
            fn simd_zero() -> Self {
                Self::splat(<$elt>::zero())
            }

            #[inline(always)]
            fn is_simd_zero(self) -> bool {
                self == Self::simd_zero()
            }

            #[inline(always)]
            fn simd_one() -> Self {
                Self::splat(<$elt>::one())
            }

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
                Self::simd_zero()
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
                Self::simd_one() / self
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

#[cfg(feature = "simd")]
impl_float_simd!(
    packed_simd::f32x2, f32, packed_simd::m32x2;
    packed_simd::f32x4, f32, packed_simd::m32x4;
    packed_simd::f32x8, f32, packed_simd::m32x8;
    packed_simd::f32x16, f32, packed_simd::m32x16;
    packed_simd::f64x2, f64, packed_simd::m64x2;
    packed_simd::f64x4, f64, packed_simd::m64x4;
    packed_simd::f64x8, f64, packed_simd::m64x8;
);

#[cfg(feature = "simd")]
impl_int_simd!(
    packed_simd::i128x1, i128;
    packed_simd::i128x2, i128;
    packed_simd::i128x4, i128;
    packed_simd::i16x2, i16;
    packed_simd::i16x4, i16;
    packed_simd::i16x8, i16;
    packed_simd::i16x16, i16;
    packed_simd::i16x32, i16;
    packed_simd::i32x2, i32;
    packed_simd::i32x4, i32;
    packed_simd::i32x8, i32;
    packed_simd::i32x16, i32;
    packed_simd::i64x2, i64;
    packed_simd::i64x4, i64;
    packed_simd::i64x8, i64;
    packed_simd::i8x2, i8;
    packed_simd::i8x4, i8;
    packed_simd::i8x8, i8;
    packed_simd::i8x16, i8;
    packed_simd::i8x32, i8;
    packed_simd::i8x64, i8;
    packed_simd::isizex2, isize;
    packed_simd::isizex4, isize;
    packed_simd::isizex8, isize;
);

#[cfg(feature = "simd")]
impl_uint_simd!(
    packed_simd::u128x1, u128;
    packed_simd::u128x2, u128;
    packed_simd::u128x4, u128;
    packed_simd::u16x2, u16;
    packed_simd::u16x4, u16;
    packed_simd::u16x8, u16;
    packed_simd::u16x16, u16;
    packed_simd::u16x32, u16;
    packed_simd::u32x2, u32;
    packed_simd::u32x4, u32;
    packed_simd::u32x8, u32;
    packed_simd::u32x16, u32;
    packed_simd::u64x2, u64;
    packed_simd::u64x4, u64;
    packed_simd::u64x8, u64;
    packed_simd::u8x2, u8;
    packed_simd::u8x4, u8;
    packed_simd::u8x8, u8;
    packed_simd::u8x16, u8;
    packed_simd::u8x32, u8;
    packed_simd::u8x64, u8;
    packed_simd::usizex2, usize;
    packed_simd::usizex4, usize;
    packed_simd::usizex8, usize;
);

#[cfg(feature = "simd")]
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

#[cfg(feature = "simd")]
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
