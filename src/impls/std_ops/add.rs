use super::*;

impl<Order: Dim, N: Dim, Data> std::ops::Add for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainerMut<Item = Data::Item>,
    Data::Item: std::ops::AddAssign,
{
    type Output = <Self as IntoOwned>::Owned;

    fn add(self, other: Self) -> Self::Output {
        self.add(&other)
    }
}

impl<Order: Dim, N: Dim, Data> std::ops::Add<&Diff<Order, N, Data>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainerMut<Item = Data::Item>,
    Data::Item: std::ops::AddAssign,
{
    type Output = <Self as IntoOwned>::Owned;

    fn add(self, other: &Self) -> Self::Output {
        let mut result = self.into_owned();
        result += other;
        result
    }
}


impl<Order: Dim, N: Dim, Data, Data2> std::ops::AddAssign<Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainerMut,
    Data2: ContiguousContainer,
    Data::Item: std::ops::AddAssign<Data2::Item>,
{
    fn add_assign(&mut self, other: Diff<Order, N, Data2>) {
        self.add_assign(&other)
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::AddAssign<&Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainerMut,
    Data2: ContiguousContainer,
    Data::Item: std::ops::AddAssign<Data2::Item>,
{
    fn add_assign(&mut self, other: &Diff<Order, N, Data2>) {
        if self.order() == other.order() && self.n() == other.n() {
            let l = self.data.slice_mut();
            let r = other.data_slice();
            assert_eq!(l.len(), r.len());
            for (a, b) in l.iter_mut().zip(r) {
                *a += b.clone();
            }
        } else {
            todo!()
        }
    }
}