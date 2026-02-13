use std::cmp::Ordering;

use autosurgeon::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use crate::types::Priority;

#[derive(Debug, Serialize, Deserialize, Hydrate, Reconcile, PartialEq, Eq)]
pub struct Group {
    name: String,
    priority: Priority,
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.p_value().cmp(&other.priority.p_value())
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
