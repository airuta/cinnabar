use cinnabar::graphs::grid::Coords;
use cinnabar::graphs::Grid;
use cinnabar::prelude::*;

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
fn grid_walk_should_visit_all_vertices() {
    let grid = create_grid();
    let start = grid.at(0, 0).unwrap();

    #[allow(clippy::needless_collect)]
    let ids = grid
        .walk(start, |id| {
            const MAX_ROW: usize = ROWS - 1;
            const MAX_COL: usize = COLS - 1;
            let Coords(row, col) = grid.coords_of(id)?;
            match (row, col) {
                (MAX_ROW, MAX_COL) => None,
                (row, MAX_COL) => grid.at(row + 1, 0),
                _ => grid.at(row, col + 1),
            }
        })
        .collect::<Vec<_>>();

    for i in 0..ROWS {
        for j in 0..COLS {
            let id = grid.at(i, j).unwrap();
            assert!(ids.contains(&id));
        }
    }
}
