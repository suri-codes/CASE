use std::cmp::Ordering;

use autosurgeon::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// The various priority levels of a `Task`.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Reconcile, Hydrate, Default)]
pub enum Priority {
    /// Highest Priority, needs to get done As Soon As Possible
    Asap,
    /// Used for High Priority items, but not immediate.
    High,
    /// Medium Priority items, note that this is also the default.
    #[default]
    Medium,
    /// Low Priority items.
    Low,
    /// For things that would be nice to get done sometime in the future.
    ///
    /// My rationale for this is things that are more like goals rather
    /// than tasks.
    Far,
}

impl Priority {
    /// Returns the `p_value` of the current priority,
    /// where the `p_value` is an integer used to calculate
    /// the ordering of tasks.
    #[must_use]
    pub const fn p_value(&self) -> u8 {
        // Fibonacci numbers
        match self {
            Self::Far => 2,
            Self::Low => 3,
            Self::Medium => 5,
            Self::High => 8,
            Self::Asap => 13,
        }
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.p_value().cmp(&other.p_value())
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
