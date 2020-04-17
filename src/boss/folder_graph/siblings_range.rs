use super::{graph::Graph, graph_id::GraphId, relations::*, ConsistencyError};

#[derive(Debug, Clone, Copy)]
pub(super) struct SiblingsRange<T> {
    first: GraphId<T>,
    last: GraphId<T>,
}

impl<T> SiblingsRange<T> {
    /// Creates a new range.
    ///
    /// It is user's responsibility to guarantee that `first` to `last` is a
    /// correct range.
    pub(super) fn new(first: GraphId<T>, last: GraphId<T>) -> Self {
        Self { first, last }
    }

    /// Detaches the range from the siblings out of the range, preserving
    /// sibling relations inside the range.
    pub(super) fn detach_from_siblings(self, arena: &mut Graph<T>) -> DetachedSiblingsRange<T> {
        // Update children's parents, siblings relations outside the range, and
        // old parent's first and last child nodes.
        let parent = arena[self.first].parent;

        // Update siblings relations outside the range and old parent's
        // children if necessary.
        let prev_of_range = arena[self.first].previous_sibling.take();
        let next_of_range = arena[self.last].next_sibling.take();
        connect_neighbors(arena, parent, prev_of_range, next_of_range);

        if cfg!(debug_assertions) {
            debug_assert_eq!(arena[self.first].previous_sibling, None);
            debug_assert_eq!(arena[self.last].next_sibling, None);
            debug_assert_triangle_nodes!(arena, parent, prev_of_range, next_of_range);
            if let Some(parent_node) = parent.map(|id| &arena[id]) {
                debug_assert_eq!(
                    parent_node.first_child.is_some(),
                    parent_node.last_child.is_some()
                );
                debug_assert_triangle_nodes!(arena, parent, None, parent_node.first_child);
                debug_assert_triangle_nodes!(arena, parent, parent_node.last_child, None);
            }
        }

        DetachedSiblingsRange {
            first: self.first,
            last: self.last,
        }
    }
}

/// Detached siblings range.
///
/// Note that the nodes in the range has outdated parent information.
/// It is user's responsibility to properly update them using
/// `rewrite_parents()`.
#[derive(Debug, Clone, Copy)]
#[must_use = "This range can have outdated parent information and they should be updated"]
pub(super) struct DetachedSiblingsRange<T> {
    /// First node.
    first: GraphId<T>,
    /// Last node.
    last: GraphId<T>,
}

impl<T> DetachedSiblingsRange<T> {
    /// Rewrites the parents.
    ///
    /// # Failures
    ///
    /// Returns an error if the given parent is a node in the range.
    pub(super) fn rewrite_parents(
        &self,
        arena: &mut Graph<T>,
        new_parent: Option<GraphId<T>>,
    ) -> Result<(), ConsistencyError> {
        // Update parents of children in the range.
        let mut child_opt = Some(self.first);
        while let Some(child) = child_opt {
            if Some(child) == new_parent {
                // Attempt to set the node itself as its parent.
                return Err(ConsistencyError::ParentChildLoop);
            }
            let child_node = &mut arena[child];
            child_node.parent = new_parent;
            child_opt = child_node.next_sibling;
        }

        Ok(())
    }

    /// Inserts the range to the given place preserving sibling relations in
    /// the range.
    ///
    /// This does `rewrite_parents()` automatically, so callers do not need to
    /// call it manually.
    ///
    /// # Failures
    ///
    /// Returns an error if the given parent is a node in the range.
    pub(super) fn transplant(
        self,
        scene_grpah: &mut Graph<T>,
        parent: Option<GraphId<T>>,
        previous_sibling: Option<GraphId<T>>,
        next_sibling: Option<GraphId<T>>,
    ) -> Result<(), ConsistencyError> {
        // Check that the given arguments are consistent.
        if cfg!(debug_assertions) {
            if let Some(previous_sibling) = previous_sibling {
                debug_assert_eq!(scene_grpah[previous_sibling].parent, parent);
            }
            if let Some(next_sibling) = next_sibling {
                debug_assert_eq!(scene_grpah[next_sibling].parent, parent);
            }
            debug_assert_triangle_nodes!(scene_grpah, parent, previous_sibling, next_sibling);
            if let Some(parent_node) = parent.map(|id| &scene_grpah[id]) {
                debug_assert_eq!(
                    parent_node.first_child.is_some(),
                    parent_node.last_child.is_some()
                );
            }
        }

        // Rewrite parents of the nodes in the range.
        self.rewrite_parents(scene_grpah, parent)?;

        // Connect the previous sibling and the first node in the range.
        connect_neighbors(scene_grpah, parent, previous_sibling, Some(self.first));

        // Connect the next sibling and the last node in the range.
        connect_neighbors(scene_grpah, parent, Some(self.last), next_sibling);

        // Ensure related nodes are consistent.
        // Check only in debug build.
        if cfg!(debug_assertions) {
            debug_assert_triangle_nodes!(scene_grpah, parent, previous_sibling, Some(self.first));
            debug_assert_triangle_nodes!(scene_grpah, parent, Some(self.last), next_sibling);
            if let Some(parent_node) = parent.map(|id| &scene_grpah[id]) {
                debug_assert!(
                    parent_node.first_child.is_some() && parent_node.last_child.is_some(),
                    "parent should have children (at least `self.first`)"
                );
                debug_assert_triangle_nodes!(scene_grpah, parent, None, parent_node.first_child);
                debug_assert_triangle_nodes!(scene_grpah, parent, parent_node.last_child, None);
            }
        }

        Ok(())
    }
}
