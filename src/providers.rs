use crate::topology::Topology;

pub trait VertexProvider<I> {
    type Vertices<'a>: Topology<Item = I>;
    fn order(&self) -> usize;
    fn vertices(&self) -> Self::Vertices<'_>;
}

pub trait EdgeProvider<I> {
    type Edges<'a>: Topology<Item = (I, I)>;
    fn size(&self) -> usize;
    fn edges(&self) -> Self::Edges<'_>;
}
