use autosurgeon::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

use crate::data::DueDateTime;

#[derive(Serialize, Deserialize, Hydrate, Reconcile)]
pub struct Task {
    name: String,
    due: DueDateTime,
    description: String,
}


