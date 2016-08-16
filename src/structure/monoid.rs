// Copyright 2013-2014 The Algebra Developers.
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
use ident::Identity;

use structure::SemigroupApprox;
use structure::Semigroup;

/// An approximate semigroup equipped with an identity element.
///
/// ~~~notrust
/// ∃ e ∈ Self, ∀ a ∈ Self, e ∘ a ≈ a and a ∘ e ≈ a
/// ~~~
pub trait MonoidApprox<O: Op>
    : SemigroupApprox<O>
    + Identity<O>
{
    /// Checks whether operating with the identity element is approximately a no-op for the given
    /// argument.
    fn prop_operating_identity_element_is_noop_approx(a: Self) -> bool {
        let a = || a.clone();
        (a().approx(Identity::id())).approx_eq(&a()) &&
        (Self::id().approx(a())).approx_eq(&a())
    }
}

impl_marker!(MonoidApprox<Additive>; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
impl_marker!(MonoidApprox<Multiplicative>; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

/// A semigroup equipped with an identity element.
///
/// ~~~notrust
/// ∃ e ∈ Self, ∀ a ∈ Self, e ∘ a = a ∘ e = a
/// ~~~
pub trait Monoid<O: Op>
    : MonoidApprox<O>
    + Semigroup<O>
{
    /// Checks whether operating with the identity element is a no-op for the given argument.
    fn prop_operating_identity_element_is_noop(a: Self) -> bool {
        let a = || a.clone();
        a().operate(Identity::id()) == a() &&
        Self::id().operate(a()) == a()
    }
}

impl_marker!(Monoid<Additive>; u8, u16, u32, u64, i8, i16, i32, i64);
impl_marker!(Monoid<Multiplicative>; u8, u16, u32, u64, i8, i16, i32, i64);

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($($T:ident),* $(,)*) => {
            $(mod $T {
                use ops::{Additive, Multiplicative};
                use structure::MonoidApprox;
                use structure::Monoid;

                #[quickcheck]
                fn prop_zero_is_noop_approx(args: $T) -> bool {
                    MonoidApprox::<Additive>::prop_operating_identity_element_is_noop_approx(args)
                }
                #[quickcheck]
                fn prop_add_zero_is_noop(args: $T) -> bool {
                    Monoid::<Additive>::prop_operating_identity_element_is_noop(args)
                }

                #[quickcheck]
                fn prop_mul_unit_is_noop_approx(args: $T) -> bool {
                    MonoidApprox::<Multiplicative>::prop_operating_identity_element_is_noop_approx(args)
                }
                #[quickcheck]
                fn prop_mul_unit_is_noop(args: $T) -> bool {
                    Monoid::<Multiplicative>::prop_operating_identity_element_is_noop(args)
                }
            })+
        }
    }
    check_int!(u8, u16, u32, u64, i8, i16, i32, i64);
}
