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
//! * Edward A. Kmett's [algebra package](http://hackage.haskell.org/package/algebra-3.1) (Haskell)
//! * [YAP: Yet Another Prelude](http://hackage.haskell.org/package/yap) (Haskell)
//! * Agda's [Algebra module](http://www.cse.chalmers.se/~nad/listings/lib-0.7/Algebra.html) (Agda)
//! * Idris' [Algebra module](https://github.com/idris-lang/Idris-dev/blob/master/libs/prelude/Prelude/Algebra.idr) (Idris)
//! * [non/spire](https://github.com/non/spire) (Scala)

use std::num::{Zero, One};

////////////////////////////////////////////////////////////////////////////////
// Binary Operator Properties
////////////////////////////////////////////////////////////////////////////////

/// Sets equipped with an associative addition operator.
///
/// ~~~
/// (a + b) + c = a + (b + c)               forall a, b, c ∈ Self
/// ~~~
trait AssociativeAdd
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
/// ~~~
/// a + b = b + a                           forall a, b, c ∈ Self
/// ~~~
trait CommutativeAdd
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
/// ~~~
/// (a * b) * c = a * (b * c)               forall a, b, c ∈ Self
/// ~~~
trait AssociativeMul
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
/// ~~~
/// a * b = b * a                           forall a, b, c ∈ Self
/// ~~~
trait CommutativeMul
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
/// ~~~
/// a * (b + c) = (a * b) + (a * c)         forall a, b, c ∈ Self
/// (a + b) * c = (a * c) + (b * c)         forall a, b, c ∈ Self
/// ~~~
trait DistributiveMulAdd
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

////////////////////////////////////////////////////////////////////////////////
// Additive Algebraic Structures
////////////////////////////////////////////////////////////////////////////////

/// A set that is equipped with an addition operator.
///
/// ~~~
/// Closure
///     a + b ∈ Self                        forall a, b ∈ Self
/// ~~~
pub trait AdditiveMagma
    : Add<Self, Self> {}

impl<T: Add<T, T>> AdditiveMagma for T {}

/// A set that is equipped with an associative addition operator.
///
/// ~~~
/// Closure
///     a + b ∈ Self                        forall a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           forall a, b, c ∈ Self
/// ~~~
pub trait AdditiveSemiGroup
    : AdditiveMagma
    + AssociativeAdd {
}

impl<T: AdditiveMagma + AssociativeAdd> AdditiveSemiGroup for T {}

/// A set that is equipped with an associative addition operator and a
/// corresponding identity.
///
/// ~~~
/// Closure
///     a + b ∈ Self                        forall a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           forall a, b, c ∈ Self
/// Identity
///     a + 0 = a                           forall a ∈ Self
///     0 + a = a                           forall a ∈ Self
/// ~~~
pub trait AdditiveMonoid
    : AdditiveSemiGroup
    + Zero {
}

impl<T: AdditiveSemiGroup + Zero> AdditiveMonoid for T {}

/// A set that is equipped with an associative addition operator and a
/// corresponding identity and inverse.
///
/// ~~~
/// Closure
///     a + b ∈ Self                        forall a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           forall a, b, c ∈ Self
/// Identity
///     a + 0 = a                           forall a ∈ Self
///     0 + a = a                           forall a ∈ Self
/// Inverse
///     a + -a = 0                          forall a ∈ Self
///     -a + a = 0                          forall a ∈ Self
/// ~~~
pub trait AdditiveGroup
    : AdditiveMonoid
    + Sub<Self, Self>
    + Neg<Self> {
}

impl<T: AdditiveMonoid + Sub<T, T> + Neg<T>> AdditiveGroup for T {}

/// A set that is equipped with an associative and commutative addition operator
/// and a corresponding identity and inverse.
///
/// ~~~
/// Closure
///     a + b ∈ Self                        forall a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           forall a, b, c ∈ Self
/// Commutativity
///     a + b = b + a                       forall a, b ∈ Self
/// Identity
///     a + 0 = a                           forall a ∈ Self
///     0 + a = a                           forall a ∈ Self
/// Inverse
///     a + -a = 0                          forall a ∈ Self
///     -a + a = 0                          forall a ∈ Self
/// ~~~
pub trait AdditiveAbelianGroup
    : AdditiveGroup
    + CommutativeAdd {
}

impl<T: AdditiveGroup + CommutativeAdd> AdditiveAbelianGroup for T {}

////////////////////////////////////////////////////////////////////////////////
// Multiplicative Algebraic Structures
////////////////////////////////////////////////////////////////////////////////

pub trait Recip {
    fn recip(&self) -> Self;
}

