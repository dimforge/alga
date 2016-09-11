//! Traits dedicated to linear algebra.

pub use self::vector::{VectorSpace, NormedSpace, InnerSpace, FiniteDimVectorSpace, FiniteDimInnerSpace,
                       AffineSpace, EuclideanSpace};
pub use self::transformation::{Isometry, DirectIsometry, Translation, OrthogonalGroup, Rotation};

mod vector;
mod transformation;
