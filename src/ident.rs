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

//! Identities for binary operators.

use ops::{Op, Additive, Multiplicative};

/// A type that is equipped with identity.
pub trait Identity<O: Op> {
    /// The identity element.
    fn id() -> Self;
}

/// The identity element.
pub fn id<T: Identity<O>, O: Op>(_: O) -> T {
    Identity::id()
}

impl_ident!(Additive; 0; u8, u16, u32, u64, i8, i16, i32, i64);
impl_ident!(Additive; 0.; f32, f64);
impl_ident!(Multiplicative; 1; u8, u16, u32, u64, i8, i16, i32, i64);
impl_ident!(Multiplicative; 1.; f32, f64);
