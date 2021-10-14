pub trait Edge<Idx> {
    type Iter<'a>: Iterator<Item = Idx>;
    fn nodes(&self) -> Self::Iter<'_>;
}
