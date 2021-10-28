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
