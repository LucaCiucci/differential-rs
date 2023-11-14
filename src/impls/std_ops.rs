
use num_traits::{Zero, NumCast, real::Real};

use super::*;

impl<Order: Dim, N: Dim, Data> PartialEq for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data_slice()[0] == other.data_slice()[0]
    }
}

impl<Order: Dim, N: Dim, Data> PartialOrd for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Item: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data_slice()[0].partial_cmp(&other.data_slice()[0])
    }
}


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

impl<Order: Dim, N: Dim, Data, Data2> std::ops::Mul<Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item> + Clone,
    Data::Owned: ContiguousContainerMut<Item = Data::Item>,
    Data2::Owned: ContiguousContainerMut<Item = Data::Item>,
    Data::Item: Zero + for <'a> std::ops::Mul<&'a Data::Item, Output = Data::Item> + std::ops::AddAssign + Real + std::ops::MulAssign,
{
    type Output = Diff<Order, N, Data::Owned>;

    fn mul(self, other: Diff<Order, N, Data2>) -> Self::Output {
        assert_eq!(self.n(), other.n());
        if self.order() == other.order() && self.n() == 1 {
            match (self.order(), true) {
                // TODO these specializations will speed up the code a lot in debug mode
                // but they slow down the code in release mode! Should I remove them for concistency
                // or enable them only in debug mode?
                (0, true) => {
                    let lhs = self.data_slice();
                    let rhs = other.data_slice();
                    Self::Output::from_data(
                        self.order,
                        self.n,
                        Data::Owned::from_slice(&[
                            lhs[0] * rhs[0],
                        ]),
                    )
                }
                (1, true) => {
                    let lhs = self.data_slice();
                    let rhs = other.data_slice();
                    Self::Output::from_data(
                        self.order,
                        self.n,
                        Data::Owned::from_slice(&[
                            lhs[0] * rhs[0],
                            lhs[1] * rhs[0] + lhs[0] * rhs[1],
                        ]),
                    )
                }
                (2, true) => {
                    let lhs = self.data_slice();
                    let rhs = other.data_slice();
                    Self::Output::from_data(
                        self.order,
                        self.n,
                        Data::Owned::from_slice(&[
                            lhs[0] * rhs[0],
                            lhs[1] * rhs[0] + lhs[0] * rhs[1],
                            lhs[2] * rhs[0] + lhs[1] * rhs[1] * <Data::Item as NumCast>::from(2).unwrap() + lhs[0] * rhs[2],
                        ]),
                    )
                }
                _ => {
                    let order = self.order;
                    let n = self.n;
                    let lhs = self.polynomial_coeffs();
                    let lhs = lhs.slice();
                    let rhs = other.clone().polynomial_coeffs();
                    let rhs = rhs.slice();
                    let mut data = Data::owned_from_fn(number_of_elements(n.value(), order.value()), |_| Zero::zero());
                    let data_slice = data.slice_mut();
                    for i in 0..=order.value() {
                        for j in 0..=(order.value() - i) {
                            data_slice[i + j] += lhs[i].clone() * &rhs[j];
                        }
                    }
                    Self::Output::from_polynomial_coeffs(data, order, n)
                }
            }
        } else {
            let value = self.value().clone() * other.value();
            if self.order() == 0 {
                Self::Output::from_data(self.order, self.n, Data::from_slice(&[value]))
            } else {
                let derivatives = self.derivatives() * &self.drop_one_order() + other.derivatives() * &other.drop_one_order();
                let data = std::iter::once(value)
                    .chain(derivatives.unwrap_data().into_iter())
                    .collect::<Vec<_>>();
                Self::Output::from_data(self.order, self.n, Data::from_slice(&data[..])) // TODO <- optimize
            }
        }
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::Mul<&Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item> + Clone,
    Data::Owned: ContiguousContainerMut<Item = Data::Item>,
    Data2::Owned: ContiguousContainerMut<Item = Data::Item>,
    Diff<Order, N, Data2>: Clone,
    Data::Item: Zero + for <'a> std::ops::Mul<&'a Data::Item, Output = Data::Item> + std::ops::AddAssign + Real + std::ops::MulAssign,
{
    type Output = Diff<Order, N, Data::Owned>;

    fn mul(self, other: &Diff<Order, N, Data2>) -> Self::Output {
        self.mul(other.clone())
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::MulAssign<&Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer, // TODO use mut to avoid clone
    Data2: ContiguousContainer<Item = Data::Item> + Clone,
    Self: for <'a> std::ops::Mul<&'a Diff<Order, N, Data2>, Output = Self> + Clone, // TODO without Clone
{
    fn mul_assign(&mut self, other: &Diff<Order, N, Data2>) {
        *self = self.clone() * other;
    }
}

/*
impl<Order: Dim, N: Dim, Data> std::ops::Div for Diff<Order, N, Data>
where
    T: Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone, // TODO without Clone
    D: Zero + Mul<Self, Output = D> + Sub<Output = D> + Div<Self, Output = D> + Clone,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(
            self.value.clone().0 / other.value.clone().0,
            (self.jacobian * other - other.jacobian * self) / (other * other/*TODO calcola un ordine non necessario di troppo qui */)
        )
    }
}

impl<Order: Dim, N: Dim, Data> std::ops::DivAssign for Diff<Order, N, Data>
where
    Self: std::ops::Div<Output = Self> + Clone, // TODO without Clone
{
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

impl<Order: Dim, N: Dim, Data> std::ops::Rem for Diff<Order, N, Data>
where
    T: std::ops::Rem<Output = T> + Div<Output = T> + Sub<Output = T> + Clone,
    D: std::ops::Mul<T, Output = D> + std::ops::Sub<Output = D>,
{
    type Output = Self;

    fn rem(self, _other: Self) -> Self {
        // TODO to check, also is it correct for negative values?
        let rem = self.value.clone() % _other.value.clone();
        let i_div = (self.value - rem.clone()) / _other.value;
        Self::new(
            rem,
            self.jacobian - _other.jacobian * i_div,
        )
    }
}

impl<Order: Dim, N: Dim, Data> std::ops::RemAssign for Diff<Order, N, Data>
where
    Self: std::ops::Rem<Output = Self> + Clone, // TODO without Clone
{
    fn rem_assign(&mut self, _other: Self) {
        *self = self.clone() % _other;
    }
}*/