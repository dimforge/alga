// Copyright 2013 The Num-rs Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

//! Operators with specific properties

/// The multiplicative inverse operation
pub trait Recip {
    fn recip(&self) -> Self;
}

impl Recip for f32 { #[inline] fn recip(&self) -> f32 { 1.0 / *self } }
impl Recip for f64 { #[inline] fn recip(&self) -> f64 { 1.0 / *self } }


/// Sets equipped with a commutative addition operator.
///
/// ~~~notrust
/// a + b = b + a                           ∀ a, b, c ∈ Self
/// ~~~
pub trait CommutativeAdd
    : Add<Self, Self>
{}

/// Ensures that the addition operator is commutative for the given arguments.
pub fn prop_add_is_commutative<T>((a, b): (T, T)) -> bool where
    T: AssociativeAdd + PartialEq,
{
    a + b == b + a
}

impl CommutativeAdd for u8   {}
impl CommutativeAdd for u16  {}
impl CommutativeAdd for u32  {}
impl CommutativeAdd for u64  {}
impl CommutativeAdd for uint {}
impl CommutativeAdd for i8   {}
impl CommutativeAdd for i16  {}
impl CommutativeAdd for i32  {}
impl CommutativeAdd for i64  {}
impl CommutativeAdd for int  {}

/// Sets equipped with an associative multiplication operator.
///
/// ~~~notrust
/// (a * b) * c = a * (b * c)               ∀ a, b, c ∈ Self
/// ~~~
pub trait AssociativeMul
    : Mul<Self, Self>
    + PartialEq {}

/// Ensures that the multiplication operator is associative for the given arguments.
pub fn prop_mul_is_associative<T>((a, b, c): (T, T, T)) -> bool where
    T: AssociativeMul + PartialEq,
{
    (a * b) * c == a * (b * c)
}

impl AssociativeMul for u8   {}
impl AssociativeMul for u16  {}
impl AssociativeMul for u32  {}
impl AssociativeMul for u64  {}
impl AssociativeMul for uint {}
impl AssociativeMul for i8   {}
impl AssociativeMul for i16  {}
impl AssociativeMul for i32  {}
impl AssociativeMul for i64  {}
impl AssociativeMul for int  {}

/// Sets equipped with a commutative multiplication operator.
///
/// ~~~notrust
/// a * b = b * a                           ∀ a, b, c ∈ Self
/// ~~~
pub trait CommutativeMul
    : Mul<Self, Self>
{}

/// Ensures that the multiplication operator is commutative for the given arguments.
pub fn prop_mul_is_commutative<T>((a, b): (T, T)) -> bool where
    T: CommutativeMul + PartialEq,
{
    a * b == b * a
}

impl CommutativeMul for u8   {}
impl CommutativeMul for u16  {}
impl CommutativeMul for u32  {}
impl CommutativeMul for u64  {}
impl CommutativeMul for uint {}
impl CommutativeMul for i8   {}
impl CommutativeMul for i16  {}
impl CommutativeMul for i32  {}
impl CommutativeMul for i64  {}
impl CommutativeMul for int  {}

/// A set that is equipped with a multiplication and addition operator where
/// multiplication distributes over addition.
///
/// ~~~notrust
/// a * (b + c) = (a * b) + (a * c)         ∀ a, b, c ∈ Self
/// (a + b) * c = (a * c) + (b * c)         ∀ a, b, c ∈ Self
/// ~~~
pub trait DistributiveMulAdd
    : Mul<Self, Self>
    + Add<Self, Self>
{}

/// Ensures that multiplication distributes over addition for the given arguments.
pub fn prop_mul_add_is_distributive<T>((a, b, c): (T, T, T)) -> bool where
    T: DistributiveMulAdd + PartialEq,
{
    a * (b + c) == (a * b) + (a * c) &&
    (a + b) * c == (a * c) + (b * c)
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


pub trait PartialAssociativeAdd
    : Add<Self, Self>
{}

impl PartialAssociativeAdd for f32 {}
impl PartialAssociativeAdd for f64 {}

pub trait PartialCommutativeAdd
    : Add<Self, Self> {}

impl PartialCommutativeAdd for f32 {}
impl PartialCommutativeAdd for f64 {}

pub trait PartialAssociativeMul
    : Mul<Self, Self>
    + PartialEq
{}

impl PartialAssociativeMul for f32 {}
impl PartialAssociativeMul for f64 {}

pub trait PartialCommutativeMul
    : Mul<Self, Self> {}

impl PartialCommutativeMul for f32 {}
impl PartialCommutativeMul for f64 {}

pub trait PartialDistributiveMulAdd
    : Mul<Self, Self>
    + Add<Self, Self>
{}

impl PartialDistributiveMulAdd for f32 {}
impl PartialDistributiveMulAdd for f64 {}

#[cfg(test)]
mod tests {
    macro_rules! quickcheck_int {
        ($T:ident) => {
            mod $T {
                #[quickcheck]
                fn prop_add_is_associative(args: ($T, $T, $T)) -> bool {
                    ::ops::prop_add_is_associative(args)
                }
                #[quickcheck]
                fn prop_add_is_commutative(args: ($T, $T)) -> bool {
                    ::ops::prop_add_is_commutative(args)
                }
                #[quickcheck]
                fn prop_mul_is_associative(args: ($T, $T, $T)) -> bool {
                    ::ops::prop_mul_is_associative(args)
                }
                #[quickcheck]
                fn prop_mul_is_commutative(args: ($T, $T)) -> bool {
                    ::ops::prop_mul_is_commutative(args)
                }
                #[quickcheck]
                fn prop_mul_add_is_distributive(args: ($T, $T, $T)) -> bool {
                    ::ops::prop_mul_add_is_distributive(args)
                }
            }
        }
    }
    quickcheck_int!(u8)
    quickcheck_int!(u16)
    quickcheck_int!(u32)
    quickcheck_int!(u64)
    quickcheck_int!(uint)
    quickcheck_int!(i8)
    quickcheck_int!(i16)
    quickcheck_int!(i32)
    quickcheck_int!(i64)
    quickcheck_int!(int)
}
