use num::Num;
use num_complex::Complex;
use approx::ApproxEq;

use general::{Operator, Additive, Multiplicative, AbstractGroupAbelian, AbstractMonoid, ClosedNeg};
use general::wrapper::Wrapper as W;

/// A ring is the combination of an abelian group and a multiplicative monoid structure.
///
/// A ring is equipped with:
///
/// * A abstract operator (usually the addition) that fulfills the constraints of an abelian group.
/// * A second abstract operator (usually the multiplication) that fulfills the constraints of a monoid.
pub trait AbstractRing<A: Operator = Additive, M: Operator = Multiplicative>:
    AbstractGroupAbelian<A> + AbstractMonoid<M> {

    /// Returns `true` if the multiplication and addition operators are distributive for
    /// the given argument tuple. Approximate equality is used for verifications.
    fn prop_mul_and_add_are_distributive_approx(args: (Self, Self, Self)) -> bool
        where Self: ApproxEq {

        let (a, b, c) = args;
        let a = || { W::<_, A, M>::new(a.clone()) };
        let b = || { W::<_, A, M>::new(b.clone()) };
        let c = || { W::<_, A, M>::new(c.clone()) };

        // Left distributivity
        relative_eq!(a() * (b() + c()), a() * b() + a() * c()) &&
        // Right distributivity
        relative_eq!((b() + c()) * a(), b() * a() + c() * a())
    }

    /// Returns `true` if the multiplication and addition operators are distributive for
    /// the given argument tuple.
    fn prop_mul_and_add_are_distributive(args: (Self, Self, Self)) -> bool
        where Self: Eq {

        let (a, b, c) = args;
        let a = || { W::<_, A, M>::new(a.clone()) };
        let b = || { W::<_, A, M>::new(b.clone()) };
        let c = || { W::<_, A, M>::new(c.clone()) };

        // Left distributivity
        (a() * b()) + c() == (a() * b()) + (a() * c()) &&
        // Right distributivity
        (b() + c()) * a() == (b() * a()) + (c() * a())
    }
}

/// Implements the ring trait for types provided.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractRing, Additive, Multiplicative, Inverse, Identity};
/// # fn main() {}
/// #[derive(PartialEq, Clone)]
/// struct Wrapper<T>(T);
///
/// impl<T: AbstractMagma<Additive>> AbstractMagma<Additive> for Wrapper<T> {
///     fn operate(&self, right: &Self) -> Self {
///         Wrapper(self.0.operate(&right.0))
///     }
/// }
///
/// impl<T: Inverse<Additive>> Inverse<Additive> for Wrapper<T> {
///     fn inverse(&self) -> Self {
///         Wrapper(self.0.inverse())
///     }
/// }
///
/// impl<T: Identity<Additive>> Identity<Additive> for Wrapper<T> {
///     fn identity() -> Self {
///         Wrapper(T::identity())
///     }
/// }
///
/// impl<T: AbstractMagma<Multiplicative>> AbstractMagma<Multiplicative> for Wrapper<T> {
///     fn operate(&self, right: &Self) -> Self {
///         Wrapper(self.0.operate(&right.0))
///     }
/// }
///
/// impl<T: Identity<Multiplicative>> Identity<Multiplicative> for Wrapper<T> {
///     fn identity() -> Self {
///         Wrapper(T::identity())
///     }
/// }
///
/// impl_ring!(<Additive, Multiplicative> for Wrapper<T> where T: AbstractRing);
/// ```
#[macro_export]
macro_rules! impl_ring(
    (<$A:ty, $M:ty> for $($T:tt)+) => {
        impl_abelian!(<$A> for $($T)+);
        impl_monoid!(<$M> for $($T)+);
        impl_marker!($crate::general::AbstractRing<$A, $M>; $($T)+);
    }
);

/// A ring with a commutative multiplication.
///
/// ```notrust
/// ∀ a, b ∈ Self, a × b = b × a
/// ```
pub trait AbstractRingCommutative<A: Operator = Additive, M: Operator = Multiplicative> : AbstractRing<A, M> {
    /// Returns `true` if the multiplication operator is commutative for the given argument tuple.
    /// Approximate equality is used for verifications.
    fn prop_mul_is_commutative_approx(args: (Self, Self)) -> bool
        where Self: ApproxEq {

        let (a, b) = args;
        let a = || { W::<_, A, M>::new(a.clone()) };
        let b = || { W::<_, A, M>::new(b.clone()) };

        relative_eq!(a() * b(), b() * a())
    }

    /// Returns `true` if the multiplication operator is commutative for the given argument tuple.
    fn prop_mul_is_commutative(args: (Self, Self)) -> bool
        where Self: Eq {

        let (a, b) = args;
        let a = || { W::<_, A, M>::new(a.clone()) };
        let b = || { W::<_, A, M>::new(b.clone()) };

        a() * b() == b() * a()
    }
}

