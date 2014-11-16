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
//! ~~~notrust
//!
//! |(• ◡•)|ノ〵(❍ᴥ❍⋃)     - "ALGEBRAIC!!!"
//!
/// ~~~

use ident;
use ident::{AdditiveIdentity, MultiplicativeIdentity};
use ops::Recip;
use ops::{AssociativeAdd, CommutativeAdd};
use ops::{AssociativeMul, CommutativeMul};
use ops::DistributiveMulAdd;

////////////////////////////////////////////////////////////////////////////////
// Additive Algebraic Structures
////////////////////////////////////////////////////////////////////////////////

/// A set that is closed over the addition operator.
///
/// ~~~notrust
/// Closure
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// ~~~
pub trait AdditiveMagma
    : Add<Self, Self> {}

impl<T: Add<T, T>> AdditiveMagma for T {}

/// A set that is equipped with an associative addition operator.
///
/// ~~~notrust
/// Closure
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// ~~~
pub trait AdditiveSemiGroup
    : AdditiveMagma
    + AssociativeAdd {
}

impl<T: AdditiveMagma + AssociativeAdd> AdditiveSemiGroup for T {}

/// A set that is equipped with an associative addition operator and a
/// corresponding identity.
///
/// ~~~notrust
/// Closure
///     a + b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// Identity
///     a + 0 = a                           ∀ a ∈ Self
///     0 + a = a                           ∀ a ∈ Self
/// ~~~
pub trait AdditiveMonoid
    : AdditiveSemiGroup
    + AdditiveIdentity {
}

impl<T: AdditiveSemiGroup + AdditiveIdentity> AdditiveMonoid for T {}

/// A set that is equipped with an associative addition operator and a
/// corresponding identity and inverse.
///
/// ~~~notrust
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
pub trait AdditiveGroup
    : AdditiveMonoid
    + Sub<Self, Self>
    + Neg<Self> {
}

impl<T: AdditiveMonoid + Sub<T, T> + Neg<T>> AdditiveGroup for T {}

/// A set that is equipped with an associative and commutative addition operator
/// and a corresponding identity and inverse.
///
/// ~~~notrust
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
pub trait AdditiveAbelianGroup
    : AdditiveGroup
    + CommutativeAdd {
}

impl<T: AdditiveGroup + CommutativeAdd> AdditiveAbelianGroup for T {}

////////////////////////////////////////////////////////////////////////////////
// Multiplicative Algebraic Structures
////////////////////////////////////////////////////////////////////////////////

/// A set that is closed over the multiplication operator.
///
/// ~~~notrust
/// Closure
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// ~~~
pub trait MultiplicativeMagma
    : Mul<Self, Self> {}

impl<T: Mul<T, T>> MultiplicativeMagma for T {}

/// A set that is equipped with an associative multiplication operator.
///
/// ~~~notrust
/// Closure
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// ~~~
pub trait MultiplicativeSemiGroup
    : MultiplicativeMagma
    + AssociativeMul {
}

impl<T: MultiplicativeMagma + AssociativeMul> MultiplicativeSemiGroup for T {}

/// A set that is equipped with an associative multiplication operator and a
/// corresponding identity.
///
/// ~~~notrust
/// Closure
///     a * b ∈ Self                        ∀ a, b ∈ Self
/// Associativity
///     (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// Identity
///     a * 1 = a                           ∀ a ∈ Self
///     1 * a = a                           ∀ a ∈ Self
/// ~~~
pub trait MultiplicativeMonoid
    : MultiplicativeSemiGroup
    + MultiplicativeIdentity {
}

impl<T: MultiplicativeSemiGroup + MultiplicativeIdentity> MultiplicativeMonoid for T {}

/// A set that is equipped with an associative multiplication operator and a
/// corresponding identity and inverse.
///
/// ~~~notrust
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
pub trait MultiplicativeGroup
    : MultiplicativeMonoid
    + Div<Self, Self>
    + Recip {
}

impl<T: MultiplicativeMonoid + Div<T, T> + Recip> MultiplicativeGroup for T {}

/// A set that is equipped with an associative and commutative multiplication
/// operator and a corresponding identity and inverse.
///
/// ~~~notrust
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
/// ~~~notrust
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
/// ~~~notrust
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
/// ~~~notrust
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
/// ~~~notrust
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
    /// ~~~notrust
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
    /// ~~~notrust
    /// sin(self) = opposite / hypotenuse
    /// ~~~
    ///
    /// Where `self` is an angle in radians.
    fn sin(&self) -> Self;
    /// The ratio of the length of the adjacent side to the length of the
    /// hypotenuse of a right angle triangle.
    ///
    /// ~~~notrust
    /// cos(self) = adjacent / hypotenuse
    /// ~~~
    ///
    /// Where `self` is an angle in radians.
    fn cos(&self) -> Self;
    /// The ratio of the length of the opposite side to the length of the
    /// adjacent side of a right angle triangle.
    ///
    /// ~~~notrust
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
    /// ~~~notrust
    /// atan2(x, y) = 2 * atan(y / (√(x² + y²) + x))
    /// ~~~
    ///
    /// or:
    ///
    /// ~~~notrust
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
    /// ~~~notrust
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

