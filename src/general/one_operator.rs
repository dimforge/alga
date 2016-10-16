use std::ops::{Add, Mul};

use approx::ApproxEq;

use general::{Operator, Additive, Multiplicative, Inverse, Identity};

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
pub trait AbstractQuasigroup<O: Operator>
    : PartialEq + AbstractMagma<O> + Inverse<O>
{
    /// Returns `true` if latin squareness holds for the given arguments. Approximate
    /// equality is used for verifications.
    fn prop_inv_is_latin_square_approx(args: (Self, Self)) -> bool
        where Self: ApproxEq {

        let (a, b) = args;
        relative_eq!(a, a.operate(&b.inverse()).operate(&b)) &&
        relative_eq!(a, a.operate(&b.operate(&b.inverse())))

        // TODO: pseudo inverse?
    }

    /// Returns `true` if latin squareness holds for the given arguments.
    fn prop_inv_is_latin_square(args: (Self, Self)) -> bool
        where Self: Eq {

        let (a, b) = args;
        a == a.operate(&b.inverse()).operate(&b) &&
        a == a.operate(&b.operate(&b.inverse()))

        // TODO: pseudo inverse?
    }
}


/// An associative magma.
///
/// ~~~notrust
/// ∀ a, b, c ∈ Self, (a ∘ b) ∘ c = a ∘ (b ∘ c)       
/// ~~~
pub trait AbstractSemigroup<O: Operator> : PartialEq + AbstractMagma<O> {
    /// Returns `true` if associativity holds for the given arguments. Approximate equality is used
    /// for verifications.
    fn prop_is_associative_approx(args: (Self, Self, Self)) -> bool
        where Self: ApproxEq {
        let (a, b, c) = args;
        relative_eq!(a.operate(&b).operate(&c), a.operate(&b.operate(&c)))
    }

    /// Returns `true` if associativity holds for the given arguments.
    fn prop_is_associative(args: (Self, Self, Self)) -> bool
        where Self: Eq {

        let (a, b, c) = args;
        a.operate(&b).operate(&c) == a.operate(&b.operate(&c))
    }
}

/// A quasigroup with an unique identity element.
///
/// The left inverse `r` and right inverse `l` are not required to be equal.
/// The following property is added to the quasigroup structure:
///
/// ~~~notrust
/// ∃ e ∈ Self, ∀ a ∈ Self, ∃ r, l ∈ Self such that l ∘ a = a ∘ r = e
/// ~~~
pub trait AbstractLoop<O: Operator>
    : AbstractQuasigroup<O>
    + Identity<O>
{ }


/// A semigroup equipped with an identity element.
///
/// ~~~notrust
/// ∃ e ∈ Self, ∀ a ∈ Self, e ∘ a = a ∘ e = a
/// ~~~
pub trait AbstractMonoid<O: Operator>
    : AbstractSemigroup<O>
    + Identity<O>
{
    /// Checks whether operating with the identity element is a no-op for the given
    /// argument. Approximate equality is used for verifications.
    fn prop_operating_identity_element_is_noop_approx(a: Self) -> bool
        where Self: ApproxEq {
        relative_eq!(a.operate(&Self::identity()), a) &&
        relative_eq!(Self::identity().operate(&a), a)
    }

    /// Checks whether operating with the identity element is a no-op for the given
    /// argument.
    fn prop_operating_identity_element_is_noop(a: Self) -> bool
        where Self: Eq {
        a.operate(&Self::identity()) == a &&
        Self::identity().operate(&a)     == a
    }
}

/// A group is a loop and a monoid at the same time.
pub trait AbstractGroup<O: Operator>
    : AbstractLoop<O> + AbstractMonoid<O>
{ }

/// An commutative group.
///
/// ```notrust
/// ∀ a, b ∈ Self, a ∘ b = b ∘ a
/// ```
pub trait AbstractGroupAbelian<O: Operator>
    : AbstractGroup<O> {
    /// Returns `true` if the operator is commutative for the given argument tuple. Approximate
    /// equality is used for verifications.
    fn prop_is_commutative_approx(args: (Self, Self)) -> bool
        where Self: ApproxEq {

        let (a, b) = args;
        relative_eq!(a.operate(&b), b.operate(&a))
    }

    /// Returns `true` if the operator is commutative for the given argument tuple.
    fn prop_is_commutative(args: (Self, Self)) -> bool
        where Self: Eq {
        let (a, b) = args;
        a.operate(&b) == b.operate(&a)
    }
}



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

impl_magma!(Additive; add; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
impl_magma!(Multiplicative; mul; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl_marker!(AbstractQuasigroup<Additive>; i8, i16, i32, i64, f32, f64);
impl_marker!(AbstractQuasigroup<Multiplicative>; f32, f64);

impl_marker!(AbstractMonoid<Additive>; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
impl_marker!(AbstractMonoid<Multiplicative>; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl_marker!(AbstractSemigroup<Additive>; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
impl_marker!(AbstractSemigroup<Multiplicative>; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl_marker!(AbstractLoop<Additive>; i8, i16, i32, i64, f32, f64);
impl_marker!(AbstractLoop<Multiplicative>; f32, f64);

impl_marker!(AbstractGroup<Additive>; i8, i16, i32, i64, f32, f64);
impl_marker!(AbstractGroup<Multiplicative>; f32, f64);

impl_marker!(AbstractGroupAbelian<Additive>; i8, i16, i32, i64, f32, f64);
impl_marker!(AbstractGroupAbelian<Multiplicative>; f32, f64);
