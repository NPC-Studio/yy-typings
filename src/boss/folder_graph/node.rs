use super::{
    graph::Graph,
    graph_id::GraphId,
    traverse::{NodeChildren, NodeChildrenId},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GraphNode<T> {
    pub(super) parent: Option<GraphId<T>>,
    pub(super) first_child: Option<GraphId<T>>,
    pub(super) last_child: Option<GraphId<T>>,
    pub(super) previous_sibling: Option<GraphId<T>>,
    pub(super) next_sibling: Option<GraphId<T>>,
    pub(super) removed: bool,
    pub(super) data: T,
}

impl<T> GraphNode<T> {
    /// Creates a new `Node` with the default state and the given data.
    pub(super) fn new(data: T) -> Self {
        Self {
            parent: None,
            previous_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
            removed: false,
            data,
        }
    }

    /// Returns a reference to the node data.
    pub fn inner(&self) -> &T {
        &self.data
    }

    /// Returns a mutable reference to the entity ID.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Returns the ID of the parent node, unless this node is the root of the
    /// tree.
    pub fn parent(&self) -> Option<GraphId<T>> {
        self.parent
    }

    /// Returns the ID of the first child of this node, unless it has no child.
    pub fn first_child(&self) -> Option<GraphId<T>> {
        self.first_child
    }

    /// Returns the ID of the last child of this node, unless it has no child.
    pub fn last_child(&self) -> Option<GraphId<T>> {
        self.last_child
    }

    /// Checks if the node is marked as removed.
    pub fn is_removed(&self) -> bool {
        self.removed
    }

    /// Returns an iterator of references to this node’s children.
    pub fn children<'a>(&self, scene_graph: &'a Graph<T>) -> NodeChildren<'a, T> {
        NodeChildren::new(scene_graph, self)
    }

    /// Returns an iterator of references to this node’s children.
    pub fn children_id<'a>(&self, scene_graph: &'a Graph<T>) -> NodeChildrenId<'a, T> {
        NodeChildrenId::new(scene_graph, self)
    }

    /// Checks if the node is detached. `is_detached()` != `parent().is_some()`
    pub fn is_detached(&self) -> bool {
        self.parent.is_none()
    }
}

impl<T: std::fmt::Display> fmt::Display for GraphNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
