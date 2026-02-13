use autosurgeon::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use crate::types::{DueDateTime, Priority};

#[derive(Serialize, Deserialize, Hydrate, Reconcile)]
pub struct Task {
    name: String,
    due: DueDateTime,
    priority: Priority,
    description: String,
    finished: bool,
}
