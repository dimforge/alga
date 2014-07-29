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

//! IEEE 754-2008 compliant floating-point arithmetic

// TODO: setBinaryRoundingDirection
// TODO: getBinaryRoundingDirection
// TODO: setDecimalRoundingDirection
// TODO: getDecimalRoundingDirection

// TODO: saveModes
// TODO: restoreModes
// TODO: defaultModes

// TODO: lowerFlags
// TODO: raiseFlags
// TODO: testFlags
// TODO: testSavedFlags
// TODO: restoreFlags
// TODO: saveAllFlags

pub enum FloatClass {
    FloatZero,
    FloatSubnormal,
    FloatInfinite,
    FloatNaN,
    FloatNormal,
}

#[repr(u8)]
#[deriving(PartialEq, Eq)]
pub enum Radix {
    Radix2  = 2,
    Radix10 = 10,
}

/// A floating point number that conforms to specifications laid out in the IEEE
/// Standard for Floating-Point Arithmetic.
pub trait Ieee754<LogBFormat, IntegralFormat> {
    fn roundToIntegralTiesToEven(self) -> Self;
    fn roundToIntegralTiesToAway(self) -> Self;
    fn roundToIntegralTowardZero(self) -> Self;
    fn roundToIntegralTowardPositive(self) -> Self;
    fn roundToIntegralTowardNegative(self) -> Self;

    fn roundToIntegralExact(self) -> Self;

    fn nextDown(self) -> Self;
    fn nextUp(self) -> Self;

    fn remainder(x: Self, y: Self) -> Self;

    fn maxNum(x: Self, y: Self) -> Self;
    fn maxNumMag(x: Self, y: Self) -> Self;
    fn minNum(x: Self, y: Self) -> Self;
    fn minNumMag(x: Self, y: Self) -> Self;

    fn quantize(x: Self, y: Self) -> Self;

    fn scaleB(self, n: LogBFormat) -> Self;
    fn logB(self) -> LogBFormat;

    fn addition(x: Self, y: Self) -> Self;
    fn subtraction(x: Self, y: Self) -> Self;
    fn multiplication(x: Self, y: Self) -> Self;
    fn division(x: Self, y: Self) -> Self;
    fn fusedMultiplyAdd(x: Self, y: Self, z: Self) -> Self;
    fn squareRoot(self) -> Self;
    fn convertFromInt(x: int) -> Self;

    fn convertToIntegerTiesToEven(self) -> IntegralFormat;
    fn convertToIntegerTowardZero(self) -> IntegralFormat;
    fn convertToIntegerTowardPositive(self) -> IntegralFormat;
    fn convertToIntegerTowardNegative(self) -> IntegralFormat;
    fn convertToIntegerTiesToAway(self) -> IntegralFormat;

    fn convertToIntegerExactTiesToEven(self) -> IntegralFormat;
    fn convertToIntegerExactTowardZero(self) -> IntegralFormat;
    fn convertToIntegerExactTowardPositive(self) -> IntegralFormat;
    fn convertToIntegerExactTowardNegative(self) -> IntegralFormat;
    fn convertToIntegerExactTiesToAway(self) -> IntegralFormat;

    // TODO: convertFormat
    // TODO: convertFromDecimalCharacter
    // TODO: convertToDecimalCharacter

    // TODO: convertFromHexCharacter
    // TODO: convertToHexCharacter

    /// Copies a floating-point operand x to a destination in the same format,
    /// with no change to the sign bit.
    ///
    /// This is practically the same as `Clone::clone`, but is included to
    /// reflect the standard.
    fn copy(self) -> Self;

    /// Copies the floating point number with the sign bit inverted. Note that
    /// `x.negate()` is not always the same as `Ieee754::subtraction(0.0, x)`.
    fn negate(self) -> Self;

    /// Copies the floating point number with the sign bit set to `0`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// |x|
    /// ~~~
    fn abs(self) -> Self;
    fn copySign(self, src: Self) -> Self;

    // TODO: encodeDecimal
    // TODO: decodeDecimal
    // TODO: encodeBinary
    // TODO: decodeBinary

