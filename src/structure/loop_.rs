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

use ops::{Op, Additive};

use structure::Quasigroup;
use structure::QuasigroupApprox;
use structure::Identity;

/// An aproximate quasigroup with a corresponding identity.
pub trait LoopApprox<O: Op>
    : QuasigroupApprox<O>
    + Identity<O>
{}

impl LoopApprox<Additive> for i8   {}
impl LoopApprox<Additive> for i16  {}
impl LoopApprox<Additive> for i32  {}
impl LoopApprox<Additive> for i64  {}

/// A quasigroup with a corresponding identity.
pub trait Loop<O: Op>
    : LoopApprox<O>
    + Quasigroup<O>
{}

impl Loop<Additive> for i8   {}
impl Loop<Additive> for i16  {}
impl Loop<Additive> for i32  {}
impl Loop<Additive> for i64  {}
