use num;
use std::ops::{Add, Sub, Mul, Div, MulAssign, DivAssign, Neg};

use general::{ClosedDiv, Module, Field, Real};

/// A vector space has a module structure over a field instead of a ring.
pub trait VectorSpace: Module<Ring = <Self as VectorSpace>::Field> +
                       ClosedDiv<<Self as VectorSpace>::Field> {
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
    fn try_normalize(&self, eps: Self::Field) -> Option<Self>;

    /// Normalizes this vector in-place or does nothing if its norm is smaller or equal to `eps`.
    ///
    /// If the normalization succeded, returns the old normal of this vector.
    fn try_normalize_mut(&mut self, eps: Self::Field) -> Option<Self::Field>;
}


/// A vector space aquipped with an inner product.
///
/// It must be a normed space as well and the norm must agree with the inner product.
/// The inner product must be symmetric, linear in its first agurment, and positive definite.
pub trait InnerSpace: NormedSpace<Field = <Self as InnerSpace>::Real> {
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

        if n1 == num::zero() || n2 == num::zero() {
            num::zero()
        }
        else {
            let cang = prod / (n1 * n2);

            cang.acos()
        }
    }
}

/// A finite-dimensional vector space.
pub trait FiniteDimVectorSpace: VectorSpace {
    /// The vector space dimension.
    fn dimension() -> usize;

    /// Applies the given closule to each element of this vector space's canonical basis. Stops if
    /// `f` returns `false`.
    // XXX: return an iterator instead when `-> impl Iterator` will be supported by Rust.
    fn canonical_basis<F: FnMut(&Self) -> bool>(mut f: F) {
        for i in 0 .. Self::dimension() {
            if !f(&Self::canonical_basis_element(i)) {
                break;
            }
        }
    }

    /// The i-the canonical basis element.
    fn canonical_basis_element(i: usize) -> Self;

    /// Retrieves the i-th component of `Self` wrt. some basis.
    ///
    /// As usual, indexing starts with 0. The actual choice of basis is usually context-dependent
    /// and is not specified to this method. It is up to the user to assume the provided component
    /// will by wrt. the suitable basis for his application.
    fn component(&self, i: usize) -> Self::Field;

    /// The dot product between two vectors.
    fn dot(&self, other: &Self) -> Self::Field;

    /// Same as `.component(i)` but without bound-checking.
    unsafe fn component_unchecked(&self, i: usize) -> Self::Field;
}

/// A finite-dimenisonal vector space equipped with an inner product that must coincide
/// with the dot product.
pub trait FiniteDimInnerSpace: InnerSpace + FiniteDimVectorSpace<Field = <Self as InnerSpace>::Real> {
    /// Applies the given closure to each element of the orthonormal basis of the subspace
    /// orthogonal to free family of vectors `vs`. If `vs` is not a free family, the result is
    /// unspecified.
    // XXX: return an iterator instead when `-> impl Iterator` will be supported by Rust.
    fn orthonormal_subspace_basis<F: FnMut(&Self) -> bool>(vs: &[Self], f: F);

    // FIXME: add another method to orthogonalize a non-free family of vector?
}

/// A set points associated with a vector space and a transitive and free additive group action
/// (the translation).
pub trait AffineSpace: Sized + Clone + PartialEq +
                       Sub<Self, Output = <Self as AffineSpace>::Translation> +
                       Add<<Self as AffineSpace>::Translation, Output = Self> {
    /// The associated vector space.
    type Translation: VectorSpace;

    /// Same as `*self + *t`. Applies the additive group action of this affine space's associated
    /// vector space on `self`.
    fn translate_by(&self, t: &Self::Translation) -> Self {
        self.clone() + t.clone()
    }

    /// Same as `*self - *other`. Returns the unique element `v` of the associated vector space
    /// such that `self = right + v`.
    fn subtract(&self, right: &Self) -> Self::Translation {
        self.clone() - right.clone()
    }
}


/// The finite-dimensional affine space based on the field of reals.
pub trait EuclideanSpace: AffineSpace<Translation = <Self as EuclideanSpace>::Vector> {
    /// The underlying finite vector space.
    type Vector: FiniteDimInnerSpace<Real = Self::Real> +
                 // XXX: the following bounds should not be necessary but the compiler does not
                 // seem to be able to find them (from the Module trait)… Also, it won't find them
                 // even if we add ClosedMul instead of Mul and MulAssign separately…
                 Mul<Self::Real, Output = Self::Vector> +
                 MulAssign<Self::Real>                  +
                 Div<Self::Real, Output = Self::Vector> +
                 DivAssign<Self::Real>                  +
                 Neg<Output = Self::Vector>;

    // XXX: we can't write the following =( :
    // type Vector: FiniteDimInnerSpace<Field = Self::Real> + InnerSpace<Real = Self::Real>;
    // The compiler won't recognize that VectorSpace::Field = Self::Real.
    // Though it will work if only one bound is used… looks like a compiler bug.

    /// The underlying reals.
    type Real: Real;

    /// The preferred origin of this euclidean space.
    ///
    /// Theoretically, an euclidean space has no clearly defined origin. Though it is almost always
    /// useful to have some reference point to express all the others as translations of it.
    fn origin() -> Self;

    /// Multiplies the distance of this point to `Self::origin()` by `s`.
    fn scale_by(&self, s: Self::Real) -> Self {
        Self::origin().translate_by(&(self.coordinates() * s))
    }

    /// The coordinates of this point, i.e., the translation from the prefered origin.
    #[inline]
    fn coordinates(&self) -> Self::Vector {
        self.subtract(&Self::origin())
    }

    /// The distance between two points.
    #[inline]
    fn distance_squared(&self, b: &Self) -> Self::Real {
        self.subtract(b).norm_squared()
    }

    /// The distance between two points.
    #[inline]
    fn distance(&self, b: &Self) -> Self::Real {
        self.subtract(b).norm()
    }
}
