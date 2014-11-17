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

use structure::ApproxAdditiveQuasigroup;
use structure::AdditiveQuasigroup;
use structure::ApproxMultiplicativeQuasigroup;
use structure::MultiplicativeQuasigroup;
use structure::AdditiveIdentity;
use structure::MultiplicativeIdentity;

/// An aproximate additive quasigroup with a corresponding identity.
pub trait ApproxAdditiveLoop
    : ApproxAdditiveQuasigroup
    + AdditiveIdentity
{}

impl ApproxAdditiveLoop for u8   {}
impl ApproxAdditiveLoop for u16  {}
impl ApproxAdditiveLoop for u32  {}
impl ApproxAdditiveLoop for u64  {}
impl ApproxAdditiveLoop for uint {}
impl ApproxAdditiveLoop for i8   {}
impl ApproxAdditiveLoop for i16  {}
impl ApproxAdditiveLoop for i32  {}
impl ApproxAdditiveLoop for i64  {}
impl ApproxAdditiveLoop for int  {}

/// An additive quasigroup with a corresponding identity.
pub trait AdditiveLoop
    : ApproxAdditiveLoop
    + AdditiveQuasigroup
{}

impl AdditiveLoop for u8   {}
impl AdditiveLoop for u16  {}
impl AdditiveLoop for u32  {}
impl AdditiveLoop for u64  {}
impl AdditiveLoop for uint {}
impl AdditiveLoop for i8   {}
impl AdditiveLoop for i16  {}
impl AdditiveLoop for i32  {}
impl AdditiveLoop for i64  {}
impl AdditiveLoop for int  {}

/// An aproximate multiplicative quasigroup with a corresponding identity.
pub trait ApproxMultiplicativeLoop
    : ApproxMultiplicativeQuasigroup
    + MultiplicativeIdentity
{}

/// An aproximate multiplicative quasigroup with a corresponding identity.
pub trait MultiplicativeLoop
    : ApproxMultiplicativeLoop
    + MultiplicativeQuasigroup
{}
