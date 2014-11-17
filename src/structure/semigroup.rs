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

use structure::ApproxAdditiveMagma;
use structure::AdditiveMagma;
use structure::ApproxMultiplicativeMagma;
use structure::MultiplicativeMagma;

/// A type that is closed over an approximately associative addition operator.
///
/// The addition operator must satisfy:
///
/// ~~~notrust
/// (a + b) + c ≈ a + (b + c)           ∀ a, b, c ∈ Self
/// ~~~
pub trait ApproxAdditiveSemigroup
    : ApproxAdditiveMagma
{
    /// Returns `true` if associativity over addition holds approximately for
    /// the given arguments.
    fn prop_add_is_approx_associative(args: (Self, Self, Self)) -> bool {
        // TODO: use ApproxEq
        let (a, b, c) = args;
        (a + b) + c == a + (b + c)
    }
}

impl ApproxAdditiveSemigroup for u8   {}
impl ApproxAdditiveSemigroup for u16  {}
impl ApproxAdditiveSemigroup for u32  {}
impl ApproxAdditiveSemigroup for u64  {}
impl ApproxAdditiveSemigroup for uint {}
impl ApproxAdditiveSemigroup for i8   {}
impl ApproxAdditiveSemigroup for i16  {}
impl ApproxAdditiveSemigroup for i32  {}
impl ApproxAdditiveSemigroup for i64  {}
impl ApproxAdditiveSemigroup for int  {}

/// A type that is closed over an associative addition operator.
///
/// The addition operator must satisfy:
///
/// ~~~notrust
/// (a + b) + c = a + (b + c)           ∀ a, b, c ∈ Self
/// ~~~
pub trait AdditiveSemigroup
    : AdditiveMagma
    + ApproxAdditiveSemigroup
{
    /// Returns `true` if associativity over addition holds for the given
    /// arguments.
    fn prop_add_is_associative(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        (a + b) + c == a + (b + c)
    }
}

impl AdditiveSemigroup for u8   {}
impl AdditiveSemigroup for u16  {}
impl AdditiveSemigroup for u32  {}
impl AdditiveSemigroup for u64  {}
impl AdditiveSemigroup for uint {}
impl AdditiveSemigroup for i8   {}
impl AdditiveSemigroup for i16  {}
impl AdditiveSemigroup for i32  {}
impl AdditiveSemigroup for i64  {}
impl AdditiveSemigroup for int  {}

/// A type that is closed over an approximately associative multiplication operator.
///
/// The multiplication operator must satisfy:
///
/// ~~~notrust
/// (a * b) * c ≈ a * (b * c)           ∀ a, b, c ∈ Self
/// ~~~
pub trait ApproxMultiplicativeSemigroup
    : ApproxMultiplicativeMagma
{
    /// Returns `true` if associativity over multiplication holds approximately for
    /// the given arguments.
    fn prop_mul_is_approx_associative(args: (Self, Self, Self)) -> bool {
        // TODO: use ApproxEq
        let (a, b, c) = args;
        (a * b) * c == a * (b * c)
    }
}

impl ApproxMultiplicativeSemigroup for u8   {}
impl ApproxMultiplicativeSemigroup for u16  {}
impl ApproxMultiplicativeSemigroup for u32  {}
impl ApproxMultiplicativeSemigroup for u64  {}
impl ApproxMultiplicativeSemigroup for uint {}
impl ApproxMultiplicativeSemigroup for i8   {}
impl ApproxMultiplicativeSemigroup for i16  {}
impl ApproxMultiplicativeSemigroup for i32  {}
impl ApproxMultiplicativeSemigroup for i64  {}
impl ApproxMultiplicativeSemigroup for int  {}

/// A type that is closed over an associative multiplication operator.
///
/// The multiplication operator must satisfy:
///
/// ~~~notrust
/// (a * b) * c = a * (b * c)           ∀ a, b, c ∈ Self
/// ~~~
pub trait MultiplicativeSemigroup
    : MultiplicativeMagma
    + ApproxMultiplicativeSemigroup
{
    /// Returns `true` if associativity over multiplication holds for the given
    /// arguments.
    fn prop_mul_is_associative(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        (a * b) * c == a * (b * c)
    }
}

impl MultiplicativeSemigroup for u8   {}
impl MultiplicativeSemigroup for u16  {}
impl MultiplicativeSemigroup for u32  {}
impl MultiplicativeSemigroup for u64  {}
impl MultiplicativeSemigroup for uint {}
impl MultiplicativeSemigroup for i8   {}
impl MultiplicativeSemigroup for i16  {}
impl MultiplicativeSemigroup for i32  {}
impl MultiplicativeSemigroup for i64  {}
impl MultiplicativeSemigroup for int  {}

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use structure::ApproxAdditiveSemigroup;
                use structure::AdditiveSemigroup;
                use structure::ApproxMultiplicativeSemigroup;
                use structure::MultiplicativeSemigroup;

                #[quickcheck]
                fn prop_add_is_approx_associative(args: ($T, $T, $T)) -> bool {
                    ApproxAdditiveSemigroup::prop_add_is_approx_associative(args)
                }
                #[quickcheck]
                fn prop_add_is_associative(args: ($T, $T, $T)) -> bool {
                    AdditiveSemigroup::prop_add_is_associative(args)
                }

                #[quickcheck]
                fn prop_mul_is_approx_associative(args: ($T, $T, $T)) -> bool {
                    ApproxMultiplicativeSemigroup::prop_mul_is_approx_associative(args)
                }
                #[quickcheck]
                fn prop_mul_is_associative(args: ($T, $T, $T)) -> bool {
                    MultiplicativeSemigroup::prop_mul_is_associative(args)
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
