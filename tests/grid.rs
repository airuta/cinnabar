use pretty_assertions::assert_eq;
use std::collections::HashSet;

use cinnabar::graphs::grid::{Edge, EdgeSet};
use cinnabar::graphs::Grid;
use cinnabar::prelude::*;
use cinnabar::traversal::*;

const ROWS: usize = 3;
const COLS: usize = 4;

fn create_grid() -> Grid<Counter> {
    Grid::new(ROWS, COLS)
}

#[test]
fn grid_should_have_correct_order() {
    let grid = create_grid();
    assert_eq!(grid.order(), ROWS * COLS, "graph order is invalid");
}

#[test]
fn grid_should_have_correct_size() {
    let grid = create_grid();
    assert_eq!(grid.size(), ROWS * (COLS - 1) + COLS * (ROWS - 1));
}

#[test]
fn grid_traverse_should_visit_all_vertices() {
    let grid = create_grid();
    #[allow(clippy::needless_collect)]
    let ids = grid.traverse_by_rows().collect::<Vec<_>>();
    for i in 0..ROWS {
        for j in 0..COLS {
            let id = grid.at(i, j).unwrap();
            assert!(ids.contains(&id));
        }
    }
    assert_eq!(ids.len(), ROWS * COLS);
}

#[test]
fn grid_construction_can_be_inspected() {
    let mut inspector_ids = HashSet::new();
    let grid = Grid::with_inspector(ROWS, COLS, |id: Counter, _, _| {
        inspector_ids.insert(id);
    });
    let traversal_ids = grid.traverse_by_rows().collect::<HashSet<_>>();
    assert_eq!(inspector_ids, traversal_ids);
}

#[test]
fn grid_vertices_can_be_dfs_traversed() {
    let grid = create_grid();
    let start = grid.at(0, 0).unwrap();
    let ids = dfs(&grid.vertices(), start).collect::<Vec<_>>();
    for i in 0..ROWS {
        for j in 0..COLS {
            let id = grid.at(i, j).unwrap();
            assert!(ids.contains(&id));
        }
    }
    assert_eq!(ids.len(), ROWS * COLS);
}

#[test]
fn grid_vertices_can_be_bfs_traversed() {
    let grid = create_grid();
    let start = grid.at(0, 0).unwrap();
    let ids = bfs(&grid.vertices(), start).collect::<Vec<_>>();
    for i in 0..ROWS {
        for j in 0..COLS {
            let id = grid.at(i, j).unwrap();
            assert!(ids.contains(&id));
        }
    }
    assert_eq!(ids.len(), ROWS * COLS);
}

#[test]
fn grid_edges_can_be_dfs_traversed() {
    let grid = create_grid();
    let a = grid.at(0, 0).unwrap();
    let b = grid.at(0, 1).unwrap();
    let dfs_edges = dfs(&grid.edges(), Edge::new(a, b)).collect::<EdgeSet<_>>();
    let all_edges = grid.edges().iter().collect::<EdgeSet<_>>();
    assert_eq!(dfs_edges, all_edges);
}

#[test]
fn grid_edges_can_be_bfs_traversed() {
    let grid = create_grid();
    let a = grid.at(0, 0).unwrap();
    let b = grid.at(0, 1).unwrap();
    let dfs_edges = bfs(&grid.edges(), Edge::new(a, b)).collect::<EdgeSet<_>>();
    let all_edges = grid.edges().iter().collect::<EdgeSet<_>>();
    assert_eq!(dfs_edges, all_edges);
}
