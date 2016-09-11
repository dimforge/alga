use general::{Group, Multiplicative};
use linear::EuclideanSpace;

// NOTE: A subgroup trait inherit from its parent groups.

/// The group `E(n)` of isometries, i.e., rotations, reflexions, and translations.
pub trait Isometry<E: EuclideanSpace>: Group<Multiplicative> {
    /// Applies this group's action on a point from the euclidean space.
    fn transform_point(&self, pt: &E) -> E;

    /// Applies this group's action on a vector from the euclidean space.
    ///
    /// If `v` is a vector and `a, b` two point such that `v = a - b`, the action `∘` on a vector
    /// is defined as `self ∘ v = (self × a) - (self × b)`.
    fn transform_vector(&self, v: &E::Vector) -> E::Vector;
}

/// The group `SE(n)` of orientation-preserving isometries, i.e., rotations and translations.
///
/// This is a subgroup of `E(n)`.
pub trait DirectIsometry<E: EuclideanSpace>: Isometry<E> {
}

/// The group `T(n)` of translations.
///
/// This is a subgroup of `SE(n)`.
pub trait Translation<E: EuclideanSpace>: DirectIsometry<E> {
}

/// The group `O(n)` of n-dimensional rotations and reflexions.
///
/// This is a subgroup of `E(n)`.
pub trait OrthogonalGroup<E: EuclideanSpace>: Isometry<E> {
}

/// The group `SO(n)` of n-dimensional of rotations.
///
/// This is a subgroup of `O(n)`.
pub trait Rotation<E: EuclideanSpace>: OrthogonalGroup<E> + DirectIsometry<E> {
}
