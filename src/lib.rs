#![feature(generic_associated_types)]

mod edge;
mod index;

pub use edge::Edge;
pub use index::Index;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
