//! This module is a home for graph traversals. It defines a traversal trait which provies
//! most common traversals, and a helper function for writing custom traversals for your own
//! graphs.

use std::collections::HashSet;

use crate::index::Index;
use crate::topology::Topology;

pub fn dfs<T: Topology>(topology: &T, start: T::Item) -> impl Iterator<Item = T::Item> + '_
where
    T::Item: Index,
{
    let build_hasher = T::BuildHasher::default();
    let mut discovered = HashSet::with_hasher(build_hasher);
    let mut stack = vec![start];

    discovered.insert(start);
    std::iter::from_fn(move || match stack.pop() {
        None => None,
        Some(item) => topology.adjacent_to(item).map(|iter| {
            for adjacent in iter {
                if !discovered.contains(&adjacent) {
                    discovered.insert(adjacent);
                    stack.push(adjacent);
                }
            }
            item
        }),
    })
}
