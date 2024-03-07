
pub trait DiffIndex {
    fn into_orders(self) -> impl Iterator<Item = usize> + Clone;
}

impl<I> DiffIndex for I
where
    I: IntoIterator<Item = usize>,
    I::IntoIter: Clone,
{
    fn into_orders(self) -> impl Iterator<Item = usize> + Clone {
        self.into_iter()
    }
}