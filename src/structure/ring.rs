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

use ops::{Additive, Multiplicative};
use cmp::ApproxEq;

use structure::Ma;

use structure::MonoidApprox;
use structure::Monoid;
use structure::GroupAbelianApprox;
use structure::GroupAbelian;

pub trait RingApprox
    : GroupAbelianApprox<Additive>
    + MonoidApprox<Multiplicative>
{
    /// Returns `true` if the multiplication and addition operators are approximately distributive for
    /// the given argument tuple.
    fn prop_mul_and_add_are_distributive_approx(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = (|| Ma(args.0.clone(), Additive), || Ma::new(args.1.clone()), || Ma::new(args.2.clone()));
        // Left distributivity
        ((a() * b()) + c()).approx_eq(&((a() * b()) + (a() * c()))) &&
        // Right distributivity
        ((b() + c()) * a()).approx_eq(&((b() * a()) + (c() * a())))
    }
}

impl_marker!(RingApprox; i8, i16, i32, i64,);

pub trait Ring
    : RingApprox
    + GroupAbelian<Additive>
    + Monoid<Multiplicative>
{
    /// Returns `true` if the multiplication and addition operators are distributive for
    /// the given argument tuple.
    fn prop_mul_and_add_are_distributive_approx(args: (Self, Self, Self)) -> bool {
        let (a, b, c) = (|| Ma(args.0.clone(), Additive), || Ma::new(args.1.clone()), || Ma::new(args.2.clone()));
        // Left distributivity
        (a() * b()) + c() == (a() * b()) + (a() * c()) &&
        // Right distributivity
        (b() + c()) * a() == (b() * a()) + (c() * a())
    }
}

impl_marker!(Ring; i8, i16, i32, i64,);

pub trait RingCommutativeApprox
    : RingApprox
{
    /// Returns `true` if the multiplication operator is approximately commutative for
    /// the given argument tuple.
    fn prop_mul_is_commutative_approx(args: (Self, Self)) -> bool {
        let (a, b) = (|| Ma(args.0.clone(), Multiplicative), || Ma::new(args.1.clone()));
        (a() * b()).approx_eq(&(b() * a()))
    }
}

impl_marker!(RingCommutativeApprox; i8, i16, i32, i64,);

pub trait RingCommutative
    : RingCommutativeApprox
    + Ring
{
    /// Returns `true` if the multiplication operator is commutative for
    /// the given argument tuple.
    fn prop_mul_is_commutative(args: (Self, Self)) -> bool {
        let (a, b) = (|| Ma(args.0.clone(), Multiplicative), || Ma::new(args.1.clone()));
        a() * b() == b() * a()
    }
}

impl_marker!(RingCommutative; i8, i16, i32, i64,);

pub trait FieldApprox
    : RingCommutativeApprox
    + GroupAbelianApprox<Multiplicative>
{}

pub trait Field
    : FieldApprox
    + RingCommutative
    + GroupAbelian<Multiplicative>
{}
