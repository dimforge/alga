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
//!
//! # Inspirations
//!
//! * [Numeric Prelude](http://www.haskell.org/haskellwiki/Numeric_Prelude) (Haskell)
//! * [Agda's Algebra module](http://www.cse.chalmers.se/~nad/listings/lib-0.7/Algebra.html) (Agda)
//! * [Idris' Algebra module](https://github.com/idris-lang/Idris-dev/blob/master/libs/prelude/Prelude/Algebra.idr) (Idris)
//! * [non/spire](https://github.com/non/spire) (Scala)
//!

/// Sets equipped with an associative binary operation `•`.
///
/// # Laws
///
/// ~~~
/// Associativity of •:
///     forall a b c,              (a • b) • c = a • (b • c)
/// ~~~
pub trait Semigroup {
    /// The binary operation `•`.
    fn op(a: &Self, b: &Self) -> Self;
}

/// A semigroup's binary operation.
#[inline]
pub fn op<T: Semigroup>(a: &T, b: &T) -> T {
    Semigroup::op(a, b)
}

/// Sets equipped with an associative binary operation and a corresponding
/// identity element.
///
/// # Laws
///
/// ~~~
/// Associativity of •:
///     forall a b c,               (a • b) • c = a • (b • c)
/// Identity for •:
///     forall a,                   a • identity = a
///     forall a,                   identity • a = a
/// ~~~
pub trait Monoid
    : Semigroup {
    /// The identity element for `•`.
    fn identity() -> Self;
}

/// The identity element of a monoid's binary operation.
#[inline]
pub fn identity<T: Monoid>() -> T {
    Monoid::identity()
}

/// Sets equipped with an associative binary operation, a corresponding
/// identity element, and an inverse.
///
/// # Laws
///
/// ~~~
/// Associativity of •:
///     forall a b c,               (a • b) • c = a • (b • c)
/// Identity for •:
///     forall a,                   a • identity = a
///     forall a,                   identity • a = a
/// Inverse for •:
///     forall a,                   a • inverse(a) = identity
///     forall a,                   inverse(a) • a = identity
/// ~~~
pub trait Group
    : Monoid {
    fn inverse(&self) -> Self;
}

/// Sets equipped with an associative and commutative binary operation, a
/// corresponding identity element, and an inverse.
///
/// # Laws
///
/// ~~~
/// Associativity of •:
///     forall a b c,               (a • b) • c = a • (b • c)
/// Commutativity of •:
///     forall a b,                 a • b = b • a
/// Identity for •:
///     forall a,                   a • identity = a
///     forall a,                   identity • a = a
/// Inverse for •:
///     forall a,                   a • inverse(a) = identity
///     forall a,                   inverse(a) • a = identity
/// ~~~
pub trait AbelianGroup
    : Group {
}

/// Sets that form a semigroup under addition.
pub trait AdditiveSemiGroup
    : Add<Self, Self> {
}

impl AdditiveSemiGroup for u8   {}
impl AdditiveSemiGroup for u16  {}
impl AdditiveSemiGroup for u32  {}
impl AdditiveSemiGroup for u64  {}
impl AdditiveSemiGroup for uint {}
impl AdditiveSemiGroup for i8   {}
impl AdditiveSemiGroup for i16  {}
impl AdditiveSemiGroup for i32  {}
impl AdditiveSemiGroup for i64  {}
impl AdditiveSemiGroup for int  {}
impl AdditiveSemiGroup for f32  {}
impl AdditiveSemiGroup for f64  {}

/// Sets that form a monoid under addition.
pub trait AdditiveMonoid
    : AdditiveSemiGroup {
    /// The additive identity, `0`.
    fn zero() -> Self;
}

/// The additive identity, `0`.
#[inline]
pub fn zero<T: AdditiveMonoid>() -> T {
    AdditiveMonoid::zero()
}

