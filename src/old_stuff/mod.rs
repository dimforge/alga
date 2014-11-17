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

//! Operators with specific properties

pub use self::associative::ApproxAssociativeAdd;
pub use self::associative::AssociativeAdd;
pub use self::associative::ApproxAssociativeMul;
pub use self::associative::AssociativeMul;

pub use self::commutative::ApproxCommutativeAdd;
pub use self::commutative::CommutativeAdd;
pub use self::commutative::ApproxCommutativeMul;
pub use self::commutative::CommutativeMul;

pub use self::distributive::ApproxDistributiveMulAdd;
pub use self::distributive::DistributiveMulAdd;

pub use self::recip::Recip;

mod associative;
mod commutative;
mod distributive;
mod recip;
