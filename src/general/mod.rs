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

//! Fundamental algebraic structures.
//!
//! For most applications requiring an abstraction over the reals, `Real`
//! should be sufficient.
//!
//! ## Algebraic properties
//!
//! The goal of algebraic structures is to allow elements of sets to be combined together using one
//! or several operators. The number and properties of those operators characterize the algebraic
//! structure. Abstract operators are usually noted `∘`, `+`, or `×`. The last two are preferred
//! when their behavior conform with the usual meaning of addition and multiplication of reals.
//! Let `Self` be a set. Here is a list of the most common properties those operator may fulfill:
//!
//! ~~~notrust
//! (Closure)       a, b ∈ Self ⇒ a ∘ b ∈ Self, 
//! (Divisibility)  ∀ a, b ∈ Self, ∃! r, l ∈ Self such that l ∘ a = b and a ∘ r = b
//! (Invertibility) ∃ e ∈ Self, ∀ a ∈ Self, ∃ r, l ∈ Self such that l ∘ a = a ∘ r = e
//!                 If the right and left inverse are equal they are usually noted r = l = a⁻¹.
//! (Associativity) ∀ a, b, c ∈ Self, (a ∘ b) ∘ c = a ∘ (b ∘ c)       
//! (Neutral Elt.)  ∃ e ∈ Self, ∀ a ∈ Self, e ∘ a = a ∘ e = a
//! (Commutativity) ∀ a, b ∈ Self, a ∘ b = b ∘ a
//! ~~~
//!
//! ## Identity elements
//!
//! Two traits are provided that allow the definition of the additive and
//! multiplicative identity elements:
//!
//! - `IdentityAdditive`
//! - `IdentityMultiplicative`
//!
//! ## Group-like structures
//!
//! These structures are provided for both the addition and multiplication.
//!
//! ~~~notrust
//!               Magma
//!                 |
//!         _______/ \______
//!        /                \
//!  divisibility      associativity
//!       |                  |
//!       V                  V
//!  Quasigroup          Semigroup
//!       |                  |
//!   identity            identity
//!       |                  |
//!       V                  V
//!     Loop               Monoid
//!       |                  |
//!  associativity     invertibility
//!        \______   _______/
//!               \ /
//!                |
//!                V
//!              Group
//!                |
//!          commutativity
//!                |
//!                V
//!           AbelianGroup
//! ~~~
//!
//! The following traits are provided:
//!
//! -      `Closure`(`Additive`|`Multiplicative`)
//! -        `Magma`(`Additive`|`Multiplicative`)
//! -   `Quasigroup`(`Additive`|`Multiplicative`)
//! -         `Loop`(`Additive`|`Multiplicative`)
//! -    `Semigroup`(`Additive`|`Multiplicative`)
//! -       `Monoid`(`Additive`|`Multiplicative`)
//! -        `Group`(`Additive`|`Multiplicative`)
//! - `AbelianGroup`(`Additive`|`Multiplicative`)
//!
//! ## Ring-like structures
//!
//! ~~~notrust
//! GroupAdditiveAbelian     MonoidMultiplicative
//!           \________   ________/
//!                    \ /
//!                     |
//!                     V
//!                    Ring
//!                     |
//!            commutativity_of_mul
//!                     |
//!                     V
//!              RingCommutative     GroupMultiplicativeAbelian
//!                      \_______   ___________/
//!                              \ /
//!                               |
//!                               V
//!                             Field
//! ~~~
//!
//! The following traits are provided:
//!
//! -            `Ring`
//! - `RingCommutative`
//! -           `Field`
//!
//! ## Module-like structures
//!
//! ~~~notrust
//! GroupAdditiveAbelian     RingCommutative
//!           \______         _____/
//!                  \       /
//!                   |     |
//!                   V     V
//!                Module<Scalar>          Field
//!                    \______         _____/
//!                           \       /
//!                            |     |
//!                            V     V
//!                      VectorSpace<Scalar>
//! ~~~
//!
//! The following traits are provided:
//!
//! - `Module`
//! - `VectorSpace`
//!
//! # Quickcheck properties
//!
//! Functions are provided to test that algebraic properties like
//! assciociativity and commutativity hold for a given set of arguments.
//!
//! For example:
//!
//! ~~~
//! # use algebra::general::SemigroupMultiplicative;
//! #[quickcheck]
//! fn prop_mul_is_associative(args: (i32, i32, i32)) -> bool {
//!     SemigroupMultiplicative::prop_mul_is_associative(args)
//! }
//! ~~~

pub use self::ops::{Inverse, Recip, Op, Multiplicative, Additive, inv};
pub use self::ident::Identity;
pub use self::wrapper::{Wrapper, id};

pub use self::magma::Magma;
pub use self::quasigroup::Quasigroup;
pub use self::loop_::Loop;
pub use self::semigroup::Semigroup;
pub use self::monoid::Monoid;

pub use self::group::Group;
pub use self::abelian::GroupAbelian;

pub use self::ring::Ring;
pub use self::ring::RingCommutative;
pub use self::ring::Field;

pub use self::real::Real;

pub use self::module::Module;

mod ops;
mod ident;
mod wrapper;
mod magma;
mod quasigroup;
mod loop_;
mod semigroup;
mod monoid;
mod group;
mod abelian;
mod ring;
mod module;
mod real;
