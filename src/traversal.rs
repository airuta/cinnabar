use crate::topology::Topology;

pub trait Traversal: Topology {}

/// This function is provides a way to define a custom traversal through the specified step function.
pub fn traverse<I, F>(start: I, mut step: F) -> impl Iterator<Item = I>
where
    I: Copy,
    F: FnMut(I) -> Option<I>,
{
    let mut current = Some(start);
    std::iter::from_fn(move || current.and_then(|id| std::mem::replace(&mut current, step(id))))
}
