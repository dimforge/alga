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

pub use std::num::{Zero, One, zero, one};
use ops::{AssociativeAdd, CommutativeAdd};
use ops::{AssociativeMul, CommutativeMul};
use ops::DistributiveMulAdd;

////////////////////////////////////////////////////////////////////////////////
// Additive Algebraic Structures
////////////////////////////////////////////////////////////////////////////////

/// A set that is equipped with an addition operator.
///
/// ~~~
/// Closure
///     a + b ∈ Self                         ∀ a, b ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Add<Self, Self>`
pub trait AdditiveMagma
    : Add<Self, Self> {}

impl<T: Add<T, T>> AdditiveMagma for T {}

/// A set that is equipped with an associative addition operator.
///
/// ~~~
/// Closure
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Add<Self, Self>`
/// * `AssociativeAdd`
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
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// Identity
///     a + 0 = a                           ∀ a ∈ Self
///     0 + a = a                           ∀ a ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Add<Self, Self>`
/// * `AssociativeAdd`
/// * `Zero`
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
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// Identity
///     a + 0 = a                           ∀ a ∈ Self
///     0 + a = a                           ∀ a ∈ Self
/// Inverse
///     a + -a = 0                          ∀ a ∈ Self
///     -a + a = 0                          ∀ a ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Add<Self, Self>`
/// * `AssociativeAdd`
/// * `Zero`
/// * `Sub<Self, Self>`
/// * `Neg<Self>`
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
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// Commutativity
///     a + b = b + a                       ∀ a, b ∈ Self
/// Identity
///     a + 0 = a                           ∀ a ∈ Self
///     0 + a = a                           ∀ a ∈ Self
/// Inverse
///     a + -a = 0                          ∀ a ∈ Self
///     -a + a = 0                          ∀ a ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Add<Self, Self>`
/// * `AssociativeAdd`
/// * `CommutativeAdd`
/// * `Zero`
/// * `Sub<Self, Self>`
/// * `Neg<Self>`
pub trait AdditiveAbelianGroup
    : AdditiveGroup
    + CommutativeAdd {
}

impl<T: AdditiveGroup + CommutativeAdd> AdditiveAbelianGroup for T {}

////////////////////////////////////////////////////////////////////////////////
// Multiplicative Algebraic Structures
////////////////////////////////////////////////////////////////////////////////

/// The multiplicative inverse operation
pub trait Recip {
    fn recip(&self) -> Self;
}

