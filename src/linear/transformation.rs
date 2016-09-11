use general::{Group, Multiplicative};
use linear::EuclideanSpace;

// NOTE: A subgroup trait inherit from its parent groups.

/// A general transformation acting on an euclidean space.
pub trait Transformation<E: EuclideanSpace>: Group<Multiplicative> {
    /// Applies this group's action on a point from the euclidean space.
    fn transform_point(&self, pt: &E) -> E;

    /// Applies this group's action on a vector from the euclidean space.
    ///
    /// If `v` is a vector and `a, b` two point such that `v = a - b`, the action `∘` on a vector
    /// is defined as `self ∘ v = (self × a) - (self × b)`.
    fn transform_vector(&self, v: &E::Vector) -> E::Vector;

    /// Applies this group's inverse action on a point from the euclidean space.
    fn inverse_transform_point(&self, pt: &E) -> E;

    /// Applies this group's inverse action on a vector from the euclidean space.
    ///
    /// If `v` is a vector and `a, b` two point such that `v = a - b`, the action `∘` on a vector
    /// is defined as `self ∘ v = (self × a) - (self × b)`.
    fn inverse_transform_vector(&self, v: &E::Vector) -> E::Vector;
}

/// The group `S(n)` of similarities, i.e., rotations, translations, and (signed) uniform scaling.
///
/// Similarities map lines to lines and preserve angles.
pub trait Similarity<E: EuclideanSpace>: Group<Multiplicative> {
    /// The type of the pure translation part of this similarity.
    type Translation: Translation<E>;
    /// The type of the pure rotational part of this similarity.
    type Rotation: Rotation<E>;

    /// The translational component of this similarity.
    fn translation(&self) -> Self::Translation;
    /// The rotational component of this similarity.
    fn rotation(&self) -> Self::Rotation;
    /// The scaling factor of this similarity.
    fn scaling_factor(&self) -> E::Real;
}

/// The group `E(n)` of isometries, i.e., rotations, reflexions, and translations.
pub trait Isometry<E: EuclideanSpace>: Similarity<E> {
}

/// The group `SE(n)` of orientation-preserving isometries, i.e., rotations and translations.
pub trait DirectIsometry<E: EuclideanSpace>: Isometry<E> {
}

/// The group `T(n)` of translations.
pub trait Translation<E: EuclideanSpace>: DirectIsometry<E> {
}

/// The group `O(n)` of n-dimensional rotations and reflexions.
pub trait OrthogonalGroup<E: EuclideanSpace>: Isometry<E> {
}

/// The group `SO(n)` of n-dimensional of rotations.
pub trait Rotation<E: EuclideanSpace>: OrthogonalGroup<E> + DirectIsometry<E> {
}
