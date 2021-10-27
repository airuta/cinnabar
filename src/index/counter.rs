use super::Unique;
use std::sync::atomic;

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Counter(usize);

impl Unique for Counter {
    fn generate() -> Self {
        Self(COUNTER.fetch_add(1, atomic::Ordering::SeqCst))
    }
}
