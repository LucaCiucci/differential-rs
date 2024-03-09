use std::fmt::Debug;



pub trait Dim: Debug + Copy {
    fn value(&self) -> Option<usize>;
    fn free() -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed<const N: usize>;

impl<const N: usize> Dim for Fixed<N> {
    fn value(&self) -> Option<usize> {
        Some(N)
    }
    fn free() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dynamic(pub Option<usize>);

impl Dim for Dynamic {
    fn value(&self) -> Option<usize> {
        self.0
    }
    fn free() -> Self {
        Self(None)
    }
}