use cinnabar::utils::UnorderedPair;
use pretty_assertions::assert_eq;

use cinnabar::graphs::AdjacencyList;
use cinnabar::prelude::*;
use cinnabar::traversal::*;

fn create_directed_graph() -> AdjacencyList<usize, Directed> {
    let mut graph = AdjacencyList::new();
    graph.add(1);
    graph.add(2);
    graph.add(3);
    graph.add(4);
    graph.link(1, 2);
    graph.link(1, 3);
    graph.link(3, 4);
    graph
}

fn create_undirected_graph() -> AdjacencyList<usize, Undirected> {
    let mut graph = AdjacencyList::new();
    graph.add(1);
    graph.add(2);
    graph.add(3);
    graph.add(4);
    graph.add(5);
    graph.link(1, 2);
    graph.link(1, 3);
    graph.link(3, 4);
    graph.link(4, 5);
    graph
}

fn uedges(edges: &[(usize, usize)]) -> Vec<UnorderedPair<usize>> {
    edges.iter().copied().map(Into::into).collect()
}

#[test]
fn adjacency_should_have_correct_order() {
    let graph = create_directed_graph();
    assert_eq!(graph.order(), 4, "graph order is invalid");
}

#[test]
fn adjacency_should_have_correct_size() {
    let graph = create_directed_graph();
    assert_eq!(graph.size(), 3, "graph size is invalid");
}

#[test]
fn adjacency_vertices_can_be_dfs_traversed() {
    let graph = create_directed_graph();
    let ids = dfs(&graph.vertices(), 1).collect::<Vec<_>>();
    assert!(ids == vec![1, 2, 3, 4] || ids == vec![1, 3, 4, 2]);
}

#[test]
fn adjacency_vertices_can_be_bfs_traversed() {
    let graph = create_directed_graph();
    let ids = bfs(&graph.vertices(), 1).collect::<Vec<_>>();
    assert!(ids == vec![1, 2, 3, 4] || ids == vec![1, 3, 2, 4]);
}

#[test]
fn adjacency_directed_edges_can_be_dfs_traversed() {
    let graph = create_directed_graph();
    let dfs_edges = dfs(&graph.edges(), (1, 3)).collect::<Vec<_>>();
    assert_eq!(dfs_edges, vec![(1, 3), (3, 4)]);
}

#[test]
fn adjacency_directed_edges_can_be_bfs_traversed() {
    let graph = create_directed_graph();
    let bfs_edges = bfs(&graph.edges(), (1, 3)).collect::<Vec<_>>();
    assert_eq!(bfs_edges, vec![(1, 3), (3, 4)]);
}

#[test]
fn adjacency_undirected_edges_can_be_dfs_traversed() {
    let graph = create_undirected_graph();
    let dfs_edges = dfs(&graph.edges(), (1, 3).into()).collect::<Vec<_>>();
    let traversal_1 = uedges(&[(1, 3), (1, 2), (3, 4), (4, 5)]);
    let traversal_2 = uedges(&[(1, 3), (3, 4), (4, 5), (1, 2)]);
    assert!(dfs_edges == traversal_1 || dfs_edges == traversal_2);
}

#[test]
fn adjacency_undirected_edges_can_be_bfs_traversed() {
    let graph = create_undirected_graph();
    let bfs_edges = bfs(&graph.edges(), (1, 3).into()).collect::<Vec<_>>();
    let traversal_1 = uedges(&[(1, 3), (1, 2), (3, 4), (4, 5)]);
    let traversal_2 = uedges(&[(1, 3), (3, 4), (1, 2), (4, 5)]);
    assert!(bfs_edges == traversal_1 || bfs_edges == traversal_2);
}
