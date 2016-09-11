// Copyright 2013-2014 The Algebra Developers.
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

use ops::{Op, Additive, Multiplicative};
use ident::Identity;

use structure::Quasigroup;

/// A quasigroup with an unique identity element.
///
/// The left inverse `r` and right inverse `l` are not required to be equal.
/// The following property is added to the quasigroup structure:
///
/// ~~~notrust
/// ∃ e ∈ Self, ∀ a ∈ Self, ∃ r, l ∈ Self such that l ∘ a = a ∘ r = e
/// ~~~
pub trait Loop<O: Op>
    : Quasigroup<O>
    + Identity<O>
{}

impl_marker!(Loop<Additive>; i8, i16, i32, i64, f32, f64);
impl_marker!(Loop<Multiplicative>; f32, f64);
