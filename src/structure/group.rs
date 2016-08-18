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

use ops::{Op, Additive, Multiplicative};

use structure::LoopApprox;
use structure::Loop;
use structure::MonoidApprox;
use structure::Monoid;
use structure::EuclideanSpaceApprox;
use structure::RealApprox;

/// An approximate group is an approx. loop and an approx. monoid simultaneously.
pub trait GroupApprox<O: Op>
    : LoopApprox<O>
    + MonoidApprox<O>
{}

impl_marker!(GroupApprox<Additive>; i8, i16, i32, i64, f32, f64);
impl_marker!(GroupApprox<Multiplicative>; f32, f64);

/// A group is a loop and a monoid at the same time.
pub trait Group<O: Op>
    : GroupApprox<O>
    + Loop<O>
    + Monoid<O>
{}

impl_marker!(Group<Additive>; i8, i16, i32, i64);

/*
 *
 * A subgroup trait inherit from its parent groups.
 *
 */

/// The group `E(n)` of isometries, i.e., rotations, reflexions, and translations.
pub trait EuclideanGroupApprox<S: RealApprox, E: EuclideanSpaceApprox<S>>: GroupApprox<Multiplicative> {
    /// Applies this group's action on a point from the euclidean space.
    fn transform_point(&self, pt: &E);
    /// Applies this group's action on a vector from the euclidean space.
    ///
    /// If `v` is a vector and `a, b` two point such that `v = a - b` the action `∘` on a vector is
    /// defined as `self ∘ v = (self × a) - (self × b)`.
    fn transform_vector(&self, v: &E::Vector);
}

/// The group `SE(n)` of orientation-preserving isometries, i.e., rotations and translations.
///
/// This is a subgroup of `E(n)`.
pub trait SpecialEuclideanGroupApprox<S: RealApprox, E: EuclideanSpaceApprox<S>>: EuclideanGroupApprox<S, E> {
}

/// The group `T(n)` of translations.
///
/// This is a subgroup of `SE(n)`.
pub trait TranslationGroupApprox<S: RealApprox, E: EuclideanSpaceApprox<S>>: SpecialEuclideanGroupApprox<S, E> {
}

/// The group `O(n)` of n-dimensional rotations and reflexions.
///
/// This is a subgroup of `E(n)`.
pub trait OrthogonalGroupApprox<S: RealApprox, E: EuclideanSpaceApprox<S>>: EuclideanGroupApprox<S, E> {
}

/// The group `SO(n)` of n-dimensional of rotations.
///
/// This is a subgroup of `O(n)`.
pub trait SpecialOrthogonalGroupApprox<S: RealApprox, E: EuclideanSpaceApprox<S>>: OrthogonalGroupApprox<S, E> {
}
