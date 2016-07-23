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

//! Identities for binary operators

use ops::{Op, Additive, Multiplicative};

/// A type that is equipped with identity.
pub trait Identity<O: Op> {
    /// The identity element.
    fn id() -> Self;
}

/// The identity element.
pub fn id<T: Identity<O>, O: Op>() -> T {
    Identity::id()
}

impl Identity<Additive> for u8   { #[inline] fn id() -> u8   { 0   } }
impl Identity<Additive> for u16  { #[inline] fn id() -> u16  { 0   } }
impl Identity<Additive> for u32  { #[inline] fn id() -> u32  { 0   } }
impl Identity<Additive> for u64  { #[inline] fn id() -> u64  { 0   } }
impl Identity<Additive> for i8   { #[inline] fn id() -> i8   { 0   } }
impl Identity<Additive> for i16  { #[inline] fn id() -> i16  { 0   } }
impl Identity<Additive> for i32  { #[inline] fn id() -> i32  { 0   } }
impl Identity<Additive> for i64  { #[inline] fn id() -> i64  { 0   } }
impl Identity<Additive> for f32  { #[inline] fn id() -> f32  { 0.0 } }
impl Identity<Additive> for f64  { #[inline] fn id() -> f64  { 0.0 } }

impl Identity<Multiplicative> for u8   { #[inline] fn id() -> u8   { 1   } }
impl Identity<Multiplicative> for u16  { #[inline] fn id() -> u16  { 1   } }
impl Identity<Multiplicative> for u32  { #[inline] fn id() -> u32  { 1   } }
impl Identity<Multiplicative> for u64  { #[inline] fn id() -> u64  { 1   } }
impl Identity<Multiplicative> for i8   { #[inline] fn id() -> i8   { 1   } }
impl Identity<Multiplicative> for i16  { #[inline] fn id() -> i16  { 1   } }
impl Identity<Multiplicative> for i32  { #[inline] fn id() -> i32  { 1   } }
impl Identity<Multiplicative> for i64  { #[inline] fn id() -> i64  { 1   } }
impl Identity<Multiplicative> for f32  { #[inline] fn id() -> f32  { 1.0 } }
impl Identity<Multiplicative> for f64  { #[inline] fn id() -> f64  { 1.0 } }
