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

use structure::ApproxAdditiveGroup;
use structure::AdditiveGroup;
use structure::ApproxMultiplicativeGroup;
use structure::MultiplicativeGroup;

pub trait ApproxAdditiveAbelianGroup
    : ApproxAdditiveGroup
{
    /// Returns `true` if the addition operator is approximately commutative for
    /// the given argument tuple.
    fn prop_add_is_approx_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a + b == b + a
    }
}

impl ApproxAdditiveAbelianGroup for u8   {}
impl ApproxAdditiveAbelianGroup for u16  {}
impl ApproxAdditiveAbelianGroup for u32  {}
impl ApproxAdditiveAbelianGroup for u64  {}
impl ApproxAdditiveAbelianGroup for uint {}
impl ApproxAdditiveAbelianGroup for i8   {}
impl ApproxAdditiveAbelianGroup for i16  {}
impl ApproxAdditiveAbelianGroup for i32  {}
impl ApproxAdditiveAbelianGroup for i64  {}
impl ApproxAdditiveAbelianGroup for int  {}

pub trait AdditiveAbelianGroup
    : ApproxAdditiveAbelianGroup
    + AdditiveGroup
{
    /// Returns `true` if the addition operator is commutative for the given
    /// argument tuple.
    fn prop_add_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a + b == b + a
    }
}

impl AdditiveAbelianGroup for u8   {}
impl AdditiveAbelianGroup for u16  {}
impl AdditiveAbelianGroup for u32  {}
impl AdditiveAbelianGroup for u64  {}
impl AdditiveAbelianGroup for uint {}
impl AdditiveAbelianGroup for i8   {}
impl AdditiveAbelianGroup for i16  {}
impl AdditiveAbelianGroup for i32  {}
impl AdditiveAbelianGroup for i64  {}
impl AdditiveAbelianGroup for int  {}

pub trait ApproxMultiplicativeAbelianGroup
    : ApproxMultiplicativeGroup
{
    /// Returns `true` if the multiplication operator is approximately
    /// commutative for the given argument tuple.
    fn prop_mul_is_approx_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a * b == b * a
    }
}

pub trait MultiplicativeAbelianGroup
    : ApproxMultiplicativeAbelianGroup
    + MultiplicativeGroup
{
    /// Returns `true` if the multiplication operator is commutative for the
    /// given argument tuple.
    fn prop_mul_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a * b == b * a
    }
}
