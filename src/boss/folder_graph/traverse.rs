use super::{graph::Graph, graph_id::GraphId, node::GraphNode};

#[derive(Clone)]
/// An iterator of references to the ancestors a given node.
pub struct Ancestors<'a, T> {
    scene_graph: &'a Graph<T>,
    node: Option<GraphId<T>>,
}

impl<'a, T> Ancestors<'a, T> {
    pub(crate) fn new(scene_graph: &'a Graph<T>, current: GraphId<T>) -> Self {
        Self {
            scene_graph,
            node: Some(current),
        }
    }
}

impl<'a, T> Iterator for Ancestors<'a, T> {
    type Item = GraphId<T>;

    fn next(&mut self) -> Option<GraphId<T>> {
        let node = self.node.take()?;
        self.node = self.scene_graph[node].parent;
        Some(node)
    }
}

#[derive(Clone)]
/// An iterator of references to the children of a given node.
pub struct Children<'a, T> {
    scene_graph: &'a Graph<T>,
    node: Option<GraphId<T>>,
}

impl<'a, T> Children<'a, T> {
    pub(super) fn new(scene_graph: &'a Graph<T>, current: GraphId<T>) -> Self {
        Self {
            scene_graph,
            node: scene_graph[current].first_child,
        }
    }
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = GraphId<T>;

    fn next(&mut self) -> Option<GraphId<T>> {
        let node = self.node.take()?;
        self.node = self.scene_graph[node].next_sibling;
        Some(node)
    }
}

#[derive(Clone)]
/// An iterator of references to the children of a given node.
pub struct NodeChildren<'a, T> {
    scene_graph: &'a Graph<T>,
    node_id: Option<GraphId<T>>,
}

impl<'a, T> NodeChildren<'a, T> {
    pub(super) fn new(scene_graph: &'a Graph<T>, current: &GraphNode<T>) -> Self {
        Self {
            scene_graph,
            node_id: current.first_child,
        }
    }
}

impl<'a, T> Iterator for NodeChildren<'a, T> {
    type Item = &'a GraphNode<T>;

    fn next(&mut self) -> Option<&'a GraphNode<T>> {
        let node: &GraphNode<T> = &self.scene_graph[self.node_id.take()?];

        self.node_id = node.next_sibling;
        Some(node)
    }
}

#[derive(Clone)]
/// An iterator of references to a given node and its descendants, in tree order.
pub struct Descendants<'a, T>(Traverse<'a, T>);

impl<'a, T> Descendants<'a, T> {
    pub(super) fn new(arena: &'a Graph<T>, current: GraphId<T>) -> Self {
        Self(Traverse::new(arena, current))
    }
}

impl<'a, T> Iterator for Descendants<'a, T> {
    type Item = GraphId<T>;

    fn next(&mut self) -> Option<GraphId<T>> {
        self.0.find_map(|edge| match edge {
            NodeEdge::Start(node) => Some(node),
            NodeEdge::End(_) => None,
        })
    }
}

#[derive(Debug, Hash)]
/// Indicator if the node is at a start or endpoint of the tree
pub enum NodeEdge<T> {
    /// Indicates that start of a node that has children.
    ///
    /// Yielded by `Traverse::next()` before the node’s descendants. In HTML or
    /// XML, this corresponds to an opening tag like `<div>`.
    Start(GraphId<T>),

    /// Indicates that end of a node that has children.
    ///
    /// Yielded by `Traverse::next()` after the node’s descendants. In HTML or
    /// XML, this corresponds to a closing tag like `</div>`
    End(GraphId<T>),
}

impl<T> std::clone::Clone for NodeEdge<T> {
    fn clone(&self) -> Self {
        match self {
            NodeEdge::Start(i) => NodeEdge::Start(i.clone()),
            NodeEdge::End(i) => NodeEdge::End(i.clone()),
        }
    }
}

impl<T> std::marker::Copy for NodeEdge<T> {}

#[derive(Clone)]
/// An iterator of references to a given node and its descendants, in tree
/// order.
pub struct Traverse<'a, T> {
    arena: &'a Graph<T>,
    root: GraphId<T>,
    next: Option<NodeEdge<T>>,
}

impl<'a, T> Traverse<'a, T> {
    pub(crate) fn new(arena: &'a Graph<T>, current: GraphId<T>) -> Self {
        Self {
            arena,
            root: current,
            next: Some(NodeEdge::Start(current)),
        }
    }

    /// Calculates the next node.
    fn next_of_next(&self, next: NodeEdge<T>) -> Option<NodeEdge<T>> {
        match next {
            NodeEdge::Start(node) => match self.arena[node].first_child {
                Some(first_child) => Some(NodeEdge::Start(first_child)),
                None => Some(NodeEdge::End(node)),
            },
            NodeEdge::End(node) => {
                if node == self.root {
                    return None;
                }
                let node = &self.arena[node];
                match node.next_sibling {
                    Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                    // `node.parent()` here can only be `None` if the tree has
                    // been modified during iteration, but silently stoping
                    // iteration seems a more sensible behavior than panicking.
                    None => node.parent.map(NodeEdge::End),
                }
            }
        }
    }
}

impl<'a, T> Iterator for Traverse<'a, T> {
    type Item = NodeEdge<T>;

    fn next(&mut self) -> Option<NodeEdge<T>> {
        let next = self.next.take()?;
        self.next = self.next_of_next(next);
        Some(next)
    }
}