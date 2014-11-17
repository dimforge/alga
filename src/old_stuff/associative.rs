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

//! Associative operators

use approx::ApproxEq;
use structure::AdditiveClosure;
use structure::MultiplicativeClosure;

/// A type that is equipped with an approximately associative adiition operator.
///
/// ~~~notrust
/// (a + b) + c ≈ a + (b + c)               ∀ a, b, c ∈ Self
/// ~~~
pub trait ApproxAssociativeAdd
    : AdditiveClosure
    + PartialEq
{
    /// Returns `true` if the addition operator is approximately associative for
    /// the given argument tuple.
    fn prop_add_is_approx_associative(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        (a + b) + c == a + (b + c)
    }
}

impl ApproxAssociativeAdd for u8   {}
impl ApproxAssociativeAdd for u16  {}
impl ApproxAssociativeAdd for u32  {}
impl ApproxAssociativeAdd for u64  {}
impl ApproxAssociativeAdd for uint {}
impl ApproxAssociativeAdd for i8   {}
impl ApproxAssociativeAdd for i16  {}
impl ApproxAssociativeAdd for i32  {}
impl ApproxAssociativeAdd for i64  {}
impl ApproxAssociativeAdd for int  {}

/// A type that is equipped with an associative adiition operator.
///
/// ~~~notrust
/// (a + b) + c = a + (b * c)               ∀ a, b, c ∈ Self
/// ~~~
pub trait AssociativeAdd
    : ApproxAssociativeAdd
{
    /// Returns `true` if the addition operator is associative for the given
    /// argument tuple.
    fn prop_add_is_associative(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        (a + b) + c == a + (b + c)
    }
}

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

/// A type that is equipped with an approximately associative multiplication
/// operator.
///
/// ~~~notrust
/// (a * b) * c ≈ a * (b * c)               ∀ a, b, c ∈ Self
/// ~~~
pub trait ApproxAssociativeMul
    : MultiplicativeClosure
    + PartialEq
{
    /// Returns `true` if the multiplication operator is approximately
    /// associative for the given argument tuple.
    fn prop_mul_is_approx_associative(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        (a * b) * c == a * (b * c)
    }
}

impl ApproxAssociativeMul for u8   {}
impl ApproxAssociativeMul for u16  {}
impl ApproxAssociativeMul for u32  {}
impl ApproxAssociativeMul for u64  {}
impl ApproxAssociativeMul for uint {}
impl ApproxAssociativeMul for i8   {}
impl ApproxAssociativeMul for i16  {}
impl ApproxAssociativeMul for i32  {}
impl ApproxAssociativeMul for i64  {}
impl ApproxAssociativeMul for int  {}

/// A type that is equipped with an associative multiplication operator.
///
/// ~~~notrust
/// (a * b) * c = a * (b * c)               ∀ a, b, c ∈ Self
/// ~~~
pub trait AssociativeMul
    : ApproxAssociativeMul
{
    /// Returns `true` if the multiplication operator is associative for the
    /// given argument tuple.
    fn prop_mul_is_associative(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        (a * b) * c == a * (b * c)
    }
}

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

// Float impls

macro_rules! impl_float_op {
    ($T:ty, $Trait:ident :: $prop:ident, $op:ident, $eps:expr) => {
        impl $Trait for $T {
            fn $prop(args: ($T, $T, $T)) -> bool {
                let (a, b, c) = args;
                ApproxEq::approx_eq_eps(&a.$op(&b).$op(&c),
                                        &a.$op(&b.$op(&c)),
                                        &$eps)
            }
        }
    }
}

impl_float_op!(f32, ApproxAssociativeAdd::prop_add_is_approx_associative, add, 1.0e-4)
impl_float_op!(f32, ApproxAssociativeMul::prop_mul_is_approx_associative, mul, 1.0e-1)
impl_float_op!(f64, ApproxAssociativeAdd::prop_add_is_approx_associative, add, 1.0e-11)
impl_float_op!(f64, ApproxAssociativeMul::prop_mul_is_approx_associative, mul, 1.0e-6)

#[cfg(test)]
mod tests {
    macro_rules! check_float {
        ($T:ident) => {
            mod $T {
                use ops::ApproxAssociativeAdd;
                use ops::ApproxAssociativeMul;

                #[quickcheck]
                fn prop_add_is_approx_associative(args: ($T, $T, $T)) -> bool {
                    ApproxAssociativeAdd::prop_add_is_approx_associative(args)
                }

                #[quickcheck]
                fn prop_mul_is_approx_associative(args: ($T, $T, $T)) -> bool {
                    ApproxAssociativeMul::prop_mul_is_approx_associative(args)
                }
            }
        }
    }
    check_float!(f32)
    check_float!(f64)

    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use ops::ApproxAssociativeAdd;
                use ops::AssociativeAdd;
                use ops::ApproxAssociativeMul;
                use ops::AssociativeMul;

                #[quickcheck]
                fn prop_add_is_approx_associative(args: ($T, $T, $T)) -> bool {
                    ApproxAssociativeAdd::prop_add_is_approx_associative(args)
                }
                #[quickcheck]
                fn prop_add_is_associative(args: ($T, $T, $T)) -> bool {
                    AssociativeAdd::prop_add_is_associative(args)
                }

                #[quickcheck]
                fn prop_mul_is_approx_associative(args: ($T, $T, $T)) -> bool {
                    ApproxAssociativeMul::prop_mul_is_approx_associative(args)
                }
                #[quickcheck]
                fn prop_mul_is_associative(args: ($T, $T, $T)) -> bool {
                    AssociativeMul::prop_mul_is_associative(args)
                }
            }
        }
    }
    check_int!(u8)
    check_int!(u16)
    check_int!(u32)
    check_int!(u64)
    check_int!(uint)
    check_int!(i8)
    check_int!(i16)
    check_int!(i32)
    check_int!(i64)
    check_int!(int)
}
