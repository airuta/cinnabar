pub trait Topology {
    type Item;
    type ItemIter<'a>: Iterator<Item = Self::Item>;
    type AdjacentIter<'a>: Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter<'_>;
    fn adjacent(&self, item: Self::Item) -> Option<Self::AdjacentIter<'_>>;
    fn contains(&self, item: Self::Item) -> bool;
}
