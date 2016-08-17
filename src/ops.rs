// Copyright 2014 The Algebra Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

//! Operators traits and structures.
pub use std::ops::{Add, Sub, Neg};
pub use std::ops::{Mul, Div, Rem};

/// Trait used to define the inverse element relative to the given operator.
///
/// The operator, e.g., `Multiplicative` or `Additive`, is identified by the type parameter `O`.
pub trait Inverse<O: Op> {
    /// Returns the inverse of `self`, relative to the operator `O`.
    fn inv(self) -> Self;
}

/// Returns the inverse of `m`, relative to the operator `_o`.
pub fn inv<O: Op, M: Inverse<O>>(_o: O, m: M) -> M {
    m.inv()
}

impl<T> Inverse<Additive> for T
where T: Neg<Output=T>
{
    fn inv(self) -> Self {
        -self
    }
}

impl<T> Inverse<Multiplicative> for T
where T: Recip<Result=T>
{
    fn inv(self) -> Self {
        self.recip()
    }
}

/// The multiplicative inverse operation
pub trait Recip {
    /// The reciprocal type.
    type Result;

    /// Returns the reciprocal of `self`.
    fn recip(self) -> Self::Result;
}

impl Recip for f32 { type Result = Self; #[inline] fn recip(self) -> f32 { 1.0 / self } }
impl Recip for f64 { type Result = Self; #[inline] fn recip(self) -> f64 { 1.0 / self } }

/// Trait implemented by types representing operators.
pub trait Op: Copy {
    /// Returns the structure that identifies the operator.
    fn oper() -> Self;
}

#[derive(Clone, Copy)]
/// The addition operator, commonly symbolized by `+`.
pub struct Additive;

impl Op for Additive {
    fn oper() -> Self {
        Additive
    }
}

#[derive(Clone, Copy)]
/// The multiplication operator, commonly symbolized by `×`.
pub struct Multiplicative;

impl Op for Multiplicative {
    fn oper() -> Self {
        Multiplicative
    }
}
