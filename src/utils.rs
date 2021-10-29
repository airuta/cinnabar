//! This module is home of various usefuls utils for graph manipulation.

/// Reverse trait provides a way to reverse some bidirectioanl objects such as graph edges.
pub trait Reverse {
    /// The type of the reversal output.
    type Output;

    /// Returns the result of reversal.
    fn rev(self) -> Self::Output;
}

impl<A, B> Reverse for (A, B) {
    type Output = (B, A);
    fn rev(self) -> Self::Output {
        (self.1, self.0)
    }
}

/// For a given object `X<A, C>`, `Bifunctor` applies two maps simultaneously to return Y<B, D>.
pub trait Bifunctor<A, B, C, D> {
    /// The type of mapping output.
    type Output;

    /// Applies two maps to the object.
    fn bimap<F, G>(self, f: F, g: G) -> Self::Output
    where
        F: Fn(A) -> B,
        G: Fn(C) -> D;
}

impl<A, B, C, D> Bifunctor<A, B, C, D> for (A, C) {
    type Output = (B, D);
    fn bimap<F, G>(self, f: F, g: G) -> Self::Output
    where
        F: Fn(A) -> B,
        G: Fn(C) -> D,
    {
        (f(self.0), g(self.1))
    }
}

/// For a given object `X<A, B>`, `Collapse` produces `Y<C>` by appliying a combinator to both components
/// of the input object.
pub trait Collapse<A, B, C> {
    /// The type of the collapse output.
    type Output;

    /// Collapse the pair.
    fn collapse<F>(self, f: F) -> Self::Output
    where
        F: Fn(A, B) -> C;
}

impl<A, B, C> Collapse<A, B, C> for (Option<A>, Option<B>) {
    type Output = Option<C>;
    fn collapse<F>(self, f: F) -> Self::Output
    where
        F: Fn(A, B) -> C,
    {
        let a = self.0?;
        let b = self.1?;
        Some(f(a, b))
    }
}
