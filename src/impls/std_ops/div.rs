use super::*;

impl<Order: Dim, N: Dim, Data, Data2> std::ops::Div<Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item> + Clone,
    Data::Owned: ContiguousContainerMut<Item = Data::Item> + Clone,
    Data2::Owned: ContiguousContainer<Item = Data::Item>,
    Data::Item: Zero + for <'a> std::ops::Div<&'a Data::Item, Output = Data::Item> + for <'a> std::ops::Mul<&'a Data::Item, Output = Data::Item> + std::ops::AddAssign + std::ops::SubAssign + Real + std::ops::MulAssign + std::ops::DivAssign,
{
    type Output = Diff<Order, N, Data::Owned>;

    fn div(self, other: Diff<Order, N, Data2>) -> Self::Output {
        assert_eq!(self.n(), other.n());
        if self.order() == other.order() && self.n() == 1 {
            let order = self.order;
            let n = self.n;
            let mut data = self.polynomial_coeffs().clone();
            let rhs = other.clone().polynomial_coeffs();
            let rhs = rhs.slice();
            // TODO the following algorithm works even is the orders are different, remove the bound on the order
            // this function perform a polynomial division, taking into account the proper order:
            /*
            let:
            1) A = this = a_0 + a_1 x + a_2 x^2 + ... + o(x^n_a)
            2) B = right = b_0 + b_1 x + b_2 x^2 + ... + o(x^n_b)
            3) supposing b_0 != 0
            where n_a and n_b are Order+1 and Order2+1 respectively

            then, to perforn the division, we use the following method
                                                                                                                A'
                                                                                                        ╔═══════════════╗
            A     a_0 + a_1 x + a_2 x^2 + ... + o(x^N_a)     (a_0/b_0)*B + ( A - (a_0/b_0)*B )     a_0    A - (a_0/b_0)*B
            ─── = ──────────────────────────────────────── = ─────────────────────────────────── =  ─── + ─────────────────
            B     b_0 + b_1 x + b_2 x^2 + ... + o(x^N_b)                   B                       b_0          B

            now A' is in the form: 0 + a'_1 x + a'_2 x^2 + ... + o(x^N), where N = min{n_a, n_b}
                                                                                                        A''
            thus we have:                                                                        ╔═════════════════╗
            A     a_0    A'   a_0      a'_1 + a'_2 * x + ... + o(x^(N-1))      a_0     a'_1       A' - (a'_0/b_0)*B
            ─── =  ─── + ─── = ─── + x ──────────────────────────────────── = ─── + x ────  + x ───────────────────
            B     b_0    B       b_0                     B                      b_0     b_0               B

            and A'' is now in the form 0 + a''_2 x + a''_3 x^2 + ... + o(x^(N - 2))

            we continue this way up to the end.

            */
            {
                let data = data.slice_mut();
                for i in 0..=order.value() {
                    // every cicle of this loop computes a coefficient
                    data[i] /= rhs[0];
                    let c = data[i];

                    // compute the rest (A', or A'', A''' ...)
                    for j in 1..=(order.value() - i) {
                        data[i + j] -= c * &rhs[j];
                    }
                }
            }
            Self::Output::from_polynomial_coeffs(data, order, n)
        } else {
            let value = self.value().clone() / other.value();
            if self.order() == 0 {
                Self::Output::from_data(self.order, self.n, Data::from_slice(&[value]))
            } else {
                // GENERAL CASE
                let derivatives = self.derivatives() * &self.drop_one_order() - other.derivatives() * &other.drop_one_order();
                let data = std::iter::once(value)
                    .chain(derivatives.unwrap_data().into_iter())
                    .collect::<Vec<_>>();
                Self::Output::from_data(self.order, self.n, Data::from_slice(&data[..])) // TODO <- optimize
            }
        }
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::Div<&Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data2: ContiguousContainer<Item = Data::Item> + Clone,
    Data::Owned: ContiguousContainerMut<Item = Data::Item> + Clone,
    Data2::Owned: ContiguousContainer<Item = Data::Item>,
    Data::Item: Zero + for <'a> std::ops::Div<&'a Data::Item, Output = Data::Item> + for <'a> std::ops::Mul<&'a Data::Item, Output = Data::Item> + std::ops::AddAssign + std::ops::SubAssign + Real + std::ops::MulAssign + std::ops::DivAssign,
{
    type Output = Diff<Order, N, Data::Owned>;

    fn div(self, other: &Diff<Order, N, Data2>) -> Self::Output {
        self.div(other.clone())
    }
}

impl<Order: Dim, N: Dim, Data, Data2> std::ops::DivAssign<&Diff<Order, N, Data2>> for Diff<Order, N, Data>
where
    Data: ContiguousContainer, // TODO use mut to avoid clone
    Data2: ContiguousContainer<Item = Data::Item> + Clone,
    Self: for <'a> std::ops::Div<&'a Diff<Order, N, Data2>, Output = Self> + Clone, // TODO without Clone
{
    fn div_assign(&mut self, other: &Diff<Order, N, Data2>) {
        *self = self.clone() / other;
    }
}