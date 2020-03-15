#![allow(missing_docs)]
//! Traits for SIMD values.

#[cfg(feature = "simd")]
use crate::general::*;
#[cfg(feature = "decimal")]
use decimal::d128;
#[cfg(feature = "simd")]
use num::{One, Zero};
#[cfg(feature = "simd")]
use packed_simd::*;
#[cfg(feature = "simd")]
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// Marker tait only implemented by SimdValue with more than 1 lanes.
///
/// This trait is not implemented by scalar types like f32, u32, etc. whereas SimdValue is.
pub trait MultiLanesSimdValue: SimdValue {}

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
    ($($t: ident;)*) => {$(
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
    ($($t: ident),*) => {$(
        impl<N2: MultiLanesSimdValue> SubsetOf<N2> for $t
            where N2::Element: SupersetOf<$t> + PartialEq {
            #[inline(always)]
            fn to_superset(&self) -> N2 {
                N2::splat(N2::Element::from_subset(self))
            }

            #[inline(always)]
            unsafe fn from_superset_unchecked(element: &N2) -> $t {
                element.extract(0).to_subset_unchecked()
            }

            #[inline(always)]
            fn is_in_subset(c: &N2) -> bool {
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
        impl MultiLanesSimdValue for $t {}

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

        impl SubsetOf<$t> for $t {
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

        impl MeetSemilattice for $t {
            #[inline(always)]
            fn meet(&self, other: &Self) -> Self {
                self.min(*other)
            }
        }

        impl JoinSemilattice for $t {
            #[inline(always)]
            fn join(&self, other: &Self) -> Self {
                self.max(*other)
            }
        }

        impl AbstractMagma<Additive> for $t {
            #[inline(always)]
            fn operate(&self, right: &Self) -> Self {
                *self + *right
            }
        }

        impl AbstractMagma<Multiplicative> for $t {
            #[inline(always)]
            fn operate(&self, right: &Self) -> Self {
                *self * *right
            }
        }

        impl AbstractSemigroup<Additive> for $t {}
        impl AbstractSemigroup<Multiplicative> for $t {}

        impl Identity<Additive> for $t {
            #[inline(always)]
            fn identity() -> Self {
                Self::splat(<$elt>::zero())
            }
        }

        impl Identity<Multiplicative> for $t {
            #[inline(always)]
            fn identity() -> Self {
                Self::splat(<$elt>::one())
            }
        }

        impl AbstractMonoid<Additive> for $t {}
        impl AbstractMonoid<Multiplicative> for $t {}
    )*)
);

#[cfg(feature = "simd")]
macro_rules! impl_int_simd(
    ($($t: ty, $elt: ty;)*) => ($(
        impl_uint_simd!($t, $elt;);

        impl TwoSidedInverse<Additive> for $t {
            #[inline(always)]
            fn two_sided_inverse(&self) -> Self {
                -*self
            }
        }

        impl AbstractQuasigroup<Additive> for $t {}
        impl AbstractLoop<Additive> for $t {}
        impl AbstractGroup<Additive> for $t {}
        impl AbstractGroupAbelian<Additive> for $t {}

        impl AbstractRing<Additive, Multiplicative> for $t {}
        impl AbstractRingCommutative<Additive, Multiplicative> for $t {}
        impl AbstractModule<Additive, Additive, Multiplicative> for $t {
            type AbstractRing = $t;

            #[inline(always)]
            fn multiply_by(&self, r: Self) -> Self {
                *self * r
            }
        }

        impl Module for $t {
            type Ring = Self;
        }
    )*)
);

#[cfg(feature = "simd")]
macro_rules! impl_float_simd(
    ($($t: ty, $elt: ty, $bool: ty;)*) => ($(
        impl_int_simd!($t, $elt;);

        impl TwoSidedInverse<Multiplicative> for $t {
            #[inline(always)]
            fn two_sided_inverse(&self) -> Self {
                Self::splat(<$elt>::one()) / *self
            }
        }

        impl AbstractQuasigroup<Multiplicative> for $t {}
        impl AbstractLoop<Multiplicative> for $t {}
        impl AbstractGroup<Multiplicative> for $t {}
        impl AbstractGroupAbelian<Multiplicative> for $t {}
        impl AbstractField<Additive, Multiplicative> for $t {}

        impl SimdRealField for $t {
            type SimdBool = $bool;

            #[inline(always)]
            fn simd_gt(self, other: Self) -> Self::SimdBool {
                self.gt(other)
            }

            #[inline(always)]
            fn simd_lt(self, other: Self) -> Self::SimdBool {
                self.lt(other)
            }

            #[inline(always)]
            fn simd_ge(self, other: Self) -> Self::SimdBool {
                self.ge(other)
            }

            #[inline(always)]
            fn simd_le(self, other: Self) -> Self::SimdBool {
                self.le(other)
            }

            #[inline(always)]
            fn simd_eq(self, other: Self) -> Self::SimdBool {
                self.eq(other)
            }

            #[inline(always)]
            fn simd_ne(self, other: Self) -> Self::SimdBool {
                self.ne(other)
            }

            #[inline(always)]
            fn simd_max(self, other: Self) -> Self {
                self.max(other)
            }
            #[inline(always)]
            fn simd_min(self, other: Self) -> Self {
                self.min(other)
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
                <$t>::PI
            }

            #[inline(always)]
            fn simd_two_pi() -> Self {
                <$t>::PI + <$t>::PI
            }

            #[inline(always)]
            fn simd_frac_pi_2() -> Self {
                <$t>::FRAC_PI_2
            }

            #[inline(always)]
            fn simd_frac_pi_3() -> Self {
                <$t>::FRAC_PI_3
            }

            #[inline(always)]
            fn simd_frac_pi_4() -> Self {
                <$t>::FRAC_PI_4
            }

            #[inline(always)]
            fn simd_frac_pi_6() -> Self {
                <$t>::FRAC_PI_6
            }

            #[inline(always)]
            fn simd_frac_pi_8() -> Self {
                <$t>::FRAC_PI_8
            }

            #[inline(always)]
            fn simd_frac_1_pi() -> Self {
                <$t>::FRAC_1_PI
            }

            #[inline(always)]
            fn simd_frac_2_pi() -> Self {
                <$t>::FRAC_2_PI
            }

            #[inline(always)]
            fn simd_frac_2_sqrt_pi() -> Self {
                <$t>::FRAC_2_SQRT_PI
            }


            #[inline(always)]
            fn simd_e() -> Self {
                <$t>::E
            }

            #[inline(always)]
            fn simd_log2_e() -> Self {
                <$t>::LOG2_E
            }

            #[inline(always)]
            fn simd_log10_e() -> Self {
                <$t>::LOG10_E
            }

            #[inline(always)]
            fn simd_ln_2() -> Self {
                <$t>::LN_2
            }

            #[inline(always)]
            fn simd_ln_10() -> Self {
                <$t>::LN_10
            }
        }

        impl SimdComplexField for $t {
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
                self.abs()
            }

            #[inline(always)]
            fn simd_modulus(self) -> Self::SimdRealField {
                self.abs()
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
                let ge = self.ge(Self::simd_one());
                let exp = ge.select(Self::simd_one(), -Self::simd_one());
                (self * exp, exp)
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
                self * factor
            }

            #[inline(always)]
            fn simd_unscale(self, factor: Self::SimdRealField) -> Self {
                self / factor
            }

            #[inline(always)]
            fn simd_floor(self) -> Self {
                self.map(|e| e.floor())
            }

            #[inline(always)]
            fn simd_ceil(self) -> Self {
                self.map(|e| e.ceil())
            }

            #[inline(always)]
            fn simd_round(self) -> Self {
                self.map(|e| e.round())
            }

            #[inline(always)]
            fn simd_trunc(self) -> Self {
                self.map(|e| e.trunc())
            }

            #[inline(always)]
            fn simd_fract(self) -> Self {
                self.map(|e| e.fract())
            }

            #[inline(always)]
            fn simd_abs(self) -> Self {
                self.abs()
            }

            #[inline(always)]
            fn simd_signum(self) -> Self {
                self.map(|e| e.signum())
            }

            #[inline(always)]
            fn simd_mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }

            #[inline(always)]
            fn simd_powi(self, n: i32) -> Self {
               self.powf(<$t>::splat(n as $elt))
            }

            #[inline(always)]
            fn simd_powf(self, n: Self) -> Self {
                self.powf(n)
            }

            #[inline(always)]
            fn simd_powc(self, n: Self) -> Self {
               self.powf(n)
            }

            #[inline(always)]
            fn simd_sqrt(self) -> Self {
                self.sqrt()
            }

            #[inline(always)]
            fn simd_exp(self) -> Self {
                self.exp()
            }

            #[inline(always)]
            fn simd_exp2(self) -> Self {
                self.map(|e| e.exp2())
            }


            #[inline(always)]
            fn simd_exp_m1(self) -> Self {
                self.map(|e| e.exp_m1())
            }

            #[inline(always)]
            fn simd_ln_1p(self) -> Self {
                self.map(|e| e.ln_1p())
            }

            #[inline(always)]
            fn simd_ln(self) -> Self {
                self.ln()
            }

            #[inline(always)]
            fn simd_log(self, base: Self) -> Self {
                self.zip_map(base, |e, b| e.log(b))
            }

            #[inline(always)]
            fn simd_log2(self) -> Self {
                self.map(|e| e.log2())
            }

            #[inline(always)]
            fn simd_log10(self) -> Self {
                self.map(|e| e.log10())
            }

            #[inline(always)]
            fn simd_cbrt(self) -> Self {
                self.map(|e| e.cbrt())
            }

            #[inline(always)]
            fn simd_hypot(self, other: Self) -> Self::SimdRealField {
                self.zip_map(other, |e, o| e.hypot(o))
            }

            #[inline(always)]
            fn simd_sin(self) -> Self {
                self.sin()
            }

            #[inline(always)]
            fn simd_cos(self) -> Self {
                self.cos()
            }

            #[inline(always)]
            fn simd_tan(self) -> Self {
                self.map(|e| e.tan())
            }

            #[inline(always)]
            fn simd_asin(self) -> Self {
                self.map(|e| e.asin())
            }

            #[inline(always)]
            fn simd_acos(self) -> Self {
                self.map(|e| e.acos())
            }

            #[inline(always)]
            fn simd_atan(self) -> Self {
                self.map(|e| e.atan())
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
                self.map(|e| e.sinh())
            }

            #[inline(always)]
            fn simd_cosh(self) -> Self {
                self.map(|e| e.cosh())
            }

            #[inline(always)]
            fn simd_tanh(self) -> Self {
                self.map(|e| e.tanh())
            }

            #[inline(always)]
            fn simd_asinh(self) -> Self {
                self.map(|e| e.asinh())
            }

            #[inline(always)]
            fn simd_acosh(self) -> Self {
                self.map(|e| e.acosh())
            }

            #[inline(always)]
            fn simd_atanh(self) -> Self {
                self.map(|e| e.atanh())
            }
        }
    )*)
);

#[cfg(feature = "simd")]
impl_float_simd!(
    f32x2, f32, m32x2;
    f32x4, f32, m32x4;
    f32x8, f32, m32x8;
    f32x16, f32, m32x16;
    f64x2, f64, m64x2;
    f64x4, f64, m64x4;
    f64x8, f64, m64x8;
);

#[cfg(feature = "simd")]
impl_int_simd!(
    i128x1, i128;
    i128x2, i128;
    i128x4, i128;
    i16x2, i16;
    i16x4, i16;
    i16x8, i16;
    i16x16, i16;
    i16x32, i16;
    i32x2, i32;
    i32x4, i32;
    i32x8, i32;
    i32x16, i32;
    i64x2, i64;
    i64x4, i64;
    i64x8, i64;
    i8x2, i8;
    i8x4, i8;
    i8x8, i8;
    i8x16, i8;
    i8x32, i8;
    i8x64, i8;
    isizex2, isize;
    isizex4, isize;
    isizex8, isize;
);

#[cfg(feature = "simd")]
impl_uint_simd!(
    u128x1, u128;
    u128x2, u128;
    u128x4, u128;
    u16x2, u16;
    u16x4, u16;
    u16x8, u16;
    u16x16, u16;
    u16x32, u16;
    u32x2, u32;
    u32x4, u32;
    u32x8, u32;
    u32x16, u32;
    u64x2, u64;
    u64x4, u64;
    u64x8, u64;
    u8x2, u8;
    u8x4, u8;
    u8x8, u8;
    u8x16, u8;
    u8x32, u8;
    u8x64, u8;
    usizex2, usize;
    usizex4, usize;
    usizex8, usize;
);

#[cfg(feature = "simd")]
impl_simd_value!(
    m128x1, bool;
    m128x2, bool;
    m128x4, bool;
    m16x2, bool;
    m16x4, bool;
    m16x8, bool;
    m16x16, bool;
    m16x32, bool;
    m32x2, bool;
    m32x4, bool;
    m32x8, bool;
    m32x16, bool;
    m64x2, bool;
    m64x4, bool;
    m64x8, bool;
    m8x2, bool;
    m8x4, bool;
    m8x8, bool;
    m8x16, bool;
    m8x32, bool;
    m8x64, bool;
    msizex2, bool;
    msizex4, bool;
    msizex8, bool;
);

#[cfg(feature = "simd")]
impl_simd_bool!(
    m128x1;
    m128x2;
    m128x4;
    m16x2;
    m16x4;
    m16x8;
    m16x16;
    m16x32;
    m32x2;
    m32x4;
    m32x8;
    m32x16;
    m64x2;
    m64x4;
    m64x8;
    m8x2;
    m8x4;
    m8x8;
    m8x16;
    m8x32;
    m8x64;
    msizex2;
    msizex4;
    msizex8;
);
