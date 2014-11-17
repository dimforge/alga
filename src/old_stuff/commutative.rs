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

//! Commutative operators

use approx::ApproxEq;
use structure::AdditiveClosure;
use structure::MultiplicativeClosure;

/// A type that is equipped with an approximately commutative addition operator.
///
/// ~~~notrust
/// a + b ≈ b + a                           ∀ a, b, c ∈ Self
/// ~~~
pub trait ApproxCommutativeAdd
    : AdditiveClosure
    + PartialEq
{
    /// Returns `true` if the addition operator is approximately commutative for
    /// the given argument tuple.
    fn prop_add_is_approx_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a + b == b + a
    }
}

impl ApproxCommutativeAdd for u8   {}
impl ApproxCommutativeAdd for u16  {}
impl ApproxCommutativeAdd for u32  {}
impl ApproxCommutativeAdd for u64  {}
impl ApproxCommutativeAdd for uint {}
impl ApproxCommutativeAdd for i8   {}
impl ApproxCommutativeAdd for i16  {}
impl ApproxCommutativeAdd for i32  {}
impl ApproxCommutativeAdd for i64  {}
impl ApproxCommutativeAdd for int  {}

/// A type that is equipped with a commutative addition operator.
///
/// ~~~notrust
/// a + b = b + a                           ∀ a, b, c ∈ Self
/// ~~~
pub trait CommutativeAdd
    : ApproxCommutativeAdd
{
    /// Returns `true` if the addition operator is commutative for the given
    /// argument tuple.
    fn prop_add_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a + b == b + a
    }
}

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

/// A type that is equipped with an approximately commutative multiplication
/// operator.
///
/// ~~~notrust
/// a * b ≈ b * a                           ∀ a, b, c ∈ Self
/// ~~~
pub trait ApproxCommutativeMul
    : MultiplicativeClosure
    + PartialEq
{
    /// Returns `true` if the multiplication operator is approximately
    /// commutative for the given argument tuple.
    fn prop_mul_is_approx_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a * b == b * a
    }
}

impl ApproxCommutativeMul for u8   {}
impl ApproxCommutativeMul for u16  {}
impl ApproxCommutativeMul for u32  {}
impl ApproxCommutativeMul for u64  {}
impl ApproxCommutativeMul for uint {}
impl ApproxCommutativeMul for i8   {}
impl ApproxCommutativeMul for i16  {}
impl ApproxCommutativeMul for i32  {}
impl ApproxCommutativeMul for i64  {}
impl ApproxCommutativeMul for int  {}

/// A type that is equipped with a commutative multiplication operator.
///
/// ~~~notrust
/// a * b = b * a                           ∀ a, b, c ∈ Self
/// ~~~
pub trait CommutativeMul
    : ApproxCommutativeMul
{
    /// Returns `true` if the multiplication operator is commutative for the
    /// given argument tuple.
    fn prop_mul_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a * b == b * a
    }
}

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

// Float impls

macro_rules! impl_float_op {
    ($T:ty, $Trait:ident :: $prop:ident, $op:ident, $eps:expr) => {
        impl $Trait for $T {
            fn $prop(args: ($T, $T)) -> bool {
                let (a, b) = args;
                ApproxEq::approx_eq_eps(&a.$op(&b), &b.$op(&a), &$eps)
            }
        }
    }
}

impl_float_op!(f32, ApproxCommutativeAdd::prop_add_is_approx_commutative, add, 1.0e-6)
impl_float_op!(f32, ApproxCommutativeMul::prop_mul_is_approx_commutative, mul, 1.0e-6)
impl_float_op!(f64, ApproxCommutativeAdd::prop_add_is_approx_commutative, add, 1.0e-11)
impl_float_op!(f64, ApproxCommutativeMul::prop_mul_is_approx_commutative, mul, 1.0e-11)

#[cfg(test)]
mod tests {
    macro_rules! check_float {
        ($T:ident) => {
            mod $T {
                use ops::ApproxCommutativeAdd;
                use ops::ApproxCommutativeMul;

                #[quickcheck]
                fn prop_add_is_approx_commutative(args: ($T, $T)) -> bool {
                    ApproxCommutativeAdd::prop_add_is_approx_commutative(args)
                }

                #[quickcheck]
                fn prop_mul_is_approx_commutative(args: ($T, $T)) -> bool {
                    ApproxCommutativeMul::prop_mul_is_approx_commutative(args)
                }
            }
        }
    }
    check_float!(f32)
    check_float!(f64)

    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use ops::ApproxCommutativeAdd;
                use ops::CommutativeAdd;
                use ops::ApproxCommutativeMul;
                use ops::CommutativeMul;

                #[quickcheck]
                fn prop_add_is_approx_commutative(args: ($T, $T)) -> bool {
                    ApproxCommutativeAdd::prop_add_is_approx_commutative(args)
                }
                #[quickcheck]
                fn prop_add_is_commutative(args: ($T, $T)) -> bool {
                    CommutativeAdd::prop_add_is_commutative(args)
                }

                #[quickcheck]
                fn prop_mul_is_approx_commutative(args: ($T, $T)) -> bool {
                    ApproxCommutativeMul::prop_mul_is_approx_commutative(args)
                }
                #[quickcheck]
                fn prop_mul_is_commutative(args: ($T, $T)) -> bool {
                    CommutativeMul::prop_mul_is_commutative(args)
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
