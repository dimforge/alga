use num::Zero;

use general::{Module, Field, Real, Identity, Additive};


/// A vector space has a module structure over a field instead of a ring.
pub trait VectorSpace : Module<Ring = <Self as VectorSpace>::Field> {
    /// The underlying scalar field.
    type Field: Field;
}


/// A normed vector space.
pub trait NormedSpace: VectorSpace {
    /// The squared norm of this vector.
    fn norm_squared(&self) -> Self::Field;

    /// The norm of this vector.
    fn norm(&self) -> Self::Field;

    /// Returns a normalized version of this vector.
    fn normalize(&self) -> Self;

    /// Normalizes this vector in-place and returns its norm.
    fn normalize_mut(&mut self) -> Self::Field;

    /// Returns a normalized version of this vector unless its norm as smaller or equal to `eps`.
    fn try_normalize(&self, eps: &Self::Field) -> Option<Self>;

    /// Normalizes this vector in-place or does nothing if its norm is smaller or equal to `eps`.
    ///
    /// If the normalization succeded, returns the old normal of this vector.
    fn try_normalize_mut(&mut self, eps: &Self::Field) -> Option<Self::Field>;
}


/// A vector space aquipped with an inner product.
///
/// It must be a normed space as well and the norm must agree with the inner product.
/// The inner product must be symmetric, linear in its first agurment, and positive definite.
pub trait InnerSpace : NormedSpace<Field = <Self as InnerSpace>::Real> {
    /// The result of inner product (same as the field used by this vector space).
    type Real: Real;

    /// Computes the inner product of `self` with `other`.
    fn inner_product(&self, other: &Self) -> Self::Real;

    /// Measures the angle between two vectors.
    #[inline]
    fn angle(&self, other: &Self) -> Self::Real {
        let prod = self.inner_product(other);
        let n1   = self.norm();
        let n2   = self.norm();

        let zero: Self::Real = Identity::<Additive>::id();

        if n1 == Zero::zero() || n2 == Zero::zero() {
            zero
        }
        else {
            let cang = prod / (n1 * n2);

            cang.acos()
        }
    }
}

/// A finite-dimensional vector space.
pub trait FiniteDimVectorSpace : VectorSpace {
    /// The vector space dimension.
    fn dimension() -> usize;

    /// The vector space canonical basis.
    fn canonical_basis<F: FnOnce(&[Self])>(f: F);

    /// Retrieves the i-th component of `Self` wrt. some basis.
    ///
    /// As usual, indexing starts with 0. The actual choice of basis is usually context-dependent
    /// and is not specified to this method. It is up to the user to assume the provided component
    /// will by wrt. the suitable basis for his application.
    fn component(&self, i: usize) -> Self::Field;

    /// Same as `.component(i)` but without bound-checking.
    unsafe fn component_unchecked(&self, i: usize) -> Self::Field;
}

/// [Alias] a finite-dimenisonal vector space equipped with a dot product.
pub trait FiniteDimInnerSpace: InnerSpace + FiniteDimVectorSpace<Field = <Self as InnerSpace>::Real> {
}

impl<T> FiniteDimInnerSpace for T
where T: InnerSpace + FiniteDimVectorSpace<Field = <T as InnerSpace>::Real> {
}

/// A set points associated with a vector space and a transitive and free additive group action
/// (the translation).
pub trait AffineSpace: Sized + Clone {
    /// The associated vector space.
    type Translation: VectorSpace;

    /// Applies the additive group action of this affine space's associated vector space on `self`.
    fn translate_by(&self, t: &Self::Translation) -> Self;

    /// Returns the unique element `v` of the associated vector space such that `self = other + v`.
    fn subtract(&self, other: &Self) -> Self::Translation;
}


/// A finite-dimensional affine space based on the field of reals.
pub trait EuclideanSpace: AffineSpace<Translation = <Self as EuclideanSpace>::Vector> {
    /// The underlying finite vector space.
    type Vector: FiniteDimInnerSpace<Real = Self::Real>;

    // XXX: we can't write the following =( :
    // The compiler won't recognize that VectorSpace::Field = Self::Real.
    // Though it will work if only one bound is used… looks like a compiler bug.
    // type Vector: FiniteDimInnerSpace<Field = Self::Real> + InnerSpace<Real = Self::Real>;

    /// The underlying reals.
    type Real: Real;

    /// The distance between two points.
    #[inline]
    fn distance_squared(&self, b: &Self) -> Self::Real {
        let ab = self.subtract(b);
        ab.norm_squared()
    }

    /// The distance between two points.
    #[inline]
    fn distance(&self, b: &Self) -> Self::Real {
        let ab = self.subtract(b);
        ab.norm()
    }
}
