//! A purpose-built library for CASE, to hold the internal tree-like
//! data structure that holds tasks and groups. Additionally, this
//! data structure is compatible with `AutoMerge`, via `AutoSurgeon`
//!
//! TODO: add example usage

use autosurgeon::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

mod node;

pub use node::Node;

/// A Node Id
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Reconcile,
    Hydrate,
)]
pub struct NodeId {
    index: u32,
}
