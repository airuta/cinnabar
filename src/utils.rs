pub trait Reverse {
    type Output;
    fn rev(self) -> Self::Output;
}

impl<A, B> Reverse for (A, B) {
    type Output = (B, A);
    fn rev(self) -> Self::Output {
        (self.1, self.0)
    }
}
