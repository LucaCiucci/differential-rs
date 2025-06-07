use std::ops::{Add, Mul};

use super::*;

#[derive(Debug)]
pub struct Derivatives<Order: Dim, N: Dim, Data>
where
    Data: ConstStorage,
{
    inner: DiffInner<Order, N, Data>,
}

impl<Order: Dim, N: Dim, Data> Derivatives<Order, N, Data>
where
    Data: ConstStorage,
{
    pub fn new_from_inner(inner: DiffInner<Order, N, Data>) -> Self {
        Self { inner }
    }

    pub fn unwrap_inner(self) -> DiffInner<Order, N, Data> {
        self.inner
    }

    pub fn new(order: Order, n: N, data: Data) -> Self {
        Self::new_from_inner(DiffInner { order, n, data })
    }

    pub fn order(&self) -> Option<usize> { // TODO maybe return Order instead of Option<usize>?
        self.inner.order.value()
    }

    pub fn n(&self) -> Option<usize> { // TODO maybe return N instead of Option<usize>?
        self.inner.n.value()
    }

    pub fn data_slice(&self) -> &[Data::Item] {
        self.inner.data.slice()
    }

    pub fn get<'s>(&'s self, i: usize) -> Differential<Dynamic, Dynamic, StorageSlice<'s, Data>>
    {
        // TODO maybe this should be implemented also for undefined shape?d
        let n = self.n().expect("n is not known");
        let order = self.order().expect("order is not known");
        let offset = offset_under(n, i, order);
        let data: &[Data::Item] = &self.inner.data.slice()[offset..];
        Differential::from_data(
            Dynamic(Some(order - 1)),
            Dynamic(Some(n - i)),
            StorageSlice::new(data),
        )
    }

    pub fn scale_by(&mut self, rhs: Data::Item)
    where
        Data: MutStorage,
        Data::Item: MulAssign,
    {
        for a in self.inner.data.slice_mut().iter_mut() {
            *a *= rhs.clone();
        }
    }

    pub fn scaled_by(mut self, rhs: Data::Item) -> Self
    where
        Data: MutStorage,
        Data::Item: MulAssign,
    {
        self.scale_by(rhs);
        self
    }

    pub fn scale_by_inv(&mut self, rhs: Data::Item)
    where
        Data: MutStorage,
        Data::Item: DivAssign,
    {
        for a in self.inner.data.slice_mut().iter_mut() {
            *a /= rhs.clone();
        }
    }

    pub fn scaled_by_inv(mut self, rhs: Data::Item) -> Self
    where
        Data: MutStorage,
        Data::Item: DivAssign,
    {
        self.scale_by_inv(rhs);
        self
    }
}

impl<Order: Dim, N: Dim, Data> IntoOwned for Derivatives<Order, N, Data>
where
    Data: ConstStorage,
    Data::Owned: ConstStorage,
{
    type Owned = Derivatives<Order, N, Data::Owned>;
    fn into_owned(self) -> Self::Owned {
        Derivatives::new(self.inner.order, self.inner.n, self.inner.data.into_owned())
    }
}

#[cfg(feature = "generic_const_exprs")]
impl<const ORDER: usize, const N: usize, Data> Derivatives<Fixed<ORDER>, Fixed<N>, Data>
where
    Data: ConstStorage,
{
    pub fn fixed_get<'s, const I: usize>(&'s self) -> Differential<Fixed<{ORDER - 1}>, Fixed<{N - I}>, &'s [Data::Item; number_of_elements(N - I, ORDER - 1)]>
    where
        [Data::Item; number_of_elements(N - I, ORDER - 1)]: ,
    {
        let offset = offset_under(N, I, ORDER);
        let data: &[Data::Item; number_of_elements(N - I, ORDER - 1)] = self.data.slice()[offset..].try_into().unwrap();
        Differential::<Fixed<{ORDER - 1}>, Fixed<{N - I}>, &'s [Data::Item; number_of_elements(N - I, ORDER - 1)]>::from_data(Fixed, Fixed, data)
    }
}

impl<Order: Dim, N: Dim, Data, Data2> Mul<&Differential<Order, N, Data2>> for Derivatives<Order, N, Data>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + Mul<Data::Item, Output = Data::Item> + AddAssign + MulAssign + NumCast + Div<Data::Item, Output = Data::Item>,
{
    type Output = Derivatives<Order, N, Data::Owned>;

    fn mul(self, rhs: &Differential<Order, N, Data2>) -> Self::Output {
        // TODO maybe this should be implemented also for undefined shape?
        let n = self.n().expect("n is not known");
        let rhs = rhs.as_dynamic();
        let data = (0..n)
            .rev()
            .map(|i| {
                let r = self.get(i) * &rhs.drop_first_derivatives(i);
                r.inner.data.make_into_iter()
            })
            .flatten();
        Derivatives::new(
            self.inner.order,
            self.inner.n,
            Data::from_iter(data),
        )
    }
}

impl<Order: Dim, N: Dim, Data, Data2> Add<Derivatives<Order, N, Data2>> for Derivatives<Order, N, Data>
where
    Data: ConstStorage,
    Data::Owned: ConstStorage,
    Data2: ConstStorage<Item = Data::Item>,
    Data::Item: Add<Data2::Item, Output = Data::Item> + Clone,
{
    type Output = Derivatives<Order, N, Data::Owned>;

    fn add(self, rhs: Derivatives<Order, N, Data2>) -> Self::Output {
        let data = self.inner.data.slice().iter()
            .zip(rhs.inner.data.slice().iter())
            .map(|(a, b)| a.clone() + b.clone());
        Derivatives::new(
            self.inner.order,
            self.inner.n,
            Data::from_iter(data),
        )
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::Sub<Derivatives<Order, N, Data2>> for Derivatives<Order, N, Data>
where
    Data: ConstStorage,
    Data::Owned: ConstStorage,
    Data2: ConstStorage<Item = Data::Item>,
    Data::Item: std::ops::Sub<Data2::Item, Output = Data::Item> + Clone,
{
    type Output = Derivatives<Order, N, Data::Owned>;

    fn sub(self, rhs: Derivatives<Order, N, Data2>) -> Self::Output {
        let data = self.inner.data.slice().iter()
            .zip(rhs.inner.data.slice().iter())
            .map(|(a, b)| a.clone() - b.clone());
        Derivatives::new(
            self.inner.order,
            self.inner.n,
            Data::from_iter(data),
        )
    }
}