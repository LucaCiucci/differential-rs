use std::ops::Mul;

use super::*;

impl<Order: Dim, N: Dim, Data, Data2> Mul<Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + AddAssign + Real + MulAssign,
{
    type Output = Differential<Order, N, Data::Owned>;

    fn mul(self, other: Differential<Order, N, Data2>) -> Self::Output {
        if !self.is_shape_defined() && !other.is_shape_defined() {
            let mut result = self.into_owned();
            result.data.slice_mut()[0] *= other.data_slice()[0];
            return result;
        } else if !self.is_shape_defined() {
            if let Some(n) = self.n().value() {
                assert_eq!(n, other.n().value().unwrap());
            }
            if let Some(order) = self.order().value() {
                assert_eq!(order, other.order().value().unwrap());
            }
            let mut result = other.into_owned();
            let data = result.data.slice_mut();
            for c in data {
                *c *= self.data_slice()[0];
            }
            return result;
        } else if !other.is_shape_defined() {
            if let Some(n) = self.n().value() {
                assert_eq!(n, self.n().value().unwrap());
            }
            if let Some(order) = self.order().value() {
                assert_eq!(order, self.order().value().unwrap());
            }
            let mut result = self.into_owned();
            let data = result.data.slice_mut();
            for c in data {
                *c *= other.data_slice()[0];
            }
            return result;
        }

        let n = self.n().value().unwrap();
        let other_n = other.n().value().unwrap();
        assert_eq!(n, other_n);

        let order = self.order().value().unwrap();
        let other_order = other.order().value().unwrap();

        if order == other_order && n == 1 {
            match (order, true) {
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
                    // TODO the following algorithm works even is the orders are different, remove the bound on the order
                    let self_n = self.n();
                    let self_order = self.order();
                    let lhs = self.polynomial_coeffs();
                    let lhs = lhs.slice();
                    let rhs = other.clone().polynomial_coeffs();
                    let rhs = rhs.slice();
                    let mut data = Data::owned_from_fn(number_of_elements(n, order), |_| Zero::zero());
                    let data_slice = data.slice_mut();
                    for i in 0..=order {
                        for j in 0..=(order - i) {
                            data_slice[i + j] += lhs[i].clone() * &rhs[j];
                        }
                    }
                    Self::Output::from_polynomial_coeffs(data, self_order, self_n)
                }
            }
        } else {
            let value = self.value().clone() * other.value();
            if order == 0 {
                Self::Output::from_data(self.order, self.n, Data::from_slice(&[value]))
            } else {
                let derivatives = self.derivatives() * &self.drop_one_order() + other.derivatives() * &other.drop_one_order();
                let data = std::iter::once(value)
                    .chain(derivatives.unwrap_data().make_into_iter());
                Self::Output::from_data(self.order, self.n, Data::from_iter(data)) // TODO <- optimize
            }
        }
    }
}

impl<Order: Dim, N: Dim, Data, Data2> Mul<&Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + AddAssign + Real + MulAssign,
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