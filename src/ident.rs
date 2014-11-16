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

//! Identities for binary operators

/// A type that is equipped with an additive identity
pub trait AdditiveIdentity
    : Add<Self, Self>
{
    /// The additive identity, `0`. This should satisfy:
    ///
    /// ~~~notrust
    /// a + 0 = a                           ∀ a ∈ Self
    /// 0 + a = a                           ∀ a ∈ Self
    /// ~~~
    fn zero() -> Self;
}

impl AdditiveIdentity for u8   { #[inline] fn zero() -> u8   { 0   } }
impl AdditiveIdentity for u16  { #[inline] fn zero() -> u16  { 0   } }
impl AdditiveIdentity for u32  { #[inline] fn zero() -> u32  { 0   } }
impl AdditiveIdentity for u64  { #[inline] fn zero() -> u64  { 0   } }
impl AdditiveIdentity for uint { #[inline] fn zero() -> uint { 0   } }
impl AdditiveIdentity for i8   { #[inline] fn zero() -> i8   { 0   } }
impl AdditiveIdentity for i16  { #[inline] fn zero() -> i16  { 0   } }
impl AdditiveIdentity for i32  { #[inline] fn zero() -> i32  { 0   } }
impl AdditiveIdentity for i64  { #[inline] fn zero() -> i64  { 0   } }
impl AdditiveIdentity for int  { #[inline] fn zero() -> int  { 0   } }
impl AdditiveIdentity for f32  { #[inline] fn zero() -> f32  { 0.0 } }
impl AdditiveIdentity for f64  { #[inline] fn zero() -> f64  { 0.0 } }

/// A type that is equipped with a multiplicative identity
pub trait MultiplicativeIdentity
    : Mul<Self, Self>
{
    /// The multiplicative identity, `1`. This should satisfy:
    ///
    /// ~~~notrust
    /// a * 1 = a                           ∀ a ∈ Self
    /// 1 * a = a                           ∀ a ∈ Self
    /// ~~~
    fn unit() -> Self;
}

impl MultiplicativeIdentity for u8   { #[inline] fn unit() -> u8   { 1   } }
impl MultiplicativeIdentity for u16  { #[inline] fn unit() -> u16  { 1   } }
impl MultiplicativeIdentity for u32  { #[inline] fn unit() -> u32  { 1   } }
impl MultiplicativeIdentity for u64  { #[inline] fn unit() -> u64  { 1   } }
impl MultiplicativeIdentity for uint { #[inline] fn unit() -> uint { 1   } }
impl MultiplicativeIdentity for i8   { #[inline] fn unit() -> i8   { 1   } }
impl MultiplicativeIdentity for i16  { #[inline] fn unit() -> i16  { 1   } }
impl MultiplicativeIdentity for i32  { #[inline] fn unit() -> i32  { 1   } }
impl MultiplicativeIdentity for i64  { #[inline] fn unit() -> i64  { 1   } }
impl MultiplicativeIdentity for int  { #[inline] fn unit() -> int  { 1   } }
impl MultiplicativeIdentity for f32  { #[inline] fn unit() -> f32  { 1.0 } }
impl MultiplicativeIdentity for f64  { #[inline] fn unit() -> f64  { 1.0 } }

/// The additive identity, `0`.
pub fn zero<T: AdditiveIdentity>() -> T {
    AdditiveIdentity::zero()
}

/// The multiplicative identity, `1`.
pub fn unit<T: MultiplicativeIdentity>() -> T {
    MultiplicativeIdentity::unit()
}
