// Copyright 2013-2014 The Num-rs Developers.
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

/// An additive closure that forms a partial equivalence relation.
///
/// ~~~notrust
/// a = b               ∃ a, b ∈ Self
/// a + b ∈ Self        ∀ a, b ∈ Self
/// ~~~
pub trait ApproxAdditiveMagma
    : Add<Self, Self>
//  + ApproxEq (TODO: requires better support for associated types)
    + PartialEq
{}

impl<T> ApproxAdditiveMagma for T where
    T: Add<T, T> /*+ ApproxEq*/ + PartialEq,
{}

/// An additive closure that forms an equivalence relation.
///
/// ~~~notrust
/// a = b               ∀ a, b ∈ Self
/// a + b ∈ Self        ∀ a, b ∈ Self
/// ~~~
pub trait AdditiveMagma
    : ApproxAdditiveMagma
    + Eq
{}

impl<T> AdditiveMagma for T where
    T: ApproxAdditiveMagma + Eq,
{}

/// A multiplicative closure that forms a partial equivalence relation.
///
/// ~~~notrust
/// a = b               ∃ a, b ∈ Self
/// a * b ∈ Self        ∀ a, b ∈ Self
/// ~~~
pub trait ApproxMultiplicativeMagma
    : Mul<Self, Self>
//  + ApproxEq (TODO: requires better support for associated types)
    + PartialEq
{}

impl<T> ApproxMultiplicativeMagma for T where
    T: Mul<T, T> /*+ ApproxEq*/ + PartialEq,
{}

/// A multiplicative closure that forms an equivalence relation.
///
/// ~~~notrust
/// a = b               ∀ a, b ∈ Self
/// a * b ∈ Self        ∀ a, b ∈ Self
/// ~~~
pub trait MultiplicativeMagma
    : ApproxMultiplicativeMagma
    + Eq
{}

impl<T> MultiplicativeMagma for T where
    T: ApproxMultiplicativeMagma + Eq,
{}
