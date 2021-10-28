//! This module defines a 2D grid graph and its related strcutres.

use crate::index::*;
use crate::providers::*;
use crate::topology::*;
use crate::traversal::*;

use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Neg;

/// Coordinates of a not in a grid in row-column order.
#[derive(Copy, Clone, Debug)]
pub struct Coords(pub usize, pub usize);

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
    pub fn neighbors_of(&self, row: usize, column: usize) -> impl Iterator<Item = Coords> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter(move |(dx, _)| inside(column, *dx, self.columns))
            .filter(move |(_, dy)| inside(row, *dy, self.rows))
            .map(move |(dx, dy)| Coords(dy as usize + row, dx as usize + column))
    }
}

fn inside(value: usize, offset: i32, high: usize) -> bool {
    let range = match offset {
        _ if offset >= 0 => 0..high - offset as usize,
        _ => offset.neg() as usize..high + offset.neg() as usize,
    };
    range.contains(&value)
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
    type Vertices<'a> = impl Topology<Item = I>;

    fn order(&self) -> usize {
        self.coords.len()
    }

    fn vertices(&self) -> Self::Vertices<'_> {
        Vertices { grid: self }
    }
}

impl<I: Index> EdgeProvider<I> for Grid<I> {
    type Edges<'a> = impl Topology<Item = (I, I)>;

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
    type ItemIter<'b> = impl Iterator<Item = Self::Item>;
    type AdjacentIter<'b> = impl Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter<'_> {
        self.grid.coords.keys().copied()
    }

    fn adjacent(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>> {
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
    type Item = (I, I);
    type ItemIter<'b> = impl Iterator<Item = Self::Item>;
    type AdjacentIter<'b> = impl Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter<'_> {
        let grid = &self.grid.grid;
        let by_rows = (0..self.grid.rows).flat_map(move |row| {
            (0..self.grid.columns).tuple_windows().map(move |(a, b)| {
                let a = grid[row][a];
                let b = grid[row][b];
                (a, b)
            })
        });
        let by_columns = (0..self.grid.columns).flat_map(move |column| {
            (0..self.grid.rows).tuple_windows().map(move |(a, b)| {
                let a = grid[a][column];
                let b = grid[b][column];
                (a, b)
            })
        });
        by_rows.chain(by_columns)
    }

    fn adjacent(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>> {
        let (a, b) = item;
        let a_neighbors = adjacent_vertices(self.grid, a, b)?;
        let b_neighbors = adjacent_vertices(self.grid, b, a)?;
        Some(a_neighbors.chain(b_neighbors))
    }

    fn contains(&self, item: Self::Item) -> bool {
        edge_coords(self.grid, item.0, item.1)
            .map(|(a, b)| adjacent(a.1, a.0, b.1, b.0))
            .unwrap_or_default()
    }
}

fn adjacent_vertices<I: Index>(
    grid: &Grid<I>,
    source: I,
    exclude: I,
) -> Option<impl Iterator<Item = (I, I)> + '_> {
    let Coords(row, column) = grid.coords_of(source)?;
    let vertices = grid
        .neighbors_of(row, column)
        .map(|Coords(row, column)| grid.at(row, column).unwrap())
        .filter(move |target| *target != exclude)
        .map(move |target| (source, target));
    Some(vertices)
}

fn edge_coords<I: Index>(grid: &Grid<I>, a: I, b: I) -> Option<(Coords, Coords)> {
    let a = grid.coords_of(a)?;
    let b = grid.coords_of(b)?;
    Some((a, b))
}

/// Additional grid-specific traversals.
impl<I: Index> Grid<I> {
    /// An additional traversal of the graph's vertices by rows. Each row is processed
    /// in order, and for each row, indices of all its vertices in order are traversed.
    pub fn traverse_by_rows(&self) -> impl Iterator<Item = I> + '_ {
        let start = self.at(0, 0).unwrap();
        let last_row = self.rows - 1;
        let last_col = self.columns - 1;
        traverse(start, move |id| match self.coords_of(id).unwrap() {
            Coords(row, col) if row == last_row && col == last_col => None,
            Coords(row, col) if col == last_col => self.at(row + 1, 0),
            Coords(row, col) => self.at(row, col + 1),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside_test() {
        assert!(inside(1, -1, 5));
        assert!(inside(5, -1, 5));
        assert!(inside(0, 1, 5));
        assert!(inside(3, 1, 5));
        assert!(!inside(4, 1, 5));
        assert!(!inside(0, -1, 5));
        assert!(!inside(5, 1, 5));
        assert!(!inside(6, 1, 5));
    }

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
}
