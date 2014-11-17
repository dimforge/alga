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

//! Distributive operators

use approx::ApproxEq;
use structure::AdditiveClosure;
use structure::MultiplicativeClosure;

/// A type that is equipped with a multiplication and addition operator where
/// multiplication approximately distributes over addition.
///
/// ~~~notrust
/// a * (b + c) ≈ (a * b) + (a * c)         ∀ a, b, c ∈ Self
/// (a + b) * c ≈ (a * c) + (b * c)         ∀ a, b, c ∈ Self
/// ~~~
pub trait ApproxDistributiveMulAdd
    : AdditiveClosure
    + MultiplicativeClosure
    + PartialEq
{
    /// Returns `true` if multiplication distributes over addition for the given
    /// arguments.
    fn prop_mul_add_is_approx_distributive(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        a * (b + c) == (a * b) + (a * c) &&
        (a + b) * c == (a * c) + (b * c)
    }
}

impl ApproxDistributiveMulAdd for u8   {}
impl ApproxDistributiveMulAdd for u16  {}
impl ApproxDistributiveMulAdd for u32  {}
impl ApproxDistributiveMulAdd for u64  {}
impl ApproxDistributiveMulAdd for uint {}
impl ApproxDistributiveMulAdd for i8   {}
impl ApproxDistributiveMulAdd for i16  {}
impl ApproxDistributiveMulAdd for i32  {}
impl ApproxDistributiveMulAdd for i64  {}
impl ApproxDistributiveMulAdd for int  {}

/// A type that is equipped with a multiplication and addition operator where
/// multiplication distributes over addition.
///
/// ~~~notrust
/// a * (b + c) = (a * b) + (a * c)         ∀ a, b, c ∈ Self
/// (a + b) * c = (a * c) + (b * c)         ∀ a, b, c ∈ Self
/// ~~~
pub trait DistributiveMulAdd
    : ApproxDistributiveMulAdd
{
    /// Returns `true` if multiplication distributes over addition for the given
    /// arguments.
    fn prop_mul_add_is_distributive(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        a * (b + c) == (a * b) + (a * c) &&
        (a + b) * c == (a * c) + (b * c)
    }
}

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

// Float impls

macro_rules! impl_float_op {
    ($T:ty, $eps:expr) => {
        impl ApproxDistributiveMulAdd for $T {
            fn prop_mul_add_is_approx_distributive(args: ($T, $T, $T)) -> bool {
                let (a, b, c) = args;
                ApproxEq::approx_eq_eps(&(a * (b + c)), &((a * b) + (a * c)), &$eps) &&
                ApproxEq::approx_eq_eps(&((a + b) * c), &((a * c) + (b * c)), &$eps)
            }
        }
    }
}
impl_float_op!(f32, 1.0e-2)
impl_float_op!(f64, 1.0e-11)

#[cfg(test)]
mod tests {
    macro_rules! check_float {
        ($T:ident) => {
            mod $T {
                use ops::ApproxDistributiveMulAdd;

                #[quickcheck]
                fn prop_mul_add_is_approx_distributive(args: ($T, $T, $T)) -> bool {
                    ApproxDistributiveMulAdd::prop_mul_add_is_approx_distributive(args)
                }
            }
        }
    }
    check_float!(f32)
    check_float!(f64)

    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use ops::ApproxDistributiveMulAdd;
                use ops::DistributiveMulAdd;

                #[quickcheck]
                fn prop_mul_add_is_approx_distributive(args: ($T, $T, $T)) -> bool {
                    ApproxDistributiveMulAdd::prop_mul_add_is_approx_distributive(args)
                }
                #[quickcheck]
                fn prop_mul_add_is_distributive(args: ($T, $T, $T)) -> bool {
                    DistributiveMulAdd::prop_mul_add_is_distributive(args)
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
