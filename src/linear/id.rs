use num;

use general::{Identity, Id};
use linear::{InnerSpace, EuclideanSpace, Transformation, AffineTransformation, Scaling, Similarity,
             Isometry, DirectIsometry, OrthogonalTransformation, Translation, Rotation};

/*
 * Implementation of linear algebra structures for the ubiquitous identity element.
 */
impl<E: EuclideanSpace> Transformation<E> for Id {
    #[inline]
    fn transform_point(&self, pt: &E) -> E {
        pt.clone()
    }

    #[inline]
    fn transform_vector(&self, v: &E::Vector) -> E::Vector {
        v.clone()
    }

    #[inline]
    fn inverse_transform_point(&self, pt: &E) -> E {
        pt.clone()
    }

    #[inline]
    fn inverse_transform_vector(&self, v: &E::Vector) -> E::Vector {
        v.clone()
    }
}

impl<E: EuclideanSpace> AffineTransformation<E> for Id {
    type PreRotation       = Id;
    type NonUniformScaling = Id;
    type PostRotation      = Id;
    type Translation       = Id;

    #[inline]
    fn decompose(&self) -> (Id, Id, Id, Id) {
        (Id::new(), Id::new(), Id::new(), Id::new())
    }
}

impl<E: EuclideanSpace> Similarity<E> for Id {
    type Rotation = Id;
    type Scaling  = Id;

    #[inline]
    fn translation(&self) -> Self::Translation {
        Id::new()
    }

    #[inline]
    fn rotation(&self) -> Self::Rotation {
        Id::new()
    }

    #[inline]
    fn scaling(&self) -> Self::Scaling {
        Id::new()
    }
}

impl<E: EuclideanSpace> Scaling<E>        for Id { }
impl<E: EuclideanSpace> Isometry<E>       for Id { }
impl<E: EuclideanSpace> DirectIsometry<E> for Id { }
impl<E: EuclideanSpace> OrthogonalTransformation<E> for Id { }


impl<E: EuclideanSpace> Rotation<E> for Id {
    #[inline]
    fn powf(&self, _: E::Real) -> Option<Self> {
        Some(Id::new())
    }

    #[inline]
    fn rotation_between(a: &E::Vector, b: &E::Vector) -> Option<Self> {
        if a.angle(b) == num::zero() {
            Some(Id::new())
        }
        else {
            None
        }
    }
}

impl<E: EuclideanSpace> Translation<E> for Id {
    #[inline]
    fn to_vector(&self) -> E::Vector {
        E::Vector::identity()
    }

    #[inline]
    fn from_vector(v: &E::Vector) -> Option<Self> {
        if *v == E::Vector::identity() {
            Some(Id::new())
        }
        else {
            None
        }
    }
}
