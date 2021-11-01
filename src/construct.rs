//! The module is the home for the constructions traits that allows createion and modification of graphs.

pub trait Construct<I> {
    fn add(&mut self, id: I) -> bool;
    fn link(&mut self, a: I, b: I) -> bool;
    fn remove(&mut self, id: I) -> bool;
    fn unlink(&mut self, a: I, b: I) -> bool;
}