/// Sets that form a boolean algebra.
///
/// These must contain exactly two elements, top and bottom: `{⊤, ⊥}`, and be
/// equipped with three basic operators: `¬`, `∧`, and `∨`, the definitions of
/// which are outlined below.
pub trait Boolean: Eq {
    /// The bottom value, `⊥`.
    fn bottom() -> Self;

    /// The top value, `⊤`.
    #[inline]
    fn top() -> Self { not(&bottom()) }

    /// The logical complement, `¬`.
    ///
    /// # Truth table
    ///
    /// ~~~notrust
    ///   p        ¬p
    /// +--------+--------+
    /// | top    | bottom |
    /// | bottom | top    |
    /// +--------+--------+
    /// ~~~
    fn not(p: &Self) -> Self;

    /// Logical conjunction, `∧`.
    ///
    /// # Truth table
    ///
    /// ~~~notrust
    ///   p        q        p ∧ q
    /// +-----------------+--------+
    /// | top      top    | top    |
    /// | top      bottom | bottom |
    /// | bottom   top    | bottom |
    /// | bottom   bottom | bottom |
    /// +-----------------+--------+
    /// ~~~
    fn and(p: &Self, q: &Self) -> Self;

    /// Logical disjunction, `∨`.
    ///
    /// # Truth table
    ///
    /// ~~~notrust
    ///   p        q        p ∨ q
    /// +-----------------+--------+
    /// | top      top    | top    |
    /// | top      bottom | top    |
    /// | bottom   top    | top    |
    /// | bottom   bottom | bottom |
    /// +-----------------+--------+
    /// ~~~
    fn or(p: &Self, q: &Self) -> Self;

    /// Exclusive disjunction, `⊕`, where:
    ///
    /// ~~~notrust
    /// p ⊕ q = (p ∨ q) ∧ ¬(p ∧ q)
    /// ~~~
    ///
    /// # Truth table
    ///
    /// ~~~notrust
    ///   p        q        p ⊕ q
    /// +-----------------+--------+
    /// | top      top    | bottom |
    /// | top      bottom | top    |
    /// | bottom   top    | top    |
    /// | bottom   bottom | bottom |
    /// +-----------------+--------+
    /// ~~~
    #[inline]
    fn xor(p: &Self, q: &Self) -> Self { and(&or(p, q), &not(&and(p, q))) }

    /// Material implication, `→`, where:
    ///
    /// ~~~notrust
    /// p → q = ¬p ∨ q
    /// ~~~
    ///
    /// # Truth table
    ///
    /// ~~~notrust
    ///   p        q        p → q
    /// +-----------------+--------+
    /// | top      top    | top    |
    /// | top      bottom | bottom |
    /// | bottom   top    | top    |
    /// | bottom   bottom | top    |
    /// +-----------------+--------+
    /// ~~~
    #[inline]
    fn implies(p: &Self, q: &Self) -> Self { or(&not(p), q) }

    /// Material biconditional, `≡`, where:
    ///
    /// ~~~notrust
    /// p ≡ q = ¬(p ⊕ q)
    /// ~~~
    ///
    /// # Truth table
    ///
    /// ~~~notrust
    ///   p        q        p ≡ q
    /// +-----------------+--------+
    /// | top      top    | top    |
    /// | top      bottom | bottom |
    /// | bottom   top    | bottom |
    /// | bottom   bottom | top    |
    /// +-----------------+--------+
    /// ~~~
    #[inline]
    fn iff(p: &Self, q: &Self) -> Self { not(&xor(p, q)) }

    /// Converts a value to the corresponding `⊥` and `⊤` value from
    /// another boolean algebra.
    #[inline]
    fn to_boolean<T: Boolean>(&self) -> T {
        if *self == top() { top() } else { bottom() }
    }

    /// Converts the value either `zero` or `unit` in a set that has those
    /// elements defined.
    ///
    /// # Example
    ///
    /// ~~~notrust
    /// use num::Boolean;
    ///
    /// assert_eq!(true.to_bit::<u8>(), 1);
    /// assert_eq!(false.to_bit::<u8>(), 0);
    /// ~~~
    #[inline]
    fn to_bit<T: AdditiveIdentity + MultiplicativeIdentity>(&self) -> T {
        if *self == top() { ident::unit() } else { ident::zero() }
    }
}

