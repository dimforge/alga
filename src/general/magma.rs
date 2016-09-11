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

use std::ops::{Add, Mul};

use general::{Op, Additive, Multiplicative};

/// Types that are closed under a given operator.
///
/// ~~~notrust
/// a, b ∈ Self ⇒ a ∘ b ∈ Self
/// ~~~
pub trait Magma<O: Op>
    : Sized + Clone
{
    /// Performs an operation.
    fn operate(self, Self) -> Self;
    /// Performs specific operation.
    #[inline]
    fn op(self, _: O, lhs: Self) -> Self {
        self.operate(lhs)
    }
}

macro_rules! impl_magma(
    ($M:ty; $op: ident; $($T:ty),* $(,)*) => {
        $(impl Magma<$M> for $T {
            #[inline]
            fn operate(self, lhs: Self) -> Self {
                self.$op(lhs)
            }
        })*
    }
);

impl_magma!(Additive; add; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
impl_magma!(Multiplicative; mul; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
