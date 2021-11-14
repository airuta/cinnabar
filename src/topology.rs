//! This module is a home to the most important trait in this library - the [`Topology`] trait. It
//! is used and inherited by many other traits in the library because it provides the minimal implementation
//! necessary for the graph traversal.
//!
//! Each graph typically provides two implementation of this trait - one for vertices and one for edges.
//! On one hand, it allows clients to traverse both vertices and edges with the same algorithms. On the other,
//! it makes it easier for graph implementors to provided extended functionality for both vertices and edges.
//! Many other libraries, in contrast, focus only on one aspect of a graph, to the neglect of the other.

/// Topology trait defines the connectivity pattern within a graph.
pub trait Topology {
    /// The type of items in a graph - typically vertices or edges.
    type Item;

    /// The type of the build hasher used to construcr hashers for items. Some items,
    /// like bidirectional edges, require special treatment.
    type BuildHasher: std::hash::BuildHasher + Default;

    /// The type of iterator used to traverse all graph items.
    type ItemIter<'a>: Iterator<Item = Self::Item>
    where
        Self: 'a;

    /// The type of iterator used to traverse adjacent items.
    type AdjacentIter<'a>: Iterator<Item = Self::Item>
    where
        Self: 'a;

    /// Iterate through all the items in a graph.
    fn iter(&self) -> Self::ItemIter<'_>;

    /// Iterate through all the items adjacent to the given `item`.
    fn adjacent_to(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>>;

    /// Checks if the given `item` exists in the graph.
    fn contains(&self, item: Self::Item) -> bool;
}
