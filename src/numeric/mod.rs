//! Basic trait for numeric types.

pub use self::partial_order::{PartialOrder, PartialOrdering};
pub use approx::ApproxEq; // Re-export.

mod partial_order;