impl Recip for f32 { #[inline] fn recip(&self) -> f32 { 1.0 / *self } }
impl Recip for f64 { #[inline] fn recip(&self) -> f64 { 1.0 / *self } }

/// A set that is equipped with a multiplication operator.
///
/// ~~~
/// Closure
///     a * b ∈ Self                        forall a, b ∈ Self
/// ~~~
pub trait MultiplicativeMagma
    : Mul<Self, Self> {}

impl<T: Mul<T, T>> MultiplicativeMagma for T {}

/// A set that is equipped with an associative multiplication operator.
///
/// ~~~
/// Closure
///     a * b ∈ Self                        forall a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           forall a, b, c ∈ Self
/// ~~~
pub trait MultiplicativeSemiGroup
    : MultiplicativeMagma
    + AssociativeMul {
}

impl<T: MultiplicativeMagma + AssociativeMul> MultiplicativeSemiGroup for T {}

/// A set that is equipped with an associative multiplication operator and a
/// corresponding identity.
///
/// ~~~
/// Closure
///     a * b ∈ Self                        forall a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           forall a, b, c ∈ Self
/// Identity
///     a * 1 = a                           forall a ∈ Self
///     1 * a = a                           forall a ∈ Self
/// ~~~
pub trait MultiplicativeMonoid
    : MultiplicativeSemiGroup
    + One {
}

impl<T: MultiplicativeSemiGroup + One> MultiplicativeMonoid for T {}

/// A set that is equipped with an associative multiplication operator and a
/// corresponding identity and inverse.
///
/// ~~~
/// Closure
///     a * b ∈ Self                        forall a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           forall a, b, c ∈ Self
/// Identity
///     a * 1 = a                           forall a ∈ Self
///     1 * a = a                           forall a ∈ Self
/// Inverse
///     a * a⁻¹ = 1                         forall a ∈ Self
///     a⁻¹ * a = 1                         forall a ∈ Self
/// ~~~
pub trait MultiplicativeGroup
    : MultiplicativeMonoid
    + Div<Self, Self>
    + Recip {
}

impl<T: MultiplicativeMonoid + Div<T, T> + Recip> MultiplicativeGroup for T {}

/// A set that is equipped with an associative and commutative multiplication
/// operator and a corresponding identity and inverse.
///
/// ~~~
/// Closure
///     a * b ∈ Self                        forall a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           forall a, b, c ∈ Self
/// Commutativity
///     a * b = b * a                       forall a, b ∈ Self
/// Identity
///     a * 1 = a                           forall a ∈ Self
///     1 * a = a                           forall a ∈ Self
/// Inverse
///     a * a⁻¹ = 1                         forall a ∈ Self
///     a⁻¹ * a = 1                         forall a ∈ Self
/// ~~~
pub trait MultiplicativeAbelianGroup
    : MultiplicativeGroup
    + CommutativeMul {
}

impl<T: MultiplicativeGroup + CommutativeMul> MultiplicativeAbelianGroup for T {}

////////////////////////////////////////////////////////////////////////////////
// Ring-like Structures
////////////////////////////////////////////////////////////////////////////////

/// Sets that form an abelian group under addition, a monoid under
/// multiplication, and where multiplication distributes over addition.
///
/// ~~~
/// Closure of +
///     a + b ∈ Self                        forall a, b ∈ Self
/// Associativity of +
///     (a + b) + c = a + (b + c)           forall a, b, c ∈ Self
/// Commutativity of +
///     a + b = b + a                       forall a, b ∈ Self
/// Identity of +
///     a + 0 = a                           forall a ∈ Self
///     0 + a = a                           forall a ∈ Self
/// Inverse of +
///     a + -a = 0                          forall a ∈ Self
///     -a + a = 0                          forall a ∈ Self
/// Closure of *
///     a * b ∈ Self                        forall a, b ∈ Self
/// Associativity of *
///     (a * b) * c = a * (b * c)           forall a, b, c ∈ Self
/// Identity of *
///     a * 1 = a                           forall a ∈ Self
///     1 * a = a                           forall a ∈ Self
/// Distributivity of * over +
///     a * (b + c) = (a * b) + (a * c)     forall a, b, c ∈ Self
///     (a + b) * c = (a * c) + (b * c)     forall a, b, c ∈ Self
/// ~~~
///
/// # Examples
///
/// Integers, matrices, quaternions.
pub trait Ring
    : AdditiveAbelianGroup
    + MultiplicativeMonoid
    + DistributiveMulAdd {
}

impl<T: AdditiveAbelianGroup + MultiplicativeMonoid + DistributiveMulAdd> Ring for T {}

/// A ring whose multiplication operation is also commutative.
///
/// ~~~
/// Closure of +
///     a + b ∈ Self                        forall a, b ∈ Self
/// Associativity of +
///     (a + b) + c = a + (b + c)           forall a, b, c ∈ Self
/// Commutativity of +
///     a + b = b + a                       forall a, b ∈ Self
/// Identity of +
///     a + 0 = a                           forall a ∈ Self
///     0 + a = a                           forall a ∈ Self
/// Inverse of +
///     a + -a = 0                          forall a ∈ Self
///     -a + a = 0                          forall a ∈ Self
/// Closure of *
///     a * b ∈ Self                        forall a, b ∈ Self
/// Associativity of *
///     (a * b) * c = a * (b * c)           forall a, b, c ∈ Self
/// Commutativity of *
///     a * b = b * a                       forall a, b ∈ Self
/// Identity of *
///     a * 1 = a                           forall a ∈ Self
///     1 * a = a                           forall a ∈ Self
/// Distributivity of * over +
///     a * (b + c) = (a * b) + (a * c)     forall a, b, c ∈ Self
///     (a + b) * c = (a * c) + (b * c)     forall a, b, c ∈ Self
/// ~~~
///
/// # Examples
///
/// Complex numbers, reals, rationals, integers.
pub trait CommutativeRing
    : Ring
    + CommutativeMul {
}

impl<T: Ring + CommutativeMul> CommutativeRing for T {}

/// A commutative ring that also has a multiplicative inverse operation for
/// every non-zero element.
///
/// ~~~
/// Closure of +
///     a + b ∈ Self                        forall a, b ∈ Self
/// Associativity of +
///     (a + b) + c = a + (b + c)           forall a, b, c ∈ Self
/// Commutativity of +
///     a + b = b + a                       forall a, b ∈ Self
/// Identity of +
///     a + 0 = a                           forall a ∈ Self
///     0 + a = a                           forall a ∈ Self
/// Inverse of +
///     a + -a = 0                          forall a ∈ Self
///     -a + a = 0                          forall a ∈ Self
/// Closure of *
///     a * b ∈ Self                        forall a, b ∈ Self
/// Associativity of *
///     (a * b) * c = a * (b * c)           forall a, b, c ∈ Self
/// Commutativity of *
///     a * b = b * a                       forall a, b ∈ Self
/// Identity of *
///     a * 1 = a                           forall a ∈ Self
///     1 * a = a                           forall a ∈ Self
/// Inverse of *
///     a * a⁻¹ = 1                         forall a ∈ Self where a ≠ 0
///     a⁻¹ * a = 1                         forall a ∈ Self where a ≠ 0
/// Distributivity of * over +
///     a * (b + c) = (a * b) + (a * c)     forall a, b, c ∈ Self
///     (a + b) * c = (a * c) + (b * c)     forall a, b, c ∈ Self
/// ~~~
///
/// # Examples
///
/// Complex numbers, rationals, reals.
pub trait Field
    : CommutativeRing
    + MultiplicativeAbelianGroup {
}

impl<T: CommutativeRing + MultiplicativeAbelianGroup> Field for T {}

/// Ring with a notion of an absolute value, satisfying:
///
/// ~~~
/// abs((signum(a)) = 1                     forall a ∈ Self where a ≠ 0
/// abs(a) * signum(a) = a                  forall a ∈ Self
/// ~~~
pub trait Absolute
    : CommutativeRing {
    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
}

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
    /// t_div(a, b) = trunc(a / b)              forall a, b ∈ Self where b ≠ 0
    /// ~~~
    ///
    /// This is the form of division adopted by the ISO C99 standard for the `/`
    /// operator, and is usually more efficient than `f_div` due to better
    /// support on processors.
    fn t_div(a: &Self, b: &Self) -> Self;

