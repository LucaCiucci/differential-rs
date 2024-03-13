use super::*;

impl<Order: Dim, N: Dim, Data> std::ops::Add for Differential<Order, N, Data>
where
    Data: ConstStorage,
    Data::Owned: MutStorage<Item = Data::Item>,
    Data::Item: std::ops::AddAssign + Zero,
{
    type Output = <Self as IntoOwned>::Owned;

    fn add(self, other: Self) -> Self::Output {
        self.add(&other)
    }
}

impl<Order: Dim, N: Dim, Data> std::ops::Add<&Differential<Order, N, Data>> for Differential<Order, N, Data>
where
    Data: ConstStorage,
    Data::Owned: MutStorage<Item = Data::Item>,
    Data::Item: std::ops::AddAssign + Zero,
{
    type Output = <Self as IntoOwned>::Owned;

    fn add(self, other: &Self) -> Self::Output {
        let mut result = self.into_owned();
        result += other;
        result
    }
}


impl<Order: Dim, N: Dim, Data, Data2> std::ops::AddAssign<Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: MutStorage,
    Data2: ConstStorage,
    Data::Item: std::ops::AddAssign<Data2::Item> + Zero,
{
    fn add_assign(&mut self, other: Differential<Order, N, Data2>) {
        self.add_assign(&other)
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::AddAssign<&Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: MutStorage,
    Data2: ConstStorage,
    Data::Item: std::ops::AddAssign<Data2::Item> + Zero,
{
    fn add_assign(&mut self, other: &Differential<Order, N, Data2>) {
        if !other.is_shape_defined() {
            self.inner.data.slice_mut()[0] += other.data_slice()[0].clone();
            return;
        } else if !self.is_shape_defined() {
            if self.n().value().is_none() {
                self.define_n(other.n());
            }
            if self.order().value().is_none() {
                self.define_order(other.order());
            }
            assert!(self.is_shape_defined());
        }

        if self.order().value() == other.order().value() && self.n().value() == other.n().value() {
            let l = self.inner.data.slice_mut();
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