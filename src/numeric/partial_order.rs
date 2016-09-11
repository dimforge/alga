use std::cmp::Ordering;

/// Result of a partial ordering.
#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub enum PartialOrdering {
    /// Result of a strict comparison.
    PartialLess,
    /// Equality relationship.
    PartialEqual,
    /// Result of a strict comparison.
    PartialGreater,
    /// Result of a comparison between two objects that are not comparable.
    NotComparable
}

impl PartialOrdering {
    /// Returns `true` if `self` is equal to `Equal`.
    pub fn is_eq(&self) -> bool {
        *self == PartialOrdering::PartialEqual
    }

    /// Returns `true` if `self` is equal to `Less`.
    pub fn is_lt(&self) -> bool {
        *self == PartialOrdering::PartialLess
    }

    /// Returns `true` if `self` is equal to `Less` or `Equal`.
    pub fn is_le(&self) -> bool {
        *self == PartialOrdering::PartialLess || *self == PartialOrdering::PartialEqual
    }

    /// Returns `true` if `self` is equal to `Greater`.
    pub fn is_gt(&self) -> bool {
        *self == PartialOrdering::PartialGreater
    }

    /// Returns `true` if `self` is equal to `Greater` or `Equal`.
    pub fn is_ge(&self) -> bool {
        *self == PartialOrdering::PartialGreater || *self == PartialOrdering::PartialEqual
    }

    /// Returns `true` if `self` is equal to `NotComparable`.
    pub fn is_not_comparable(&self) -> bool {
        *self == PartialOrdering::NotComparable
    }

    /// Creates a `PartialOrdering` from an `Ordering`.
    pub fn from_ordering(ord: Ordering) -> PartialOrdering {
        match ord {
            Ordering::Less    => PartialOrdering::PartialLess,
            Ordering::Equal   => PartialOrdering::PartialEqual,
            Ordering::Greater => PartialOrdering::PartialGreater
        }
    }

    /// Converts this `PartialOrdering` to an `Ordering`.
    ///
    /// Returns `None` if `self` is `NotComparable`.
    pub fn to_ordering(self) -> Option<Ordering> {
        match self {
            PartialOrdering::PartialLess    => Some(Ordering::Less),
            PartialOrdering::PartialEqual   => Some(Ordering::Equal),
            PartialOrdering::PartialGreater => Some(Ordering::Greater),
            PartialOrdering::NotComparable  => None
        }
    }
}

/// Pointwise ordering operations.
pub trait PartialOrder: Sized {
    /// Returns the infimum of this value and another or `None` if they don't have any.
    fn inf(&self, other: &Self) -> Option<Self>;

    /// Returns the supremum of this value and another or `None` if they don't have any
    fn sup(&self, other: &Self) -> Option<Self>;

    /// Compare `self` and `other` using a partial ordering relation.
    fn partial_cmp(&self, other: &Self) -> PartialOrdering;

    /// Returns `true` iff `self` and `other` are comparable and `self <= other`.
    #[inline]
    fn partial_le(&self, other: &Self) -> bool {
        PartialOrder::partial_cmp(self, other).is_le()
    }

    /// Returns `true` iff `self` and `other` are comparable and `self < other`.
    #[inline]
    fn partial_lt(&self, other: &Self) -> bool {
        PartialOrder::partial_cmp(self, other).is_lt()
    }

    /// Returns `true` iff `self` and `other` are comparable and `self >= other`.
    #[inline]
    fn partial_ge(&self, other: &Self) -> bool {
        PartialOrder::partial_cmp(self, other).is_ge()
    }

    /// Returns `true` iff `self` and `other` are comparable and `self > other`.
    #[inline]
    fn partial_gt(&self, other: &Self) -> bool {
        PartialOrder::partial_cmp(self, other).is_gt()
    }

    /// Return the minimum of `self` and `other` if they are comparable.
    #[inline]
    fn partial_min<'a>(&'a self, other: &'a Self) -> Option<&'a Self> {
        match PartialOrder::partial_cmp(self, other) {
            PartialOrdering::PartialLess | PartialOrdering::PartialEqual => Some(self),
            PartialOrdering::PartialGreater                              => Some(other),
            PartialOrdering::NotComparable                               => None
        }
    }

    /// Return the maximum of `self` and `other` if they are comparable.
    #[inline]
    fn partial_max<'a>(&'a self, other: &'a Self) -> Option<&'a Self> {
        match PartialOrder::partial_cmp(self, other) {
            PartialOrdering::PartialGreater | PartialOrdering::PartialEqual => Some(self),
            PartialOrdering::PartialLess                                    => Some(other),
            PartialOrdering::NotComparable                                  => None
        }
    }

    /// Clamp `value` between `min` and `max`. Returns `None` if `value` is not comparable to
    /// `min` or `max`.
    #[inline]
    fn partial_clamp<'a>(&'a self, min: &'a Self, max: &'a Self) -> Option<&'a Self> {
        let v_min = self.partial_cmp(min);
        let v_max = self.partial_cmp(max);

        if v_min.is_not_comparable() || v_max.is_not_comparable() {
            None
        }
        else if v_min.is_lt() {
            Some(min)
        }
        else if v_max.is_gt() {
            Some(max)
        }
        else {
            Some(self)
        }
    }
}

macro_rules! impl_partial_order_float(
    ($($t: ty),*) => ($(
        impl PartialOrder for $t {
            /// Returns the infimum of this value and another or `None` if they don't have any.
            #[inline]
            fn inf(&self, other: &Self) -> Option<Self> {
                if !self.is_nan() && !other.is_nan() {
                    if *self < *other {
                        Some(*self)
                    }
                    else { // *self >= *other
                        Some(*other)
                    }
                }
                else {
                    None
                }
            }
        
            /// Returns the supremum of this value and another or `None` if they don't have any
            #[inline]
            fn sup(&self, other: &Self) -> Option<Self> {
                if !self.is_nan() && !other.is_nan() {
                    if *self > *other {
                        Some(*self)
                    }
                    else { // *self <= *other
                        Some(*other)
                    }
                }
                else {
                    None
                }
            }
        
            /// Compare `self` and `other` using a partial ordering relation.
            #[inline]
            fn partial_cmp(&self, other: &Self) -> PartialOrdering {
                match (self <= other, self >= other) {
                    (false, false) => PartialOrdering::NotComparable,
                    (false, true)  => PartialOrdering::PartialGreater,
                    (true, false)  => PartialOrdering::PartialLess,
                    (true, true)   => PartialOrdering::PartialEqual
                }
            }
        }
    )*)
);

impl_partial_order_float!(f32, f64);
