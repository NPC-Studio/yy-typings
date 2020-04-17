use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
/// Possible node failures.
pub enum NodeError {
    #[error("Cannot append a node to itself")]
    AppendSelf,
    #[error("Cannot prepend a node to itself")]
    PrependSelf,
    #[error("Removed node cannot have any parent, siblings, and children")]
    Removed,
}

/// An error type that represents the given structure or argument is
/// inconsistent or invalid.
// Intended for internal use.
#[derive(Debug, Clone, Copy, Error)]
pub(super) enum ConsistencyError {
    /// Specified a node as its parent.
    ParentChildLoop,
    /// Specified a node as its sibling.
    SiblingsLoop,
}

impl fmt::Display for ConsistencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsistencyError::ParentChildLoop => f.write_str("Specified a node as its parent"),
            ConsistencyError::SiblingsLoop => f.write_str("Specified a node as its sibling"),
        }
    }
}
