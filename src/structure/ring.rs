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

use ops::{Additive, Multiplicative};
use cmp::ApproxEq;

use structure::Monoid;
use structure::GroupAbelian;

use wrapper::Wrapper as W;

/// A ring is the combination of an abelian group and a multiplicative monoid structure.
///
/// A ring is equipped with:
///
/// * An addition operator `+` that fulfills the constraints of an abelian group.
/// * A multiplication operator `×` that fulfills the constraints of a monoid.
pub trait Ring
    : GroupAbelian<Additive>
    + Monoid<Multiplicative>
{
    /// Returns `true` if the multiplication and addition operators are distributive for
    /// the given argument tuple.
    fn prop_mul_and_add_are_distributive(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = (|| W(args.0.clone()), || W(args.1.clone()), || W(args.2.clone()));
        // Left distributivity
        ((a() * b()) + c()).approx_eq(&((a() * b()) + (a() * c()))) &&
        // Right distributivity
        ((b() + c()) * a()).approx_eq(&((b() * a()) + (c() * a())))
    }
}

impl_marker!(Ring; i8, i16, i32, i64, f32, f64);


/// A ring with a commutative multiplication.
///
/// ```notrust
/// ∀ a, b ∈ Self, a × b ≈ b × a
/// ```
pub trait RingCommutative
    : Ring
{
    /// Returns `true` if the multiplication operator is commutative for the given argument tuple.
    fn prop_mul_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = (|| W(args.0.clone()), || W(args.1.clone()));
        (a() * b()).approx_eq(&(b() * a()))
    }
}

impl_marker!(RingCommutative; i8, i16, i32, i64, f32, f64);

/// A field is a commutative ring, and an abelian group under the multiplication operator.
pub trait Field
    : RingCommutative
    + GroupAbelian<Multiplicative>
{}

impl_marker!(Field; f32, f64);
