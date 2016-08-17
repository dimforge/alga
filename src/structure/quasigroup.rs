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

use ops::{Op, Inverse, Additive};

use structure::MagmaApprox;
use structure::Magma;

/// A magma with the approximate divisibility property.
///
/// Approximate divisibility is a weak form of approximate right and left invertibility:
///
/// ```notrust
/// ∀ a, b ∈ Self, ∃ r, l ∈ Self such that l ∘ a ≈ b and a ∘ r ≈ b
/// ```
pub trait QuasigroupApprox<O: Op>
    : MagmaApprox<O>
    + Inverse<O>
{
    /// Returns `true` if latin squareness holds approximately for
    /// the given arguments.
    fn prop_inv_is_latin_square_approx(args: (Self, Self)) -> bool {
        let (a, b) = (|| args.0.clone(), || args.1.clone());

        a().approx_eq(&(a().approx(b().inv()).approx(b()))) &&
        a().approx_eq(&(a().approx(b().approx(b().inv()))))

        // TODO: pseudo inverse?
    }
}

impl_marker!(QuasigroupApprox<Additive>; i8, i16, i32, i64);

/// A magma with the divisibility property.
///
/// Divisibility is a weak form of right and left invertibility:
///
/// ```notrust
/// ∀ a, b ∈ Self, ∃! r, l ∈ Self such that l ∘ a = b and a ∘ r = b
/// ```
pub trait Quasigroup<O: Op>
    : QuasigroupApprox<O>
    + Magma<O>
{
    /// Returns `true` if latin squareness holds for
    /// the given arguments.
    fn prop_inv_is_latin_square(args: (Self, Self)) -> bool {
        let (a, b) = (|| args.0.clone(), || args.1.clone());

        a() == a().operate(b().inv()).operate(b()) &&
        a() == a().operate(b()).operate(b().inv())

        // TODO: pseudo inverse?
    }
}

impl_marker!(Quasigroup<Additive>; i8, i16, i32, i64);

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($($T:ident),* $(,)*) => {
            $(mod $T {
                use ops::Additive;
                use structure::QuasigroupApprox;
                use structure::Quasigroup;

                #[quickcheck]
                fn prop_inv_is_latin_square_approx(args: ($T, $T)) -> bool {
                    QuasigroupApprox::<Additive>::prop_inv_is_latin_square_approx(args)
                }
                #[quickcheck]
                fn prop_inv_is_latin_square(args: ($T, $T)) -> bool {
                    Quasigroup::<Additive>::prop_inv_is_latin_square(args)
                }
            })+
        }
    }
    //check_int!(u8);
    //check_int!(u16);
    //check_int!(u32);
    //check_int!(u64);
    check_int!(i8, i16, i32, i64);
}
