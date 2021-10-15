use crate::utils::Reverse;
use crate::Index;
use crate::{providers::*, VertexWalk};

use itertools::Itertools;
use std::collections::HashMap;
use std::default::Default;

pub struct Grid<I, E, V> {
    rows: i32,
    columns: i32,
    grid: Vec<Vec<I>>,
    vertices: HashMap<I, (i32, i32, V)>,
    edges: HashMap<(I, I), E>,
}

impl<I, E, V> Grid<I, E, V>
where
    I: Index,
    E: Default,
    V: Default,
{
    pub fn new(rows: usize, columns: usize) -> Self {
        Self::construct(rows as i32, columns as i32).wireup()
    }

    fn construct(rows: i32, columns: i32) -> Self {
        let mut vertices = HashMap::new();
        let mut grid = Vec::with_capacity(rows as usize);
        for r in 0..rows {
            let mut row = Vec::with_capacity(columns as usize);
            for c in 0..columns {
                let idx = Index::generate();
                let vertex = (r, c, V::default());
                vertices.insert(idx, vertex);
                row.push(idx);
            }
            grid.push(row)
        }
        Self {
            rows,
            columns,
            grid,
            vertices,
            edges: HashMap::new(),
        }
    }

    fn wireup(mut self) -> Self {
        for row in 0..self.rows as usize {
            for (a, b) in (0..self.columns as usize).tuple_windows() {
                let a = self.grid[row][a];
                let b = self.grid[row][b];
                self.edges.insert((a, b), E::default());
            }
        }
        for column in 0..self.columns as usize {
            for (a, b) in (0..self.rows as usize).tuple_windows() {
                let a = self.grid[a][column];
                let b = self.grid[b][column];
                self.edges.insert((a, b), E::default());
            }
        }
        self
    }
}

// Getter interface

impl<I: Index, E, V> Grid<I, E, V> {
    pub fn get_vertex(&self, id: I) -> Option<&V> {
        self.vertices.get(&id).map(|(_, _, vertex)| vertex)
    }

    pub fn get_vertex_mut(&mut self, id: I) -> Option<&mut V> {
        self.vertices.get_mut(&id).map(|(_, _, vertex)| vertex)
    }

    pub fn get_edge(&self, a: I, b: I) -> Option<&E> {
        let edge = (a, b);
        self.edges
            .get(&edge)
            .or_else(|| self.edges.get(&edge.rev()))
    }

    pub fn get_edge_mut(&mut self, a: I, b: I) -> Option<&mut E> {
        let edge = (a, b);
        match self.edges.contains_key(&edge) {
            true => self.edges.get_mut(&edge),
            _ => self.edges.get_mut(&edge.rev()),
        }
    }
}

// Coords interface

impl<I: Index, E, V> Grid<I, E, V> {
    pub fn at(&self, row: i32, column: i32) -> Option<I> {
        self.grid
            .get(row as usize)
            .and_then(|row| row.get(column as usize))
            .copied()
    }

    pub fn coords_of(&self, id: I) -> (i32, i32) {
        let (row, column, _) = self.vertices.get(&id).unwrap();
        (*row, *column)
    }

    fn neighbors_of(&self, row: i32, column: i32) -> impl Iterator<Item = (i32, i32)> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| (row + dy, column + dx))
            .filter(|coords| (0..self.rows).contains(&coords.1))
            .filter(|coords| (0..self.columns).contains(&coords.0))
    }
}

// Vertex provider

impl<I: Index, E, V> VertexProvider<I> for Grid<I, E, V> {
    type VertexIter<'a> = impl Iterator<Item = I>;
    type NeighborIter<'a> = impl Iterator<Item = I>;

    fn order(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&self) -> Self::VertexIter<'_> {
        self.vertices.keys().copied()
    }

    fn neighbors(&self, id: I) -> Self::NeighborIter<'_> {
        let (row, column) = self.coords_of(id);
        self.neighbors_of(row, column)
            .map(|(row, column)| self.at(row, column).unwrap())
    }

    fn has_vertex(&self, id: I) -> bool {
        self.vertices.contains_key(&id)
    }
}

// Edge provider

impl<I: Index, E, V> EdgeProvider<I> for Grid<I, E, V> {
    type EdgeIter<'a> = impl Iterator<Item = (I, I)>;
    type OutboundIter<'a> = impl Iterator<Item = (I, I)>;

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> Self::EdgeIter<'_> {
        self.edges.keys().copied()
    }

    fn outbound(&self, id: I) -> Self::OutboundIter<'_> {
        let (row, column) = self.coords_of(id);
        self.neighbors_of(row, column)
            .map(move |(row, column)| (id, self.at(row, column).unwrap()))
    }

    fn has_edge(&self, source: I, target: I) -> bool {
        self.edges.contains_key(&(source, target))
    }
}

// Walker interface

impl<I: Index, E, V> VertexWalk<I> for Grid<I, E, V> {
    type Iter<F> = impl Iterator<Item = I>;
    fn walk<F>(&self, start: I, mut step: F) -> Self::Iter<F>
    where
        F: FnMut(I) -> Option<I>,
    {
        let mut current = start;
        std::iter::from_fn(move || step(current).map(|id| std::mem::replace(&mut current, id)))
    }
}
