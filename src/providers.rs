//! This modules defines tratis the provide topologies for a graph. In general, a graph
//! implementation provides two topologie, one for edges and one for vertices. They are
//! typically represented by separate structs implementing the [`Topology`] trait.

use crate::topology::Topology;

/// This trait defines methods to access overall number of vertices (graph order) and
/// a vertex topology.
pub trait VertexProvider<I> {
    /// A particular implementation of a topology trait for vertices.
    type Vertices<'a>: Topology<Item = I>
    where
        Self: 'a;

    /// `order` retruns the number of vertices in a graph.
    fn order(&self) -> usize;

    /// `vertices` returns the instance of vertex topology.
    fn vertices(&self) -> Self::Vertices<'_>;
}

/// This trait defines methods to access overall number of edges (graph size) and
/// a vertex topology.
pub trait EdgeProvider<I> {
    /// Edge type.
    type Edge;

    /// A particular implementation of a topology trait for edges.
    type Edges<'a>: Topology<Item = Self::Edge>
    where
        Self: 'a;

    /// `size` retruns the number of edges in a graph.
    fn size(&self) -> usize;

    /// `vertices` returns the instance of edge topology.
    fn edges(&self) -> Self::Edges<'_>;
}
