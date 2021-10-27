//! This module is a home for graph traversals. It defines a traversal trait which provies
//! most common traversals, and a helper function for writing custom traversals for your own
//! graphs.

use crate::topology::Topology;

/// Traversal is a trait the definies common traversals like DFS and BFS and provides a
/// default implementation. Each traversal is essentially an iterator of all vertices in
/// a graph in a particular order. For this to work, a graph or a supporting struct must
/// implement a [`Topology`] trait. Please note that a graph can provide other, graph-specific
/// traversal methods not definied by this trait.
pub trait Traversal: Topology {}

/// The `traverse` function provides a way to define a custom traversal through the specified
/// `step` argument. It takes an index of a vertex, and returns a `Some` option with the index
/// of the next vertex, or `None` if traversal should stop. The `start` parameters is the index
/// of the starting vertex.
pub fn traverse<I, F>(start: I, mut step: F) -> impl Iterator<Item = I>
where
    I: Copy,
    F: FnMut(I) -> Option<I>,
{
    let mut current = Some(start);
    std::iter::from_fn(move || current.and_then(|id| std::mem::replace(&mut current, step(id))))
}