/// Implements the commutative ring trait for types provided.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractRingCommutative, Additive, Multiplicative, Inverse, Identity};
/// # fn main() {}
/// #[derive(PartialEq, Clone)]
/// struct Wrapper<T>(T);
///
/// impl<T: AbstractMagma<Additive>> AbstractMagma<Additive> for Wrapper<T> {
///     fn operate(&self, right: &Self) -> Self {
///         Wrapper(self.0.operate(&right.0))
///     }
/// }
///
/// impl<T: Inverse<Additive>> Inverse<Additive> for Wrapper<T> {
///     fn inverse(&self) -> Self {
///         Wrapper(self.0.inverse())
///     }
/// }
///
/// impl<T: Identity<Additive>> Identity<Additive> for Wrapper<T> {
///     fn identity() -> Self {
///         Wrapper(T::identity())
///     }
/// }
///
/// impl<T: AbstractMagma<Multiplicative>> AbstractMagma<Multiplicative> for Wrapper<T> {
///     fn operate(&self, right: &Self) -> Self {
///         Wrapper(self.0.operate(&right.0))
///     }
/// }
///
/// impl<T: Identity<Multiplicative>> Identity<Multiplicative> for Wrapper<T> {
///     fn identity() -> Self {
///         Wrapper(T::identity())
///     }
/// }
///
/// impl_ring!(<Additive, Multiplicative> for Wrapper<T> where T: AbstractRingCommutative);
/// ```
#[macro_export]
macro_rules! impl_ring_commutative(
    (<$A:ty, $M:ty> for $($T:tt)+) => {
        impl_ring!(<$A, $M> for $($T)+);
        impl_marker!($crate::general::AbstractRingCommutative<$A, $M>; $($T)+);
    }
);

/// A field is a commutative ring, and an abelian group under both operators.
pub trait AbstractField<A: Operator = Additive, M: Operator = Multiplicative>
    : AbstractRingCommutative<A, M>
    + AbstractGroupAbelian<M>
{ }

/// Implements the field trait for types provided.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractField, Additive, Multiplicative, Inverse, Identity};
/// # fn main() {}
/// #[derive(PartialEq, Clone)]
/// struct Wrapper<T>(T);
///
/// impl<T: AbstractMagma<Additive>> AbstractMagma<Additive> for Wrapper<T> {
///     fn operate(&self, right: &Self) -> Self {
///         Wrapper(self.0.operate(&right.0))
///     }
/// }
///
/// impl<T: Inverse<Additive>> Inverse<Additive> for Wrapper<T> {
///     fn inverse(&self) -> Self {
///         Wrapper(self.0.inverse())
///     }
/// }
///
/// impl<T: Identity<Additive>> Identity<Additive> for Wrapper<T> {
///     fn identity() -> Self {
///         Wrapper(T::identity())
///     }
/// }
///
/// impl<T: AbstractMagma<Multiplicative>> AbstractMagma<Multiplicative> for Wrapper<T> {
///     fn operate(&self, right: &Self) -> Self {
///         Wrapper(self.0.operate(&right.0))
///     }
/// }
/// impl<T: Inverse<Multiplicative>> Inverse<Multiplicative> for Wrapper<T> {
///     fn inverse(&self) -> Self {
///         Wrapper(self.0.inverse())
///     }
/// }
///
/// impl<T: Identity<Multiplicative>> Identity<Multiplicative> for Wrapper<T> {
///     fn identity() -> Self {
///         Wrapper(T::identity())
///     }
/// }
///
/// impl_field!(<Additive, Multiplicative> for Wrapper<T> where T: AbstractField);
/// ```
#[macro_export]
macro_rules! impl_field(
    (<$A:ty, $M:ty> for $($T:tt)+) => {
        impl_ring_commutative!(<$A, $M> for $($T)+);
        impl_marker!($crate::general::AbstractQuasigroup<$M>; $($T)+);
        impl_marker!($crate::general::AbstractLoop<$M>; $($T)+);
        impl_marker!($crate::general::AbstractGroup<$M>; $($T)+);
        impl_marker!($crate::general::AbstractGroupAbelian<$M>; $($T)+);
        impl_marker!($crate::general::AbstractField<$A, $M>; $($T)+);
    }
);

/*
 *
 * Implementations.
 *
 */
impl_ring_commutative!(<Additive, Multiplicative> for i8; i16; i32; i64; isize);
impl_field!(<Additive, Multiplicative> for f32; f64);

impl<N: Num + Clone + ClosedNeg + AbstractRing>            AbstractRing            for Complex<N> { }
impl<N: Num + Clone + ClosedNeg + AbstractRingCommutative> AbstractRingCommutative for Complex<N> { }
impl<N: Num + Clone + ClosedNeg + AbstractField>           AbstractField           for Complex<N> { }
