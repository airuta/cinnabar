use std::hash::Hash;

pub trait Index: Eq + Hash + Copy {
    fn generate() -> Self;
}
