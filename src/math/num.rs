// Copyright 2013 The Num-rs Developers. For a full listing of the authors,
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

use algebra::{Field, CommutativeRing};

/// A number that can be written without a fractional or decimal component.
///
/// # Handling of Division and Modulus
///
/// The most interesting part of this trait is the support for different
/// interpretations of integer division and modulus for discrete numbers. This
/// topic is discussed in greater depth in Daan Leijen's
/// _[Division and Modulus for Computer Scientists]
/// (http://legacy.cs.uu.nl/daan/download/papers/divmodnote-letter.pdf)_.
trait Integral
    : Eq + Ord
    + CommutativeRing {
    /// Truncated division satisfying:
    ///
    /// ~~~
    /// t_div(a, b) = trunc(a / b)              ∀ a, b ∈ Self where b ≠ 0
    /// ~~~
    ///
    /// This is the form of division adopted by the ISO C99 standard for the `/`
    /// operator, and is usually more efficient than `f_div` due to better
    /// support on processors.
    fn t_div(a: &Self, b: &Self) -> Self;

    /// The remainder after truncated division satisfying:
    ///
    /// ~~~
    /// t_mod(a, b) = a - (b * t_div(a, b))     ∀ a, b ∈ Self where b ≠ 0
    /// ~~~
    ///
    /// This is the form of modulus adopted by the ISO C99 standard for the `%`
    /// operator, and is usually more efficient than `f_mod` due to better
    /// support on processors.
    fn t_mod(a: &Self, b: &Self) -> Self;

    /// Calculates `t_div` and `t_mod` simultaneously.
    fn t_div_mod(a: &Self, b: &Self) -> (Self, Self);

    /// Floored division satisfying:
    ///
    /// ~~~
    /// f_div(a, b) = ⌊a / b⌋                   ∀ a, b ∈ Self where b ≠ 0
    /// ~~~
    fn f_div(a: &Self, b: &Self) -> Self;

    /// The remainder after floored division satisfying:
    ///
    /// ~~~
    /// f_mod(a, b) = a - (b * f_div(a, b))     ∀ a, b where b ≠ 0
    /// ~~~
    fn f_mod(a: &Self, b: &Self) -> Self;

    /// Calculates `f_div` and `f_mod` simultaneously.
    fn f_div_mod(a: &Self, b: &Self) -> (Self, Self);

    /// Greatest Common Divisor
    fn gcd(&self) -> Self;

    /// Lowest Common Multiple
    fn lcm(&self) -> Self;
}

#[inline]
pub fn t_div<T: Integral>(a: &T, b: &T) -> T {
    Integral::t_div(a, b)
}

#[inline]
pub fn t_mod<T: Integral>(a: &T, b: &T) -> T {
    Integral::t_mod(a, b)
}

#[inline]
pub fn t_div_mod<T: Integral>(a: &T, b: &T) -> (T, T) {
    Integral::t_div_mod(a, b)
}

#[inline]
pub fn f_div<T: Integral>(a: &T, b: &T) -> T {
    Integral::f_div(a, b)
}

#[inline]
pub fn f_mod<T: Integral>(a: &T, b: &T) -> T {
    Integral::f_mod(a, b)
}

#[inline]
pub fn f_div_mod<T: Integral>(a: &T, b: &T) -> (T, T) {
    Integral::f_div_mod(a, b)
}

trait Real
    : Eq + Ord
    + Field {
}

impl Real for f32  {}
impl Real for f64  {}
