use std::ops::{Mul, MulAssign, Add, AddAssign, Div, DivAssign};
use std::marker::PhantomData;
use std::cmp::{PartialOrd, Ordering};
use std::fmt;

use num::{Zero, One};

use approx::ApproxEq;

use general::{AbstractMagma, AbstractGroup, AbstractLoop, AbstractMonoid, AbstractQuasigroup,
              AbstractSemigroup, Operator, Inverse, AbstractGroupAbelian, SubsetOf, Additive,
              Multiplicative, MeetSemilattice, JoinSemilattice, Lattice};

/// A type that is equipped with identity.
pub trait Identity<O: Operator> {
    /// The identity element.
    fn identity() -> Self;
}

impl_ident!(Additive; 0; u8, u16, u32, u64, i8, i16, i32, i64);
impl_ident!(Additive; 0.; f32, f64);
impl_ident!(Multiplicative; 1; u8, u16, u32, u64, i8, i16, i32, i64);
impl_ident!(Multiplicative; 1.; f32, f64);


/// The universal identity element wrt. a given operator, usually noted `Id` with a
/// context-dependent subscript.
///
/// By default, it is the multiplicative identity element. It represents the degenerate set
/// containing only the identity element of any group-like structure.  It has no dimension known at
/// compile-time. All its operations are no-ops.
#[repr(C)]
#[derive(Debug)]
pub struct Id<O: Operator = Multiplicative> {
    _op: PhantomData<O>
}

impl<O: Operator> Id<O> {
    /// Creates a new identity element.
    #[inline]
    pub fn new() -> Id<O> {
        Id {
            _op: PhantomData
        }
    }
}

impl<O: Operator> Copy for Id<O> { }

impl<O: Operator> Clone for Id<O> {
    #[inline]
    fn clone(&self) -> Id<O> {
        Id::new()
    }
}

impl<O: Operator> fmt::Display for Id<O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Identity element")
    }
}

impl<O: Operator> PartialEq for Id<O> {
    #[inline]
    fn eq(&self, _: &Id<O>) -> bool {
        true
    }
}

impl<O: Operator> Eq for Id<O> { }

impl<O: Operator> PartialOrd for Id<O> {
    #[inline]
    fn partial_cmp(&self, _: &Id<O>) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<O: Operator> Identity<O> for Id<O> {
    #[inline]
    fn identity() -> Id<O> {
        Id::new()
    }
}

impl<O: Operator> ApproxEq for Id<O> {
    type Epsilon = Id<O>;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        Id::new()
    }

    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        Id::new()
    }

    #[inline]
    fn default_max_ulps() -> u32 {
        0
    }

    #[inline]
    fn relative_eq(&self, _: &Self, _: Self::Epsilon, _: Self::Epsilon) -> bool {
        true
    }

    #[inline]
    fn ulps_eq(&self, _: &Self, _: Self::Epsilon, _: u32) -> bool {
        true
    }
}

/*
 *
 * Algebraic structures.
 *
 */
impl Mul<Id> for Id {
    type Output = Id;

    fn mul(self, _: Id) -> Id {
        self
    }
}

impl MulAssign<Id> for Id {
    fn mul_assign(&mut self, _: Id) {
        // no-op
    }
}

impl Div<Id> for Id {
    type Output = Id;

    fn div(self, _: Id) -> Id {
        self
    }
}

impl DivAssign<Id> for Id {
    fn div_assign(&mut self, _: Id) {
        // no-op
    }
}

impl Add<Id> for Id {
    type Output = Id;

    fn add(self, _: Id) -> Id {
        self
    }
}

impl AddAssign<Id> for Id {
    fn add_assign(&mut self, _: Id) {
        // no-op
    }
}

impl<O: Operator> AbstractMagma<O> for Id<O> {
    #[inline]
    fn operate(&self, _: &Self) -> Id<O> {
        Id::new()
    }
}

impl<O: Operator> Inverse<O> for Id<O> {
    #[inline]
    fn inverse(&self) -> Self {
        Id::new()
    }

    #[inline]
    fn inverse_mut(&mut self) {
        // no-op
    }
}

impl<O: Operator> AbstractSemigroup<O>    for Id<O> { }
impl<O: Operator> AbstractQuasigroup<O>   for Id<O> { }
impl<O: Operator> AbstractMonoid<O>       for Id<O> { }
impl<O: Operator> AbstractLoop<O>         for Id<O> { }
impl<O: Operator> AbstractGroup<O>        for Id<O> { }
impl<O: Operator> AbstractGroupAbelian<O> for Id<O> { }

impl One for Id {
    #[inline]
    fn one() -> Id {
        Id::new()
    }
}

impl Zero for Id {
    #[inline]
    fn zero() -> Id {
        Id::new()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        true
    }
}

/*
 *
 * Conversions.
 *
 */
impl<O: Operator, T: PartialEq + Identity<O>> SubsetOf<T> for Id<O> {
    #[inline]
    fn to_superset(&self) -> T {
        T::identity()
    }

    #[inline]
    fn is_in_subset(t: &T) -> bool {
        *t == T::identity()
    }

    #[inline]
    unsafe fn from_superset_unchecked(_: &T) -> Self {
        Id::new()
    }
}

impl<O: Operator> MeetSemilattice for Id<O> {
    #[inline]
    fn meet(&self, _: &Self) -> Self {
        Id::new()
    }
}

impl<O: Operator> JoinSemilattice for Id<O> {
    #[inline]
    fn join(&self, _: &Self) -> Self {
        Id::new()
    }
}

impl<O: Operator> Lattice for Id<O> {
}
