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
//! For most applications requiring an abstraction over the reals, `ApproxField`
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
//! - `AdditiveIdentity`
//! - `MultiplicativeIdentity`
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
//! - (`Approx`)?(`Additive`|`Multiplicative`)`Closure`
//! - (`Approx`)?(`Additive`|`Multiplicative`)`Magma`
//! - (`Approx`)?(`Additive`|`Multiplicative`)`Quasigroup`
//! - (`Approx`)?(`Additive`|`Multiplicative`)`Loop`
//! - (`Approx`)?(`Additive`|`Multiplicative`)`Semigroup`
//! - (`Approx`)?(`Additive`|`Multiplicative`)`Monoid`
//! - (`Approx`)?(`Additive`|`Multiplicative`)`Group`
//! - (`Approx`)?(`Additive`|`Multiplicative`)`AbelianGroup`
//!
//! ## Ring-like structures
//!
//! ~~~notrust
//! AdditiveAbelianGroup     MultiplicativeMonoid
//!           \________   ________/
//!                    \ /
//!                     |
//!                     V
//!                    Ring
//!                     |
//!            commutativity_of_mul
//!                     |
//!                     V
//!              CommutativeRing     MultiplicativeAbelianGroup
//!                      \_______   ___________/
//!                              \ /
//!                               |
//!                               V
//!                             Field
//! ~~~
//!
//! The following traits are provided:
//!
//! - (`Approx`)?`Ring`
//! - (`Approx`)?`CommutativeRing`
//! - (`Approx`)?`Field`
//!
//! # Quickcheck properties
//!
//! Functions are provided to test that algebraic properties like
//! assciociativity and commutativity hold for a given set of arguments.
//!
//! For example:
//!
//! ~~~
//! # use math::structure::ApproxMultiplicativeSemigroup;
//! #[quickcheck]
//! fn prop_mul_is_approx_associative(args: (i32, i32, i32)) -> bool {
//!     ApproxMultiplicativeSemigroup::prop_mul_is_approx_associative(args)
//! }
//! ~~~

pub use self::ident::{AdditiveIdentity, zero};
pub use self::ident::{MultiplicativeIdentity, unit};

pub use self::magma::ApproxAdditiveMagma;
pub use self::magma::ApproxMultiplicativeMagma;
pub use self::magma::AdditiveMagma;
pub use self::magma::MultiplicativeMagma;

pub use self::quasigroup::ApproxAdditiveQuasigroup;
pub use self::quasigroup::ApproxMultiplicativeQuasigroup;
pub use self::quasigroup::AdditiveQuasigroup;
pub use self::quasigroup::MultiplicativeQuasigroup;

pub use self::loop_::ApproxAdditiveLoop;
pub use self::loop_::ApproxMultiplicativeLoop;
pub use self::loop_::AdditiveLoop;
pub use self::loop_::MultiplicativeLoop;

pub use self::semigroup::ApproxAdditiveSemigroup;
pub use self::semigroup::ApproxMultiplicativeSemigroup;
pub use self::semigroup::AdditiveSemigroup;
pub use self::semigroup::MultiplicativeSemigroup;

pub use self::monoid::ApproxAdditiveMonoid;
pub use self::monoid::ApproxMultiplicativeMonoid;
pub use self::monoid::AdditiveMonoid;
pub use self::monoid::MultiplicativeMonoid;

pub use self::group::ApproxAdditiveGroup;
pub use self::group::ApproxMultiplicativeGroup;
pub use self::group::AdditiveGroup;
pub use self::group::MultiplicativeGroup;

// pub use self::abelian::ApproxAdditiveAbelianGroup;
// pub use self::abelian::ApproxMultiplicativeAbelianGroup;
// pub use self::abelian::AdditiveAbelianGroup;
// pub use self::abelian::MultiplicativeAbelianGroup;

// pub use self::ring::ApproxRing;
// pub use self::ring::Ring;
// pub use self::ring::ApproxCommutativeRing;
// pub use self::ring::CommutativeRing;
// pub use self::ring::ApproxField;
// pub use self::ring::Field;

mod ident;

mod magma;
mod quasigroup;
mod loop_;
mod semigroup;
mod monoid;
mod group;
// mod abelian;

// mod ring;
