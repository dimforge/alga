// Copyright 2013-2014 The Algebra Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num::Zero;

use ops::Additive;

use structure::GroupAbelian;
use structure::RingCommutative;
use structure::Field;
use structure::Real;

use ident;


/// A module combines two sets: one with an additive abelian group structure and another with a
/// commutative ring structure.
///
/// In addition, and external multiplicative law `∘` is defined. Let `S` be the ring with
/// multiplicative operator noted `×` and multiplicative identity element noted `1`. Then:
///
/// ```notrust
/// ∀ a, b ∈ S
/// ∀ x, y ∈ Self
///
/// a ∘ (x + y) = (a ∘ x) + (a ∘ y)
/// (a + b) ∘ x = (a ∘ x) + (b ∘ x)
/// (a × b) ∘ x = a ∘ (b ∘ x)
/// 1 ∘ x       = x
/// ```
pub trait Module<S: RingCommutative>
    : GroupAbelian<Additive>
{}


/// A vector space has a module structure over a field instead of a ring.
pub trait VectorSpace<S: Field>
    : Module<S>
{}



/// A normed vector space.
pub trait NormedSpace<S: Field>
    : VectorSpace<S> {
    /// The squared norm of this vector.
    fn norm_squared(&self) -> S;

    /// The norm of this vector.
    fn norm(&self) -> S;

    /// Returns a normalized version of this vector.
    fn normalize(&self) -> Self;

    /// Normalizes this vector in-place and returns its norm.
    fn normalize_mut(&mut self) -> S;

    /// Returns a normalized version of this vector unless its norm as smaller or equal to `eps`.
    fn try_normalize(&self, eps: &S) -> Option<Self>;

    /// Normalizes this vector in-place or does nothing if its norm is smaller or equal to `eps`.
    ///
    /// If the normalization succeded, returns the old normal of this vector.
    fn try_normalize_mut(&mut self, eps: &S) -> Option<S>;
}


/// A vector space aquipped with an inner product.
///
/// It must be a normed space as well and the norm must agree with the inner product.
/// The inner product must be symmetric, linear in its first agurment, and positive definite.
pub trait InnerSpace<S: Real>
    : NormedSpace<S> {
    /// Computes the inner product of `self` with `other`.
    fn inner_product(&self, other: &Self) -> S;

    /// Measures the angle between two vectors.
    #[inline]
    fn angle(&self, other: &Self) -> S {
        let prod = self.inner_product(other);
        let n1   = self.norm();
        let n2   = self.norm();

        let zero: S = ident::id(Additive);

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
pub trait FiniteDimVectorSpace<S: Field>
    : VectorSpace<S> {

    /// The vector space dimension.
    fn dimension() -> usize;

    /// The vector space canonical basis.
    fn canonical_basis<F: FnOnce(&[Self])>(f: F);

    /// Retrieves the i-th component of `Self` wrt. some basis.
    ///
    /// As usual, indexing starts with 0. The actual choice of basis is usually context-dependent
    /// and is not specified to this method. It is up to the user to assume the provided component
    /// will by wrt. the suitable basis for his application.
    fn component(&self, i: usize) -> S;

    /// Same as `.component(i)` but without bound-checking.
    unsafe fn component_unchecked(&self, i: usize) -> S;
}

/// A set of elements called "points" associated with a vector space and a transitive and free
/// additive group action called a "translation".
///
/// The group action is commonly called 
pub trait AffineSpace<S: Field> {
    /// The associated vector space.
    type Translation: VectorSpace<S>;

    /// Applies the additive group action of this affine space's associated vector space on `self`.
    fn translate_by(&self, t: &Self::Translation) -> Self;

    /// Returns the unique element `v` of the associated vector space such that `self = other + v`.
    fn subtract(&self, other: &Self) -> Self::Translation;
}


/// A finite-dimensional affine space based on the field of reals.
pub trait EuclideanSpace<S: Real>: Sized + AffineSpace<S, Translation = <Self as EuclideanSpace<S>>::Vector> {
    /// The associated finite-dimensional inner vector space space.
    type Vector: InnerSpace<S> + FiniteDimVectorSpace<S>;

    /// The distance between two points.
    #[inline]
    fn distance_squared(&self, b: &Self) -> S {
        let ab = self.subtract(b);
        ab.norm_squared()
    }

    /// The distance between two points.
    #[inline]
    fn distance(&self, b: &Self) -> S {
        let ab = self.subtract(b);
        ab.norm()
    }
}
