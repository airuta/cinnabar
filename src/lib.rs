#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

mod counter;
mod index;
mod providers;
mod traversal;

pub mod graphs;
pub mod utils;

pub mod prelude {
    use super::*;
    pub use counter::Counter;
    pub use index::Index;
    pub use providers::*;
    pub use traversal::Traversal;
}
