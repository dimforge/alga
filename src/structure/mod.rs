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

#![deny(missing_docs)]

//! Fundamental algebraic structures
//!
//! ~~~notrust
//! |(• ◡•)|ノ〵(❍ᴥ❍⋃)     - "ALGEBRAIC!!!"
//! ~~~
//!
//! For most applications requiring an abstraction over the reals, `FieldApprox`
//! should be suffient.
//!
//! # Fundamental algebraic structures
//!
//! Most of these traits also come in an approximate flavor for types that do
//! not satisfy the required properties exactly, but would still benefit from
//! abstractions over the structure in question.
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
//!   divisiblity      associativity
//!       |                  |
//!       V                  V
//!  Quasigroup          Semigroup
//!       |                  |
//!   identity            identity
//!       |                  |
//!       V                  V
//!     Loop               Monoid
//!       |                  |
//!  associative     invertablility
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

pub use self::ident::{IdentityAdditive, zero};
pub use self::ident::{IdentityMultiplicative, unit};

pub use self::magma::MagmaAdditiveApprox;
pub use self::magma::MagmaMultiplicativeApprox;
pub use self::magma::MagmaAdditive;
pub use self::magma::MagmaMultiplicative;

pub use self::quasigroup::QuasigroupAdditiveApprox;
pub use self::quasigroup::QuasigroupMultiplicativeApprox;
pub use self::quasigroup::QuasigroupAdditive;
pub use self::quasigroup::QuasigroupMultiplicative;

pub use self::loop_::LoopAdditiveApprox;
pub use self::loop_::LoopMultiplicativeApprox;
pub use self::loop_::LoopAdditive;
pub use self::loop_::LoopMultiplicative;

pub use self::semigroup::SemigroupAdditiveApprox;
pub use self::semigroup::SemigroupMultiplicativeApprox;
pub use self::semigroup::SemigroupAdditive;
pub use self::semigroup::SemigroupMultiplicative;

pub use self::monoid::MonoidAdditiveApprox;
pub use self::monoid::MonoidMultiplicativeApprox;
pub use self::monoid::MonoidAdditive;
pub use self::monoid::MonoidMultiplicative;

pub use self::group::GroupAdditiveApprox;
pub use self::group::GroupMultiplicativeApprox;
pub use self::group::GroupAdditive;
pub use self::group::GroupMultiplicative;

pub use self::abelian::GroupAdditiveAbelianApprox;
pub use self::abelian::GroupMultiplicativeAbelianApprox;
pub use self::abelian::GroupAdditiveAbelian;
pub use self::abelian::GroupMultiplicativeAbelian;

pub use self::ring::RingApprox;
pub use self::ring::Ring;
pub use self::ring::RingCommutativeApprox;
pub use self::ring::RingCommutative;
pub use self::ring::FieldApprox;
pub use self::ring::Field;

mod ident;

mod magma;
mod quasigroup;
mod loop_;
mod semigroup;
mod monoid;
mod group;
mod abelian;
mod ring;
