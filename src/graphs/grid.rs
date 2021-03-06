//! This module defines a 2D grid graph and its related strcutres.

use crate::index::*;
use crate::providers::*;
use crate::topology::*;
use crate::utils::{Bifunctor, Collapse};
use crate::utils::{UnorderedBuildHasher, UnorderedPair};

use itertools::Itertools;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Coordinates of a vertex in a grid in row-column order.
#[derive(Copy, Clone, Debug)]
pub struct Coords(pub usize, pub usize);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Edge<I>(UnorderedPair<I>);

impl<I> Edge<I> {
    pub fn new(a: I, b: I) -> Self {
        Self(UnorderedPair(a, b))
    }
}

/// Use `EdgeSet` whenever you need to store a hash set of `Grid`'s edges.
pub type EdgeSet<I> = HashSet<Edge<I>, UnorderedBuildHasher>;

/// A 2D grid graph, arranging it's nodes in a rectangular grid. It does not provide any
/// vertex or edge-related storage for weights. Every vertex is connected to all its
/// neighbors by a bidirectional edge, where neighborhood is defined as having the same
/// row or column. In other words, any vertex will have 2, 3, or 4 neighbors at most.
///
/// Edges in this graph are not stored directly and thus don't take up memory. Edge topology,
/// and correspondingly, traversals are available, and can be used to associate edges
/// with weights.
pub struct Grid<I = Counter> {
    rows: usize,
    columns: usize,
    grid: Vec<Vec<I>>,
    coords: HashMap<I, Coords>,
}

/// Construction interface.
impl<I: Unique + Index> Grid<I> {
    /// Create a new grid with the given size.
    pub fn new(rows: usize, columns: usize) -> Self {
        Self::with_inspector(rows, columns, |_, _, _| ())
    }

    /// Create a new grid with the given size, calling `inspector` function for
    /// each created vertex. `inspector` is called with an index of a created vertex
    /// and its coordinates in row-columns order.
    pub fn with_inspector(
        rows: usize,
        columns: usize,
        mut inspector: impl FnMut(I, usize, usize),
    ) -> Self {
        let mut coords = HashMap::new();
        let mut grid = Vec::with_capacity(rows);
        for r in 0..rows {
            let mut row = Vec::with_capacity(columns);
            for c in 0..columns {
                let id = Unique::generate();
                coords.insert(id, Coords(r, c));
                row.push(id);
                inspector(id, r, c);
            }
            grid.push(row)
        }
        Self {
            rows,
            columns,
            grid,
            coords,
        }
    }
}

/// Grid-specific interface provides several coordinate-related methods.
impl<I: Index> Grid<I> {
    /// Return the index of a node at the given coordinates.
    pub fn at(&self, row: usize, column: usize) -> Option<I> {
        self.grid.get(row).and_then(|row| row.get(column)).copied()
    }

    /// Return the coordinates of the node by the given index.
    pub fn coords_of(&self, id: I) -> Option<Coords> {
        self.coords.get(&id).copied()
    }

    /// Return the coordinates of all neighbors to a vertex at the given coordinates.
    /// This method is primarily used internal buy can be used to simplify handling
    /// of edge and corner vertices.
    pub fn neighbors_of(&self, row: usize, col: usize) -> impl Iterator<Item = Coords> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(move |pair| {
                pair.bimap(
                    |dy| row.checked_add_signed(dy),
                    |dx| col.checked_add_signed(dx),
                )
                .collapse(Coords)
            })
            .filter(|coords| coords.0 < self.rows && coords.1 < self.columns)
    }
}

fn adjacent(ax: usize, ay: usize, bx: usize, by: usize) -> bool {
    match 0 {
        _ if ax == bx && ay < by => by - ay == 1,
        _ if ax == bx && ay > by => ay - by == 1,
        _ if ay == by && ax < bx => bx - ax == 1,
        _ if ay == by && ax > bx => ax - bx == 1,
        _ => false,
    }
}

// Vertex and edge providers

impl<I: Index> VertexProvider<I> for Grid<I> {
    type Vertices<'a>
    where
        Self: 'a,
    = impl Topology<Item = I>;

    fn order(&self) -> usize {
        self.coords.len()
    }

    fn vertices(&self) -> Self::Vertices<'_> {
        Vertices { grid: self }
    }
}

impl<I: Index> EdgeProvider<I> for Grid<I> {
    type Edge = Edge<I>;
    type Edges<'a>
    where
        Self: 'a,
    = impl Topology<Item = Self::Edge>;

    fn size(&self) -> usize {
        self.rows * (self.columns - 1) + self.columns * (self.rows - 1)
    }

