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

use structure::ApproxAdditiveLoop;
use structure::AdditiveLoop;
use structure::ApproxMultiplicativeLoop;
use structure::MultiplicativeLoop;
use structure::ApproxAdditiveMonoid;
use structure::AdditiveMonoid;
use structure::ApproxMultiplicativeMonoid;
use structure::MultiplicativeMonoid;

pub trait ApproxAdditiveGroup
    : ApproxAdditiveLoop
    + ApproxAdditiveMonoid
{}

impl ApproxAdditiveGroup for u8   {}
impl ApproxAdditiveGroup for u16  {}
impl ApproxAdditiveGroup for u32  {}
impl ApproxAdditiveGroup for u64  {}
impl ApproxAdditiveGroup for uint {}
impl ApproxAdditiveGroup for i8   {}
impl ApproxAdditiveGroup for i16  {}
impl ApproxAdditiveGroup for i32  {}
impl ApproxAdditiveGroup for i64  {}
impl ApproxAdditiveGroup for int  {}

pub trait AdditiveGroup
    : ApproxAdditiveGroup
    + AdditiveLoop
    + AdditiveMonoid
{}

impl AdditiveGroup for u8   {}
impl AdditiveGroup for u16  {}
impl AdditiveGroup for u32  {}
impl AdditiveGroup for u64  {}
impl AdditiveGroup for uint {}
impl AdditiveGroup for i8   {}
impl AdditiveGroup for i16  {}
impl AdditiveGroup for i32  {}
impl AdditiveGroup for i64  {}
impl AdditiveGroup for int  {}

pub trait ApproxMultiplicativeGroup
    : ApproxMultiplicativeLoop
    + ApproxMultiplicativeMonoid
{}

pub trait MultiplicativeGroup
    : ApproxMultiplicativeGroup
    + MultiplicativeLoop
    + MultiplicativeMonoid
{}
