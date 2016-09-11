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

use structure::Magma;

/// An associative magma.
///
/// ~~~notrust
/// ∀ a, b, c ∈ Self, (a ∘ b) ∘ c = a ∘ (b ∘ c)       
/// ~~~
pub trait Semigroup<O: Op>
    : Magma<O>
{
    /// Returns `true` if associativity holds for the given arguments.
    fn prop_is_associative(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = (|| args.0.clone(), || args.1.clone(), || args.2.clone());
        (a().operate(b()).operate(c())).approx_eq(&a().operate(b().operate(c())))
    }
}

impl_marker!(Semigroup<Additive>; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
impl_marker!(Semigroup<Multiplicative>; u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($($T:ident),* $(,)*) => {
            $(mod $T {
                use ops::{Additive, Multiplicative};
                use structure::Semigroup;

                #[quickcheck]
                fn prop_add_is_associative(args: ($T, $T, $T)) -> bool {
                    Semigroup::<Additive>::prop_is_associative(args)
                }

                #[quickcheck]
                fn prop_mul_is_associative(args: ($T, $T, $T)) -> bool {
                    Semigroup::<Multiplicative>::prop_is_associative(args)
                }
            })+
        }
    }
    check_int!(u8, u16, u32, u64, i8, i16, i32, i64);
}
