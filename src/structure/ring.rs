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

use structure::MonoidMultiplicativeApprox;
use structure::MonoidMultiplicative;
use structure::GroupAdditiveAbelianApprox;
use structure::GroupAdditiveAbelian;
use structure::GroupMultiplicativeAbelian;
use structure::GroupMultiplicativeAbelianApprox;

pub trait RingApprox
    : GroupAdditiveAbelianApprox
    + MonoidMultiplicativeApprox
{
    /// Returns `true` if the multiplication and addition operators are approximately distributive for
    /// the given argument tuple.
    fn prop_mul_and_add_are_distributive_approx(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        // Left distributivity
        a.clone() * (b.clone() + c.clone()) == (a.clone() * b.clone()) + (a.clone() * c.clone()) &&
        // Right distributivity
        (b.clone() + c.clone()) * a.clone() == (b * a.clone()) + (c * a)
    }
}

impl RingApprox for i8   {}
impl RingApprox for i16  {}
impl RingApprox for i32  {}
impl RingApprox for i64  {}

pub trait Ring
    : RingApprox
    + GroupAdditiveAbelian
    + MonoidMultiplicative
{
    /// Returns `true` if the multiplication and addition operators are distributive for
    /// the given argument tuple.
    fn prop_mul_and_add_are_distributive(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = args;
        // Left distributivity
        a.clone() * (b.clone() + c.clone()) == (a.clone() * b.clone()) + (a.clone() * c.clone()) &&
        // Right distributivity
        (b.clone() + c.clone()) * a.clone() == (b * a.clone()) + (c * a)
    }
}

impl Ring for i8   {}
impl Ring for i16  {}
impl Ring for i32  {}
impl Ring for i64  {}

pub trait RingCommutativeApprox
    : RingApprox
{
    /// Returns `true` if the multiplication operator is approximately commutative for
    /// the given argument tuple.
    fn prop_mul_is_commutative_approx(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a.clone() * b.clone() == b * a
    }
}

impl RingCommutativeApprox for i8   {}
impl RingCommutativeApprox for i16  {}
impl RingCommutativeApprox for i32  {}
impl RingCommutativeApprox for i64  {}

pub trait RingCommutative
    : RingCommutativeApprox
    + Ring
{
    /// Returns `true` if the multiplication operator is commutative for
    /// the given argument tuple.
    fn prop_mul_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = args;
        a.clone() * b.clone() == b * a
    }
}

impl RingCommutative for i8   {}
impl RingCommutative for i16  {}
impl RingCommutative for i32  {}
impl RingCommutative for i64  {}

pub trait FieldApprox
    : RingCommutativeApprox
    + GroupMultiplicativeAbelianApprox
{}

pub trait Field
    : FieldApprox
    + RingCommutative
    + GroupMultiplicativeAbelian
{}
