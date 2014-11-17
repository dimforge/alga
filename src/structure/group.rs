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

use structure::LoopAdditiveApprox;
use structure::LoopAdditive;
use structure::LoopMultiplicativeApprox;
use structure::LoopMultiplicative;
use structure::MonoidAdditiveApprox;
use structure::MonoidAdditive;
use structure::MonoidMultiplicativeApprox;
use structure::MonoidMultiplicative;

pub trait GroupAdditiveApprox
    : LoopAdditiveApprox
    + MonoidAdditiveApprox
{}

impl GroupAdditiveApprox for u8   {}
impl GroupAdditiveApprox for u16  {}
impl GroupAdditiveApprox for u32  {}
impl GroupAdditiveApprox for u64  {}
impl GroupAdditiveApprox for uint {}
impl GroupAdditiveApprox for i8   {}
impl GroupAdditiveApprox for i16  {}
impl GroupAdditiveApprox for i32  {}
impl GroupAdditiveApprox for i64  {}
impl GroupAdditiveApprox for int  {}

pub trait GroupAdditive
    : GroupAdditiveApprox
    + LoopAdditive
    + MonoidAdditive
{}

impl GroupAdditive for u8   {}
impl GroupAdditive for u16  {}
impl GroupAdditive for u32  {}
impl GroupAdditive for u64  {}
impl GroupAdditive for uint {}
impl GroupAdditive for i8   {}
impl GroupAdditive for i16  {}
impl GroupAdditive for i32  {}
impl GroupAdditive for i64  {}
impl GroupAdditive for int  {}

pub trait GroupMultiplicativeApprox
    : LoopMultiplicativeApprox
    + MonoidMultiplicativeApprox
{}

pub trait GroupMultiplicative
    : GroupMultiplicativeApprox
    + LoopMultiplicative
    + MonoidMultiplicative
{}
