use crate::simd::SimdBool;
use num::Signed;

pub trait SimdSigned {
    type SimdBool: SimdBool;

    fn simd_abs(&self) -> Self;
    fn simd_abs_sub(&self, other: &Self) -> Self;
    fn simd_signum(&self) -> Self;
    fn is_simd_positive(&self) -> Self::SimdBool;
    fn is_simd_negative(&self) -> Self::SimdBool;
}

impl<T: Signed> SimdSigned for T {
    type SimdBool = bool;

    #[inline(always)]
    fn simd_abs(&self) -> Self {
        self.abs()
    }

    #[inline(always)]
    fn simd_abs_sub(&self, other: &Self) -> Self {
        self.abs_sub(other)
    }

    #[inline(always)]
    fn simd_signum(&self) -> Self {
        self.signum()
    }

    #[inline(always)]
    fn is_simd_positive(&self) -> Self::SimdBool {
        self.is_positive()
    }

    #[inline(always)]
    fn is_simd_negative(&self) -> Self::SimdBool {
        self.is_negative()
    }
}
