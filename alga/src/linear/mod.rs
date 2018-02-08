//! Traits dedicated to linear algebra.

pub use self::vector::{AffineSpace, EuclideanSpace, FiniteDimInnerSpace, FiniteDimVectorSpace,
                       InnerSpace, NormedSpace, VectorSpace};
pub use self::transformation::{AffineTransformation, DirectIsometry, Isometry,
                               OrthogonalTransformation, ProjectiveTransformation, Rotation,
                               Scaling, Similarity, Transformation, Translation};
pub use self::matrix::{InversibleSquareMatrix, Matrix, MatrixMut, SquareMatrix, SquareMatrixMut};

mod vector;
mod transformation;
mod matrix;
mod id;
