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

use ops::Additive;

use structure::GroupAbelianApprox;
use structure::GroupAbelian;
use structure::RingCommutativeApprox;
use structure::RingCommutative;
use structure::FieldApprox;
use structure::Field;
use structure::RealApprox;
use structure::EuclideanGroupApprox;

use ident;


/// A module with approximate operators.
pub trait ModuleApprox<S: RingCommutativeApprox>
    : GroupAbelianApprox<Additive>
{}


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
    : ModuleApprox<S>
    + GroupAbelian<Additive>
{}

/// A approximate vector space has an approx. module structure over an approx. field.
pub trait VectorSpaceApprox<S: FieldApprox>
    : ModuleApprox<S>
{}


/// A vector space has a module structure over a field instead of a ring.
pub trait VectorSpace<S: Field>
    : VectorSpaceApprox<S>
    + Module<S>
{}


/// A normed approximate vector space.
pub trait NormedSpaceApprox<S: FieldApprox>
    : VectorSpaceApprox<S> {
    /// The squared norm of this vector.
    fn squared_norm(&self) -> S;

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


/// An approximate vector space aquipped with an inner product.
///
/// It must be a normed space as well and the norm must agree with the inner product.
/// The inner product must be symmetric, linear in its first agurment, and positive definite.
pub trait InnerSpaceApprox<S: RealApprox>
    : NormedSpaceApprox<S> {
    /// Computes the inner product of `self` with `other`.
    fn inner_product(&self, other: &Self) -> S;

    /// Measures the angle between two vectors.
    #[inline]
    fn angle(&self, other: &Self) -> S {
        let prod = self.inner_product(other);
        let n1   = self.norm();
        let n2   = self.norm();

        let zero: S = ident::id(Additive);

        if n1 == zero || n2 == zero {
            zero
        }
        else {
            let cang = prod / (n1 * n2);

            cang.acos()
        }
    }
}

/// An approximate finite-dimensional vector space.
pub trait FiniteDimVectorSpaceApprox<S: FieldApprox>
    : VectorSpaceApprox<S> {

    /// The vector space dimension.
    fn dimension() -> usize;

    /// The vector space canonical basis.
    fn canonical_basis() -> &'static [Self];

    /// Retrieves the i-th component of `Self` wrt. the canonical basis.
    ///
    /// As usual, indexing starts with 0.
    fn component(&self, i: usize) -> S;

    /// Retrieves the i-th component of `Self` wrt. the canonical basis without bound checking.
    unsafe fn component_unchecked(&self, i: usize)  -> S;
}

/// A finite-dimensional inner space.
pub trait FiniteDimInnerSpaceApprox<S: RealApprox>
    : FiniteDimVectorSpaceApprox<S> +
      InnerSpaceApprox<S> {
}


/// A set of elements called "points" associated with a vector space and a transitive and free
/// additive group action called a "translation".
///
/// The group action is commonly called 
pub trait AffineSpaceApprox<S: FieldApprox> {
    /// The associated vector space.
    type Vector: VectorSpaceApprox<S>;

    /// Applies the additive group action of this affine space's associated vector space on `self`.
    fn translate_by(vector: &Self::Vector) -> Self;

    /// Returns the unique element `v` of the associated vector space such that `self = other + v`.
    fn subtract(&self, other: &Self) -> Self::Vector;
}

// XXX: because of the associated type, we cannot make this inherit from `AffineSpaceApprox<S>`.
/// A finite-dimensional affine space.
pub trait FiniteDimAffineSpaceApprox<S: FieldApprox> {
    /// The associated finite-dimensionale vector space.
    type Vector: FiniteDimVectorSpaceApprox<S>;

    /// Applies the additive group action of this affine space's associated vector space on `self`.
    fn translate_by(vector: &Self::Vector) -> Self;

    /// Returns the unique element `v` of the associated vector space such that `self = other + v`.
    fn subtract(&self, other: &Self) -> Self::Vector;
}

/// A space that equips an affine space with isometries.
pub trait EuclideanSpaceApprox<S: RealApprox>: Sized + FiniteDimAffineSpaceApprox<S> {
    /// The type of isometries.
    type Isometry: EuclideanGroupApprox<S, Self>;
}
