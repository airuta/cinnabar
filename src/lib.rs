#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

mod edge;
mod index;
mod mutators;
mod providers;

pub mod graphs;

pub use edge::Edge;
pub use index::Index;
pub use mutators::*;
pub use providers::*;
