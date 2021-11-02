//! Module for the adjacency list based graph.

use crate::construct::Construct;
use crate::index::Index;
use crate::marker::*;
use crate::providers::*;
use crate::topology::Topology;
use crate::utils::UnorderedBuildHasher;
use crate::utils::UnorderedPair;

use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

#[derive(Default)]
pub struct AdjacencyGraph<I, D> {
    phantom: PhantomData<D>,
    storage: HashMap<I, HashSet<I>>,
}

impl<I, D> AdjacencyGraph<I, D> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            storage: HashMap::new(),
        }
    }
}

impl<I: Index, D> AdjacencyGraph<I, D> {
    fn has_edge(&self, start: I, end: I) -> bool {
        self.storage.get(&start).map(|edges| edges.contains(&end)) == Some(true)
    }
}

impl<I: Index> Construct<I> for AdjacencyGraph<I, Directed> {
    fn add(&mut self, id: I) -> bool {
        if self.storage.contains_key(&id) {
            return false;
        }
        self.storage.insert(id, HashSet::new());
        true
    }

    fn remove(&mut self, id: I) -> bool {
        self.storage.remove(&id).is_some()
    }

    fn link(&mut self, a: I, b: I) -> bool {
        self.storage.contains_key(&b)
            && self.storage.get_mut(&a).map(|links| links.insert(b)) == Some(true)
    }

    fn unlink(&mut self, a: I, b: I) -> bool {
        self.storage.contains_key(&b)
            && self.storage.get_mut(&a).map(|links| links.remove(&b)) == Some(true)
    }
}

// Vertex and edge providers

pub fn uedge<I>(a: I, b: I) -> UnorderedPair<I> {
    UnorderedPair(a, b)
}

impl<I: Index, D> VertexProvider<I> for AdjacencyGraph<I, D> {
    type Vertices<'a> = impl Topology<Item = I>;

    fn order(&self) -> usize {
        self.storage.len()
    }

    fn vertices(&self) -> Self::Vertices<'_> {
        Vertices { graph: self }
    }
}

impl<I: Index> EdgeProvider<I> for AdjacencyGraph<I, Directed> {
    type Edge = (I, I);
    type Edges<'a> = impl Topology<Item = Self::Edge>;

    fn size(&self) -> usize {
        self.storage.values().map(|edges| edges.len()).sum()
    }

    fn edges(&self) -> Self::Edges<'_> {
        Edges { graph: self }
    }
}

// Vertex topology

struct Vertices<'a, I, D> {
    graph: &'a AdjacencyGraph<I, D>,
}

impl<'a, I: Index, D> Topology for Vertices<'a, I, D> {
    type Item = I;
    type BuildHasher = RandomState;
    type ItemIter<'b> = impl Iterator<Item = Self::Item>;
    type AdjacentIter<'b> = impl Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter<'_> {
        self.graph.storage.keys().copied()
    }

    fn adjacent_to(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>> {
        self.graph
            .storage
            .get(&item)
            .map(|edges| edges.iter().copied())
    }

    fn contains(&self, item: Self::Item) -> bool {
        self.graph.storage.contains_key(&item)
    }
}

// Edge topology

struct Edges<'a, I, D> {
    graph: &'a AdjacencyGraph<I, D>,
}

impl<'a, I: Index> Topology for Edges<'a, I, Directed> {
    type Item = (I, I);
    type BuildHasher = RandomState;
    type ItemIter<'b> = impl Iterator<Item = Self::Item>;
    type AdjacentIter<'b> = impl Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter<'_> {
        self.graph
            .storage
            .iter()
            .map(|(start, edges)| edges.iter().map(|end| (*start, *end)))
            .flatten()
    }

    fn adjacent_to(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>> {
        let (start, end) = item;
        outbound_edges(self.graph, end, start)
    }

    fn contains(&self, item: Self::Item) -> bool {
        let (start, end) = item;
        self.graph.has_edge(start, end)
    }
}

impl<'a, I: Index> Topology for Edges<'a, I, Undirected> {
    type Item = UnorderedPair<I>;
    type BuildHasher = UnorderedBuildHasher;
    type ItemIter<'b> = impl Iterator<Item = Self::Item>;
    type AdjacentIter<'b> = impl Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter<'_> {
        self.graph
            .storage
            .iter()
            .map(|(start, edges)| edges.iter().map(|end| uedge(*start, *end)))
            .flatten()
    }

    fn adjacent_to(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>> {
        let UnorderedPair(a, b) = item;
        let a_edges = outbound_edges(self.graph, a, b)?;
        let b_edges = outbound_edges(self.graph, b, a)?;
        Some(a_edges.chain(b_edges).map(|(a, b)| uedge(a, b)))
    }

    fn contains(&self, item: Self::Item) -> bool {
        let UnorderedPair(a, b) = item;
        self.graph.has_edge(a, b) || self.graph.has_edge(b, a)
    }
}

fn outbound_edges<I: Index, D>(
    graph: &AdjacencyGraph<I, D>,
    source: I,
    exclude: I,
) -> Option<impl Iterator<Item = (I, I)> + '_> {
    graph.storage.get(&source).map(move |edges| {
        edges
            .iter()
            .filter(move |target| **target != exclude)
            .map(move |target| (source, *target))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    type DGraph = AdjacencyGraph<usize, Directed>;

    #[test]
    fn can_add_vertices() {
        let mut graph = DGraph::new();
        graph.add(1);
        graph.add(2);
        assert!(graph.storage.contains_key(&1));
        assert!(graph.storage.contains_key(&2));
    }

    #[test]
    fn can_remove_vertices() {
        let mut graph = DGraph::new();
        graph.add(1);
        graph.add(2);
        graph.remove(2);
        assert!(graph.storage.contains_key(&1));
        assert!(!graph.storage.contains_key(&2));
    }

    #[test]
    fn can_create_unidirectional_edges() {
        let mut graph = DGraph::new();
        graph.add(1);
        graph.add(2);
        graph.link(1, 2);
        assert!(graph.storage[&1].contains(&2));
        assert!(!graph.storage[&2].contains(&1));
    }

    #[test]
    fn can_remove_unidirectional_edges() {
        let mut graph = DGraph::new();
        graph.add(1);
        graph.add(2);
        graph.link(1, 2);
        graph.link(2, 1);
        graph.unlink(1, 2);
        assert!(!graph.storage[&1].contains(&2));
        assert!(graph.storage[&2].contains(&1));
    }
}
