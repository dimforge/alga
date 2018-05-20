use num::Num;
use num_complex::Complex;
use std::ops::{Add, Mul};

use approx::RelativeEq;

use general::{Additive, ClosedNeg, Identity, Inverse, Multiplicative, Operator};

/// Types that are closed under a given operator.
///
/// ~~~notrust
/// a, b ∈ Self ⇒ a ∘ b ∈ Self
/// ~~~
pub trait AbstractMagma<O: Operator>: Sized + Clone {
    /// Performs an operation.
    fn operate(&self, right: &Self) -> Self;

    /// Performs specific operation.
    #[inline]
    fn op(&self, _: O, lhs: &Self) -> Self {
        self.operate(lhs)
    }
}

/// A magma with the divisibility property.
///
/// Divisibility is a weak form of right and left invertibility:
///
/// ```notrust
/// ∀ a, b ∈ Self, ∃! r, l ∈ Self such that l ∘ a = b and a ∘ r = b
/// ```
pub trait AbstractQuasigroup<O: Operator>: PartialEq + AbstractMagma<O> + Inverse<O> {
    /// Returns `true` if latin squareness holds for the given arguments. Approximate
    /// equality is used for verifications.
    fn prop_inv_is_latin_square_approx(args: (Self, Self)) -> bool
    where
        Self: RelativeEq,
    {
        let (a, b) = args;
        relative_eq!(a, a.operate(&b.inverse()).operate(&b))
            && relative_eq!(a, a.operate(&b.operate(&b.inverse())))

        // TODO: pseudo inverse?
    }

    /// Returns `true` if latin squareness holds for the given arguments.
    fn prop_inv_is_latin_square(args: (Self, Self)) -> bool
    where
        Self: Eq,
    {
        let (a, b) = args;
        a == a.operate(&b.inverse()).operate(&b) && a == a.operate(&b.operate(&b.inverse()))

        // TODO: pseudo inverse?
    }
}

/// Implements the quasigroup trait for types provided.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractQuasigroup, Additive, Inverse};
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
/// impl_quasigroup!(<Additive> for Wrapper<T> where T: AbstractQuasigroup<Additive>);
/// ```
macro_rules! impl_quasigroup(
    (<$M:ty> for $($T:tt)+) => {
        impl_marker!($crate::general::AbstractQuasigroup<$M>; $($T)+);
    }
);

/// An associative magma.
///
/// ~~~notrust
/// ∀ a, b, c ∈ Self, (a ∘ b) ∘ c = a ∘ (b ∘ c)
/// ~~~
pub trait AbstractSemigroup<O: Operator>: PartialEq + AbstractMagma<O> {
    /// Returns `true` if associativity holds for the given arguments. Approximate equality is used
    /// for verifications.
    fn prop_is_associative_approx(args: (Self, Self, Self)) -> bool
    where
        Self: RelativeEq,
    {
        let (a, b, c) = args;
        relative_eq!(a.operate(&b).operate(&c), a.operate(&b.operate(&c)))
    }

    /// Returns `true` if associativity holds for the given arguments.
    fn prop_is_associative(args: (Self, Self, Self)) -> bool
    where
        Self: Eq,
    {
        let (a, b, c) = args;
        a.operate(&b).operate(&c) == a.operate(&b.operate(&c))
    }
}

/// Implements the semigroup trait for types provded.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractSemigroup, Additive};
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
/// impl_semigroup!(<Additive> for Wrapper<T> where T: AbstractSemigroup<Additive>);
/// ```
macro_rules! impl_semigroup(
    (<$M:ty> for $($T:tt)+) => {
        impl_marker!($crate::general::AbstractSemigroup<$M>; $($T)+);
    }
);

/// A quasigroup with an unique identity element.
///
/// The left inverse `r` and right inverse `l` are not required to be equal.
/// The following property is added to the quasigroup structure:
///
/// ~~~notrust
/// ∃ e ∈ Self, ∀ a ∈ Self, ∃ r, l ∈ Self such that l ∘ a = a ∘ r = e
/// ~~~
pub trait AbstractLoop<O: Operator>: AbstractQuasigroup<O> + Identity<O> {}

/// Implements the loop trait for types provided.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractLoop, Additive, Inverse, Identity};
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
/// impl_loop!(<Additive> for Wrapper<T> where T: AbstractLoop<Additive>);
/// ```
macro_rules! impl_loop(
    (<$M:ty> for $($T:tt)+) => {
        impl_quasigroup!(<$M> for $($T)+);
        impl_marker!($crate::general::AbstractLoop<$M>; $($T)+);
    }
);

