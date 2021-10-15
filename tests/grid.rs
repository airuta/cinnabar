use cinnabar::Counter;
use cinnabar::EdgeProvider;
use cinnabar::VertexProvider;
use cinnabar::VertexWalk;

type Grid = cinnabar::graphs::Grid<Counter, (), ()>;

const ROWS: usize = 3;
const COLS: usize = 4;

fn create_grid() -> Grid {
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
            const MAX_ROW: i32 = ROWS as i32 - 1;
            const MAX_COL: i32 = COLS as i32 - 1;
            let (row, col) = grid.coords_of(id);
            match (row, col) {
                (MAX_ROW, MAX_COL) => None,
                (row, MAX_COL) => grid.at(row + 1, 0),
                _ => grid.at(row, col + 1),
            }
        })
        .collect::<Vec<_>>();

    for i in 0..ROWS {
        for j in 0..COLS {
            let id = grid.at(i as i32, j as i32).unwrap();
            assert!(ids.contains(&id));
        }
    }
}
