# Cinnabar
Cinnabar is a pure-rust library of graph algorithms and data structures. It is designed to be fast and easy to use.
Its defining feature is the decomposition of the multitude of graph-related functionality into several simple traits.
No need to reinvent the algorithms from scratch for your specific case. After implementing a couple of traits, your
will be able to tap into the power of graph and netork theory.

## Quick start
Add cinnabar to the dependencies section of your project.

``` toml
[dependencies]
cinnabar = "*"
```

To simplify development, you can use one of the pre-made graphs provided by the library.

``` rust
use cinnabar::prelude::*;
use cinnabar::graphs::Grid;
use cinnabar::graphs::grid::Coords;
use std::collections::HashMap;

// Associated each vertexl in a grid with Manhattan distance to it.
let mut weights = HashMap::new();
let grid = Grid::with_inspector(2, 3, |id: Counter, row, col| {
    weights.insert(id, row + col);
});

// Traverse vertices by grid rows
for id in grid.traverse_by_rows() {
    let Coords(row, col) = grid.coords_of(id).unwrap();
    let weight = weights.get(&id).unwrap();
    println!("The weight of a vertex at {}, {} is {}", row, col, weight);
}
```

## Features

This is a short list of features provided by the library. The list is not exhaustive, please consult the crate documentation for the
detailed information.

### Pre-made grahs
- [x] Grid
- [x] Adjacency list-based graph
- [ ] Incidence matrix-based graph
  
### Traversals
- [x] DFS
- [x] BFS

### Search
- [ ] DFS-based
- [ ] BFS-based
- [ ] Dijkstra
- [ ] A-star

## Supported Rust versions

Cinnabar requires two features of Rust that are currently only provided in nightly builds:
* [impl Trait in type aliases](https://github.com/rust-lang/rust/issues/63063)
* [Generic associated types](https://github.com/rust-lang/rust/issues/44265).

Both proposals are moving quite fast and are close to stabilization. It is possible to implement the library without them,
but at present it easier to focus the efforts on the functionality and not implementation. This can change if there is
a sufficient interest in the community to have this library working in stable Rust, and the aforementioned features are not
yet in stable Rust.

## License

This project is licensed under the [MIT license](https://github.com/airuta/cinnabar/blob/develop/LICENSE).