    fn compareQuietEqual(x: Self, y: Self) -> bool;
    fn compareQuietGreater(x: Self, y: Self) -> bool;
    fn compareQuietGreaterEqual(x: Self, y: Self) -> bool;
    fn compareQuietGreaterUnordered(x: Self, y: Self) -> bool;
    fn compareQuietLess(x: Self, y: Self) -> bool;
    fn compareQuietLessEqual(x: Self, y: Self) -> bool;
    fn compareQuietLessUnordered(x: Self, y: Self) -> bool;
    fn compareQuietNotEqual(x: Self, y: Self) -> bool;
    fn compareQuietNotGreater(x: Self, y: Self) -> bool;
    fn compareQuietNotLess(x: Self, y: Self) -> bool;
    fn compareQuietOrdered(x: Self, y: Self) -> bool;
    fn compareQuietUnordered(x: Self, y: Self) -> bool;
    fn compareSignalingEqual(x: Self, y: Self) -> bool;
    fn compareSignalingGreater(x: Self, y: Self) -> bool;
    fn compareSignalingGreaterEqual(x: Self, y: Self) -> bool;
    fn compareSignalingGreaterUnordered(x: Self, y: Self) -> bool;
    fn compareSignalingLess(x: Self, y: Self) -> bool;
    fn compareSignalingLessEqual(x: Self, y: Self) -> bool;
    fn compareSignalingLessUnordered(x: Self, y: Self) -> bool;
    fn compareSignalingNotEqual(x: Self, y: Self) -> bool;
    fn compareSignalingNotGreater(x: Self, y: Self) -> bool;
    fn compareSignalingNotLess(x: Self, y: Self) -> bool;

    /// Returns `true` if and only if the implementation of the floating point
    /// type conforms to the earlier IEEE 754-1985 standard.
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `is754version1985` in section 5.7.1 of the IEEE 754-2008
    /// Standard for Floating-Point Arithmetic.
    fn is754version1985(_: Option<Self>) -> bool;

    /// Returns `true` if and only if the implementation of the floating point
    /// type conforms to the current IEEE 754-2008 standard.
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `is754version2008` in section 5.7.1 of the IEEE 754-2008
    /// Standard for Floating-Point Arithmetic.
    fn is754version2008(_: Option<Self>) -> bool;

    /// Returns the floating point class of `self`.
    fn class(self) -> FloatClass;
    /// Returns `true` if and only if `self` has negative sign (applies to zeros
    /// and NaNs as well).
    fn isSignMinus(self) -> bool;
    /// Returns `true` if and only if `self` is normal (not zero, subnormal,
    /// infinite, or NaN).
    fn isNormal(self) -> bool;
    /// Returns `true` if and only if `self` is zero, subnormal or normal (not
    /// infinite or NaN).
    fn isFinite(self) -> bool;
    /// Returns `true` if and only if `self` is ±0.
    fn isZero(self) -> bool;
    /// Returns `true` if and only if `self` is subnormal.
    fn isSubnormal(self) -> bool;
    /// Returns `true` if and only if `self` is infinite.
    fn isInfinite(self) -> bool;
    /// Returns `true` if and only if `self` is a NaN.
    fn isNaN(self) -> bool;
    /// Returns `true` if and only if `self` is a signalling NaN.
    fn isSignaling(self) -> bool;
    /// Returns `true` if and only if `self` is a finite number, infinity, or a 
    /// NaN that is canonical.
    fn isCanonical(self) -> bool;
    /// The radix of the format of `self`.
    fn radix(self) -> Radix;
    ///
    fn totalOrder(x: Self, y: Self) -> bool;
    /// `Ieee754::totalOrder(x.abs(), y.abs())`.
    fn totalOrderMag(x: Self, y: Self) -> bool;

    fn sameQuantum(x: Self, y: Self) -> bool;

    /// Computes `e` (Euler's number) raised to the exponent `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - an exponent in the domain `[−∞, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// e^self
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `exp` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn exp(self) -> Self;

    /// Computes `e` (Euler's number) raised to the exponent `self`, minus `1`.
    /// The returned value is more accurate than `x.exp() - 1.0` when the value
    /// of `x` is close to zero.
    ///
    /// # Parameters
    ///
    /// * `self` - an exponent in the domain `[−∞, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// e^self - 1
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `expm1` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn expm1(self) -> Self;

    /// Computes `2` raised to the exponent `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - an exponent in the domain `[−∞, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// 2^self
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `exp2` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn exp2(self) -> Self;

