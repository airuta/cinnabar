pub trait Vertices<I> {
    type VertexIter<'a>: Iterator<Item = I>;
    type NeighborIter<'a>: Iterator<Item = I>;

    fn order(&self) -> usize;
    fn nodes(&self) -> Self::VertexIter<'_>;
    fn neighbors(&self, id: I) -> Self::NeighborIter<'_>;
    fn has_vertex(&self, id: I) -> bool;
}

pub trait Edges<I> {
    type EdgeIter<'a>: Iterator<Item = (I, I)>;
    type OutIter<'a>: Iterator<Item = (I, I)>;

    fn size(&self) -> usize;
    fn edges(&self) -> Self::EdgeIter<'_>;
    fn out(&self, id: I) -> Self::OutIter<'_>;
    fn has_edge(&self, source: I, target: I) -> bool;
}
