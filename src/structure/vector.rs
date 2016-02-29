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

use structure::GroupAdditiveAbelianApprox;
use structure::GroupAdditiveAbelian;
use structure::RingCommutativeApprox;
use structure::RingCommutative;
use structure::FieldApprox;
use structure::Field;

pub trait ModuleApprox<S: RingCommutativeApprox>
    : GroupAdditiveAbelianApprox
{}

pub trait Module<S: RingCommutative>
    : ModuleApprox<S>
    + GroupAdditiveAbelian
{}

pub trait VectorApprox<S: FieldApprox>
    : ModuleApprox<S>
{}

pub trait Vector<S: Field>
    : VectorApprox<S>
    + Module<S>
{}
