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

//! Binary operators with specific properties

/// The multiplicative inverse operation
pub trait Recip {
    fn recip(&self) -> Self;
}

impl Recip for f32 { #[inline] fn recip(&self) -> f32 { 1.0 / *self } }
impl Recip for f64 { #[inline] fn recip(&self) -> f64 { 1.0 / *self } }

/// Sets equipped with an associative addition operator.
///
/// ~~~notrust
/// (a + b) + c = a + (b + c)               ∀ a, b, c ∈ Self
/// ~~~
pub trait AssociativeAdd
    : Add<Self, Self> {}

impl AssociativeAdd for u8   {}
impl AssociativeAdd for u16  {}
impl AssociativeAdd for u32  {}
impl AssociativeAdd for u64  {}
impl AssociativeAdd for uint {}
impl AssociativeAdd for i8   {}
impl AssociativeAdd for i16  {}
impl AssociativeAdd for i32  {}
impl AssociativeAdd for i64  {}
impl AssociativeAdd for int  {}
impl AssociativeAdd for f32  {}
impl AssociativeAdd for f64  {}

/// Sets equipped with a commutative addition operator.
///
/// ~~~notrust
/// a + b = b + a                           ∀ a, b, c ∈ Self
/// ~~~
pub trait CommutativeAdd
    : Add<Self, Self> {}

impl CommutativeAdd for u8   {}
impl CommutativeAdd for u16  {}
impl CommutativeAdd for u32  {}
impl CommutativeAdd for u64  {}
impl CommutativeAdd for uint {}
impl CommutativeAdd for i8   {}
impl CommutativeAdd for i16  {}
impl CommutativeAdd for i32  {}
impl CommutativeAdd for i64  {}
impl CommutativeAdd for int  {}
impl CommutativeAdd for f32  {}
impl CommutativeAdd for f64  {}

/// Sets equipped with an associative multiplication operator.
///
/// ~~~notrust
/// (a * b) * c = a * (b * c)               ∀ a, b, c ∈ Self
/// ~~~
pub trait AssociativeMul
    : Mul<Self, Self> {}

impl AssociativeMul for u8   {}
impl AssociativeMul for u16  {}
impl AssociativeMul for u32  {}
impl AssociativeMul for u64  {}
impl AssociativeMul for uint {}
impl AssociativeMul for i8   {}
impl AssociativeMul for i16  {}
impl AssociativeMul for i32  {}
impl AssociativeMul for i64  {}
impl AssociativeMul for int  {}
impl AssociativeMul for f32  {}
impl AssociativeMul for f64  {}

/// Sets equipped with a commutative multiplication operator.
///
/// ~~~notrust
/// a * b = b * a                           ∀ a, b, c ∈ Self
/// ~~~
pub trait CommutativeMul
    : Mul<Self, Self> {}

impl CommutativeMul for u8   {}
impl CommutativeMul for u16  {}
impl CommutativeMul for u32  {}
impl CommutativeMul for u64  {}
impl CommutativeMul for uint {}
impl CommutativeMul for i8   {}
impl CommutativeMul for i16  {}
impl CommutativeMul for i32  {}
impl CommutativeMul for i64  {}
impl CommutativeMul for int  {}
impl CommutativeMul for f32  {}
impl CommutativeMul for f64  {}

/// A set that is equipped with a multiplication and addition operator where
/// multiplication distributes over addition.
///
/// ~~~notrust
/// a * (b + c) = (a * b) + (a * c)         ∀ a, b, c ∈ Self
/// (a + b) * c = (a * c) + (b * c)         ∀ a, b, c ∈ Self
/// ~~~
pub trait DistributiveMulAdd
    : Mul<Self, Self>
    + Add<Self, Self> {}

impl DistributiveMulAdd for u8   {}
impl DistributiveMulAdd for u16  {}
impl DistributiveMulAdd for u32  {}
impl DistributiveMulAdd for u64  {}
impl DistributiveMulAdd for uint {}
impl DistributiveMulAdd for i8   {}
impl DistributiveMulAdd for i16  {}
impl DistributiveMulAdd for i32  {}
impl DistributiveMulAdd for i64  {}
impl DistributiveMulAdd for int  {}
impl DistributiveMulAdd for f32  {}
impl DistributiveMulAdd for f64  {}
