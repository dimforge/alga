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

use general::{Magma, Op, Inverse, Additive, Multiplicative};
use cmp::ApproxEq;

/// A magma with the divisibility property.
///
/// Divisibility is a weak form of right and left invertibility:
///
/// ```notrust
/// ∀ a, b ∈ Self, ∃! r, l ∈ Self such that l ∘ a = b and a ∘ r = b
/// ```
pub trait Quasigroup<O: Op>
    : Magma<O> + Inverse<O>
{
    /// Returns `true` if latin squareness holds for the given arguments.
    fn prop_inv_is_latin_square(args: (Self, Self)) -> bool
        where Self: ApproxEq {
        let (a, b) = (|| args.0.clone(), || args.1.clone());

        a().approx_eq(&(a().operate(b().inv()).operate(b()))) &&
        a().approx_eq(&(a().operate(b().operate(b().inv()))))

        // TODO: pseudo inverse?
    }
}

impl_marker!(Quasigroup<Additive>; i8, i16, i32, i64, f32, f64);
impl_marker!(Quasigroup<Multiplicative>; f32, f64);


#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($($T:ident),* $(,)*) => {
            $(mod $T {
                use ops::Additive;
                use general::Quasigroup;

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