impl Recip for f32 { #[inline] fn recip(&self) -> f32 { 1.0 / *self } }
impl Recip for f64 { #[inline] fn recip(&self) -> f64 { 1.0 / *self } }

/// A set that is equipped with a multiplication operator.
///
/// ~~~
/// Closure
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Mul<Self, Self>`
pub trait MultiplicativeMagma
    : Mul<Self, Self> {}

impl<T: Mul<T, T>> MultiplicativeMagma for T {}

/// A set that is equipped with an associative multiplication operator.
///
/// ~~~
/// Closure
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Mul<Self, Self>`
/// * `AssociativeMul`
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
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// Identity
///     a * 1 = a                           ∀ a ∈ Self
///     1 * a = a                           ∀ a ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Mul<Self, Self>`
/// * `AssociativeMul`
/// * `One`
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
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// Identity
///     a * 1 = a                           ∀ a ∈ Self
///     1 * a = a                           ∀ a ∈ Self
/// Inverse
///     a * a⁻¹ = 1                         ∀ a ∈ Self
///     a⁻¹ * a = 1                         ∀ a ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Mul<Self, Self>`
/// * `AssociativeMul`
/// * `One`
/// * `Div<Self, Self>`
/// * `Recip`
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
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// Commutativity
///     a * b = b * a                       ∀ a, b ∈ Self
/// Identity
///     a * 1 = a                           ∀ a ∈ Self
///     1 * a = a                           ∀ a ∈ Self
/// Inverse
///     a * a⁻¹ = 1                         ∀ a ∈ Self
///     a⁻¹ * a = 1                         ∀ a ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Mul<Self, Self>`
/// * `AssociativeMul`
/// * `CommutativeMul`
/// * `One`
/// * `Div<Self, Self>`
/// * `Recip`
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
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity of +
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// Commutativity of +
///     a + b = b + a                       ∀ a, b ∈ Self
/// Identity of +
///     a + 0 = a                           ∀ a ∈ Self
///     0 + a = a                           ∀ a ∈ Self
/// Inverse of +
///     a + -a = 0                          ∀ a ∈ Self
///     -a + a = 0                          ∀ a ∈ Self
/// Closure of *
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity of *
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// Identity of *
///     a * 1 = a                           ∀ a ∈ Self
///     1 * a = a                           ∀ a ∈ Self
/// Distributivity of * over +
///     a * (b + c) = (a * b) + (a * c)     ∀ a, b, c ∈ Self
///     (a + b) * c = (a * c) + (b * c)     ∀ a, b, c ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Add<Self, Self>`
/// * `AssociativeAdd`
/// * `CommutativeAdd`
/// * `Zero`
/// * `Sub<Self, Self>`
/// * `Neg<Self>`
/// * `Mul<Self, Self>`
/// * `AssociativeMul`
/// * `One`
/// * `DistributiveMulAdd`
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
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity of +
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// Commutativity of +
///     a + b = b + a                       ∀ a, b ∈ Self
/// Identity of +
///     a + 0 = a                           ∀ a ∈ Self
///     0 + a = a                           ∀ a ∈ Self
/// Inverse of +
///     a + -a = 0                          ∀ a ∈ Self
///     -a + a = 0                          ∀ a ∈ Self
/// Closure of *
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity of *
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// Commutativity of *
///     a * b = b * a                       ∀ a, b ∈ Self
/// Identity of *
///     a * 1 = a                           ∀ a ∈ Self
///     1 * a = a                           ∀ a ∈ Self
/// Distributivity of * over +
///     a * (b + c) = (a * b) + (a * c)     ∀ a, b, c ∈ Self
///     (a + b) * c = (a * c) + (b * c)     ∀ a, b, c ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Add<Self, Self>`
/// * `AssociativeAdd`
/// * `CommutativeAdd`
/// * `Zero`
/// * `Sub<Self, Self>`
/// * `Neg<Self>`
/// * `Mul<Self, Self>`
/// * `AssociativeMul`
/// * `CommutativeMul`
/// * `One`
/// * `DistributiveMulAdd`
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
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity of +
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// Commutativity of +
///     a + b = b + a                       ∀ a, b ∈ Self
/// Identity of +
///     a + 0 = a                           ∀ a ∈ Self
///     0 + a = a                           ∀ a ∈ Self
/// Inverse of +
///     a + -a = 0                          ∀ a ∈ Self
///     -a + a = 0                          ∀ a ∈ Self
/// Closure of *
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity of *
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// Commutativity of *
///     a * b = b * a                       ∀ a, b ∈ Self
/// Identity of *
///     a * 1 = a                           ∀ a ∈ Self
///     1 * a = a                           ∀ a ∈ Self
/// Inverse of *
///     a * a⁻¹ = 1                         ∀ a ∈ Self where a ≠ 0
///     a⁻¹ * a = 1                         ∀ a ∈ Self where a ≠ 0
/// Distributivity of * over +
///     a * (b + c) = (a * b) + (a * c)     ∀ a, b, c ∈ Self
///     (a + b) * c = (a * c) + (b * c)     ∀ a, b, c ∈ Self
/// ~~~
///
/// This trait is implemented globally for all types that satisfy:
///
/// * `Add<Self, Self>`
/// * `AssociativeAdd`
/// * `CommutativeAdd`
/// * `Zero`
/// * `Sub<Self, Self>`
/// * `Neg<Self>`
/// * `Mul<Self, Self>`
/// * `AssociativeMul`
/// * `CommutativeMul`
/// * `One`
/// * `Div<Self, Self>`
/// * `Recip`
/// * `DistributiveMulAdd`
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
/// abs((signum(a)) = 1                     ∀ a ∈ Self where a ≠ 0
/// abs(a) * signum(a) = a                  ∀ a ∈ Self
/// ~~~
pub trait Absolute
    : CommutativeRing {
    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
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
    : Algebraic
    + Trigonometric {
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
