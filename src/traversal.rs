//! This module is a home for graph traversals. It defines two fundamental traversal algorithms,
//! DFS and BFS, that rely on provided topolgy to traverse the items.

use std::collections::{HashSet, VecDeque};

use crate::index::Index;
use crate::topology::Topology;

/// Given a `topology`, start at the `start` item and traverse everything from that point in DFS order.
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

/// Given a `topology`, start at the `start` item and traverse everything from that point in BFS order.
pub fn bfs<T: Topology>(topology: &T, start: T::Item) -> impl Iterator<Item = T::Item> + '_
where
    T::Item: Index,
{
    let build_hasher = T::BuildHasher::default();
    let mut discovered = HashSet::with_hasher(build_hasher);
    let mut queue = VecDeque::from([start]);

    discovered.insert(start);
    std::iter::from_fn(move || match queue.pop_front() {
        None => None,
        Some(item) => topology.adjacent_to(item).map(|iter| {
            for adjacent in iter {
                if !discovered.contains(&adjacent) {
                    discovered.insert(adjacent);
                    queue.push_back(adjacent);
                }
            }
            item
        }),
    })
}