    /// Computes `2` raised to the exponent `self`, minus `1`. The returned
    /// value is more accurate than `x.exp2() - 1.0` when the value of `x` is
    /// close to zero.
    ///
    /// # Parameters
    ///
    /// * `self` - an exponent in the domain `[−∞, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// 2^self - 1
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `exp2m1` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn exp2m1(self) -> Self;

    /// Computes `10` raised to the exponent `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - an exponent in the domain `[−∞, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// 10^self
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `exp10` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn exp10(self) -> Self;

    /// Computes `10` raised to the exponent `self`, minus `1`. The returned
    /// value is more accurate than `x.exp10() - 1.0` when the value of `x` is
    /// close to zero.
    ///
    /// # Parameters
    ///
    /// * `self` - an exponent in the domain `[−∞, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// 10^self - 1
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `exp10m1` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn exp10m1(self) -> Self;

    /// Computes the natural (base `e`) logarithm of `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - a number in the domain `[0, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// log_e(self)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `log` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn log(self) -> Self;

    /// Computes the base `2` logarithm of `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - a number in the domain `[0, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// log_2(self)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `log2` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn log2(self) -> Self;

    /// Computes the base `10` logarithm of `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - a number in the domain `[0, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// log_10(self)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `log10` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn log10(self) -> Self;

    /// Computes the natural (base `e`) logarithm of `1 + self`. The returned
    /// value is more accurate than `(x + 1.0).log()` when the value of `x` is
    /// close to zero.
    ///
    /// # Parameters
    ///
    /// * `self` - a number in the domain `[-1, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// log_e(1 + self)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `logp1` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn logp1(self) -> Self;

    /// Computes the base `2` logarithm of `1 + self`. The returned value is
    /// more accurate than `(x + 1.0).log2()` when the value of `x` is close to
    /// zero.
    ///
    /// # Parameters
    ///
    /// * `self` - a number in the domain `[-1, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// log_2(1 + self)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `log2p1` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn log2p1(self) -> Self;

    /// Computes the base `10` logarithm of `1 + self`. The returned value is
    /// more accurate than `(x + 1.0).log10()` when the value of `x` is close to
    /// zero.
    ///
    /// # Parameters
    ///
    /// * `self` - a number in the domain `[-1, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// log_10(1 + self)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `log10p1` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn log10p1(self) -> Self;

    /// Computes the square root of the sum of the squares of `x` and `y`
    /// without the undue overflow or underflow that could result from
    /// performing `(x.pown(2) + y.pown(2)).sqrt()` directly.
    ///
    /// # Parameters
    ///
    /// * `x` - a number in the domain `[−∞, +∞]`.
    /// * `y` - a number in the domain `[−∞, +∞]`.
    ///
    /// # Return value
    ///
    /// The length of the hypotenuse of a right-angle triangle with sides of
    /// length `x` and `y`.
    ///
    /// ~~~
    /// √(x^2 + y^2)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `hypot` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn hypot(x: Self, y: Self) -> Self;

    /// Computes the inverse of the square root of `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - a number in the domain `[0, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// 1/√(self)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `rSqrt` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn rsqrt(self) -> Self;

    fn compound(self, n: LogBFormat);

    /// Computes the inverse of the square root of `self`.
    ///
    /// # Parameters
    ///
    /// * `self` - a number in the domain `[0, +∞]`.
    ///
    /// # Return value
    ///
    /// ~~~
    /// self^(1/n)
    /// ~~~
    ///
    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `rootn` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn rootn(self, n: LogBFormat) -> Self;

    /// Computes `self` raised to an exponent `n`.
    ///
    /// # Parameters
    fn pown(self, n: LogBFormat) -> Self;
    fn pow(self, n: Self) -> Self;
    fn powr(self, n: Self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;

    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `sinPi` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn sin_pi(self) -> Self;

    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `cosPi` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn cos_pi(self) -> Self;

    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `atanPi` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn atan_pi(self) -> Self;

    /// # IEEE 754-2008 conformance
    ///
    /// The implementation of this function should conform to the specification
    /// given for `atan2Pi` in section 9.2 of the IEEE 754-2008 Standard for
    /// Floating-Point Arithmetic.
    fn atan2_pi(x: Self, y: Self) -> Self;

    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(x: Self, y: Self) -> Self;

    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;

    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;

    // TODO: sum
    // TODO: dot
    // TODO: sumSquare
    // TODO: sumAbs

    // TODO: scaledProd
    // TODO: scaledProdSum
    // TODO: scaledProdDiff
}
