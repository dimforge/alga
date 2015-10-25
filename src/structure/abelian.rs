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

use structure::GroupAdditiveApprox;
use structure::GroupAdditive;
use structure::GroupMultiplicativeApprox;
use structure::GroupMultiplicative;

pub trait GroupAdditiveAbelianApprox
    : GroupAdditiveApprox
{
    /// Returns `true` if the addition operator is approximately commutative for
    /// the given argument tuple.
    fn prop_add_is_commutative_approx(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a.clone() + b.clone() == b + a
    }
}

//impl GroupAdditiveAbelianApprox for u8   {}
//impl GroupAdditiveAbelianApprox for u16  {}
//impl GroupAdditiveAbelianApprox for u32  {}
//impl GroupAdditiveAbelianApprox for u64  {}
impl GroupAdditiveAbelianApprox for i8   {}
impl GroupAdditiveAbelianApprox for i16  {}
impl GroupAdditiveAbelianApprox for i32  {}
impl GroupAdditiveAbelianApprox for i64  {}

pub trait GroupAdditiveAbelian
    : GroupAdditiveAbelianApprox
    + GroupAdditive
{
    /// Returns `true` if the addition operator is commutative for the given
    /// argument tuple.
    fn prop_add_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a.clone() + b.clone() == b + a
    }
}

//impl GroupAdditiveAbelian for u8   {}
//impl GroupAdditiveAbelian for u16  {}
//impl GroupAdditiveAbelian for u32  {}
//impl GroupAdditiveAbelian for u64  {}
impl GroupAdditiveAbelian for i8   {}
impl GroupAdditiveAbelian for i16  {}
impl GroupAdditiveAbelian for i32  {}
impl GroupAdditiveAbelian for i64  {}

pub trait GroupMultiplicativeAbelianApprox
    : GroupMultiplicativeApprox
{
    /// Returns `true` if the multiplication operator is approximately
    /// commutative for the given argument tuple.
    fn prop_mul_is_commutative_approx(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a.clone() * b.clone() == b * a
    }
}

pub trait GroupMultiplicativeAbelian
    : GroupMultiplicativeAbelianApprox
    + GroupMultiplicative
{
    /// Returns `true` if the multiplication operator is commutative for the
    /// given argument tuple.
    fn prop_mul_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a.clone() * b.clone() == b * a
    }
}