    fn edges(&self) -> Self::Edges<'_> {
        Edges { grid: self }
    }
}

// Vertex topology

struct Vertices<'a, I> {
    grid: &'a Grid<I>,
}

impl<'a, I: Index> Topology for Vertices<'a, I> {
    type Item = I;
    type BuildHasher = RandomState;
    type ItemIter<'b>
    where
        Self: 'b,
    = impl Iterator<Item = Self::Item>;
    type AdjacentIter<'b>
    where
        Self: 'b,
    = impl Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter<'_> {
        self.grid.coords.keys().copied()
    }

    fn adjacent_to(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>> {
        let Coords(row, column) = self.grid.coords_of(item)?;
        let iter = self
            .grid
            .neighbors_of(row, column)
            .map(|Coords(row, column)| self.grid.at(row, column).unwrap());
        Some(iter)
    }

    fn contains(&self, item: Self::Item) -> bool {
        self.grid.coords.contains_key(&item)
    }
}

// Edge topology

struct Edges<'a, I> {
    grid: &'a Grid<I>,
}

impl<'a, I: Index> Topology for Edges<'a, I> {
    type Item = Edge<I>;
    type BuildHasher = UnorderedBuildHasher;
    type ItemIter<'b>
    where
        Self: 'b,
    = impl Iterator<Item = Self::Item>;
    type AdjacentIter<'b>
    where
        Self: 'b,
    = impl Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter<'_> {
        let grid = &self.grid.grid;
        let by_rows = (0..self.grid.rows).flat_map(move |row| {
            (0..self.grid.columns).tuple_windows().map(move |(a, b)| {
                let a = grid[row][a];
                let b = grid[row][b];
                Edge::new(a, b)
            })
        });
        let by_columns = (0..self.grid.columns).flat_map(move |column| {
            (0..self.grid.rows).tuple_windows().map(move |(a, b)| {
                let a = grid[a][column];
                let b = grid[b][column];
                Edge::new(a, b)
            })
        });
        by_rows.chain(by_columns)
    }

    fn adjacent_to(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>> {
        let Edge(UnorderedPair(a, b)) = item;
        let a_neighbors = outbound_edges(self.grid, a, b)?;
        let b_neighbors = outbound_edges(self.grid, b, a)?;
        Some(a_neighbors.chain(b_neighbors))
    }

    fn contains(&self, item: Self::Item) -> bool {
        let item = item.0;
        let a = self.grid.coords_of(item.0);
        let b = self.grid.coords_of(item.1);
        (a, b)
            .collapse(|a, b| adjacent(a.1, a.0, b.1, b.0))
            .unwrap_or_default()
    }
}

fn outbound_edges<I: Index>(
    grid: &Grid<I>,
    source: I,
    exclude: I,
) -> Option<impl Iterator<Item = Edge<I>> + '_> {
    let Coords(row, column) = grid.coords_of(source)?;
    let vertices = grid
        .neighbors_of(row, column)
        .map(|Coords(row, column)| grid.at(row, column).unwrap())
        .filter(move |target| *target != exclude)
        .map(move |target| Edge::new(source, target));
    Some(vertices)
}

/// Additional grid-specific traversals.
impl<I: Index> Grid<I> {
    /// An additional traversal of the graph's vertices by rows. Each row is processed
    /// in order, and for each row, indices of all its vertices in order are traversed.
    pub fn traverse_by_rows(&self) -> impl Iterator<Item = I> + '_ {
        let start = self.at(0, 0).unwrap();
        let last_row = self.rows - 1;
        let last_col = self.columns - 1;
        std::iter::successors(Some(start), move |id| match self.coords_of(*id).unwrap() {
            Coords(row, col) if row == last_row && col == last_col => None,
            Coords(row, col) if col == last_col => self.at(row + 1, 0),
            Coords(row, col) => self.at(row, col + 1),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn adjacent_test() {
        assert!(adjacent(5, 3, 5, 4));
        assert!(adjacent(5, 4, 5, 3));
        assert!(adjacent(3, 5, 4, 5));
        assert!(adjacent(4, 5, 3, 5));
        assert!(!adjacent(4, 5, 4, 5));
        assert!(!adjacent(3, 5, 6, 5));
        assert!(!adjacent(6, 5, 3, 5));
        assert!(!adjacent(5, 3, 5, 6));
        assert!(!adjacent(5, 6, 5, 3));
        assert!(!adjacent(5, 3, 6, 4));
    }

    #[test]
    fn edges_are_bidirectional() {
        let e1 = Edge::new(1, 2);
        let e2 = Edge::new(2, 1);
        assert_eq!(e1, e2);
        assert_eq!(e2, e1);
    }
}
