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

use general::{Group, Op, Additive, Multiplicative};
use numeric::ApproxEq;

/// An commutative group.
///
/// ```notrust
/// ∀ a, b ∈ Self, a ∘ b = b ∘ a
/// ```
pub trait GroupAbelian<O: Op>
    : Group<O> {
    /// Returns `true` if the operator is commutative for the given argument tuple. Approximate
    /// equality is used for verifications.
    fn prop_is_commutative_approx(args: (Self, Self)) -> bool
        where Self: ApproxEq {
        let (a, b) = (|| args.0.clone(), || args.1.clone());
        relative_eq!(a().operate(b()), b().operate(a()))
    }

    /// Returns `true` if the operator is commutative for the given argument tuple.
    fn prop_is_commutative(args: (Self, Self)) -> bool
        where Self: Eq {
        let (a, b) = (|| args.0.clone(), || args.1.clone());
        a().operate(b()) == b().operate(a())
    }
}

impl_marker!(GroupAbelian<Additive>; i8, i16, i32, i64, f32, f64);
impl_marker!(GroupAbelian<Multiplicative>; f32, f64);
