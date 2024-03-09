/*!
Provides some differentiation utilities.
*/

#![cfg_attr(feature = "generic_const_exprs", feature(generic_const_exprs))]
//#![feature(generic_const_exprs)]

use std::borrow::Cow;
use std::fmt::Debug;
use std::ops::{Index, IndexMut, MulAssign, DivAssign, AddAssign, SubAssign};

use num_traits::real::Real;
use num_traits::{NumCast, Zero};

mod alias;
mod storage; pub use storage::*;
mod dim; pub use dim::*;
mod derivatives; pub use derivatives::*;
mod utils; use utils::*;
mod impls;
mod diff_index; pub use diff_index::*;

pub use alias::*;

/// A differential.
///
/// This struct represents the differential of a function.
#[derive(Debug, Clone, Copy)]
pub struct Differential<Order: Dim, N: Dim, Data>
where
    Data: ConstStorage,
{
    order: Order,
    n: N,
    data: Data,
}

impl<Order: Dim, N: Dim, Data> Differential<Order, N, Data>
where
    Data: ConstStorage,
{
    pub fn from_data(order: Order, n: N, data: Data) -> Self {
        assert!(data.slice().len() >= maybenumber_of_elements(n.value(), order.value()));
        Self {
            order,
            n,
            data,
        }
    }

    pub fn new_constant(order: Order, n: N, value: Data::Item) -> Self
    where
        Data::Item: Zero,
        Data: Owned,
    {
        Self::from_data(
            order,
            n,
            Data::from_order_0(value, || Zero::zero()),
        )
    }

    pub fn into_defined_order(mut self, order: Order) -> Self
    where
        Data: MutStorage,
        Data::Item: Zero,
    {
        let Some(new_order) = order.value() else {
            panic!("Cannot redefine the order of a differential to an undefined order");
        };

        let current_order = self.order().value();
        if let Some(current_order) = current_order {
            assert_eq!(current_order, new_order, "Cannot redefine the order of a differential to a different order");
        } else {
            self.order = order;
            if self.is_shape_defined() {
                let n = self.n().value().unwrap();
                let order = order.value().unwrap();
                let count = number_of_elements(n, order);
                self.data.assign_iter(1, std::iter::repeat_with(Zero::zero).take(count - 1));
            }
        }
        self
    }

    pub fn into_defined_n(mut self, n: N) -> Self
    where
        Data: MutStorage,
        Data::Item: Zero,
    {
        let Some(new_n) = n.value() else {
            panic!("Cannot redefine the n of a differential to an undefined n");
        };

        let current_n = self.n().value();
        if let Some(current_n) = current_n {
            assert_eq!(current_n, new_n, "Cannot redefine the n of a differential to a different n");
        } else {
            self.n = n;
            if self.is_shape_defined() {
                let n = n.value().unwrap();
                let order = self.order().value().unwrap();
                let count = number_of_elements(n, order);
                self.data.assign_iter(1, std::iter::repeat_with(Zero::zero).take(count - 1));
            }
        }
        self
    }

    pub fn order(&self) -> Order {
        self.order
    }

    pub fn n(&self) -> N {
        self.n
    }

    pub fn is_shape_defined(&self) -> bool {
        self.n.value().is_some() && self.order.value().is_some()
    }

    pub fn value(&self) -> &Data::Item {
        &self.data.slice()[0]
    }

    pub fn data_slice(&self) -> &[Data::Item] {
        self.data.slice()
    }

    pub fn value_mut(&mut self) -> &mut Data::Item
    where
        Data: MutStorage,
    {
        &mut self.data.slice_mut()[0]
    }

    pub fn derivatives(&self) -> Derivatives<Dynamic, N, &[Data::Item]> // TODO Derivatives<Dynamic, N, &[Data::Item]>
    {
        Derivatives::<Dynamic, N, &[Data::Item]>::new(
            Dynamic(self.order().value()),
            self.n(),
            &self.data.slice()[1..]
        )
    }

    pub fn drop_one_order(&self) -> Differential<Dynamic, N, Cow<[Data::Item]>> { // TODO Derivatives<Dynamic, N, &[Data::Item]>
        let n = self.n();
        let order = self.order();
        let (n, order) = match (n.value(), order.value()) {
            (Some(n), Some(order)) => (n, order),
            (_, Some(order)) => return Differential::from_data(
                Dynamic(Some(order - 1)),
                n,
                self.data.slice()[..1].into(), // TODO is this correct? maybe this?:
                //<&[Data::Item] as ConstStorage>::from_order_0(
                //    self.data.slice()[0].clone(),
                //    || unreachable!(),
                //).into(),
            ),
            _ => return Differential::from_data(
                Dynamic(None),
                n,
                self.data.slice()[..1].into(),
            ),
        };

        assert!(order > 0);
        if n == 1 {
            Differential::from_data(
                Dynamic(Some(order - 1)),
                self.n,
                self.data.slice()[0..order].into() // TODO correct? maybe -1?
            )
        } else {
            if order == 1 {
                Differential::from_data(
                    Dynamic(Some(0)),
                    self.n(),
                    self.data.slice()[0..1].into()
                )
            } else {
                let derivatives = self.derivatives();

                // TODO extremely inefficient, multiple allocations
                // this might be solved by using drop_one_order that accepts a visitor
                // instead of returning a new Diff
                let data = std::iter::once(self.data.slice()[0].clone())
                    .chain((0..n)
                        .rev()
                        .map(|i| {
                            let d = derivatives.get(i);
                            let d = d.drop_one_order();
                            Cow::into_owned(d.data).into_iter()
                        })
                        .flatten()
                    )
                    .collect::<Vec<_>>();

                Differential::from_data(
                    Dynamic(Some(order - 1)),
                    self.n(),
                    data.into()
                )
            }
        }
    }

    pub fn drop_first_derivatives(
        &self,
        offset: usize,
    ) -> Differential<Dynamic, Dynamic, &[Data::Item]> {
        match (self.n().value(), self.order().value()) {
            (Some(n), order) => Differential::from_data(
                Dynamic(order),
                Dynamic(Some(n - offset)),
                &self.data.slice(), // TODO <- correct range
            ),
            (None, order) => Differential::from_data(
                Dynamic(order),
                Dynamic(None),
                &self.data.slice(), // TODO <- correct range
            ),
        }
    }

    pub fn as_dynamic(&self) -> Differential<Dynamic, Dynamic, &[Data::Item]> {
        Differential::from_data(
            Dynamic(self.order().value()),
            Dynamic(self.n().value()),
            self.data.slice()
        )
    }

    pub fn polynomial_coeffs(self) -> Data::Owned
    where
        Data::Item: Real + MulAssign,
    {
        assert!(self.n().value() == Some(1)); // TODO correct???
        let mut divider = <Data::Item as NumCast>::from(1).unwrap();
        self.data.map_into_owned(|c, i| {
            divider *= <Data::Item as NumCast>::from(i.max(1)).unwrap();
            let r = c / divider;
            r
        })
    }

    pub fn from_polynomial_coeffs(data: Data, order: Order, n: N) -> Differential<Order, N, Data::Owned>
    where
        Data::Owned: ConstStorage,
        Data::Item: Real + MulAssign,
    {
        assert!(n.value() == Some(1)); // TODO correct???
        let mut multiplier = <Data::Item as NumCast>::from(1).unwrap();
        let data = data.map_into_owned(|c, i| {
            multiplier *= <Data::Item as NumCast>::from(i.max(1)).unwrap();
            let r = c * multiplier;
            r
        });
        Differential::from_data(order, n, data)
    }

    pub fn add_value(&mut self, rhs: Data::Item)
    where
        Data: MutStorage,
        Data::Item: AddAssign,
    {
        self.data.slice_mut()[0] += rhs;
    }

    pub fn with_added_value(mut self, rhs: Data::Item) -> Self
    where
        Data: MutStorage,
        Data::Item: AddAssign,
    {
        self.add_value(rhs);
        self
    }

    pub fn sub_value(&mut self, rhs: Data::Item)
    where
        Data: MutStorage,
        Data::Item: SubAssign,
    {
        self.data.slice_mut()[0] -= rhs;
    }

    pub fn with_subbed_value(mut self, rhs: Data::Item) -> Self
    where
        Data: MutStorage,
        Data::Item: SubAssign,
    {
        self.sub_value(rhs);
        self
    }

    pub fn scale_by(&mut self, rhs: Data::Item)
    where
        Data: MutStorage,
        Data::Item: MulAssign,
    {
        for a in self.data.slice_mut().iter_mut() {
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
        for a in self.data.slice_mut().iter_mut() {
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

#[cfg(feature = "generic_const_exprs")]
impl<const ORDER: usize, const N: usize, Data> Differential<Fixed<ORDER>, Fixed<N>, Data>
where
    Data: ConstStorage,
{
    pub fn fixed_derivatives(&self) -> Derivatives<Fixed<ORDER>, Fixed<N>, &[Data::Item; number_of_elements(N, ORDER) - 1]>
    where
        [(); number_of_elements(N, ORDER) - 1]: ,
    {
        Derivatives::<Fixed<ORDER>, Fixed<N>, &[Data::Item; number_of_elements(N, ORDER) - 1]>::new(Fixed, Fixed, self.data.slice()[1..].try_into().unwrap())
    }
}

impl<Order: Dim, N: Dim, Data, Idx: DiffIndex> Index<Idx> for Differential<Order, N, Data>
where
    Data: ConstStorage,
{
    type Output = Data::Item;

    fn index(&self, index: Idx) -> &Self::Output {
        let n = self.n().value();
        let order = self.order().value();
        if let (Some(n), Some(order)) = (n, order) {
            let offset = offset_of(index, n, order);
            &self.data.slice()[offset]
        } else {
            if index.into_orders().all(|o| o == 0) {
                &self.data.slice()[0]
            } else {
                panic!("Cannot get derivatives from a differential with undefined shape");
            }
        }
    }
}

impl<Order: Dim, N: Dim, Data, Idx: DiffIndex> IndexMut<Idx> for Differential<Order, N, Data>
where
    Data: MutStorage,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let n = self.n().value();
        let order = self.order().value();
        if let (Some(n), Some(order)) = (n, order) {
            let offset = offset_of(index, n, order);
            &mut self.data.slice_mut()[offset]
        } else {
            if index.into_orders().all(|o| o == 0) {
                &mut self.data.slice_mut()[0]
            } else {
                panic!("Cannot get derivatives from a differential with undefined shape");
            }
        }
    }
}

impl<Order: Dim, N: Dim, Data> IntoOwned for Differential<Order, N, Data>
where
    Data: ConstStorage,
    Data::Owned: ConstStorage,
{
    type Owned = Differential<Order, N, Data::Owned>;

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