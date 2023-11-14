
use super::*;

#[derive(Debug)]
pub struct Derivatives<ORDER: Dim, N: Dim, Data>
where
    Data: ContiguousContainer,
{
    order: ORDER,
    n: N,
    data: Data,
}

impl<ORDER: Dim, N: Dim, Data> Derivatives<ORDER, N, Data>
where
    Data: ContiguousContainer,
{
    pub fn new(order: ORDER, n: N, data: Data) -> Self {
        Self {
            order,
            n,
            data,
        }
    }

    pub fn order(&self) -> usize {
        self.order.value()
    }

    pub fn n(&self) -> usize {
        self.n.value()
    }

    pub fn data_slice(&self) -> &[Data::Item] {
        self.data.slice()
    }

    pub fn unwrap_data(self) -> Data {
        self.data
    }

    pub fn get<'s>(&'s self, i: usize) -> Diff<Dynamic, Dynamic, &'s [Data::Item]>
    {
        let offset = offset_under(self.n(), i, self.order());
        let data: &[Data::Item] = &self.data.slice()[offset..];
        Diff::<Dynamic, Dynamic, &'s [Data::Item]>::from_data(
            Dynamic(self.order() - 1),
            Dynamic(self.n() - i),
            data,
        )
    }

    pub fn scale_by(&mut self, rhs: Data::Item)
    where
        Data: ContiguousContainerMut,
        Data::Item: MulAssign,
    {
        for a in self.data.slice_mut().iter_mut() {
            *a *= rhs.clone();
        }
    }

    pub fn scaled_by(mut self, rhs: Data::Item) -> Self
    where
        Data: ContiguousContainerMut,
        Data::Item: MulAssign,
    {
        self.scale_by(rhs);
        self
    }

    pub fn scale_by_inv(&mut self, rhs: Data::Item)
    where
        Data: ContiguousContainerMut,
        Data::Item: DivAssign,
    {
        for a in self.data.slice_mut().iter_mut() {
            *a /= rhs.clone();
        }
    }

    pub fn scaled_by_inv(mut self, rhs: Data::Item) -> Self
    where
        Data: ContiguousContainerMut,
        Data::Item: DivAssign,
    {
        self.scale_by_inv(rhs);
        self
    }
}

impl<ORDER: Dim, N: Dim, Data> IntoOwned for Derivatives<ORDER, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainer,
{
    type Owned = Derivatives<ORDER, N, Data::Owned>;
    fn into_owned(self) -> Self::Owned {
        Derivatives::new(self.order, self.n, self.data.into_owned())
    }
}

#[cfg(feature = "generic_const_exprs")]
impl<const ORDER: usize, const N: usize, Data> Derivatives<Fixed<ORDER>, Fixed<N>, Data>
where
    Data: ContiguousContainer,
{
    pub fn fixed_get<'s, const I: usize>(&'s self) -> Diff<Fixed<{ORDER - 1}>, Fixed<{N - I}>, &'s [Data::Item; number_of_elements(N - I, ORDER - 1)]>
    where
        [Data::Item; number_of_elements(N - I, ORDER - 1)]: ,
    {
        let offset = offset_under(N, I, ORDER);
        let data: &[Data::Item; number_of_elements(N - I, ORDER - 1)] = self.data.slice()[offset..].try_into().unwrap();
        Diff::<Fixed<{ORDER - 1}>, Fixed<{N - I}>, &'s [Data::Item; number_of_elements(N - I, ORDER - 1)]>::from_data(Fixed, Fixed, data)
    }
}

impl<ORDER: Dim, N: Dim, Data, Data2> std::ops::Mul<&Diff<ORDER, N, Data2>> for Derivatives<ORDER, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item>,
    for <'a, 'b> Diff<Dynamic, Dynamic, &'a [Data::Item]>: std::ops::Mul<&'b Diff<Dynamic, Dynamic, &'b [Data::Item]>, Output = Diff<Dynamic, Dynamic, Vec<Data::Item>>>,
{
    type Output = Derivatives<ORDER, N, Vec<Data::Item>>;

    fn mul(self, rhs: &Diff<ORDER, N, Data2>) -> Self::Output {
        let rhs = rhs.as_dynamic();
        let data = (0..self.n())
            .rev()
            .map(|i| {
                let r = self.get(i) * &rhs.drop_first_derivatives(i);
                r.data.into_iter()
            })
            .flatten()
            .collect::<Vec<_>>();
        Derivatives::new(
            self.order,
            self.n,
            data
        )
    }
}

impl<ORDER: Dim, N: Dim, Data, Data2> std::ops::Add<Derivatives<ORDER, N, Data2>> for Derivatives<ORDER, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item>,
    Data::Item: std::ops::Add<Data2::Item, Output = Data::Item> + Clone,
{
    type Output = Derivatives<ORDER, N, Vec<Data::Item>>;

    fn add(self, rhs: Derivatives<ORDER, N, Data2>) -> Self::Output {
        let data = self.data.slice().iter()
            .zip(rhs.data.slice().iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect::<Vec<_>>();
        Derivatives::new(
            self.order,
            self.n,
            data
        )
    }
}

impl<ORDER: Dim, N: Dim, Data, Data2> std::ops::Sub<Derivatives<ORDER, N, Data2>> for Derivatives<ORDER, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item>,
    Data::Item: std::ops::Sub<Data2::Item, Output = Data::Item> + Clone,
{
    type Output = Derivatives<ORDER, N, Vec<Data::Item>>;

    fn sub(self, rhs: Derivatives<ORDER, N, Data2>) -> Self::Output {
        let data = self.data.slice().iter()
            .zip(rhs.data.slice().iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect::<Vec<_>>();
        Derivatives::new(
            self.order,
            self.n,
            data
        )
    }
}