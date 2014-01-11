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

#[crate_type = "lib"];
#[crate_id = "github.com/bjz/num-rs#num:0.1"];
#[comment = "Fundamental algebraic structures."];

//! Fundamental algebraic structures

/// A group whose group operation is identified with addition.
///
/// # Laws
///
/// ~~~
/// (a + b) + c = a + (b + c)
///       0 + a = a
///       a + 0 = a
/// ~~~
///
/// Where:
///
/// * The binary `+` operator corresponds to `Add::add`.
/// * The `0` element corresponds to `Additive::zero`.
///
pub trait Additive
    : Add<Self, Self> {
    /// The additive inverse element, `0`.
    fn zero() -> Self;
}

/// The additive inverse element
#[inline]
pub fn zero<T: Additive>() -> T { Additive::zero() }

impl Additive for u8   { #[inline] fn zero() -> u8   { 0   } }
impl Additive for u16  { #[inline] fn zero() -> u16  { 0   } }
impl Additive for u32  { #[inline] fn zero() -> u32  { 0   } }
impl Additive for u64  { #[inline] fn zero() -> u64  { 0   } }
impl Additive for uint { #[inline] fn zero() -> uint { 0   } }
impl Additive for i8   { #[inline] fn zero() -> i8   { 0   } }
impl Additive for i16  { #[inline] fn zero() -> i16  { 0   } }
impl Additive for i32  { #[inline] fn zero() -> i32  { 0   } }
impl Additive for i64  { #[inline] fn zero() -> i64  { 0   } }
impl Additive for int  { #[inline] fn zero() -> int  { 0   } }
impl Additive for f32  { #[inline] fn zero() -> f32  { 0.0 } }
impl Additive for f64  { #[inline] fn zero() -> f64  { 0.0 } }

/// An additive group for which the addition operation is commutative.
///
/// # Laws
///
/// In addition to the requirements inherited from `Additive`, the following
/// properties must also hold:
///
/// ~~~
///  a + b = b + a
/// a + -a = 0
/// ~~~
///
/// Where:
///
/// * The unary `-` operator corresponds to `Neg::neg`.
///
pub trait AdditiveC
    : Additive
    + Neg<Self> {
}

impl AdditiveC for u8   {}
impl AdditiveC for u16  {}
impl AdditiveC for u32  {}
impl AdditiveC for u64  {}
impl AdditiveC for uint {}
impl AdditiveC for i8   {}
impl AdditiveC for i16  {}
impl AdditiveC for i32  {}
impl AdditiveC for i64  {}
impl AdditiveC for int  {}
impl AdditiveC for f32  {}
impl AdditiveC for f64  {}

/// A group whose group operation is identified with multiplication.
///
/// # Laws
///
/// ~~~
/// (a * b) * c = a * (b * c)
///       1 * a = a
///       a * 1 = a
/// ~~~
///
/// Where:
///
/// * The binary `*` operator corresponds to `Mul::mul`.
/// * The `1` element corresponds to `Multiplicative::one`.
///
pub trait Multiplicative
    : Mul<Self, Self> {
    /// The multiplicative inverse element, `1`.
    fn one() -> Self;
}

/// The multiplicative inverse element
#[inline]
pub fn one<T: Multiplicative>() -> T { Multiplicative::one() }

impl Multiplicative for u8   { #[inline] fn one() -> u8   { 1   } }
impl Multiplicative for u16  { #[inline] fn one() -> u16  { 1   } }
impl Multiplicative for u32  { #[inline] fn one() -> u32  { 1   } }
impl Multiplicative for u64  { #[inline] fn one() -> u64  { 1   } }
impl Multiplicative for uint { #[inline] fn one() -> uint { 1   } }
impl Multiplicative for i8   { #[inline] fn one() -> i8   { 1   } }
impl Multiplicative for i16  { #[inline] fn one() -> i16  { 1   } }
impl Multiplicative for i32  { #[inline] fn one() -> i32  { 1   } }
impl Multiplicative for i64  { #[inline] fn one() -> i64  { 1   } }
impl Multiplicative for int  { #[inline] fn one() -> int  { 1   } }
impl Multiplicative for f32  { #[inline] fn one() -> f32  { 1.0 } }
impl Multiplicative for f64  { #[inline] fn one() -> f64  { 1.0 } }

/// An multiplicative group for which the addition operation is commutative.
///
/// # Definition
///
/// In addition to the requirements inherited from `Multiplicative`, the
/// following properties must also hold:
///
/// ~~~
///   a * b = b * a
/// a * a⁻¹ = 1
/// ~~~
///
/// Where:
///
/// * The `⁻¹` operation corresponds to `MultiplicativeC::inverse`.
///
pub trait MultiplicativeC
    : Multiplicative {
    /// The multiplicative inverse operation, `self⁻¹`.
    fn inverse(&self) -> Self;
}

impl MultiplicativeC for f32 { fn inverse(&self) -> f32 { 1.0 / *self } }
impl MultiplicativeC for f64 { fn inverse(&self) -> f64 { 1.0 / *self } }

/// An algebraic structure that generalises the addition and multiplication
/// arithmetic operations.
///
/// # Laws
///
/// In addition to the requirements inherited from `AdditiveC` and
/// `Multiplicative`, the following properties must also hold:
///
/// ~~~
/// a * (b + c) = (a * b) + (a * c)
/// (b + c) * a = (b * a) + (c * a)
/// ~~~
///
pub trait Ring
    : AdditiveC
    + Multiplicative {
}

impl Ring for u8   {}
impl Ring for u16  {}
impl Ring for u32  {}
impl Ring for u64  {}
impl Ring for uint {}
impl Ring for i8   {}
impl Ring for i16  {}
impl Ring for i32  {}
impl Ring for i64  {}
impl Ring for int  {}
impl Ring for f32  {}
impl Ring for f64  {}

/// A commutative ring.
pub trait RingC
    : Ring
    + MultiplicativeC {
}

impl RingC for f32  {}
impl RingC for f64  {}

/// A commutative ring that also a multiplicative inverse operation for every
/// non-zero element.
pub trait Field
    : RingC
    + Sub<Self, Self>
    + Div<Self, Self>
    + Rem<Self, Self> {
}

impl Field for f32 {}
impl Field for f64 {}
