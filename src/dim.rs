use std::fmt::Debug;



pub trait Dim: Debug + Copy {
    fn value(&self) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed<const N: usize>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dynamic(pub usize);

impl<const N: usize> Dim for Fixed<N> {
    fn value(&self) -> usize {
        N
    }
}

impl Dim for Dynamic {
    fn value(&self) -> usize {
        self.0
    }
}