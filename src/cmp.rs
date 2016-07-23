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

/// A type with an approximate equivalence relation.
pub trait ApproxEq {
    type Eps: Sized;

    /// The default epsilon value to use in `ApproxEq::approx_eq`.
    fn default_epsilon() -> Self::Eps;

    /// Compare `a` and `b` for approximate equality using the specified
    /// epsilon value.
    fn approx_eq_eps(a: &Self, b: &Self, epsilon: &Self::Eps) -> bool;

    /// Compare `a` and `b` for approximate equality using the default
    /// epsilon value returned by `ApproxEq::default_epsilon`.
    #[inline]
    fn approx_eq(a: &Self, b: &Self) -> bool {
        Self::approx_eq_eps(a, b, &Self::default_epsilon())
    }
}

macro_rules! impl_approx_eq {
    ($V:expr; $($T:ty,)+) => {
        $(impl ApproxEq for $T {
            type Eps = $T;
            #[inline]
            fn default_epsilon() -> Self::Eps { $V }

            #[inline]
            fn approx_eq_eps(a: &$T, b: &$T, epsilon: &$T) -> bool {
                (*a - *b) < *epsilon
            }
        })+
    }
}

impl_approx_eq!(0; u8, u16, u32, u64, i8, i16, i32, i64,);
impl_approx_eq!(1.0e-6; f32, f64,);
