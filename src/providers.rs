pub trait Nodes<Idx> {
    type NodeIter<'a>: Iterator<Item = Idx>;
    type NeighborIter<'a>: Iterator<Item = Idx>;

    fn nodes(&self) -> Self::NodeIter<'_>;
    fn order(&self) -> usize;
    fn neighbors(&self, id: Idx) -> Self::NeighborIter<'_>;
}
