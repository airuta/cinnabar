use crate::counter::Counter;
use crate::index::Index;
use crate::providers::*;
use crate::traversal::*;

use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Neg;

#[derive(Copy, Clone, Debug)]
pub struct Coords(pub usize, pub usize);

pub struct Grid<I = Counter> {
    rows: usize,
    columns: usize,
    grid: Vec<Vec<I>>,
    coords: HashMap<I, Coords>,
}

impl<I: Index> Grid<I> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self::with_inspector(rows, columns, |_, _, _| ())
    }

    pub fn with_inspector(
        rows: usize,
        columns: usize,
        mut inspector: impl FnMut(usize, usize, I),
    ) -> Self {
        let mut coords = HashMap::new();
        let mut grid = Vec::with_capacity(rows);
        for r in 0..rows {
            let mut row = Vec::with_capacity(columns);
            for c in 0..columns {
                let id = Index::generate();
                coords.insert(id, Coords(r, c));
                row.push(id);
                inspector(r, c, id);
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

// Grid-specific interface

impl<I: Copy + Hash + Eq> Grid<I> {
    pub fn at(&self, row: usize, column: usize) -> Option<I> {
        self.grid.get(row).and_then(|row| row.get(column)).copied()
    }

    pub fn coords_of(&self, id: I) -> Option<Coords> {
        self.coords.get(&id).copied()
    }

    fn neighbors_of(&self, row: usize, column: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter(move |(dx, _)| inside(column, *dx, self.columns))
            .filter(move |(_, dy)| inside(row, *dy, self.rows))
            .map(move |(dx, dy)| (dy as usize + row, dx as usize + column))
    }

    fn edge_coords(&self, a: I, b: I) -> Option<(Coords, Coords)> {
        let a = self.coords_of(a)?;
        let b = self.coords_of(b)?;
        Some((a, b))
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

// Vertex provider

impl<I: Copy + Hash + Eq> VertexProvider<I> for Grid<I> {
    type VertexIter<'a> = impl Iterator<Item = I>;
    type NeighborIter<'a> = impl Iterator<Item = I>;

    fn order(&self) -> usize {
        self.coords.len()
    }

    fn vertices(&self) -> Self::VertexIter<'_> {
        self.coords.keys().copied()
    }

    fn neighbors(&self, id: I) -> Option<Self::NeighborIter<'_>> {
        let Coords(row, column) = self.coords_of(id)?;
        let iter = self
            .neighbors_of(row, column)
            .map(|(row, column)| self.at(row, column).unwrap());
        Some(iter)
    }

    fn has_vertex(&self, id: I) -> bool {
        self.coords.contains_key(&id)
    }
}

// Edge provider

impl<I: Copy + Hash + Eq> EdgeProvider<I> for Grid<I> {
    type EdgeIter<'a> = impl Iterator<Item = (I, I)>;
    type OutboundIter<'a> = impl Iterator<Item = (I, I)>;

    fn size(&self) -> usize {
        self.rows * (self.columns - 1) + self.columns * (self.rows - 1)
    }

    fn edges(&self) -> Self::EdgeIter<'_> {
        let by_rows = (0..self.rows).flat_map(move |row| {
            (0..self.columns).tuple_windows().map(move |(a, b)| {
                let a = self.grid[row][a];
                let b = self.grid[row][b];
                (a, b)
            })
        });
        let by_columns = (0..self.columns).flat_map(move |column| {
            (0..self.rows).tuple_windows().map(move |(a, b)| {
                let a = self.grid[a][column];
                let b = self.grid[b][column];
                (a, b)
            })
        });
        by_rows.chain(by_columns)
    }

    fn outbound(&self, id: I) -> Option<Self::OutboundIter<'_>> {
        let Coords(row, column) = self.coords_of(id)?;
        let iter = self
            .neighbors_of(row, column)
            .map(move |(row, column)| (id, self.at(row, column).unwrap()));
        Some(iter)
    }

    fn has_edge(&self, source: I, target: I) -> bool {
        self.edge_coords(source, target)
            .map(|(a, b)| adjacent(a.1, a.0, b.1, b.0))
            .unwrap_or_default()
    }
}

// Traversal interface

impl<I: Copy> Traversal<I> for Grid<I> {
    type Iter<F> = impl Iterator<Item = I>;
    fn traverse<F>(&self, start: I, step: F) -> Self::Iter<F>
    where
        F: FnMut(I) -> Option<I>,
    {
        traverse(start, step)
    }
}

impl<I: Copy + Hash + Eq> Grid<I> {
    pub fn traverse_by_row(&self) -> impl Iterator<Item = I> + '_ {
        let start = self.at(0, 0).unwrap();
        let last_row = self.rows - 1;
        let last_col = self.columns - 1;
        self.traverse(start, move |id| match self.coords_of(id).unwrap() {
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