/// A semigroup equipped with an identity element.
///
/// ~~~notrust
/// ∃ e ∈ Self, ∀ a ∈ Self, e ∘ a = a ∘ e = a
/// ~~~
pub trait AbstractMonoid<O: Operator>: AbstractSemigroup<O> + Identity<O> {
    /// Checks whether operating with the identity element is a no-op for the given
    /// argument. Approximate equality is used for verifications.
    fn prop_operating_identity_element_is_noop_approx(args: (Self,)) -> bool
    where
        Self: RelativeEq,
    {
        let (a,) = args;
        relative_eq!(a.operate(&Self::identity()), a)
            && relative_eq!(Self::identity().operate(&a), a)
    }

    /// Checks whether operating with the identity element is a no-op for the given
    /// argument.
    fn prop_operating_identity_element_is_noop(args: (Self,)) -> bool
    where
        Self: Eq,
    {
        let (a,) = args;
        a.operate(&Self::identity()) == a && Self::identity().operate(&a) == a
    }
}

/// Implements the monoid trait for types provided.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractMonoid, Additive, Identity};
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
/// impl<T: Identity<Additive>> Identity<Additive> for Wrapper<T> {
///     fn identity() -> Self {
///         Wrapper(T::identity())
///     }
/// }
///
/// impl_monoid!(<Additive> for Wrapper<T> where T: AbstractMonoid<Additive>);
/// ```
macro_rules! impl_monoid(
    (<$M:ty> for $($T:tt)+) => {
        impl_semigroup!(<$M> for $($T)+);
        impl_marker!($crate::general::AbstractMonoid<$M>; $($T)+);
    }
);

/// A group is a loop and a monoid at the same time.
pub trait AbstractGroup<O: Operator>: AbstractLoop<O> + AbstractMonoid<O> {}

/// Implements the group trait for types provided.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractGroup, Additive, Inverse, Identity};
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
/// impl_group!(<Additive> for Wrapper<T> where T: AbstractGroup<Additive>);
/// ```
macro_rules! impl_group(
    (<$M:ty> for $($T:tt)+) => {
        impl_monoid!(<$M> for $($T)+);
        impl_marker!($crate::general::AbstractQuasigroup<$M>; $($T)+);
        impl_marker!($crate::general::AbstractLoop<$M>; $($T)+);
        impl_marker!($crate::general::AbstractGroup<$M>; $($T)+);
    }
);

/// An commutative group.
///
/// ```notrust
/// ∀ a, b ∈ Self, a ∘ b = b ∘ a
/// ```
pub trait AbstractGroupAbelian<O: Operator>: AbstractGroup<O> {
    /// Returns `true` if the operator is commutative for the given argument tuple. Approximate
    /// equality is used for verifications.
    fn prop_is_commutative_approx(args: (Self, Self)) -> bool
    where
        Self: RelativeEq,
    {
        let (a, b) = args;
        relative_eq!(a.operate(&b), b.operate(&a))
    }

    /// Returns `true` if the operator is commutative for the given argument tuple.
    fn prop_is_commutative(args: (Self, Self)) -> bool
    where
        Self: Eq,
    {
        let (a, b) = args;
        a.operate(&b) == b.operate(&a)
    }
}

/// Implements the abelian group trait for types provided.
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate alga;
/// # use alga::general::{AbstractMagma, AbstractGroupAbelian, Additive, Inverse, Identity};
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
/// impl_abelian!(<Additive> for Wrapper<T> where T: AbstractGroupAbelian<Additive>);
/// ```
macro_rules! impl_abelian(
    (<$M:ty> for $($T:tt)+) => {
        impl_group!(<$M> for $($T)+);
        impl_marker!($crate::general::AbstractGroupAbelian<$M>; $($T)+);
    }
);

/*
 *
 *
 * Implementations.
 *
 *
 *
 */
macro_rules! impl_magma(
    ($M:ty; $op: ident; $($T:ty),* $(,)*) => {
        $(impl AbstractMagma<$M> for $T {
            #[inline]
            fn operate(&self, lhs: &Self) -> Self {
                self.$op(*lhs)
            }
        })*
    }
);

impl_magma!(Additive; add; u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
#[cfg(decimal)]
impl_ident!(Additive; add; decimal::d128);
impl_magma!(Multiplicative; mul; u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
#[cfg(decimal)]
impl_ident!(Multiplicative; mul; decimal::d128);

impl_monoid!(<Additive> for u8; u16; u32; u64; usize);
impl_monoid!(<Multiplicative> for u8; u16; u32; u64; usize);

impl<N: AbstractMagma<Additive>> AbstractMagma<Additive> for Complex<N> {
    #[inline]
    fn operate(&self, lhs: &Self) -> Self {
        Complex {
            re: self.re.operate(&lhs.re),
            im: self.im.operate(&lhs.im),
        }
    }
}

impl<N: Num + Clone> AbstractMagma<Multiplicative> for Complex<N> {
    #[inline]
    fn operate(&self, lhs: &Self) -> Self {
        self * lhs
    }
}

impl_abelian!(<Multiplicative> for Complex<N> where N: Num + Clone + ClosedNeg);
impl_abelian!(<Additive> for Complex<N> where N: AbstractGroupAbelian<Additive>);
