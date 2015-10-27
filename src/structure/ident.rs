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

//! Identities for binary operators

use std::ops::{Add, Mul};

/// A type that is equipped with an additive identity.
pub trait IdentityAdditive
    : Add<Self, Output=Self>
{
    /// The additive identity element, `0`.
    fn zero() -> Self;
}

impl IdentityAdditive for u8   { #[inline] fn zero() -> u8   { 0   } }
impl IdentityAdditive for u16  { #[inline] fn zero() -> u16  { 0   } }
impl IdentityAdditive for u32  { #[inline] fn zero() -> u32  { 0   } }
impl IdentityAdditive for u64  { #[inline] fn zero() -> u64  { 0   } }
impl IdentityAdditive for i8   { #[inline] fn zero() -> i8   { 0   } }
impl IdentityAdditive for i16  { #[inline] fn zero() -> i16  { 0   } }
impl IdentityAdditive for i32  { #[inline] fn zero() -> i32  { 0   } }
impl IdentityAdditive for i64  { #[inline] fn zero() -> i64  { 0   } }
impl IdentityAdditive for f32  { #[inline] fn zero() -> f32  { 0.0 } }
impl IdentityAdditive for f64  { #[inline] fn zero() -> f64  { 0.0 } }

/// The additive identity element, `0`.
pub fn zero<T: IdentityAdditive>() -> T {
    IdentityAdditive::zero()
}

/// A type that is equipped with a multiplicative identity.
pub trait IdentityMultiplicative
    : Mul<Self, Output=Self>
{
    /// The multiplicative identity element, `1`.
    fn unit() -> Self;
}

impl IdentityMultiplicative for u8   { #[inline] fn unit() -> u8   { 1   } }
impl IdentityMultiplicative for u16  { #[inline] fn unit() -> u16  { 1   } }
impl IdentityMultiplicative for u32  { #[inline] fn unit() -> u32  { 1   } }
impl IdentityMultiplicative for u64  { #[inline] fn unit() -> u64  { 1   } }
impl IdentityMultiplicative for i8   { #[inline] fn unit() -> i8   { 1   } }
impl IdentityMultiplicative for i16  { #[inline] fn unit() -> i16  { 1   } }
impl IdentityMultiplicative for i32  { #[inline] fn unit() -> i32  { 1   } }
impl IdentityMultiplicative for i64  { #[inline] fn unit() -> i64  { 1   } }
impl IdentityMultiplicative for f32  { #[inline] fn unit() -> f32  { 1.0 } }
impl IdentityMultiplicative for f64  { #[inline] fn unit() -> f64  { 1.0 } }

/// The multiplicative identity element, `1`.
pub fn unit<T: IdentityMultiplicative>() -> T {
    IdentityMultiplicative::unit()
}
