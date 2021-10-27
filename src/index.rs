//! This modules houses definitions for index traits, as well as predefined implementations
//! that can be used to avoid rolling out custom indices.

use std::hash::Hash;

mod counter;

pub use counter::Counter;

/// `Unique` trait provides a way to generate unique index. It is used by most graph
/// constructrs instead of [`Default`] standard library trait, as all indices in a graph
/// must be different.
///
/// [`Default`] std::default::Default
pub trait Unique {
    fn generate() -> Self;
}

/// `Index` is a convenience trait for types that can be used a indices in a graph.
/// Since index is an interface between data that is usually associated with graph
/// elements like edge weights, and graph topology, indices must be lightweight and
/// hashable. This marker trait combines these feature and provides a blanket
/// implementation.
pub trait Index: Copy + Eq + Hash {}
impl<T: Copy + Eq + Hash> Index for T {}
