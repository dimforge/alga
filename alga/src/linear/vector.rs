use num;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use general::{ClosedAdd, ClosedDiv, ClosedMul, Field, Module, Real};

/// A vector space has a module structure over a field instead of a ring.
pub trait VectorSpace: Module<Ring = <Self as VectorSpace>::Field>
/* +
                       ClosedDiv<<Self as VectorSpace>::Field> */
{
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
        let n1 = self.norm();
        let n2 = other.norm();

        if n1 == num::zero() || n2 == num::zero() {
            num::zero()
        } else {
            let cang = prod / (n1 * n2);

            if cang > num::one() {
                num::zero()
            } else if cang < -num::one::<Self::Real>() {
                Self::Real::pi()
            } else {
                cang.acos()
            }
        }
    }
}

/// A finite-dimensional vector space.
pub trait FiniteDimVectorSpace:
    VectorSpace
    + Index<usize, Output = <Self as VectorSpace>::Field>
    + IndexMut<usize, Output = <Self as VectorSpace>::Field>
{
    /// The vector space dimension.
    fn dimension() -> usize;

    /// Applies the given closule to each element of this vector space's canonical basis. Stops if
    /// `f` returns `false`.
    // XXX: return an iterator instead when `-> impl Iterator` will be supported by Rust.
    fn canonical_basis<F: FnMut(&Self) -> bool>(mut f: F) {
        for i in 0..Self::dimension() {
            if !f(&Self::canonical_basis_element(i)) {
                break;
            }
        }
    }

    /// The i-the canonical basis element.
    fn canonical_basis_element(i: usize) -> Self;

    /// The dot product between two vectors.
    fn dot(&self, other: &Self) -> Self::Field;

    /// Same as `&self[i]` but without bound-checking.
    unsafe fn component_unchecked(&self, i: usize) -> &Self::Field;

    /// Same as `&mut self[i]` but without bound-checking.
    unsafe fn component_unchecked_mut(&mut self, i: usize) -> &mut Self::Field;
}

/// A finite-dimensional vector space equipped with an inner product that must coincide
/// with the dot product.
pub trait FiniteDimInnerSpace:
    InnerSpace + FiniteDimVectorSpace<Field = <Self as InnerSpace>::Real>
{
    /// Orthonormalizes the given family of vectors. The largest free family of vectors is moved at
    /// the beginning of the array and its size is returned. Vectors at an indices larger or equal to
    /// this length can be modified to an arbitrary value.
    fn orthonormalize(vs: &mut [Self]) -> usize;

    /// Applies the given closure to each element of the orthonormal basis of the subspace
    /// orthogonal to free family of vectors `vs`. If `vs` is not a free family, the result is
    /// unspecified.
    // XXX: return an iterator instead when `-> impl Iterator` will be supported by Rust.
    fn orthonormal_subspace_basis<F: FnMut(&Self) -> bool>(vs: &[Self], f: F);
}

/// A set points associated with a vector space and a transitive and free additive group action
/// (the translation).
pub trait AffineSpace:
    Sized
    + Clone
    + PartialEq
    + Sub<Self, Output = <Self as AffineSpace>::Translation>
    + ClosedAdd<<Self as AffineSpace>::Translation>
{
    /// The associated vector space.
    type Translation: VectorSpace;

    /// Same as `*self + *t`. Applies the additive group action of this affine space's associated
    /// vector space on `self`.
    #[inline]
    fn translate_by(&self, t: &Self::Translation) -> Self {
        self.clone() + t.clone()
    }

    /// Same as `*self - *other`. Returns the unique element `v` of the associated vector space
    /// such that `self = right + v`.
    #[inline]
    fn subtract(&self, right: &Self) -> Self::Translation {
        self.clone() - right.clone()
    }
}

/// The finite-dimensional affine space based on the field of reals.
pub trait EuclideanSpace: AffineSpace<Translation = <Self as EuclideanSpace>::Coordinates> +
                          // Equivalent to `.scale_by`.
                          ClosedMul<<Self as EuclideanSpace>::Real> +
                          // Equivalent to `.scale_by`.
                          ClosedDiv<<Self as EuclideanSpace>::Real> +
                          // Equivalent to `.scale_by(-1.0)`.
                          Neg<Output = Self> {
    /// The underlying finite vector space.
    type Coordinates: FiniteDimInnerSpace<Real = Self::Real> +
                 // XXX: the following bounds should not be necessary but the compiler does not
                 // seem to be able to find them (from supertraits of VectorSpace)… Also, it won't
                 // find them even if we add ClosedMul instead of Mul and MulAssign separately…
                 Add<Self::Coordinates, Output = Self::Coordinates> +
                 AddAssign<Self::Coordinates> +
                 Sub<Self::Coordinates, Output = Self::Coordinates> +
                 SubAssign<Self::Coordinates> +
                 Mul<Self::Real, Output = Self::Coordinates> +
                 MulAssign<Self::Real>                  +
                 Div<Self::Real, Output = Self::Coordinates> +
                 DivAssign<Self::Real>                  +
                 Neg<Output = Self::Coordinates>;

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
    ///
    /// Same as self * s.
    #[inline]
    fn scale_by(&self, s: Self::Real) -> Self {
        Self::from_coordinates(self.coordinates() * s)
    }

    // FIXME: take self by-value?
    /// The coordinates of this point, i.e., the translation from the origin.
    #[inline]
    fn coordinates(&self) -> Self::Coordinates {
        self.subtract(&Self::origin())
    }

    /// Builds a point from its coordinates relative to the origin.
    #[inline]
    fn from_coordinates(coords: Self::Coordinates) -> Self {
        Self::origin().translate_by(&coords)
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
