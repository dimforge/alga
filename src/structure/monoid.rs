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

use structure::ApproxAdditiveSemigroup;
use structure::AdditiveSemigroup;
use structure::ApproxMultiplicativeSemigroup;
use structure::MultiplicativeSemigroup;
use structure::{AdditiveIdentity, zero};
use structure::{MultiplicativeIdentity, unit};

/// A type that is equipped with an approximately associative addition operator
/// and a corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a + 0 ≈ a       ∀ a ∈ Self
/// 0 + a ≈ a       ∀ a ∈ Self
/// ~~~
pub trait ApproxAdditiveMonoid
    : ApproxAdditiveSemigroup
    + AdditiveIdentity
{
    /// Checks whether adding `0` is approximately a no-op for the given
    /// argument.
    fn prop_add_zero_is_approx_noop(a: Self) -> bool {
        a + zero::<Self>() == a &&
        zero::<Self>() + a == a
    }
}

impl ApproxAdditiveMonoid for u8   {}
impl ApproxAdditiveMonoid for u16  {}
impl ApproxAdditiveMonoid for u32  {}
impl ApproxAdditiveMonoid for u64  {}
impl ApproxAdditiveMonoid for uint {}
impl ApproxAdditiveMonoid for i8   {}
impl ApproxAdditiveMonoid for i16  {}
impl ApproxAdditiveMonoid for i32  {}
impl ApproxAdditiveMonoid for i64  {}
impl ApproxAdditiveMonoid for int  {}

/// A type that is equipped with an associative addition operator and a
/// corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a + 0 = a                           ∀ a ∈ Self
/// 0 + a = a                           ∀ a ∈ Self
/// ~~~
pub trait AdditiveMonoid
    : ApproxAdditiveMonoid
    + AdditiveSemigroup
{
    /// Checks whether adding `0` is a no-op for the given argument.
    fn prop_add_zero_is_noop(a: Self) -> bool {
        a + zero::<Self>() == a &&
        zero::<Self>() + a == a
    }
}

impl AdditiveMonoid for u8   {}
impl AdditiveMonoid for u16  {}
impl AdditiveMonoid for u32  {}
impl AdditiveMonoid for u64  {}
impl AdditiveMonoid for uint {}
impl AdditiveMonoid for i8   {}
impl AdditiveMonoid for i16  {}
impl AdditiveMonoid for i32  {}
impl AdditiveMonoid for i64  {}
impl AdditiveMonoid for int  {}

/// A type that is equipped with an approximately associative multiplication
/// operator and a corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a * 1 ≈ a       ∀ a ∈ Self
/// 1 * a ≈ a       ∀ a ∈ Self
/// ~~~
pub trait ApproxMultiplicativeMonoid
    : ApproxMultiplicativeSemigroup
    + MultiplicativeIdentity
{
    /// Checks whether multiplying by `1` is approximately a no-op for the given
    /// argument.
    fn prop_mul_unit_is_approx_noop(a: Self) -> bool {
        a * unit::<Self>() == a &&
        unit::<Self>() * a == a
    }
}

impl ApproxMultiplicativeMonoid for u8   {}
impl ApproxMultiplicativeMonoid for u16  {}
impl ApproxMultiplicativeMonoid for u32  {}
impl ApproxMultiplicativeMonoid for u64  {}
impl ApproxMultiplicativeMonoid for uint {}
impl ApproxMultiplicativeMonoid for i8   {}
impl ApproxMultiplicativeMonoid for i16  {}
impl ApproxMultiplicativeMonoid for i32  {}
impl ApproxMultiplicativeMonoid for i64  {}
impl ApproxMultiplicativeMonoid for int  {}

/// A type that is equipped with an associative multiplication operator and a
/// corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a * 1 = a       ∀ a ∈ Self
/// 1 * a = a       ∀ a ∈ Self
/// ~~~
pub trait MultiplicativeMonoid
    : ApproxMultiplicativeMonoid
    + MultiplicativeSemigroup
{
    /// Checks whether multiplying by `1` is a no-op for the given argument.
    fn prop_mul_unit_is_noop(a: Self) -> bool {
        a * unit::<Self>() == a &&
        unit::<Self>() * a == a
    }
}

impl MultiplicativeMonoid for u8   {}
impl MultiplicativeMonoid for u16  {}
impl MultiplicativeMonoid for u32  {}
impl MultiplicativeMonoid for u64  {}
impl MultiplicativeMonoid for uint {}
impl MultiplicativeMonoid for i8   {}
impl MultiplicativeMonoid for i16  {}
impl MultiplicativeMonoid for i32  {}
impl MultiplicativeMonoid for i64  {}
impl MultiplicativeMonoid for int  {}

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use structure::ApproxAdditiveMonoid;
                use structure::AdditiveMonoid;
                use structure::ApproxMultiplicativeMonoid;
                use structure::MultiplicativeMonoid;

                #[quickcheck]
                fn prop_add_zero_is_approx_noop(args: $T) -> bool {
                    ApproxAdditiveMonoid::prop_add_zero_is_approx_noop(args)
                }
                #[quickcheck]
                fn prop_add_zero_is_noop(args: $T) -> bool {
                    AdditiveMonoid::prop_add_zero_is_noop(args)
                }

                #[quickcheck]
                fn prop_mul_unit_is_approx_noop(args: $T) -> bool {
                    ApproxMultiplicativeMonoid::prop_mul_unit_is_approx_noop(args)
                }
                #[quickcheck]
                fn prop_mul_unit_is_noop(args: $T) -> bool {
                    MultiplicativeMonoid::prop_mul_unit_is_noop(args)
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
