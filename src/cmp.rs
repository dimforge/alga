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

use std::num::{Float, SignedInt};

/// A type with an approximate equivalence relation.
pub trait ApproxEq<Eps> {
    /// The default epsilon value to use in `ApproxEq::approx_eq`.
    fn default_epsilon(_: Option<Self>) -> Eps;

    /// Compare `a` and `b` for approximate equality using the specified
    /// epsilon value.
    fn approx_eq_eps(a: &Self, b: &Self, epsilon: &Eps) -> bool;

    /// Compare `a` and `b` for approximate equality using the default
    /// epsilon value returned by `ApproxEq::default_epsilon`.
    #[inline]
    fn approx_eq(a: &Self, b: &Self) -> bool {
        ApproxEq::approx_eq_eps(a, b, &ApproxEq::default_epsilon(None::<Self>))
    }
}

macro_rules! impl_approx_eq_for_uint {
    ($T:ty) => {
        impl ApproxEq<$T> for $T {
            #[inline]
            fn default_epsilon(_: Option<$T>) -> $T { 0 }
            #[inline]
            fn approx_eq_eps(a: &$T, b: &$T, epsilon: &$T) -> bool {
                (*a - *b) < *epsilon
            }
        }

    }
}

macro_rules! impl_approx_eq_for_int {
    ($T:ty) => {
        impl ApproxEq<$T> for $T {
            #[inline]
            fn default_epsilon(_: Option<$T>) -> $T { 0 }
            #[inline]
            fn approx_eq_eps(a: &$T, b: &$T, epsilon: &$T) -> bool {
                (*a - *b).abs() < *epsilon
            }
        }

    }
}

macro_rules! impl_approx_eq_for_float {
    ($T:ty) => {
        impl ApproxEq<$T> for $T {
            #[inline]
            fn default_epsilon(_: Option<$T>) -> $T { 1.0e-6 }
            #[inline]
            fn approx_eq_eps(a: &$T, b: &$T, epsilon: &$T) -> bool {
                (*a - *b).abs() < *epsilon
            }
        }

    }
}

impl_approx_eq_for_uint!(u8)
impl_approx_eq_for_uint!(u16)
impl_approx_eq_for_uint!(u32)
impl_approx_eq_for_uint!(u64)
impl_approx_eq_for_uint!(uint)
impl_approx_eq_for_int!(i8)
impl_approx_eq_for_int!(i16)
impl_approx_eq_for_int!(i32)
impl_approx_eq_for_int!(i64)
impl_approx_eq_for_int!(int)
impl_approx_eq_for_float!(f32)
impl_approx_eq_for_float!(f64)
