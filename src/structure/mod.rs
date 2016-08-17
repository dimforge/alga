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

#![deny(missing_docs)]

//! Fundamental algebraic structures.
//!
//! ~~~notrust
//! |(• ◡•)|ノ〵(❍ᴥ❍⋃)     - "ALGEBRAIC!!!"
//! ~~~
//!
//! For most applications requiring an abstraction over the reals, `FieldApprox`
//! should be sufficient.
//!
//! # Fundamental algebraic structures
//!
//! Most of these traits also come in an approximate flavor for types that do
//! not satisfy the required properties exactly, but would still benefit from
//! abstractions over the structure in question.
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
//! When one works with inexact arithmetic, e.g. using floating point numbers, those properties
//! cannot possibly be satisfied due to the discrete nature of our computing tools. Thus a looser,
//! *approximate*, version is available. Note that fulfilling a property listed above implies that
//! its approximate version is fulfilled as well.
//!
//! ```notrust
//! (Approx. Closure) a, b ∈ Self ⇒ ∃ c ≈ a ∘ b such that c ∈ Self, 
//! (Approx. Div.)    ∀ a, b ∈ Self, ∃ r, l ∈ Self such that l ∘ a ≈ b and a ∘ r ≈ b
//! (Approx. Inv.)    ∃ e ∈ Self, ∀ a ∈ Self, ∃ r, l ∈ Self such that l ∘ a ≈ e and a ∘ r ≈ e
//! (Approx. Assoc.)  ∀ a, b, c ∈ Self, (a ∘ b) ∘ c ≈ a ∘ (b ∘ c)       
//! (Approx. Neutr.)  ∃ e ∈ Self, ∀ a ∈ Self, e ∘ a ≈ a and a ∘ e ≈ a
//! (Approx. Commut.) ∀ a, b ∈ Self, a ∘ b ≈ b ∘ a
//! ```
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
//! -      `Closure`(`Additive`|`Multiplicative`)(`Approx`)?
//! -        `Magma`(`Additive`|`Multiplicative`)(`Approx`)?
//! -   `Quasigroup`(`Additive`|`Multiplicative`)(`Approx`)?
//! -         `Loop`(`Additive`|`Multiplicative`)(`Approx`)?
//! -    `Semigroup`(`Additive`|`Multiplicative`)(`Approx`)?
//! -       `Monoid`(`Additive`|`Multiplicative`)(`Approx`)?
//! -        `Group`(`Additive`|`Multiplicative`)(`Approx`)?
//! - `AbelianGroup`(`Additive`|`Multiplicative`)(`Approx`)?
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
//! -            `Ring`(`Approx`)?
//! - `RingCommutative`(`Approx`)?
//! -           `Field`(`Approx`)?
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
//!                         Vector<Scalar>
//! ~~~
//!
//! The following traits are provided:
//!
//! - `Module`(`Approx`)?
//! - `Vector`(`Approx`)?
//!
//! # Quickcheck properties
//!
//! Functions are provided to test that algebraic properties like
//! assciociativity and commutativity hold for a given set of arguments.
//!
//! For example:
//!
//! ~~~
//! # use algebra::structure::SemigroupMultiplicativeApprox;
//! #[quickcheck]
//! fn prop_mul_is_associative_approx(args: (i32, i32, i32)) -> bool {
//!     SemigroupMultiplicativeApprox::prop_mul_is_associative_approx(args)
//! }
//! ~~~

pub use self::magma::MagmaApprox;
pub use self::magma::Magma;

pub use self::quasigroup::QuasigroupApprox;
pub use self::quasigroup::Quasigroup;

pub use self::loop_::LoopApprox;
pub use self::loop_::Loop;

pub use self::semigroup::SemigroupApprox;
pub use self::semigroup::Semigroup;

pub use self::monoid::MonoidApprox;
pub use self::monoid::Monoid;

pub use self::group::GroupApprox;
pub use self::group::Group;

pub use self::abelian::GroupAbelianApprox;
pub use self::abelian::GroupAbelian;

pub use self::ring::RingApprox;
pub use self::ring::Ring;
pub use self::ring::RingCommutativeApprox;
pub use self::ring::RingCommutative;
pub use self::ring::FieldApprox;
pub use self::ring::Field;

pub use self::module::ModuleApprox;
pub use self::module::Module;
pub use self::module::VectorApprox;
pub use self::module::Vector;

mod magma;
mod quasigroup;
mod loop_;
mod semigroup;
mod monoid;
mod group;
mod abelian;
mod ring;
mod module;
