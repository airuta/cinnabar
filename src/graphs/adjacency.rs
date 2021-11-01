//! Module for the adjacency list based graph.

use crate::construct::Construct;
use crate::index::Index;
use crate::marker::*;

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

impl<I: Index> Construct<I> for AdjacencyGraph<I, Unidirection> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_vertices() {
        let mut graph = AdjacencyGraph::<usize, Unidirection>::new();
        graph.add(1);
        graph.add(2);
        assert!(graph.storage.contains_key(&1));
        assert!(graph.storage.contains_key(&2));
    }

    #[test]
    fn can_remove_vertices() {
        let mut graph = AdjacencyGraph::<usize, Unidirection>::new();
        graph.add(1);
        graph.add(2);
        graph.remove(2);
        assert!(graph.storage.contains_key(&1));
        assert!(!graph.storage.contains_key(&2));
    }

    #[test]
    fn can_create_unidirectional_edges() {
        let mut graph = AdjacencyGraph::<usize, Unidirection>::new();
        graph.add(1);
        graph.add(2);
        graph.link(1, 2);
        assert!(graph.storage[&1].contains(&2));
        assert!(!graph.storage[&2].contains(&1));
    }

    #[test]
    fn can_remove_unidirectional_edges() {
        let mut graph = AdjacencyGraph::<usize, Unidirection>::new();
        graph.add(1);
        graph.add(2);
        graph.link(1, 2);
        graph.link(2, 1);
        graph.unlink(1, 2);
        assert!(!graph.storage[&1].contains(&2));
        assert!(graph.storage[&2].contains(&1));
    }
}
