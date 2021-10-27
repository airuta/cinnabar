//! Cinnabar is a library of graph algorithms that work with any kind of data strcutre implementing
//! several specific traits provided by the library. It also includes several predefined graphs that
//! can be used as is, or can serve as an example on how to implement graph trait in your specific
//! case.
//!
//! The philosphy of this library is to only provide functionality related to graph topology, namely
//! connection between vertices. Any associated data should be handled by the client through the use
//! of lightweight indices.
//!
//! # Quick start
//!
//! To get started quickly, pick and instantiate a suitable graph from the [`graphs`] module. You can
//! associate graph vertices with your own data through vertex indices when constructing a garph.
//!
//! ```
//! use cinnabar::prelude::*;
//! use cinnabar::graphs::Grid;
//! use cinnabar::graphs::grid::Coords;
//! use std::collections::HashMap;
//!
//! // Associated each verte in a grid with its weight = row * column
//! let mut weights = HashMap::new();
//! let grid = Grid::with_inspector(2, 3, |id: Counter, row, col| {
//!     weights.insert(id, row * col);
//! });
//!
//! // Traverse vertices by grid rows
//! for id in grid.traverse_by_rows() {
//!     let Coords(row, col) = grid.coords_of(id).unwrap();
//!     let weight = weights.get(&id).unwrap();
//!     println!("The weight of a vertex at {}, {} is {}", row, col, weight);
//! }
//! ```

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

mod counter;
mod index;
mod providers;
mod topology;
mod traversal;

pub mod graphs;
pub mod utils;

/// This module simplifiies the reexport of commonly-used members.
/// Note that pre-made graphs are not included into the prelude. Addtional convenieice utils
/// in the [`utils`] module are not provided and require a separate use statement as well.
pub mod prelude {
    use super::*;
    pub use counter::Counter;
    pub use index::Index;
    pub use providers::*;
    pub use traversal::Traversal;
}
