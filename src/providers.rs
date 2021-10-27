use crate::topology::Topology;

pub trait VertexProvider<I> {
    type Vertices<'a>: Topology<Item = I>;
    fn order(&self) -> usize;
    fn vertices(&self) -> Self::Vertices<'_>;
}

pub trait EdgeProvider<I> {
    type EdgeIter<'a>: Iterator<Item = (I, I)>;
    type OutboundIter<'a>: Iterator<Item = (I, I)>;

    fn size(&self) -> usize;
    fn edges(&self) -> Self::EdgeIter<'_>;
    fn outbound(&self, id: I) -> Option<Self::OutboundIter<'_>>;
    fn has_edge(&self, source: I, target: I) -> bool;
}
