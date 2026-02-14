use autosurgeon::{Hydrate, Reconcile};
use sakura::{Node, NodeId, Tree};
use serde::{Deserialize, Serialize};

use crate::types::{Group, Task};

/// The core data structure for the CASE application.
/// Stores groups and tasks in nodes.
#[derive(Debug, Serialize, Deserialize, Hydrate, Reconcile)]
pub struct CaseTree {
    tree: Tree<CaseNode>,
}

#[derive(Debug, Serialize, Deserialize, Hydrate, Reconcile)]
pub enum CaseNode {
    Task(Task),
    Group(Group),
}

impl CaseTree {
    /// # Errors
    /// could error if the parent node is invalid!
    pub fn insert(&mut self, node: CaseNode, parent: &NodeId) -> crate::Result<NodeId> {
        let node = Node::new(node);

        Ok(self
            .tree
            .insert(node, sakura::InsertBehavior::UnderNode(parent))?)
    }
}
