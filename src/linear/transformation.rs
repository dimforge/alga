use general::{ClosedMul, ClosedDiv, ClosedNeg, Real, MultiplicativeGroup, SubsetOf, Id};
use linear::{NormedSpace, EuclideanSpace};

// NOTE: A subgroup trait inherit from its parent groups.

/// A general transformation acting on an euclidean space.
pub trait Transformation<E: EuclideanSpace>: MultiplicativeGroup {
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

/// The group of affine transformations. They are decomposable into a rotation, a non-uniform
/// scaling, a second rotation, and a translation (applied in that order).
pub trait AffineTransformation<E: EuclideanSpace>: Transformation<E> {
    /// Type of the first rotation to be applied.
    type PreRotation:  Rotation<E>;
    /// Type of the non-uniform scaling to be applied.
    type NonUniformScaling: AffineTransformation<E>;
    /// Type of the second rotation to be applied.
    type PostRotation: Rotation<E>;
    /// The type of the pure translation part of this affine transformation.
    type Translation:  Translation<E>;

    /// Decomposes this affine transformation into a rotation followed by a non-uniform scaling,
    /// followed by a rotation, followed by a translation.
    fn decompose(&self) -> (Self::Translation, Self::PostRotation,
                            Self::NonUniformScaling, Self::PreRotation);
    // FIXME: add a `recompose` method?
}

/// Subgroups of the similarity group `S(n)`, i.e., rotations, translations, and (signed) uniform scaling.
///
/// Similarities map lines to lines and preserve angles.
pub trait Similarity<E: EuclideanSpace>: AffineTransformation<E,
                                                              PreRotation       = Id,
                                                              NonUniformScaling = <Self as Similarity<E>>::Scaling,
                                                              PostRotation      = <Self as Similarity<E>>::Rotation> {
    /// The type of the pure rotational part of this similarity transformation.
    type Rotation: Rotation<E>;

    /// The type of the pure (uniform) scaling part of this similarity transformation.
    type Scaling: Scaling<E>;

    /*
     * Components retrieval.
     */
    /// The pure translational component of this similarity transformation.
    fn translation(&self) -> Self::Translation;

    /// The pure rotational component of this similarity transformation.
    fn rotation(&self) -> Self::Rotation;

    /// The pure scaling component of this similarity transformation.
    fn scaling(&self) -> Self::Scaling;


    /*
     * Transformations.
     */
    /// Applies this transformation's pure translational part to a point.
    #[inline]
    fn translate_point(&self, pt: &E) -> E {
        self.translation().transform_point(pt)
    }

    /// Applies this transformation's pure rotational part to a point.
    #[inline]
    fn rotate_point(&self, pt: &E) -> E {
        self.rotation().transform_point(pt)
    }

    /// Applies this transformation's pure scaling part to a point.
    #[inline]
    fn scale_point(&self, pt: &E) -> E {
        self.scaling().transform_point(pt)
    }

    /// Applies this transformation's pure rotational part to a vector.
    #[inline]
    fn rotate_vector(&self, pt: &E::Vector) -> E::Vector {
        self.rotation().transform_vector(pt)
    }

    /// Applies this transformation's pure scaling part to a vector.
    #[inline]
    fn scale_vector(&self, pt: &E::Vector) -> E::Vector {
        self.scaling().transform_vector(pt)
    }

    /*
     * Inverse transformations.
     */
    /// Applies this transformation inverse's pure translational part to a point.
    #[inline]
    fn inverse_translate_point(&self, pt: &E) -> E {
        self.translation().inverse_transform_point(pt)
    }

    /// Applies this transformation inverse's pure rotational part to a point.
    #[inline]
    fn inverse_rotate_point(&self, pt: &E) -> E {
        self.rotation().inverse_transform_point(pt)
    }

    /// Applies this transformation inverse's pure scaling part to a point.
    #[inline]
    fn inverse_scale_point(&self, pt: &E) -> E {
        self.scaling().inverse_transform_point(pt)
    }

    /// Applies this transformation inverse's pure rotational part to a vector.
    #[inline]
    fn inverse_rotate_vector(&self, pt: &E::Vector) -> E::Vector {
        self.rotation().inverse_transform_vector(pt)
    }

    /// Applies this transformation inverse's pure scaling part to a vector.
    #[inline]
    fn inverse_scale_vector(&self, pt: &E::Vector) -> E::Vector {
        self.scaling().inverse_transform_vector(pt)
    }
}

/// Subgroups of the isometry group `E(n)`, i.e., rotations, reflexions, and translations.
pub trait Isometry<E: EuclideanSpace>: Similarity<E> {
}

/// Subgroups of the orientation-preserving isometry group `SE(n)`, i.e., rotations and translations.
pub trait DirectIsometry<E: EuclideanSpace>: Isometry<E> {
}

/// Subgroups of the n-dimensional rotations and scaling `O(n)`.
pub trait OrthogonalTransformation<E: EuclideanSpace>: Isometry<E> {
}

/// Subgroups of the (signed) uniform scaling group.
pub trait Scaling<E: EuclideanSpace>: AffineTransformation<E> + SubsetOf<E::Real> {
    /// Converts this scaling factor to a real. Same as `self.to_superset()`.
    #[inline]
    fn to_real(&self) -> E::Real {
        self.to_superset()
    }

