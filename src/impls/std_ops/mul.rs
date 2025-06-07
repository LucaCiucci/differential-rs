use std::ops::Mul;

use super::*;

impl<Order: Dim, N: Dim, Data, Data2> Mul<Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + Mul<Data::Item, Output = Data::Item> + AddAssign + MulAssign + NumCast + Div<Data::Item, Output = Data::Item>,
{
    type Output = Differential<Order, N, Data::Owned>;

    fn mul(self, other: Differential<Order, N, Data2>) -> Self::Output {
        if !self.is_shape_defined() || !other.is_shape_defined() {
            return undefined_shape_mul(self, other);
        }

        let n = self.n().value().unwrap();
        let other_n = other.n().value().unwrap();
        assert_eq!(n, other_n);

        let order = self.order().value().unwrap();
        let other_order = other.order().value().unwrap();

        if order == other_order && n == 1 {
            univariate_mul(self, &other)
        } else {
            nested_mul(&self, &other)
        }
    }
}

impl<Order: Dim, N: Dim, Data, Data2> Mul<&Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + Mul<Data::Item, Output = Data::Item> + AddAssign + MulAssign + NumCast + Div<Data::Item, Output = Data::Item>,
{
    type Output = Differential<Order, N, Data::Owned>;

    fn mul(self, other: &Differential<Order, N, Data2>) -> Self::Output {
        self.mul(other.clone())
    }
}

impl<Order: Dim, N: Dim, Data, Data2> MulAssign<&Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: ConstStorage, // TODO use mut to avoid clone
    Data2: ConstStorage<Item = Data::Item> + Clone,
    for <'a> Self: Mul<&'a Differential<Order, N, Data2>, Output = Self> + Clone, // TODO without Clone
{
    fn mul_assign(&mut self, other: &Differential<Order, N, Data2>) {
        *self = self.clone() * other;
    }
}

/// Implements @nested-mul
pub fn nested_mul<Order: Dim, N: Dim, Data, Data2>(
    lhs: &Differential<Order, N, Data>,
    rhs: &Differential<Order, N, Data2>,
) -> Differential<Order, N, Data::Owned>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + Mul<Data::Item, Output = Data::Item> + AddAssign + MulAssign + NumCast + Div<Data::Item, Output = Data::Item>,
{
    assert_eq!(lhs.n().value(), rhs.n().value());
    assert_eq!(lhs.order(), rhs.order());
    assert!(lhs.is_shape_defined() && rhs.is_shape_defined(), "The shape of the differentials must be defined");
    let order = lhs.order().value().unwrap();

    let value = lhs.value().clone() * rhs.value();
    if order == 0 {
        Differential::from_data(lhs.inner.order, lhs.inner.n, Data::from_slice(&[value]))
    } else {
        let derivatives = lhs.derivatives() * &rhs.drop_one_order() + rhs.derivatives() * &lhs.drop_one_order();
        let data = std::iter::once(value)
            .chain(derivatives.unwrap_inner().data.make_into_iter());
        Differential::from_data(lhs.inner.order, lhs.inner.n, Data::from_iter(data)) // TODO <- optimize
    }
}

pub fn univariate_mul<Order: Dim, N: Dim, Data, Data2>(
    lhs: Differential<Order, N, Data>,
    rhs: &Differential<Order, N, Data2>,
) -> Differential<Order, N, Data::Owned>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + Mul<Data::Item, Output = Data::Item> + AddAssign + MulAssign + NumCast + Div<Output = Data::Item>,
{
    assert_eq!(lhs.n(), rhs.n());
    assert_eq!(lhs.order(), rhs.order());
    assert!(lhs.is_shape_defined() && rhs.is_shape_defined(), "The shape of the differentials must be defined");
    let order = lhs.order().value().unwrap();

    match (order, true) {
        // TODO these specializations will speed up the code a lot in debug mode
        // but they slow down the code in release mode! Should I remove them for consistency
        // or enable them only in debug mode?
        (0, true) => {
            let lhs_slice = lhs.data_slice();
            let rhs_slice = rhs.data_slice();
            Differential::from_data(
                lhs.inner.order,
                lhs.inner.n,
                Data::Owned::from_slice(&[
                    lhs_slice[0].clone() * &rhs_slice[0],
                ]),
            )
        }
        (1, true) => {
            let lhs_slice = lhs.data_slice();
            let rhs_slice = rhs.data_slice();
            Differential::from_data(
                lhs.inner.order,
                lhs.inner.n,
                Data::Owned::from_slice(&[
                    lhs_slice[0].clone() * &rhs_slice[0],
                    lhs_slice[1].clone() * &rhs_slice[0] + lhs_slice[0].clone() * &rhs_slice[1],
                ]),
            )
        }
        (2, true) => {
            let lhs_slice = lhs.data_slice();
            let rhs_slice = rhs.data_slice();
            Differential::from_data(
                lhs.inner.order,
                lhs.inner.n,
                Data::Owned::from_slice(&[
                    lhs_slice[0].clone() * &rhs_slice[0],
                    lhs_slice[1].clone() * &rhs_slice[0] + lhs_slice[0].clone() * &rhs_slice[1],
                    lhs_slice[2].clone() * &rhs_slice[0] + lhs_slice[1].clone() * &rhs_slice[1] * &<Data::Item as NumCast>::from(2).unwrap() + lhs_slice[0].clone() * &rhs_slice[2],
                ]),
            )
        }
        _ => {
            // TODO the following algorithm works even is the orders are different, remove the bound on the order
            let self_n = lhs.n();
            let self_order = lhs.order();
            let lhs = lhs.polynomial_coeffs();
            let lhs = lhs.slice();
            let rhs = rhs.clone().polynomial_coeffs();
            let rhs = rhs.slice();
            let mut data = Data::owned_from_fn(number_of_elements(self_n.value().unwrap(), order), |_| Zero::zero());
            let data_slice = data.slice_mut();
            for i in 0..=order {
                for j in 0..=(order - i) {
                    data_slice[i + j] += lhs[i].clone() * &rhs[j];
                }
            }
            Differential::from_polynomial_coeffs(data, self_order, self_n)
        }
    }
}

pub fn undefined_shape_mul<Order: Dim, N: Dim, Data, Data2>(
    lhs: Differential<Order, N, Data>,
    rhs: Differential<Order, N, Data2>,
) -> Differential<Order, N, Data::Owned>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + AddAssign + MulAssign,
{
    if !lhs.is_shape_defined() && !rhs.is_shape_defined() {
        let mut result = lhs.into_owned();
        result.inner.data.slice_mut()[0] *= rhs.data_slice()[0].clone();
        result
    } else if !lhs.is_shape_defined() {
        if let Some(n) = lhs.n().value() {
            assert_eq!(n, rhs.n().value().unwrap());
        }
        if let Some(order) = lhs.order().value() {
            assert_eq!(order, rhs.order().value().unwrap());
        }
        let mut result = rhs.into_owned();
        let data = result.inner.data.slice_mut();
        for c in data {
            *c *= lhs.data_slice()[0].clone();
        }
        result
    } else if !rhs.is_shape_defined() {
        if let Some(n) = lhs.n().value() {
            assert_eq!(n, lhs.n().value().unwrap());
        }
        if let Some(order) = lhs.order().value() {
            assert_eq!(order, lhs.order().value().unwrap());
        }
        let mut result = lhs.into_owned();
        let data = result.inner.data.slice_mut();
        for c in data {
            *c *= rhs.data_slice()[0].clone();
        }
        result
    } else {
        panic!("cannot call this function with defined shapes");
    }
}