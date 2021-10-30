use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::marker::PhantomData;

/// Unordered pair requires special handling of [`PartialEq`] and [`Hash`] trait.
/// The first one is solved with a custom implementation, but the second one requires
/// using a special Hasher and BuildHasher implementation with your data structures.
#[derive(Debug, Copy, Clone, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnorderedPair<I>(pub I, pub I);

impl<T> From<(T, T)> for UnorderedPair<T> {
    fn from(tuple: (T, T)) -> UnorderedPair<T> {
        UnorderedPair(tuple.0, tuple.1)
    }
}

impl<T> From<UnorderedPair<T>> for (T, T) {
    fn from(pair: UnorderedPair<T>) -> (T, T) {
        (pair.0, pair.1)
    }
}

impl<T> PartialEq for UnorderedPair<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &UnorderedPair<T>) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

/// Hasher for unordered collections. This is a very inefficient approach, and there exist better
/// ways to construct an order-invariant hasher (see, for example, "Incremental Multiset Hash Functions
/// and Their Application to Memory Integrity Checking" paper), but it is beyond the scope of this
/// library and currently there is no known implementation in Rust for such hash.
#[derive(Default)]
pub struct UnorderedHasher<H> {
    phantom: PhantomData<H>,
    hashes: BTreeSet<u64>,
}

impl<H: Hasher + Default> Hasher for UnorderedHasher<H> {
    fn write(&mut self, bytes: &[u8]) {
        let mut hasher = H::default();
        bytes.hash(&mut hasher);
        self.hashes.insert(hasher.finish());
    }

    fn finish(&self) -> u64 {
        let mut hasher = H::default();
        for hash in &self.hashes {
            hash.hash(&mut hasher);
        }
        hasher.finish()
    }
}

/// Build hasher that constructs [`UnorderedHasher`] for hash calculation on [`UnorderedPair`].
pub type UnorderedBuildHasher = BuildHasherDefault<UnorderedHasher<DefaultHasher>>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    #[test]
    fn unordered_pairs_are_equal() {
        let p1 = UnorderedPair(1, 2);
        let p2 = UnorderedPair(2, 1);
        assert_eq!(p1, p2);
        assert_eq!(p2, p1);
    }

    #[test]
    fn unordered_pairs_have_same_hash() {
        let p1 = UnorderedPair(1, 2);
        let p2 = UnorderedPair(2, 1);
        let build_hasher = UnorderedBuildHasher::default();
        let mut set = HashSet::with_hasher(build_hasher);
        set.insert(p1);
        assert!(set.contains(&p2));
    }
}
