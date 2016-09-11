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

use general::{GroupAbelian, RingCommutative, Additive};


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
pub trait Module: GroupAbelian<Additive> {
    /// The underlying scalar field.
    type Ring: RingCommutative;
}
