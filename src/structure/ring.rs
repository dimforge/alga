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
        use ops::Multiplicative as Mul;
        use ops::Additive as Add;
        let (a, b, c) = (|| args.0.clone(), || args.1.clone(), || args.2.clone());
        // Left distributivity
        Self::approx_eq(&a().ap(Mul, b().ap(Add, c())), &a().ap(Mul, b()).ap(Add, a().ap(Mul, c()))) &&
        // Right distributivity
        Self::approx_eq(&b().ap(Add, c()).ap(Mul, a()), &b().ap(Mul, a()).ap(Add, c().ap(Mul, a())))
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
        use ops::Multiplicative as Mul;
        use ops::Additive as Add;
        let (a, b, c) = (|| args.0.clone(), || args.1.clone(), || args.2.clone());
        // Left distributivity
        a().op(Mul, b().op(Add, c())) == a().op(Mul, b()).op(Add, a().op(Mul, c())) &&
        // Right distributivity
        b().op(Add, c()).op(Mul, a()) == b().op(Mul, a()).op(Add, c().op(Mul, a()))
    }
}

impl_marker!(Ring; i8, i16, i32, i64,);

pub trait RingCommutativeApprox
    : RingApprox
{
    /// Returns `true` if the multiplication operator is approximately commutative for
    /// the given argument tuple.
    fn prop_mul_is_commutative_approx(args: (Self, Self)) -> bool {
        use ops::Multiplicative as Mul;
        let (a, b) = (|| args.0.clone(), || args.1.clone());
        Self::approx_eq(&a().ap(Mul, b()), &b().ap(Mul, a()))
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
        use ops::Multiplicative as Mul;
        let (a, b) = (|| args.0.clone(), || args.1.clone());
        a().op(Mul, b()) == b().op(Mul, a())
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
