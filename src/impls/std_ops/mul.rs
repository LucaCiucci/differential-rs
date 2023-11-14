use super::*;

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