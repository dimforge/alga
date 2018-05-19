//! Operators traits and structures.
pub use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use num::Num;
#[cfg(feature = "std")]
use num_complex::Complex;

/// Trait implemented by types representing abstract operators.
pub trait Operator: Copy {
    /// Returns the structure that identifies the operator.
    fn operator_token() -> Self;
}

/// Trait used to define the inverse element relative to the given operator.
///
/// The operator, e.g., `Multiplicative` or `Additive`, is identified by the type parameter `O`.
pub trait Inverse<O: Operator>: Sized {
    /// Returns the inverse of `self`, relative to the operator `O`.
    fn inverse(&self) -> Self;

    /// In-place inversin of `self`.
    #[inline]
    fn inverse_mut(&mut self) {
        *self = self.inverse()
    }
}

/*
 *
 * Implementations.
 *
 */

#[derive(Clone, Copy)]
/// The addition operator, commonly symbolized by `+`.
pub struct Additive;

#[derive(Clone, Copy)]
/// The multiplication operator, commonly symbolized by `×`.
pub struct Multiplicative;

#[derive(Clone, Copy)]
/// The default abstract operator.
pub struct AbstractOperator;

impl Operator for Additive {
    #[inline]
    fn operator_token() -> Self {
        Additive
    }
}

impl Operator for Multiplicative {
    #[inline]
    fn operator_token() -> Self {
        Multiplicative
    }
}

impl Operator for AbstractOperator {
    #[inline]
    fn operator_token() -> Self {
        AbstractOperator
    }
}

macro_rules! impl_additive_inverse(
    ($($T:ty),* $(,)*) => {$(
        impl Inverse<Additive> for $T {
            fn inverse(&self) -> Self {
                -*self
            }
        }
    )*}
);

impl_additive_inverse!(i8, i16, i32, i64, isize, f32, f64);
#[cfg(decimal)]
impl_additive_inverse!(decimal::d128);

#[cfg(feature = "std")]
impl<N: Inverse<Additive>> Inverse<Additive> for Complex<N> {
    #[inline]
    fn inverse(&self) -> Complex<N> {
        Complex {
            re: self.re.inverse(),
            im: self.im.inverse(),
        }
    }
}

impl Inverse<Multiplicative> for f32 {
    #[inline]
    fn inverse(&self) -> f32 {
        1.0 / self
    }
}

impl Inverse<Multiplicative> for f64 {
    #[inline]
    fn inverse(&self) -> f64 {
        1.0 / self
    }
}

#[cfg(decimal)]
impl Inverse<Multiplicative> for decimal::d128 {
    #[inline]
    fn inverse(&self) -> decimal::d128 {
        d128!(1.0) / self
    }
}

#[cfg(feature = "std")]
impl<N: Num + Clone + ClosedNeg> Inverse<Multiplicative> for Complex<N> {
    #[inline]
    fn inverse(&self) -> Self {
        self.inv()
    }
}

/// [Alias] Trait alias for `Add` and `AddAsign` with result of type `Self`.
pub trait ClosedAdd<Right = Self>: Sized + Add<Right, Output = Self> + AddAssign<Right> {}

/// [Alias] Trait alias for `Sub` and `SubAsign` with result of type `Self`.
pub trait ClosedSub<Right = Self>: Sized + Sub<Right, Output = Self> + SubAssign<Right> {}

/// [Alias] Trait alias for `Mul` and `MulAsign` with result of type `Self`.
pub trait ClosedMul<Right = Self>: Sized + Mul<Right, Output = Self> + MulAssign<Right> {}

/// [Alias] Trait alias for `Div` and `DivAsign` with result of type `Self`.
pub trait ClosedDiv<Right = Self>: Sized + Div<Right, Output = Self> + DivAssign<Right> {}

/// [Alias] Trait alias for `Neg` with result of type `Self`.
pub trait ClosedNeg: Sized + Neg<Output = Self> {}

impl<T, Right> ClosedAdd<Right> for T
where
    T: Add<Right, Output = T> + AddAssign<Right>,
{
}
impl<T, Right> ClosedSub<Right> for T
where
    T: Sub<Right, Output = T> + SubAssign<Right>,
{
}
impl<T, Right> ClosedMul<Right> for T
where
    T: Mul<Right, Output = T> + MulAssign<Right>,
{
}
impl<T, Right> ClosedDiv<Right> for T
where
    T: Div<Right, Output = T> + DivAssign<Right>,
{
}
impl<T> ClosedNeg for T
where
    T: Neg<Output = T>,
{
}
