# Change Log
All notable changes to `alga`, starting with the version 0.4.0 will be
documented here.

This project adheres to [Semantic Versioning](http://semver.org/).

## [0.5.0]
  * Traits are now also implemented for `Complex`.
  * `AbstractModule` now inherit from `AdditiveGroupAbelian` as well.

## [0.4.0]
  * The `Real` trait now includes some commonly implemented markers, e.g.,
    Sync, Any, 'static, etc.
  * The `Module` trait no longer inherit from operator overload traits. This is
    due to a [limitation](https://github.com/rust-lang/rust/issues/37883) in
    the compiler that prevent them from being used properly in generic code.
  * The `Transform` trait is split into `Transform` and `ProjectiveTransform`.
    Inverse transformation methods have been moved from the first to the second.
  * The `Affine` trait now has methods for appending/prepending pure
    translation/rotation/scaling.
  * `EuclideanSpace::Vector` has been renamed `EuclideanSpace::Coordinates`.
  * Added `Rotation::scaled_rotation_between` wich is a combination of
    `Rotation::rotation_between` and `Rotation::powf`.
  * `FiniteDimVectorSpace` looses `::component` but gains the
    `::component_unchecked_mut` method (for mutable compoent borrowing without
    bound-checking).
  * Added `EuclideanSpace::from_coordinates` that builds a point from its
    coordinates.


