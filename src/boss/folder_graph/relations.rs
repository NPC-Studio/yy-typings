use super::{siblings_range::SiblingsRange, ConsistencyError, graph_id::GraphId, graph::Graph};

/// Ensures the given parent, previous, and next nodes are consistent.
///
/// This assert is only enabled in debug build.
#[macro_use]
macro_rules! debug_assert_triangle_nodes {
    ($scene_graph:expr, $parent:expr, $previous:expr, $next:expr $(,)?) => {{
        if cfg!(debug_assertions) {
            assert_triangle_nodes($scene_graph, $parent, $previous, $next);
        }
    }};
}

/// Ensures the given parent, previous, and next nodes are consistent.
///
/// # Panics
///
/// Panics if the given nodes are inconsistent.
pub(super) fn assert_triangle_nodes<T>(
    scene_graph: &Graph<T>,
    parent: Option<GraphId<T>>,
    previous: Option<GraphId<T>>,
    next: Option<GraphId<T>>,
) {
    if let Some(previous_node) = previous.map(|id| &scene_graph[id]) {
        assert_eq!(
            previous_node.parent, parent,
            "`prev->parent` must equal to `parent`"
        );
        assert_eq!(
            previous_node.next_sibling, next,
            "`prev->next` must equal to `next`"
        );
    }
    if let Some(next_node) = next.map(|id| &scene_graph[id]) {
        assert_eq!(next_node.parent, parent, "`next->parent` must equal to `parent`");
        assert_eq!(
            next_node.previous_sibling, previous,
            "`next->prev` must equal to `prev`"
        );
    }
}

/// Connects the given adjacent neighbor nodes and update fields properly.
///
/// This connects the given three nodes (if `Some(_)`) and update fields to make
/// them consistent.
///
/// ```text
///    parent
///     /  \
///    /    \
/// prev -> next
/// ```
pub(super) fn connect_neighbors<T>(
    scene_graph: &mut Graph<T>,
    parent: Option<GraphId<T>>,
    previous: Option<GraphId<T>>,
    next: Option<GraphId<T>>,
) {
    if cfg!(debug_assertions) {
        if let Some(parent_node) = parent.map(|id| &scene_graph[id]) {
            debug_assert_eq!(
                parent_node.first_child.is_some(),
                parent_node.last_child.is_some()
            );
            debug_assert!(!parent_node.is_removed());
        }
        debug_assert!(!previous.map_or(false, |id| scene_graph[id].is_removed()));
        debug_assert!(!next.map_or(false, |id| scene_graph[id].is_removed()));
    }

    let (mut parent_first_child, mut parent_last_child) = parent
        .map(|id| &scene_graph[id])
        .map_or((None, None), |node| (node.first_child, node.last_child));
    if let Some(previous) = previous {
        // `previous` ==> `next`
        scene_graph[previous].next_sibling = next;
        parent_first_child = parent_first_child.or_else(|| Some(previous));
    } else {
        // `next` is the first child of the parent.
        parent_first_child = next;
    }
    if let Some(next) = next {
        // `previous` <== `next`
        scene_graph[next].previous_sibling = previous;
        parent_last_child = parent_last_child.or_else(|| Some(next));
    } else {
        // `previous` is the last child of the parent.
        parent_last_child = previous;
    }

    if let Some(parent_node) = parent.map(|id| &mut scene_graph[id]) {
        debug_assert_eq!(parent_first_child.is_some(), parent_last_child.is_some());
        parent_node.first_child = parent_first_child;
        parent_node.last_child = parent_last_child;
    }

    debug_assert_triangle_nodes!(scene_graph, parent, previous, next);
}

/// Detaches, inserts, and updates the given node using the given neighbors.
///
/// ```text
/// Before:
///
///    parent
///     /  \
///    /    \
/// prev -> next
///
/// After:
///
///        parent
///    ______/|\_____
///   /       |      \
/// prev -> (new) -> next
/// ```
pub(super) fn insert_with_neighbors<T>(
    scene_graph: &mut Graph<T>,
    new: GraphId<T>,
    parent: Option<GraphId<T>>,
    previous_sibling: Option<GraphId<T>>,
    next_sibling: Option<GraphId<T>>,
) -> Result<(), ConsistencyError> {
    debug_assert_triangle_nodes!(scene_graph, parent, previous_sibling, next_sibling);
    if previous_sibling == Some(new) || next_sibling == Some(new) {
        // One of the given neighbors is going to be detached.
        return Err(ConsistencyError::SiblingsLoop);
    }
    if parent == Some(new) {
        // The given parent is the node itself.
        return Err(ConsistencyError::ParentChildLoop);
    }

    SiblingsRange::new(new, new)
        .detach_from_siblings(scene_graph)
        .transplant(scene_graph, parent, previous_sibling, next_sibling)
        .expect("Should never fail: neighbors including parent are not `self`");

    debug_assert_triangle_nodes!(scene_graph, parent, previous_sibling, Some(new));
    debug_assert_triangle_nodes!(scene_graph, parent, Some(new), next_sibling);

    Ok(())
}