impl AdditiveMonoid for u8   { #[inline] fn zero() -> u8   { 0   } }
impl AdditiveMonoid for u16  { #[inline] fn zero() -> u16  { 0   } }
impl AdditiveMonoid for u32  { #[inline] fn zero() -> u32  { 0   } }
impl AdditiveMonoid for u64  { #[inline] fn zero() -> u64  { 0   } }
impl AdditiveMonoid for uint { #[inline] fn zero() -> uint { 0   } }
impl AdditiveMonoid for i8   { #[inline] fn zero() -> i8   { 0   } }
impl AdditiveMonoid for i16  { #[inline] fn zero() -> i16  { 0   } }
impl AdditiveMonoid for i32  { #[inline] fn zero() -> i32  { 0   } }
impl AdditiveMonoid for i64  { #[inline] fn zero() -> i64  { 0   } }
impl AdditiveMonoid for int  { #[inline] fn zero() -> int  { 0   } }
impl AdditiveMonoid for f32  { #[inline] fn zero() -> f32  { 0.0 } }
impl AdditiveMonoid for f64  { #[inline] fn zero() -> f64  { 0.0 } }

/// Sets that form a group under addition.
pub trait AdditiveGroup
    : AdditiveMonoid
    + Sub<Self, Self>
    + Neg<Self> {
}

impl AdditiveGroup for u8   {}
impl AdditiveGroup for u16  {}
impl AdditiveGroup for u32  {}
impl AdditiveGroup for u64  {}
impl AdditiveGroup for uint {}
impl AdditiveGroup for i8   {}
impl AdditiveGroup for i16  {}
impl AdditiveGroup for i32  {}
impl AdditiveGroup for i64  {}
impl AdditiveGroup for int  {}
impl AdditiveGroup for f32  {}
impl AdditiveGroup for f64  {}

/// Sets that form an abelian group under addition.
pub trait AdditiveAbelianGroup
    : AdditiveGroup {
}

impl AdditiveAbelianGroup for u8   {}
impl AdditiveAbelianGroup for u16  {}
impl AdditiveAbelianGroup for u32  {}
impl AdditiveAbelianGroup for u64  {}
impl AdditiveAbelianGroup for uint {}
impl AdditiveAbelianGroup for i8   {}
impl AdditiveAbelianGroup for i16  {}
impl AdditiveAbelianGroup for i32  {}
impl AdditiveAbelianGroup for i64  {}
impl AdditiveAbelianGroup for int  {}
impl AdditiveAbelianGroup for f32  {}
impl AdditiveAbelianGroup for f64  {}

/// Sets that form a semigroup under multiplication.
pub trait MultiplicativeSemiGroup
    : Mul<Self, Self> {
}

impl MultiplicativeSemiGroup for u8   {}
impl MultiplicativeSemiGroup for u16  {}
impl MultiplicativeSemiGroup for u32  {}
impl MultiplicativeSemiGroup for u64  {}
impl MultiplicativeSemiGroup for uint {}
impl MultiplicativeSemiGroup for i8   {}
impl MultiplicativeSemiGroup for i16  {}
impl MultiplicativeSemiGroup for i32  {}
impl MultiplicativeSemiGroup for i64  {}
impl MultiplicativeSemiGroup for int  {}
impl MultiplicativeSemiGroup for f32  {}
impl MultiplicativeSemiGroup for f64  {}

/// Sets that form a monoid under multiplication.
pub trait MultiplicativeMonoid
    : MultiplicativeSemiGroup {
    fn one() -> Self;
}

/// The multiplicative identity, `1`.
#[inline]
pub fn one<T: MultiplicativeMonoid>() -> T {
    MultiplicativeMonoid::one()
}

impl MultiplicativeMonoid for u8   { #[inline] fn one() -> u8   { 1   } }
impl MultiplicativeMonoid for u16  { #[inline] fn one() -> u16  { 1   } }
impl MultiplicativeMonoid for u32  { #[inline] fn one() -> u32  { 1   } }
impl MultiplicativeMonoid for u64  { #[inline] fn one() -> u64  { 1   } }
impl MultiplicativeMonoid for uint { #[inline] fn one() -> uint { 1   } }
impl MultiplicativeMonoid for i8   { #[inline] fn one() -> i8   { 1   } }
impl MultiplicativeMonoid for i16  { #[inline] fn one() -> i16  { 1   } }
impl MultiplicativeMonoid for i32  { #[inline] fn one() -> i32  { 1   } }
impl MultiplicativeMonoid for i64  { #[inline] fn one() -> i64  { 1   } }
impl MultiplicativeMonoid for int  { #[inline] fn one() -> int  { 1   } }
impl MultiplicativeMonoid for f32  { #[inline] fn one() -> f32  { 1.0 } }
impl MultiplicativeMonoid for f64  { #[inline] fn one() -> f64  { 1.0 } }

/// Sets that form a group under multiplication.
pub trait MultiplicativeGroup
    : MultiplicativeMonoid
    + Div<Self, Self> {
    #[inline]
    fn recip(&self) -> Self { one::<Self>() / *self }
}

impl MultiplicativeGroup for f32  {}
impl MultiplicativeGroup for f64  {}

/// Sets that form an abelian group under multiplication.
pub trait MultiplicativeAbelianGroup
    : MultiplicativeGroup {
}

impl MultiplicativeAbelianGroup for f32  {}
impl MultiplicativeAbelianGroup for f64  {}

/// Sets that form an abelian group under addition, a monoid under
/// multiplication, and where multiplication distributes over addition.
///
/// # Laws
///
/// ~~~
/// Associativity of +:
///     forall a b c,               (a + b) + c = a + (b + c)
/// Commutativity of +:
///     forall a b,                 a + b = b + a
/// Identity for +:
///     forall a,                   a + 0 = a
///     forall a,                   0 + a = a
/// Inverse for +:
///     forall a,                   a + -a = 0
///     forall a,                   -a + a = 0
/// Associativity of *:
///     forall a b c,               (a * b) * c = a * (b * c)
/// Identity for *:
///     forall a,                   a * 1 = a
///     forall a,                   1 * a = a
/// Distributivity of * and +:
///     forall a b c,               a * (b + c) = (a * b) + (a * c)
///     forall a b c,               (a + b) * c = (a * c) + (b * c)
/// ~~~
///
/// # Examples
///
/// Integers, matrices, quaternions.
pub trait Ring
    : AdditiveAbelianGroup
    + MultiplicativeMonoid {
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

/// A ring whose multiplication operation is also commutative.
///
/// # Laws
///
/// ~~~
/// Associativity of +:
///     forall a b c,               (a + b) + c = a + (b + c)
/// Commutativity of +:
///     forall a b,                 a + b = b + a
/// Identity for +:
///     forall a,                   a + 0 = a
///     forall a,                   0 + a = a
/// Inverse for +:
///     forall a,                   a + -a = 0
///     forall a,                   -a + a = 0
/// Associativity of *:
///     forall a b c,               (a * b) * c = a * (b * c)
/// Commutativity of *:
///     forall a b,                 a * b = b * a
/// Identity for *:
///     forall a,                   a * 1 = a
///     forall a,                   1 * a = a
/// Distributivity of * and +:
///     forall a b c,               a * (b + c) = (a * b) + (a * c)
///     forall a b c,               (a + b) * c = (a * c) + (b * c)
/// ~~~
///
/// # Examples
///
/// Complex numbers, reals, rationals, integers.
pub trait CommutativeRing
    : Ring {
}

impl CommutativeRing for u8   {}
impl CommutativeRing for u16  {}
impl CommutativeRing for u32  {}
impl CommutativeRing for u64  {}
impl CommutativeRing for uint {}
impl CommutativeRing for i8   {}
impl CommutativeRing for i16  {}
impl CommutativeRing for i32  {}
impl CommutativeRing for i64  {}
impl CommutativeRing for int  {}
impl CommutativeRing for f32  {}
impl CommutativeRing for f64  {}

/// A commutative ring that also has a multiplicative inverse operation for
/// every non-zero element.
///
/// # Laws
///
/// ~~~
/// Associativity of +:
///     forall a b c,               (a + b) + c = a + (b + c)
/// Commutativity of +:
///     forall a b,                 a + b = b + a
/// Identity for +:
///     forall a,                   a + 0 = a
///     forall a,                   0 + a = a
/// Inverse for +:
///     forall a,                   a + -a = 0
///     forall a,                   -a + a = 0
/// Associativity of *:
///     forall a b c,               (a * b) * c = a * (b * c)
/// Commutativity of *:
///     forall a b,                 a * b = b * a
/// Identity for *:
///     forall a,                   a * 1 = a
///     forall a,                   1 * a = a
/// Inverse for *:
///     forall a,                   a * a⁻¹ = 1
///     forall a,                   a⁻¹ * a = 1
/// Distributivity of * and +:
///     forall a b c,               a * (b + c) = (a * b) + (a * c)
///     forall a b c,               (a + b) * c = (a * c) + (b * c)
/// ~~~
///
/// # Examples
///
/// Complex numbers, rationals, reals.
pub trait Field
    : CommutativeRing
    + MultiplicativeAbelianGroup
    + Rem<Self, Self> {
}

impl Field for f32 {}
impl Field for f64 {}

/// Ring with a notion of an absolute value
///
/// # Laws
///
/// ~~~
/// forall a where a ≠ 0        abs((signum(a)) = 1
/// forall a                    abs(a) * signum(a) = a
/// ~~~
pub trait Absolute
    : CommutativeRing {
    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
}

trait Integral
    : Eq + Ord
    + CommutativeRing {
    /// Truncated integer division satisfying:
    ///
    /// ~~~
    /// t_div(a, b) = trunc(a / b)
    /// ~~~
    ///
    /// This is the form of division adopted by the ISO C99 standard for the `/`
    /// operator, and is usually more efficient than `f_div` due to better
    /// support on processors.
    fn t_div(a: &Self, b: &Self) -> Self;

    /// The integer remainder after truncated division satisfying:
    ///
    /// ~~~
    /// t_mod(a, b) = a - (b * t_div(a, b))
    /// ~~~
    ///
    /// This is the form of modulus adopted by the ISO C99 standard for the `%`
    /// operator, and is usually more efficient than `f_mod` due to better
    /// support on processors.
    fn t_mod(a: &Self, b: &Self) -> Self;

    /// Calculates `t_div` and `t_mod` simultaneously.
    fn t_div_mod(a: &Self, b: &Self) -> (Self, Self);

    /// Floored integer division satisfying:
    ///
    /// ~~~
    /// f_div(a, b) = ⌊a / b⌋
    /// ~~~
    fn f_div(a: &Self, b: &Self) -> Self;

    /// The integer remainder after floored division satisfying:
    ///
    /// ~~~
    /// f_mod(a, b) = a - (b * f_div(a, b))
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

/// Functions that can be defined as the root of a polynomial equation.
pub trait Algebraic {
    /// Square root
    fn sqrt(&self) -> Self;
    /// Cube root
    fn cbrt(&self) -> Self;
    /// Root-n
    fn root(&self, n: i64) -> Self;
    /// Calculate the length of the hypotenuse of a right-angle triangle given
    /// the lengths of the other two sides, `x` and `y`.
    ///
    /// ~~~
    /// hypot(x, y) = √(x² + y²)
    /// ~~~
    fn hypot(x: &Self, y: &Self) -> Self;
}

/// Functions that relate the angles of a triangle to the lengths of its sides.
pub trait Trigonometric
    : Algebraic {
    /// The sine of a number in radians.
    fn sin(&self) -> Self;
    /// The cosine of a number in radians.
    fn cos(&self) -> Self;
    /// The tangent of a number in radians.
    fn tan(&self) -> Self;
    /// The arcsine of a number.
    fn asin(&self) -> Self;
    /// The arccosine of a number.
    fn acos(&self) -> Self;
    /// The arctangent of a number.
    fn atan(&self) -> Self;
    /// The four quadrant arctangent of a number, `y`, and another number `x`.
    fn atan2(x: &Self, y: &Self) -> Self;
    /// Simultaneously computes the sine and cosine of the number.
    fn sin_cos(&self) -> (Self, Self);
}

/// Functions that cannot be expressed in terms of a finite sequence of the
/// algebraic operations of addition, multiplication, and root extraction.
pub trait Transcendental
    : Trigonometric {
    /// The ratio of a circle's circumference to its diameter.
    ///
    /// ~~~
    /// π = C / d
    /// ~~~
    fn pi() -> Self;

    /// Exponential functions

    /// Returns `e^(self)`, (the exponential function).
    fn exp(&self) -> Self;
    /// Returns 2 raised to the power of the number, `2^(self)`.
    fn exp2(&self) -> Self;
    /// Returns the natural logarithm of the number.
    fn ln(&self) -> Self;
    /// Returns the logarithm of the number with respect to an arbitrary base.
    fn log(&self, base: &Self) -> Self;
    /// Returns the base 2 logarithm of the number.
    fn log2(&self) -> Self;
    /// Returns the base 10 logarithm of the number.
    fn log10(&self) -> Self;

    // Hyperbolic functions

    /// Hyperbolic sine function.
    fn sinh(&self) -> Self;
    /// Hyperbolic cosine function.
    fn cosh(&self) -> Self;
    /// Hyperbolic tangent function.
    fn tanh(&self) -> Self;
    /// Inverse hyperbolic sine function.
    fn asinh(&self) -> Self;
    /// Inverse hyperbolic cosine function.
    fn acosh(&self) -> Self;
    /// Inverse hyperbolic tangent function.
    fn atanh(&self) -> Self;
}

trait Real
    : Eq + Ord
    + Field {
}

impl Real for f32  {}
impl Real for f64  {}