    /// The remainder after truncated division satisfying:
    ///
    /// ~~~
    /// t_mod(a, b) = a - (b * t_div(a, b))     forall a, b ∈ Self where b ≠ 0
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
    /// f_div(a, b) = ⌊a / b⌋                   forall a, b ∈ Self where b ≠ 0
    /// ~~~
    fn f_div(a: &Self, b: &Self) -> Self;

    /// The remainder after floored division satisfying:
    ///
    /// ~~~
    /// f_mod(a, b) = a - (b * f_div(a, b))     forall a, b where b ≠ 0
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
    /// The ratio of the length of the opposite side to the length of the
    /// hypotenuse of a right angle triangle.
    ///
    /// ~~~
    /// sin(self) = opposite / hypotenuse
    /// ~~~
    ///
    /// Where `self` is an angle in radians.
    fn sin(&self) -> Self;
    /// The ratio of the length of the adjacent side to the length of the
    /// hypotenuse of a right angle triangle.
    ///
    /// ~~~
    /// cos(self) = adjacent / hypotenuse
    /// ~~~
    ///
    /// Where `self` is an angle in radians.
    fn cos(&self) -> Self;
    /// The ratio of the length of the opposite side to the length of the
    /// adjacent side of a right angle triangle.
    ///
    /// ~~~
    /// tan(self) = opposite / adjacent
    /// ~~~
    ///
    /// Where `self` is an angle in radians.
    fn tan(&self) -> Self;
    /// The arcsine of a number.
    fn asin(&self) -> Self;
    /// The arccosine of a number.
    fn acos(&self) -> Self;
    /// The arctangent of a number.
    fn atan(&self) -> Self;
    /// The four quadrant arctangent of a number, `y`, and another number `x`.
    ///
    /// This can be defined as either:
    ///
    /// ~~~
    /// atan2(x, y) = 2 * atan(y / (√(x² + y²) + x))
    /// ~~~
    ///
    /// or:
    ///
    /// ~~~
    /// x > 0           atan(y / x)
    /// y ≥ 0, x < 0    π + atan(y / x)
    /// y < 0, x < 0    -π + atan(y / x)
    /// y > 0, x = 0    π / 2
    /// y < 0, x = 0    -π / 2
    /// y = 0, x = 0    undefined
    /// ~~~
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
