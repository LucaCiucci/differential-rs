/*!
Provides some differentiation utilities.
*/

#![cfg_attr(feature = "generic_const_exprs", feature(generic_const_exprs))]
//#![feature(generic_const_exprs)]

use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::{Index, IndexMut, MulAssign, DivAssign, AddAssign, SubAssign};

use num_traits::real::Real;
use num_traits::NumCast;

mod impls;


mod containers; pub use containers::*;
mod dim; pub use dim::*;
mod derivatives; pub use derivatives::*;
mod utils; use utils::*;

#[derive(Debug, Clone, Copy)]
pub struct Diff<Order: Dim, N: Dim, Data>
where
    Data: ContiguousContainer,
{
    order: Order,
    n: N,
    data: Data,
}

impl<ORDER: Dim, N: Dim, Data> Diff<ORDER, N, Data>
where
    Data: ContiguousContainer,
{
    pub fn from_data(order: ORDER, n: N, data: Data) -> Self {
        assert!(data.slice().len() >= number_of_elements(n.value(), order.value()));
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

    pub fn value(&self) -> &Data::Item {
        &self.data.slice()[0]
    }

    pub fn data_slice(&self) -> &[Data::Item] {
        self.data.slice()
    }

    pub fn value_mut(&mut self) -> &mut Data::Item
    where
        Data: ContiguousContainerMut,
    {
        &mut self.data.slice_mut()[0]
    }

    pub fn derivatives(&self) -> Derivatives<Dynamic, Dynamic, &[Data::Item]>
    {
        Derivatives::<Dynamic, Dynamic, &[Data::Item]>::new(
            Dynamic(self.order()),
            Dynamic(self.n()),
            &self.data.slice()[1..]
        )
    }

    pub fn drop_one_order(&self) -> Diff<Dynamic, Dynamic, Cow<[Data::Item]>> {
        assert!(self.order() > 0);
        if self.n() == 1 {
            Diff::from_data(
                Dynamic(self.order() - 1),
                Dynamic(1),
                self.data.slice()[0..self.order()].into()
            )
        } else {
            if self.order() == 1 {
                Diff::from_data(
                    Dynamic(0),
                    Dynamic(self.n()),
                    self.data.slice()[0..1].into()
                )
            } else {
                let derivatives = self.derivatives();

                // TODO extremely inefficient, multiple allocations
                // this might be solved by using drop_one_order that accepts a visitor
                // instead of returning a new Diff
                let data = std::iter::once(self.data.slice()[0].clone())
                    .chain((0..self.n())
                        .rev()
                        .map(|i| {
                            let d = derivatives.get(i);
                            let d = d.drop_one_order();
                            Cow::into_owned(d.data).into_iter()
                        })
                        .flatten()
                    )
                    .collect::<Vec<_>>();

                Diff::from_data(
                    Dynamic(self.order() - 1),
                    Dynamic(self.n()),
                    data.into()
                )
            }
        }
    }

    pub fn drop_first_derivatives(
        &self,
        offset: usize,
    ) -> Diff<Dynamic, Dynamic, &[Data::Item]> {
        Diff::from_data(
            Dynamic(self.order()),
            Dynamic(self.n() - offset),
            &self.data.slice(), // TODO <- correct range
        )
    }

    pub fn as_dynamic(&self) -> Diff<Dynamic, Dynamic, &[Data::Item]> {
        Diff::from_data(
            Dynamic(self.order()),
            Dynamic(self.n()),
            self.data.slice()
        )
    }

    pub fn polynomial_coeffs(self) -> Data::Owned
    where
        Data::Item: Real + MulAssign,
    {
        assert!(self.n() == 1);
        let mut divider = <Data::Item as NumCast>::from(1).unwrap();
        self.data.map_into_owned(|c, i| {
            divider *= <Data::Item as NumCast>::from(i.max(1)).unwrap();
            let r = c / divider;
            r
        })
    }

    pub fn from_polynomial_coeffs(data: Data, order: ORDER, n: N) -> Diff<ORDER, N, Data::Owned>
    where
        Data::Owned: ContiguousContainer,
        Data::Item: Real + MulAssign,
    {
        assert!(n.value() == 1);
        let mut multiplier = <Data::Item as NumCast>::from(1).unwrap();
        let data = data.map_into_owned(|c, i| {
            multiplier *= <Data::Item as NumCast>::from(i.max(1)).unwrap();
            let r = c * multiplier;
            r
        });
        Diff::from_data(order, n, data)
    }

    pub fn add_value(&mut self, rhs: Data::Item)
    where
        Data: ContiguousContainerMut,
        Data::Item: AddAssign,
    {
        self.data.slice_mut()[0] += rhs;
    }

    pub fn with_added_value(mut self, rhs: Data::Item) -> Self
    where
        Data: ContiguousContainerMut,
        Data::Item: AddAssign,
    {
        self.add_value(rhs);
        self
    }

    pub fn sub_value(&mut self, rhs: Data::Item)
    where
        Data: ContiguousContainerMut,
        Data::Item: SubAssign,
    {
        self.data.slice_mut()[0] -= rhs;
    }

    pub fn with_subbed_value(mut self, rhs: Data::Item) -> Self
    where
        Data: ContiguousContainerMut,
        Data::Item: SubAssign,
    {
        self.sub_value(rhs);
        self
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

#[cfg(feature = "generic_const_exprs")]
impl<const ORDER: usize, const N: usize, Data> Diff<Fixed<ORDER>, Fixed<N>, Data>
where
    Data: ContiguousContainer,
{
    pub fn fixed_derivatives(&self) -> Derivatives<Fixed<ORDER>, Fixed<N>, &[Data::Item; number_of_elements(N, ORDER) - 1]>
    where
        [(); number_of_elements(N, ORDER) - 1]: ,
    {
        Derivatives::<Fixed<ORDER>, Fixed<N>, &[Data::Item; number_of_elements(N, ORDER) - 1]>::new(Fixed, Fixed, self.data.slice()[1..].try_into().unwrap())
    }
}

impl<ORDER: Dim, N: Dim, Data> Index<&[usize]> for Diff<ORDER, N, Data>
where
    Data: ContiguousContainer,
{
    type Output = Data::Item;

    fn index(&self, index: &[usize]) -> &Self::Output {
        let offset = offset_of(index, self.n(), self.order());
        &self.data.slice()[offset]
    }
}

impl<ORDER: Dim, N: Dim, Data> IndexMut<&[usize]> for Diff<ORDER, N, Data>
where
    Data: ContiguousContainerMut,
{
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let offset = offset_of(index, self.n(), self.order());
        &mut self.data.slice_mut()[offset]
    }
}

impl<const ORDER: usize, const N: usize, Data> Index<&[usize; N]> for Diff<Fixed<ORDER>, Fixed<N>, Data>
where
    Data: ContiguousContainer,
{
    type Output = Data::Item;
    fn index(&self, index: &[usize; N]) -> &Self::Output {
        let offset = offset_of(index, N, ORDER);
        &self.data.slice()[offset]
    }
}

impl<const ORDER: usize, const N: usize, Data> IndexMut<&[usize; N]> for Diff<Fixed<ORDER>, Fixed<N>, Data>
where
    Data: ContiguousContainerMut,
{
    fn index_mut(&mut self, index: &[usize; N]) -> &mut Self::Output {
        let offset = offset_of(index, N, ORDER);
        &mut self.data.slice_mut()[offset]
    }
}

impl<Order: Dim, N: Dim, Data> IntoOwned for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Owned: ContiguousContainer,
{
    type Owned = Diff<Order, N, Data::Owned>;

    fn into_owned(self) -> Self::Owned {
        Self::Owned::from_data(self.order, self.n, self.data.into_owned())
    }
}

// TODO
//impl<Order: Dim, N: Dim, Data> ContiguousContainer for Diff<Order, N, Data>
//where
//    Data: ContiguousContainer,
//    Data::Owned: ContiguousContainer,
//{
//    type Item = Data::Item;
//
//    fn slice(&self) -> &[Self::Item] {
//        self.data.slice()
//    }
//
//    fn map_into_owned(self, f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
//        Self::Owned::from_data(
//            self.order,
//            self.n,
//            self.data.map_into_owned(f)
//        )
//    }
//
//    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
//        Self::Owned::from_data(
//            self.o,
//            N::from_usize(len).unwrap(),
//            Data::Owned::owned_from_fn(len, f),
//        )
//    }
//}