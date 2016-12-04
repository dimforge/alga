//! Traits dedicated to linear algebra.

pub use self::vector::{VectorSpace, NormedSpace, InnerSpace, FiniteDimVectorSpace, FiniteDimInnerSpace,
                       AffineSpace, EuclideanSpace};
pub use self::transformation::{Transformation, AffineTransformation, Scaling, Similarity, Isometry,
                               DirectIsometry, Translation, OrthogonalTransformation, Rotation,
                               ProjectiveTransformation};
pub use self::matrix::{Matrix, MatrixMut, SquareMatrix, SquareMatrixMut, InversibleSquareMatrix};

mod vector;
mod transformation;
mod matrix;
mod id;
