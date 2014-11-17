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

use structure::QuasigroupAdditiveApprox;
use structure::QuasigroupAdditive;
use structure::QuasigroupMultiplicativeApprox;
use structure::QuasigroupMultiplicative;
use structure::IdentityAdditive;
use structure::IdentityMultiplicative;

/// An aproximate additive quasigroup with a corresponding identity.
pub trait LoopAdditiveApprox
    : QuasigroupAdditiveApprox
    + IdentityAdditive
{}

impl LoopAdditiveApprox for u8   {}
impl LoopAdditiveApprox for u16  {}
impl LoopAdditiveApprox for u32  {}
impl LoopAdditiveApprox for u64  {}
impl LoopAdditiveApprox for uint {}
impl LoopAdditiveApprox for i8   {}
impl LoopAdditiveApprox for i16  {}
impl LoopAdditiveApprox for i32  {}
impl LoopAdditiveApprox for i64  {}
impl LoopAdditiveApprox for int  {}

/// An additive quasigroup with a corresponding identity.
pub trait LoopAdditive
    : LoopAdditiveApprox
    + QuasigroupAdditive
{}

impl LoopAdditive for u8   {}
impl LoopAdditive for u16  {}
impl LoopAdditive for u32  {}
impl LoopAdditive for u64  {}
impl LoopAdditive for uint {}
impl LoopAdditive for i8   {}
impl LoopAdditive for i16  {}
impl LoopAdditive for i32  {}
impl LoopAdditive for i64  {}
impl LoopAdditive for int  {}

/// An aproximate multiplicative quasigroup with a corresponding identity.
pub trait LoopMultiplicativeApprox
    : QuasigroupMultiplicativeApprox
    + IdentityMultiplicative
{}

/// An aproximate multiplicative quasigroup with a corresponding identity.
pub trait LoopMultiplicative
    : LoopMultiplicativeApprox
    + QuasigroupMultiplicative
{}
