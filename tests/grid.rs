use cinnabar::Counter;
use cinnabar::EdgeProvider;
use cinnabar::VertexProvider;

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
