pub trait VertexProvider<I> {
    type VertexIter<'a>: Iterator<Item = I>;
    type NeighborIter<'a>: Iterator<Item = I>;

    fn order(&self) -> usize;
    fn vertices(&self) -> Self::VertexIter<'_>;
    fn neighbors(&self, id: I) -> Option<Self::NeighborIter<'_>>;
    fn has_vertex(&self, id: I) -> bool;
}

pub trait EdgeProvider<I> {
    type EdgeIter<'a>: Iterator<Item = (I, I)>;
    type OutboundIter<'a>: Iterator<Item = (I, I)>;

    fn size(&self) -> usize;
    fn edges(&self) -> Self::EdgeIter<'_>;
    fn outbound(&self, id: I) -> Option<Self::OutboundIter<'_>>;
    fn has_edge(&self, source: I, target: I) -> bool;
}
