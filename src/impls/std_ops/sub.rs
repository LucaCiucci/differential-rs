use super::*;

impl<Order: Dim, N: Dim, Data> std::ops::Sub for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainerMut<Item = Data::Item>,
    Data::Item: std::ops::SubAssign,
{
    type Output = <Self as IntoOwned>::Owned;

    fn sub(self, other: Self) -> Self::Output {
        self.sub(&other)
    }
}

impl<Order: Dim, N: Dim, Data> std::ops::Sub<&Diff<Order, N, Data>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainerMut<Item = Data::Item>,
    Data::Item: std::ops::SubAssign,
{
    type Output = <Self as IntoOwned>::Owned;

    fn sub(self, other: &Self) -> Self::Output {
        let mut result = self.into_owned();
        result -= other;
        result
    }
}


impl<Order: Dim, N: Dim, Data, Data2> std::ops::SubAssign<Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainerMut,
    Data2: ContiguousContainer,
    Data::Item: std::ops::SubAssign<Data2::Item>,
{
    fn sub_assign(&mut self, other: Diff<Order, N, Data2>) {
        self.sub_assign(&other)
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::SubAssign<&Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainerMut,
    Data2: ContiguousContainer,
    Data::Item: std::ops::SubAssign<Data2::Item>,
{
    fn sub_assign(&mut self, other: &Diff<Order, N, Data2>) {
        if self.order() == other.order() && self.n() == other.n() {
            let l = self.data.slice_mut();
            let r = other.data_slice();
            assert_eq!(l.len(), r.len());
            for (a, b) in l.iter_mut().zip(r) {
                *a -= b.clone();
            }
        } else {
            todo!()
        }
    }
}

impl<Order: Dim, N: Dim, Data> std::ops::Neg for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainerMut<Item = Data::Item>,
    Data::Item: std::ops::Neg<Output = Data::Item>,
{
    type Output = <Self as IntoOwned>::Owned;

    fn neg(self) -> Self::Output {
        let result = self.data.map_into_owned(|x, _| -x);
        Self::Output::from_data(self.order, self.n, result)
    }
}