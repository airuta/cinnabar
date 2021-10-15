pub trait VertexWalk<I> {
    type Iter<F>: Iterator<Item = I>;
    fn walk<F>(&self, start: I, step: F) -> Self::Iter<F>
    where
        F: FnMut(I) -> Option<I>;
}
