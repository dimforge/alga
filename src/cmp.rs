//! Order and equivalence relations.

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

/// A type with an approximate equivalence relation.
pub trait ApproxEq {
    /// The epsilon type used measure an error.
    type Eps: Sized;

    /// The default epsilon value to use in `ApproxEq::approx_eq`.
    fn default_epsilon() -> Self::Eps;

    /// Compare `a` and `b` for approximate equality using the specified
    /// epsilon value.
    fn approx_eq_eps(&self, b: &Self, epsilon: &Self::Eps) -> bool;

    /// Compare `a` and `b` for approximate equality using the default
    /// epsilon value returned by `ApproxEq::default_epsilon`.
    #[inline]
    fn approx_eq(&self, b: &Self) -> bool {
        Self::approx_eq_eps(self, b, &Self::default_epsilon())
    }
}

impl_approx_eq!(0; u8, u16, u32, u64, i8, i16, i32, i64,);
impl_approx_eq!(1.0e-6; f32, f64,);
