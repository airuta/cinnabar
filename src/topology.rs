pub trait Topology {
    type Item;
    type ItemIter: Iterator<Item = Self::Item>;
    type AdjacentIter: Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::ItemIter;
    fn adjacent(&self, item: Self::Item) -> Option<Self::AdjacentIter>;
    fn contains(&self, item: Self::Item) -> bool;
}
