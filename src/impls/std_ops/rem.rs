use super::*;

impl<Order: Dim, N: Dim, Data, Data2> std::ops::Rem<Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item>,
    Data::Owned: ContiguousContainer<Item = Data::Item>,
    Data::Item: std::ops::Rem<Output = Data::Item> + std::ops::MulAssign + Clone + Zero + Real,// + std::ops::AddAssign + std::ops::MulAssign,
{
    type Output = Diff<Order, N, Data::Owned>;

    fn rem(self, rhs: Diff<Order, N, Data2>) -> Self::Output {
        // TODO to check, also is it correct for negative values?
        let rem = self.value().clone() % rhs.value().clone();
        let i_div = (self.value().clone() - rem.clone()) / rhs.value().clone();
        let derivatives = self.derivatives() - rhs.derivatives().into_owned().scaled_by(i_div);
        let data = std::iter::once(rem)
            .chain(derivatives.unwrap_data().into_iter())
            .collect::<Vec<_>>(); // TODO <- optimize
        Self::Output::from_data(self.order, self.n, Data::from_slice(&data[..])) // TODO <- optimize
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::RemAssign<Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item>,
    Data::Owned: ContiguousContainer<Item = Data::Item>,
    Self: std::ops::Rem<Diff<Order, N, Data2>, Output = Self> + Clone, // TODO without Clone
{
    fn rem_assign(&mut self, rhs: Diff<Order, N, Data2>) {
        *self = self.clone() % rhs;
    }
}