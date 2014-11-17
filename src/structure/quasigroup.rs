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

#![allow(missing_docs)]

use ops::Recip;
use structure::ApproxAdditiveMagma;
use structure::AdditiveMagma;
use structure::ApproxMultiplicativeMagma;
use structure::MultiplicativeMagma;

/// An additive magma that for which subtraction is always possible.
pub trait ApproxAdditiveQuasigroup
    : ApproxAdditiveMagma
    + Sub<Self, Self>
    + Neg<Self>
{
    fn prop_sub_is_approx_latin_square(args: (Self, Self)) -> bool {
        let (a, b) = args;

        a == (a + -b) + b &&
        a == (a + b) + -b &&
        a == (a - b) + b &&
        a == (a + b) - b

        // TODO: psuedo inverse?
    }
}

impl ApproxAdditiveQuasigroup for u8   {}
impl ApproxAdditiveQuasigroup for u16  {}
impl ApproxAdditiveQuasigroup for u32  {}
impl ApproxAdditiveQuasigroup for u64  {}
impl ApproxAdditiveQuasigroup for uint {}
impl ApproxAdditiveQuasigroup for i8   {}
impl ApproxAdditiveQuasigroup for i16  {}
impl ApproxAdditiveQuasigroup for i32  {}
impl ApproxAdditiveQuasigroup for i64  {}
impl ApproxAdditiveQuasigroup for int  {}

/// An additive magma that for which subtraction is always possible.
pub trait AdditiveQuasigroup
    : AdditiveMagma
    + ApproxAdditiveQuasigroup
{
    fn prop_sub_is_latin_square(args: (Self, Self)) -> bool {
        let (a, b) = args;

        a == (a + -b) + b &&
        a == (a + b) + -b &&
        a == (a - b) + b &&
        a == (a + b) - b

        // TODO: psuedo inverse?
    }
}

impl AdditiveQuasigroup for u8   {}
impl AdditiveQuasigroup for u16  {}
impl AdditiveQuasigroup for u32  {}
impl AdditiveQuasigroup for u64  {}
impl AdditiveQuasigroup for uint {}
impl AdditiveQuasigroup for i8   {}
impl AdditiveQuasigroup for i16  {}
impl AdditiveQuasigroup for i32  {}
impl AdditiveQuasigroup for i64  {}
impl AdditiveQuasigroup for int  {}

/// An multiplicative magma that for which division is always possible.
pub trait ApproxMultiplicativeQuasigroup
    : ApproxMultiplicativeMagma
    + Div<Self, Self>
    + Recip<Self>
{
    fn prop_div_is_approx_latin_square(args: (Self, Self)) -> bool {
        let (a, b) = args;

        a == (a * b.recip()) * b &&
        a == (a * b) * b.recip() &&
        a == (a / b) * b &&
        a == (a * b) / b

        // TODO: psuedo inverse?
    }
}

/// An multiplicative magma that for which division is always possible.
pub trait MultiplicativeQuasigroup
    : MultiplicativeMagma
    + ApproxMultiplicativeQuasigroup
{
    fn prop_div_is_latin_square(args: (Self, Self)) -> bool {
        let (a, b) = args;

        a == (a * b.recip()) * b &&
        a == (a * b) * b.recip() &&
        a == (a / b) * b &&
        a == (a * b) / b

        // TODO: psuedo inverse?
    }
}

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use structure::ApproxAdditiveQuasigroup;
                use structure::AdditiveQuasigroup;

                #[quickcheck]
                fn prop_sub_is_approx_latin_square(args: ($T, $T)) -> bool {
                    ApproxAdditiveQuasigroup::prop_sub_is_approx_latin_square(args)
                }
                #[quickcheck]
                fn prop_sub_is_latin_square(args: ($T, $T)) -> bool {
                    AdditiveQuasigroup::prop_sub_is_latin_square(args)
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
