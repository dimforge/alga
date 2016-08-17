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

#![allow(missing_docs)]

use ops::{Op, Additive};

use structure::GroupApprox;
use structure::Group;

/// An approximately commutative group.
///
/// ```notrust
/// ∀ a, b ∈ Self, a ∘ b ≈ b ∘ a
/// ```
pub trait GroupAbelianApprox<O: Op>
    : GroupApprox<O>
{
    /// Returns `true` if the operator is approximately commutative for
    /// the given argument tuple.
    fn prop_is_commutative_approx(args: (Self, Self)) -> bool {
        let (a, b) = (|| args.0.clone(), || args.1.clone());
        (a().approx(b())).approx_eq(&b().approx(a()))
    }
}

impl_marker!(GroupAbelianApprox<Additive>; i8, i16, i32, i64);

/// A commutative group.
///
/// ```notrust
/// ∀ a, b ∈ Self, a ∘ b = b ∘ a
/// ```
pub trait GroupAbelian<O: Op>
    : GroupAbelianApprox<O>
    + Group<O>
{
    /// Returns `true` if the operator is commutative for
    /// the given argument tuple.
    fn prop_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = (|| args.0.clone(), || args.1.clone());
        a().operate(b()) == b().operate(a())
    }
}

impl_marker!(GroupAbelian<Additive>; i8, i16, i32, i64);
