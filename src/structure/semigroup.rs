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

use ops::{Op, Additive, Multiplicative};

use structure::MagmaApprox;
use structure::Magma;

/// A type that is closed over an approximately associative operator.
///
/// The operator must satisfy:
///
/// ~~~notrust
/// (a ∘ b) ∘ c ≈ a ∘ (b ∘ c)           ∀ a, b, c ∈ Self
/// ~~~
pub trait SemigroupApprox<O: Op>
    : MagmaApprox<O>
{
    /// Returns `true` if associativity holds approximately for
    /// the given arguments.
    fn prop_is_associative_approx(args: (Self, Self, Self)) -> bool {
         // TODO: use ApproxEq
        let (a, b, c) = (|| args.0.clone(), || args.1.clone(), || args.2.clone());
        Self::approx_eq(&a().approx(b()).approx(c()), &a().approx(b().approx(c())))
    }
}

impl SemigroupApprox<Additive> for u8   {}
impl SemigroupApprox<Additive> for u16  {}
impl SemigroupApprox<Additive> for u32  {}
impl SemigroupApprox<Additive> for u64  {}
impl SemigroupApprox<Additive> for i8   {}
impl SemigroupApprox<Additive> for i16  {}
impl SemigroupApprox<Additive> for i32  {}
impl SemigroupApprox<Additive> for i64  {}

impl SemigroupApprox<Multiplicative> for u8   {}
impl SemigroupApprox<Multiplicative> for u16  {}
impl SemigroupApprox<Multiplicative> for u32  {}
impl SemigroupApprox<Multiplicative> for u64  {}
impl SemigroupApprox<Multiplicative> for i8   {}
impl SemigroupApprox<Multiplicative> for i16  {}
impl SemigroupApprox<Multiplicative> for i32  {}
impl SemigroupApprox<Multiplicative> for i64  {}

/// A type that is closed over an associative operator.
/// The operator must satisfy:
///
///
/// ~~~notrust
/// (a ∘ b) ∘ c = a ∘ (b ∘ c)           ∀ a, b, c ∈ Self
/// ~~~
pub trait Semigroup<O: Op>
    : SemigroupApprox<O>
    + Magma<O>
{
    /// Returns `true` if associativity holds for the given
    /// arguments.
    fn prop_is_associative(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = (|| args.0.clone(), || args.1.clone(), || args.2.clone());
        a().operate(b()).operate(c()) == a().operate(b().operate(c()))
    }
}

impl Semigroup<Additive> for u8   {}
impl Semigroup<Additive> for u16  {}
impl Semigroup<Additive> for u32  {}
impl Semigroup<Additive> for u64  {}
impl Semigroup<Additive> for i8   {}
impl Semigroup<Additive> for i16  {}
impl Semigroup<Additive> for i32  {}
impl Semigroup<Additive> for i64  {}

impl Semigroup<Multiplicative> for u8   {}
impl Semigroup<Multiplicative> for u16  {}
impl Semigroup<Multiplicative> for u32  {}
impl Semigroup<Multiplicative> for u64  {}
impl Semigroup<Multiplicative> for i8   {}
impl Semigroup<Multiplicative> for i16  {}
impl Semigroup<Multiplicative> for i32  {}
impl Semigroup<Multiplicative> for i64  {}

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use structure::SemigroupAdditiveApprox;
                use structure::SemigroupAdditive;
                use structure::SemigroupMultiplicativeApprox;
                use structure::SemigroupMultiplicative;

                #[quickcheck]
                fn prop_add_is_associative_approx(args: ($T, $T, $T)) -> bool {
                    SemigroupAdditiveApprox::prop_add_is_associative_approx(args)
                }
                #[quickcheck]
                fn prop_add_is_associative(args: ($T, $T, $T)) -> bool {
                    SemigroupAdditive::prop_add_is_associative(args)
                }

                #[quickcheck]
                fn prop_mul_is_associative_approx(args: ($T, $T, $T)) -> bool {
                    SemigroupMultiplicativeApprox::prop_mul_is_associative_approx(args)
                }
                #[quickcheck]
                fn prop_mul_is_associative(args: ($T, $T, $T)) -> bool {
                    SemigroupMultiplicative::prop_mul_is_associative(args)
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
