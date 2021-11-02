use pretty_assertions::assert_eq;

use cinnabar::graphs::AdjacencyGraph;
use cinnabar::prelude::*;
use cinnabar::traversal::*;

fn create_uni_graph() -> AdjacencyGraph<usize, Unidirectional> {
    let mut graph = AdjacencyGraph::new();
    graph.add(1);
    graph.add(2);
    graph.add(3);
    graph.add(4);
    graph.link(1, 2);
    graph.link(1, 3);
    graph.link(3, 4);
    graph
}

#[test]
fn adjacency_should_have_correct_order() {
    let graph = create_uni_graph();
    assert_eq!(graph.order(), 4, "graph order is invalid");
}

#[test]
fn adjacency_should_have_correct_size() {
    let graph = create_uni_graph();
    assert_eq!(graph.size(), 3, "graph size is invalid");
}

#[test]
fn adjacency_vertices_can_be_dfs_traversed() {
    let graph = create_uni_graph();
    let ids = dfs(&graph.vertices(), 1).collect::<Vec<_>>();
    assert!(ids == vec![1, 2, 3, 4] || ids == vec![1, 3, 4, 2]);
}

#[test]
fn adjacency_vertices_can_be_bfs_traversed() {
    let graph = create_uni_graph();
    let ids = dfs(&graph.vertices(), 1).collect::<Vec<_>>();
    assert_eq!(ids, vec![1, 2, 3, 4]);
}

#[test]
fn adjacency_edges_can_be_dfs_traversed() {
    let graph = create_uni_graph();
    let dfs_edges = dfs(&graph.edges(), (1, 3)).collect::<Vec<_>>();
    assert_eq!(dfs_edges, vec![(1, 3), (3, 4)]);
}

#[test]
fn adjacency_edges_can_be_bfs_traversed() {
    let graph = create_uni_graph();
    let dfs_edges = bfs(&graph.edges(), (1, 3)).collect::<Vec<_>>();
    assert_eq!(dfs_edges, vec![(1, 3), (3, 4)]);
}