    /// Attempts to convert a real to an element of this scaling subgroup. Same as
    /// `Self::from_superset()`. Returns `None` if no such scaling is possible for this subgroup.
    #[inline]
    fn from_real(r: E::Real) -> Option<Self> {
        Self::from_superset(&r)
    }

    /// Raises the scaling to a power. The result must be equivalent to
    /// `self.to_superset().powf(n)`. Returns `None` if the result is not representable by `Self`.
    #[inline]
    fn powf(&self, n: E::Real) -> Option<Self> {
        Self::from_superset(&self.to_superset().powf(n))
    }

    /// The scaling required to make `a` have the same norm as `b`, i.e., `|b| = |a| * norm_ratio(a,
    /// b)`.
    #[inline]
    fn scale_between(a: &E::Vector, b: &E::Vector) -> Option<Self> {
        Self::from_superset(&(b.norm() / a.norm()))
    }
}

/// Subgroups of the n-dimensional translation group `T(n)`.
pub trait Translation<E: EuclideanSpace>: DirectIsometry<E> /* + SubsetOf<E::Vector> */ {
    // NOTE: we must define those two conversions here (instead of just using SubsetOf) because the
    // structure of Self uses the multiplication for composition, while E::Vector uses addition.
    // Having a trait that sais "remap this operator to this other one" does not seem to be
    // possible without higher kinded traits.
    /// Converts this translation to a vector.
    fn to_vector(&self) -> E::Vector;

    /// Attempts to convert a vector to this translation. Returns `None` if the translation
    /// represented by `v` is not part of the translation subgroup represented by `Self`.
    fn from_vector(v: &E::Vector) -> Option<Self>;

    /// Raises the translation to a power. The result must be equivalent to
    /// `self.to_superset() * n`.  Returns `None` if the result is not representable by `Self`.
    #[inline]
    fn powf(&self, n: E::Real) -> Option<Self> {
        Self::from_vector(&(self.to_vector() * n))
    }

    /// The translation needed to make `a` coincide with `b`, i.e., `b = a * translation_to(a, b)`.
    #[inline]
    fn translation_between(a: &E, b: &E) -> Option<Self> {
        Self::from_vector(&(b.clone() - a.clone()))
    }
}

/// Subgroups of the n-dimensional rotation group `SO(n)`.
pub trait Rotation<E: EuclideanSpace>: OrthogonalTransformation<E> + DirectIsometry<E> {
    /// Raises this rotation to a power. If this is a simple rotation, the result must be
    /// equivalent to multiplying the rotation angle by `n`.
    fn powf(&self, n: E::Real) -> Option<Self>;

    /// Computes a simple rotation that makes the angle between `a` and `b` equal to zero, i.e.,
    /// `b.angle(a * delta_rotation(a, b)) = 0`. If `a` and `b` are collinear, the computed
    /// rotation may not be unique. Returns `None` if no such simple rotation exists in the
    /// subgroup represented by `Self`.
    fn rotation_between(a: &E::Vector, b: &E::Vector) -> Option<Self>;
}



/*
 *
 * Implementation for floats.
 *
 */

impl<R, E> Transformation<E> for R
where R: Real,
      E: EuclideanSpace<Real = R>,
      E::Vector: ClosedMul<R> + ClosedDiv<R> + ClosedNeg {
    fn transform_point(&self, pt: &E) -> E {
        pt.scale_by(*self)
    }

    fn transform_vector(&self, v: &E::Vector) -> E::Vector {
        v.clone() * *self
    }

    fn inverse_transform_point(&self, pt: &E) -> E {
        assert!(*self != R::zero());
        pt.scale_by(R::one() / *self)
    }

    fn inverse_transform_vector(&self, v: &E::Vector) -> E::Vector {
        assert!(*self != R::zero());
        v.clone() * (R::one() / *self)
    }
}

impl<R, E> AffineTransformation<E> for R
where R: Real,
      E: EuclideanSpace<Real = R>,
      E::Vector: ClosedMul<R> + ClosedDiv<R> + ClosedNeg {
    type PreRotation       = Id;
    type NonUniformScaling = R;
    type PostRotation      = Id;
    type Translation       = Id;

    #[inline]
    fn decompose(&self) -> (Id, Id, R, Id) {
        (Id::new(), Id::new(), *self, Id::new())
    }
}

impl<R, E> Scaling<E> for R
where R: Real + SubsetOf<R>,
      E: EuclideanSpace<Real = R>,
      E::Vector: ClosedMul<R> + ClosedDiv<R> + ClosedNeg {
    #[inline]
    fn to_real(&self) -> E::Real {
        *self
    }

    #[inline]
    fn from_real(r: E::Real) -> Option<Self> {
        Some(r)
    }

    #[inline]
    fn powf(&self, n: E::Real) -> Option<Self> {
        Some(n.powf(n))
    }

    #[inline]
    fn scale_between(a: &E::Vector, b: &E::Vector) -> Option<Self> {
        Some((b.norm() / a.norm()))
    }
}

impl<R, E> Similarity<E> for R
where R: Real + SubsetOf<R>,
      E: EuclideanSpace<Real = R>,
      E::Vector: ClosedMul<R> + ClosedDiv<R> + ClosedNeg {
    type Rotation    = Id;
    type Scaling     = R;

    fn translation(&self) -> Self::Translation {
        Id::new()
    }

    fn rotation(&self) -> Self::Rotation {
        Id::new()
    }

    fn scaling(&self) -> Self::Scaling {
        *self
    }
}
