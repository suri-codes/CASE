use autosurgeon::{Hydrate, Reconcile};
use sakura::{Node, NodeId, Tree};
use serde::{Deserialize, Serialize};

use crate::types::{Group, Task};

#[derive(Debug, Serialize, Deserialize, Hydrate, Reconcile)]
pub struct CaseTree {
    tree: Tree<CaseNode>,
}

#[derive(Debug, Serialize, Deserialize, Hydrate, Reconcile)]
enum CaseNode {
    Task(Task),
    Group(Group),
}

impl CaseTree {
    pub fn insert(&mut self, node: CaseNode, parent: &NodeId) {
        let node = Node::new(node);

        self.tree
            .insert(node, sakura::InsertBehavior::UnderNode(parent))
            .unwrap();
    }
}
