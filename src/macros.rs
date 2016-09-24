// Copyright 2014 The Algebra Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

#[macro_export]
macro_rules! impl_marker(
    (@para_rec
        [$M:ty, ($($G:tt)+), ($($F:tt)*)]
        (< $($R:tt)*)
    ) => {
        impl< $($R)* $M for $($F)*< $($R)*
            where $($G)+
        {}
    };
    (@para_rec
        [$M:ty, ($($G:tt)+), ($($F:tt)*)]
        ($C:tt $($R:tt)*)
    ) => {
        impl_marker!(@para_rec
            [$M, ($($G)+), ($($F)* $C)]
            ($($R)*)
        );
    };
    (@where_rec
        [$M:ty, ($($P:tt)+), ($($G:tt)+)]
        ($(;)*)
    ) => {
        impl_marker!(@para_rec
            [$M, ($($G)+), ()]
            ($($P)+)
        );
    };
    (@where_rec
        [$M:ty, ($($P:tt)+), ($($G:tt)+)]
        (; $($R:tt)+)
    ) => {
        impl_marker!(@para_rec
            [$M, ($($G)+), ()]
            ($($P)+)
        );
        impl_marker!(@rec
            [$M, ()]
            ($($R)+)
        );
    };
    (@where_rec
        [$M:ty, ($($P:tt)+), ($($F:tt)*)]
        ($C:tt $($R:tt)*)
    ) => {
        impl_marker!(@where_rec
            [$M, ($($P)+), ($($F)* $C)]
            ($($R)*)
        );
    };
    (@rec
        [$M:ty, ($($F:tt)*)]
        ($(;)*)
    ) => {
        impl $M for $($F)* { }
    };
    (@rec
        [$M:ty, ($($F:tt)*)]
        (; $($R:tt)+)
    ) => {
        impl $M for $($F)* { }
        impl_marker!(@rec
            [$M, ()]
            ($($R)+)
        );
    };
    (@rec
        [$M:ty, ($($F:tt)+)]
        (where $($G:tt)+)
    ) => {
        impl_marker!(@where_rec
            [$M, ($($F)+), ()]
            ($($G)+)
        );
    };
    (@rec
        [$M:ty, ($($F:tt)*)]
        ($C:tt $($R:tt)*)
    ) => {
        impl_marker!(@rec
            [$M, ($($F)* $C)]
            ($($R)*)
        );
    };
    ($M:ty; $($R:tt)+) => {
        impl_marker!(@rec
            [$M, ()]
            ($($R)+)
        );
    };
);

macro_rules! impl_ident {
    ($M:ty; $V:expr; $($T:ty),* $(,)*) => {
        $(impl Identity<$M> for $T { #[inline] fn identity() -> $T {$V} })+
    }
}

macro_rules! impl_approx_eq {
    ($V:expr; $($T:ty),* $(,)*) => {
        $(impl ApproxEq for $T {
            type Eps = $T;
            #[inline]
            fn default_epsilon() -> Self::Eps { $V }
            #[inline]
            fn approx_eq_eps(&self, b: &$T, epsilon: &$T) -> bool {
                if self < b {
                    *b - *self <= *epsilon
                } else {
                    *self - *b <= *epsilon
                }
            }
        })+
    }
}
