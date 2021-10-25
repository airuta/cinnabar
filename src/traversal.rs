pub trait Traversal<I> {
    type Iter<F>: Iterator<Item = I>;
    fn traverse<F>(&self, start: I, step: F) -> Self::Iter<F>
    where
        F: FnMut(I) -> Option<I>;
}

/// Unfortunately, the current implementation of type aliases in traits does not allow
/// one to use as a default method in a trait. Implementors of the Traversal trait can
/// use this method instead to avoid code duplication.
pub fn traverse<I, F>(start: I, mut step: F) -> impl Iterator<Item = I>
where
    I: Copy,
    F: FnMut(I) -> Option<I>,
{
    let mut current = Some(start);
    std::iter::from_fn(move || current.and_then(|id| std::mem::replace(&mut current, step(id))))
}
