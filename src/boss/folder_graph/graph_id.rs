use super::{
    graph::Graph,
    relations::{assert_triangle_nodes, insert_with_neighbors},
    siblings_range::SiblingsRange,
    traverse::{Ancestors, Children, Descendants},
    NodeError,
};
use serde::{Serialize, Deserialize};
use std::{fmt, marker::PhantomData};

#[derive(Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GraphId<T> {
    index: usize,
    #[serde(skip)]
    data: PhantomData<T>,
}

impl<T> fmt::Display for GraphId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

impl<T> std::cmp::PartialEq for GraphId<T> {
    fn eq(&self, other: &GraphId<T>) -> bool {
        self.index == other.index
    }
}

impl<T> std::clone::Clone for GraphId<T> {
    fn clone(&self) -> Self {
        GraphId {
            index: self.index,
            data: PhantomData,
        }
    }
}

impl<T> std::marker::Copy for GraphId<T> {}

impl<T> fmt::Debug for GraphId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GraphId {{ index: {}}}", self.index)
    }
}

impl<T> GraphId<T> {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            data: std::marker::PhantomData,
        }
    }

    pub fn index(self) -> usize {
        self.index
    }

    /// Returns an iterator of references to this node and its ancestors.
    ///
    /// Use [`.skip(1)`][`skip`] or call `.next()` once on the iterator to skip
    /// the node itself.
    pub fn ancestors(self, scene_graph: &Graph<T>) -> Ancestors<'_, T> {
        Ancestors::new(scene_graph, self)
    }

    /// Returns an iterator of references to this node’s children.
    pub fn children(self, scene_graph: &Graph<T>) -> Children<'_, T> {
        Children::new(scene_graph, self)
    }

    pub fn descendants(self, scene_graph: &Graph<T>) -> Descendants<'_, T> {
        Descendants::new(scene_graph, self)
    }

    /// Detaches a node from its parent. Children are not affected.
    pub fn detach(self, scene_graph: &mut Graph<T>) {
        let range = SiblingsRange::new(self, self).detach_from_siblings(scene_graph);
        range
            .rewrite_parents(scene_graph, None)
            .expect("Should never happen: `None` as parent is always valid");

        // Ensure the node is surely detached.
        debug_assert!(
            scene_graph[self].is_detached(),
            "The node should be successfully detached"
        );
    }

    /// Appends a new child to this node, after existing children.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * the given new child is `self`, or
    /// * the current node or the given new child was already [`remove`]d.
    ///
    /// To check if the node is removed or not, use [`Node::is_removed()`].
    pub fn append(self, new_child: Self, scene_graph: &mut Graph<T>) {
        self.checked_append(new_child, scene_graph)
            .expect("Preconditions not met: invalid argument");
    }

    /// Appends a new child to this node, after existing children.
    ///
    /// # Failures
    ///
    /// * Returns [`NodeError::AppendSelf`] error if the given new child is
    ///   `self`.
    /// * Returns [`NodeError::Removed`] error if the given new child or `self`
    ///   is [`remove`]d.
    ///
    /// To check if the node is removed or not, use [`Node::is_removed()`].
    pub fn checked_append(self, new_child: GraphId<T>, scene_graph: &mut Graph<T>) -> Result<(), NodeError> {
        if new_child == self {
            return Err(NodeError::AppendSelf);
        }
        if scene_graph[self].is_removed() || scene_graph[new_child].is_removed() {
            return Err(NodeError::Removed);
        }
        new_child.detach(scene_graph);

        insert_with_neighbors(
            scene_graph,
            new_child,
            Some(self),
            scene_graph[self].last_child,
            None,
        )
        .expect("Should never fail: `new_child` is not `self` and they are not removed");

        Ok(())
    }

    /// Prepends a new child to this node, before existing children.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * the given new child is `self`, or
    /// * the current node or the given new child was already [`remove`]d.
    ///
    /// To check if the node is removed or not, use [`Node::is_removed()`].
    pub fn prepend(self, new_child: Self, scene_graph: &mut Graph<T>) {
        self.checked_prepend(new_child, scene_graph)
            .expect("Preconditions not met: invalid argument");
    }

    /// Prepends a new child to this node, before existing children.
    ///
    /// # Failures
    ///
    /// * Returns [`NodeError::PrependSelf`] error if the given new child is
    ///   `self`.
    /// * Returns [`NodeError::Removed`] error if the given new child or `self`
    ///   is [`remove`]d.
    ///
    /// To check if the node is removed or not, use [`Node::is_removed()`].
    pub fn checked_prepend(self, new_child: Self, scene_graph: &mut Graph<T>) -> Result<(), NodeError> {
        if new_child == self {
            return Err(NodeError::PrependSelf);
        }
        if scene_graph[self].is_removed() || scene_graph[new_child].is_removed() {
            return Err(NodeError::Removed);
        }
        insert_with_neighbors(
            scene_graph,
            new_child,
            Some(self),
            None,
            scene_graph[self].first_child,
        )
        .expect("Should never fail: `new_child` is not `self` and they are not removed");

        Ok(())
    }

    /// Removes a node from the arena.
    ///
    /// Children of the removed node will be inserted to the place where the
    /// removed node was.
    ///
    /// Please note that the node will not be removed from the internal arena
    /// storage, but marked as `removed`. Traversing the arena returns a
    /// plain iterator and contains removed elements too.
    ///
    /// To check if the node is removed or not, use [`Node::is_removed()`].
    pub fn remove(self, arena: &mut Graph<T>) {
        debug_assert_triangle_nodes!(
            arena,
            arena[self].parent,
            arena[self].previous_sibling,
            Some(self)
        );
        debug_assert_triangle_nodes!(arena, arena[self].parent, Some(self), arena[self].next_sibling);
        debug_assert_triangle_nodes!(arena, Some(self), None, arena[self].first_child);
        debug_assert_triangle_nodes!(arena, Some(self), arena[self].last_child, None);

        // Retrieve needed values.
        let (parent, previous_sibling, next_sibling, first_child, last_child) = {
            let node = &arena[self];
            (
                node.parent,
                node.previous_sibling,
                node.next_sibling,
                node.first_child,
                node.last_child,
            )
        };

        assert_eq!(first_child.is_some(), last_child.is_some());
        self.detach(arena);
        if let (Some(first_child), Some(last_child)) = (first_child, last_child) {
            let range = SiblingsRange::new(first_child, last_child).detach_from_siblings(arena);
            range
                .transplant(arena, parent, previous_sibling, next_sibling)
                .expect("Should never fail: neighbors and children must be consistent");
        }
        arena[self].removed = true;
        debug_assert!(arena[self].is_detached());
    }
}