/// The truth value, `⊤`.
#[inline] pub fn top<T: Boolean>() -> T { Boolean::top() }
/// The false value, `⊥`.
#[inline] pub fn bottom<T: Boolean>() -> T { Boolean::bottom() }
/// The logical complement, `¬`.
#[inline] pub fn not<T: Boolean>(p: &T) -> T { Boolean::not(p) }
/// Logical conjunction, `∧`.
#[inline] pub fn and<T: Boolean>(p: &T, q: &T) -> T { Boolean::and(p, q) }
/// Logical disjunction, `∨`.
#[inline] pub fn or<T: Boolean>(p: &T, q: &T) -> T { Boolean::or(p, q) }
/// Exclusive disjunction, `⊕`.
#[inline] pub fn xor<T: Boolean>(p: &T, q: &T) -> T { Boolean::xor(p, q) }
/// Material implication, `→`.
#[inline] pub fn implies<T: Boolean>(p: &T, q: &T) -> T { Boolean::implies(p, q) }
/// Logical biconditional, `≡`.
#[inline] pub fn iff<T: Boolean>(p: &T, q: &T) -> T { Boolean::iff(p, q) }

impl Boolean for bool {
    #[inline] fn bottom() -> bool { false }
    #[inline] fn top() -> bool { true }
    #[inline] fn not(p: &bool) -> bool { !*p }
    #[inline] fn and(p: &bool, q: &bool) -> bool { *p & *q }
    #[inline] fn or(p: &bool, q: &bool) -> bool { *p | *q }
    #[inline] fn xor(p: &bool, q: &bool) -> bool { *p ^ *q }
    #[inline] fn iff(p: &bool, q: &bool) -> bool { *p == *q }
}

#[cfg(test)]
mod test_boolean {
    use super::{Boolean, top, bottom, not, and, or, xor, iff};

    #[test]
    fn test_values() {
        assert_eq!(top::<bool>(), true);
        assert_eq!(bottom::<bool>(), false);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(&true),  false);
        assert_eq!(not(&false), true);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(&true,  &true),  true);
        assert_eq!(and(&true,  &false), false);
        assert_eq!(and(&false, &true),  false);
        assert_eq!(and(&false, &false), false);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(&true,  &true),  true);
        assert_eq!(or(&true,  &false), true);
        assert_eq!(or(&false, &true),  true);
        assert_eq!(or(&false, &false), false);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(&true,  &true),  false);
        assert_eq!(xor(&true,  &false), true);
        assert_eq!(xor(&false, &true),  true);
        assert_eq!(xor(&false, &false), false);
    }

    #[test]
    fn test_iff() {
        assert_eq!(iff(&true,  &true),  true);
        assert_eq!(iff(&true,  &false), false);
        assert_eq!(iff(&false, &true),  false);
        assert_eq!(iff(&false, &false), true);
    }

    macro_rules! assert_demorgans_laws(
        ($p:expr, $q: expr) => {{
            let (p, q) = ($p, $q);
            assert_eq!(not(&or(&p, &q)), and(&not(&p), &not(&q)));
            assert_eq!(not(&and(&p, &q)), or(&not(&p), &not(&q)));
        }}
    )

    #[test]
    fn test_demorgans_laws() {
        assert_demorgans_laws!(true,  true);
        assert_demorgans_laws!(true,  false);
        assert_demorgans_laws!(false, true);
        assert_demorgans_laws!(false, false);
    }

    #[test]
    fn test_to_boolean() {
        assert_eq!(true.to_boolean::<bool>(),  true);
        assert_eq!(false.to_boolean::<bool>(), false);
    }

    #[test]
    fn test_to_bit() {
        assert_eq!(true.to_bit::<u8>(),  1);
        assert_eq!(false.to_bit::<u8>(), 0);
    }

    #[deriving(PartialEq, Eq, Show)]
    enum B { T, F }

    impl Boolean for B {
        fn bottom() -> B { F }
        fn not(p: &B) -> B {
            match *p {
                T => F,
                F => T,
            }
        }
        fn and(p: &B, q: &B) -> B {
            match (*p, *q) {
                (T, T) => T,
                (T, F) => F,
                (F, T) => F,
                (F, F) => F,
            }
        }
        fn or(p: &B, q: &B) -> B {
            match (*p, *q) {
                (T, T) => T,
                (T, F) => T,
                (F, T) => T,
                (F, F) => F,
            }
        }
    }

    #[test]
    fn test_derived() {
        assert_eq!(xor(&T, &T), F);
        assert_eq!(xor(&T, &F), T);
        assert_eq!(xor(&F, &T), T);
        assert_eq!(xor(&F, &F), F);

        assert_eq!(iff(&T, &T), T);
        assert_eq!(iff(&T, &F), F);
        assert_eq!(iff(&F, &T), F);
        assert_eq!(iff(&F, &F), T);

        assert_demorgans_laws!(T, T);
        assert_demorgans_laws!(T, F);
        assert_demorgans_laws!(F, T);
        assert_demorgans_laws!(F, F);

        assert_eq!(T.to_boolean::<bool>(), true);
        assert_eq!(F.to_boolean::<bool>(), false);

        assert_eq!(T.to_bit::<u8>(), 1);
        assert_eq!(F.to_bit::<u8>(), 0);
    }
}
