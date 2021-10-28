use super::Unique;
use std::sync::atomic;

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

/// A `Counter` is a naive implementation of a unique index, satisfying both `Index` and
/// `Unique` constraints. In generates new usize-based integers using atomic add operation.
/// There is no guarantee that indices will be sequential or, in fact, that they will be
/// monotonically increasing.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Counter(usize);

impl Unique for Counter {
    fn generate() -> Self {
        Self(COUNTER.fetch_add(1, atomic::Ordering::SeqCst))
    }
}
