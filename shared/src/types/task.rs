use std::cmp::Ordering;

use autosurgeon::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use crate::types::{DueDateTime, Priority};

/// Represents a `Task`
#[derive(Debug, Serialize, Deserialize, Hydrate, Reconcile, PartialEq, Eq)]
pub struct Task {
    name: String,
    due: DueDateTime,
    priority: Priority,
    description: String,
    finished: bool,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.p_value().cmp(&other.priority.p_value())
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
