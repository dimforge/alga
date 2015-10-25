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

use structure::SemigroupAdditiveApprox;
use structure::SemigroupAdditive;
use structure::SemigroupMultiplicativeApprox;
use structure::SemigroupMultiplicative;
use structure::{IdentityAdditive, zero};
use structure::{IdentityMultiplicative, unit};

/// A type that is equipped with an approximately associative addition operator
/// and a corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a + 0 ≈ a       ∀ a ∈ Self
/// 0 + a ≈ a       ∀ a ∈ Self
/// ~~~
pub trait MonoidAdditiveApprox
    : SemigroupAdditiveApprox
    + IdentityAdditive
{
    /// Checks whether adding `0` is approximately a no-op for the given
    /// argument.
    fn prop_add_zero_is_noop_approx(a: Self) -> bool {
        a.clone() + zero::<Self>() == a.clone() &&
        zero::<Self>() + a.clone() == a.clone()
    }
}

impl MonoidAdditiveApprox for u8   {}
impl MonoidAdditiveApprox for u16  {}
impl MonoidAdditiveApprox for u32  {}
impl MonoidAdditiveApprox for u64  {}
impl MonoidAdditiveApprox for i8   {}
impl MonoidAdditiveApprox for i16  {}
impl MonoidAdditiveApprox for i32  {}
impl MonoidAdditiveApprox for i64  {}

/// A type that is equipped with an associative addition operator and a
/// corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a + 0 = a                           ∀ a ∈ Self
/// 0 + a = a                           ∀ a ∈ Self
/// ~~~
pub trait MonoidAdditive
    : MonoidAdditiveApprox
    + SemigroupAdditive
{
    /// Checks whether adding `0` is a no-op for the given argument.
    fn prop_add_zero_is_noop(a: Self) -> bool {
        a.clone() + zero::<Self>() == a.clone() &&
        zero::<Self>() + a.clone() == a.clone()
    }
}

impl MonoidAdditive for u8   {}
impl MonoidAdditive for u16  {}
impl MonoidAdditive for u32  {}
impl MonoidAdditive for u64  {}
impl MonoidAdditive for i8   {}
impl MonoidAdditive for i16  {}
impl MonoidAdditive for i32  {}
impl MonoidAdditive for i64  {}

/// A type that is equipped with an approximately associative multiplication
/// operator and a corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a * 1 ≈ a       ∀ a ∈ Self
/// 1 * a ≈ a       ∀ a ∈ Self
/// ~~~
pub trait MonoidMultiplicativeApprox
    : SemigroupMultiplicativeApprox
    + IdentityMultiplicative
{
    /// Checks whether multiplying by `1` is approximately a no-op for the given
    /// argument.
    fn prop_mul_unit_is_noop_approx(a: Self) -> bool {
        a.clone() * unit::<Self>() == a.clone() &&
        unit::<Self>() * a.clone() == a.clone()
    }
}

impl MonoidMultiplicativeApprox for u8   {}
impl MonoidMultiplicativeApprox for u16  {}
impl MonoidMultiplicativeApprox for u32  {}
impl MonoidMultiplicativeApprox for u64  {}
impl MonoidMultiplicativeApprox for i8   {}
impl MonoidMultiplicativeApprox for i16  {}
impl MonoidMultiplicativeApprox for i32  {}
impl MonoidMultiplicativeApprox for i64  {}

/// A type that is equipped with an associative multiplication operator and a
/// corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a * 1 = a       ∀ a ∈ Self
/// 1 * a = a       ∀ a ∈ Self
/// ~~~
pub trait MonoidMultiplicative
    : MonoidMultiplicativeApprox
    + SemigroupMultiplicative
{
    /// Checks whether multiplying by `1` is a no-op for the given argument.
    fn prop_mul_unit_is_noop(a: Self) -> bool {
        a.clone() * unit::<Self>() == a.clone() &&
        unit::<Self>() * a.clone() == a.clone()
    }
}

impl MonoidMultiplicative for u8   {}
impl MonoidMultiplicative for u16  {}
impl MonoidMultiplicative for u32  {}
impl MonoidMultiplicative for u64  {}
impl MonoidMultiplicative for i8   {}
impl MonoidMultiplicative for i16  {}
impl MonoidMultiplicative for i32  {}
impl MonoidMultiplicative for i64  {}

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use structure::MonoidAdditiveApprox;
                use structure::MonoidAdditive;
                use structure::MonoidMultiplicativeApprox;
                use structure::MonoidMultiplicative;

                #[quickcheck]
                fn prop_add_zero_is_noop_approx(args: $T) -> bool {
                    MonoidAdditiveApprox::prop_add_zero_is_noop_approx(args)
                }
                #[quickcheck]
                fn prop_add_zero_is_noop(args: $T) -> bool {
                    MonoidAdditive::prop_add_zero_is_noop(args)
                }

                #[quickcheck]
                fn prop_mul_unit_is_noop_approx(args: $T) -> bool {
                    MonoidMultiplicativeApprox::prop_mul_unit_is_noop_approx(args)
                }
                #[quickcheck]
                fn prop_mul_unit_is_noop(args: $T) -> bool {
                    MonoidMultiplicative::prop_mul_unit_is_noop(args)
                }
            }
        }
    }
    check_int!(u8);
    check_int!(u16);
    check_int!(u32);
    check_int!(u64);
    check_int!(i8);
    check_int!(i16);
    check_int!(i32);
    check_int!(i64);
}
