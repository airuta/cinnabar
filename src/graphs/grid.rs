use crate::Index;
use crate::Nodes;
use std::collections::HashMap;
use std::default::Default;

pub struct Grid<Idx, Payload> {
    rows: i32,
    columns: i32,
    grid: Vec<Vec<Idx>>,
    data: HashMap<Idx, (i32, i32, Payload)>,
}

impl<Idx: Index, Payload: Default> Grid<Idx, Payload> {
    pub fn new(rows: i32, columns: i32) -> Self {
        Self::construct(rows, columns)
    }

    fn construct(rows: i32, columns: i32) -> Self {
        let mut data = HashMap::new();
        let mut grid = Vec::with_capacity(rows as usize);
        for r in 0..rows {
            let mut row = Vec::with_capacity(columns as usize);
            for c in 0..columns {
                let idx = Index::generate();
                row.push(idx);
                data.insert(idx, (r, c, Payload::default()));
            }
            grid.push(row)
        }
        Self {
            rows,
            columns,
            grid,
            data,
        }
    }
}

// Coords interface

impl<Idx: Index, Payload> Grid<Idx, Payload> {
    pub fn get(&self, row: i32, column: i32) -> Option<Idx> {
        self.grid
            .get(row as usize)
            .and_then(|row| row.get(column as usize))
            .copied()
    }

    pub fn coords_of(&self, id: Idx) -> (i32, i32) {
        let (row, column, _) = self.data.get(&id).unwrap();
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

// Nodes provider

impl<Idx: Index, Payload> Nodes<Idx> for Grid<Idx, Payload> {
    type NodeIter<'a> = impl Iterator<Item = Idx>;
    type NeighborIter<'a> = impl Iterator<Item = Idx>;

    fn nodes(&self) -> Self::NodeIter<'_> {
        self.grid.iter().flat_map(|row| row.iter()).copied()
    }

    fn order(&self) -> usize {
        self.rows as usize * self.columns as usize
    }

    fn neighbors(&self, id: Idx) -> Self::NeighborIter<'_> {
        let (row, column) = self.coords_of(id);
        self.neighbors_of(row, column)
            .map(|(row, column)| self.get(row, column).unwrap())
    }
}
