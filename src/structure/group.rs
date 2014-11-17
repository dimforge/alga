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

use equivalence::ApproxEq;
use monoid::{AdditiveSemigroup, PartialAdditiveSemigroup};
use monoid::{MultiplicativeSemigroup, PartialMultiplicativeSemigroup};
use std::num::Float;

/// The multiplicative inverse operation
pub trait Recip {
    fn recip(&self) -> Self;
}

impl Recip for f32 {
    #[inline]
    fn recip(&self) -> f32 { 1.0 / *self }
}

impl Recip for f64 {
    #[inline]
    fn recip(&self) -> f64 { 1.0 / *self }
}

/// A type that is equipped with an associative addition operator and a
/// corresponding identity and inverse.
///
/// ~~~notrust
/// a + -a = 0      ∀ a ∈ Self
/// -a + a = 0      ∀ a ∈ Self
/// ~~~
pub trait PartialAdditiveGroup
    : PartialAdditiveMonoid
    + Sub<Self, Self>
    + Neg<Self>
{}

impl PartialAdditiveGroup for u8   {}
impl PartialAdditiveGroup for u16  {}
impl PartialAdditiveGroup for u32  {}
impl PartialAdditiveGroup for u64  {}
impl PartialAdditiveGroup for uint {}
impl PartialAdditiveGroup for i8   {}
impl PartialAdditiveGroup for i16  {}
impl PartialAdditiveGroup for i32  {}
impl PartialAdditiveGroup for i64  {}
impl PartialAdditiveGroup for int  {}
impl PartialAdditiveGroup for f32  {}
impl PartialAdditiveGroup for f64  {}

/// A type that is equipped with an associative addition operator and a
/// corresponding identity and inverse.
///
/// ~~~notrust
/// a + -a = 0      ∀ a ∈ Self
/// -a + a = 0      ∀ a ∈ Self
/// ~~~
pub trait AdditiveGroup
    : AdditiveMonoid
    + PartialAdditiveGroup
{}

impl AdditiveGroup for u8   {}
impl AdditiveGroup for u16  {}
impl AdditiveGroup for u32  {}
impl AdditiveGroup for u64  {}
impl AdditiveGroup for uint {}
impl AdditiveGroup for i8   {}
impl AdditiveGroup for i16  {}
impl AdditiveGroup for i32  {}
impl AdditiveGroup for i64  {}
impl AdditiveGroup for int  {}
