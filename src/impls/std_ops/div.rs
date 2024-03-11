use std::ops::{Div, Mul};

use super::*;

impl<Order: Dim, N: Dim, Data, Data2> Div<Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + Div<&'a Data::Item, Output = Data::Item> + AddAssign + Real + MulAssign + DivAssign + SubAssign,
{
    type Output = Differential<Order, N, Data::Owned>;

    fn div(self, other: Differential<Order, N, Data2>) -> Self::Output {
        let rhs = if !self.is_shape_defined() && !other.is_shape_defined() {
            let mut result = self.into_owned();
            result.data.slice_mut()[0] /= other.data_slice()[0];
            return result;
        } else if !self.is_shape_defined() {
            if let Some(n) = self.n().value() {
                assert_eq!(n, other.n().value().unwrap());
            }
            if let Some(order) = self.order().value() {
                assert_eq!(order, other.order().value().unwrap());
            }
            let mut rhs = self.into_owned();
            if rhs.n().value().is_none() {
                rhs.define_n(other.n());
            }
            if rhs.order().value().is_none() {
                rhs.define_order(other.order());
            }
            rhs
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
                *c /= other.data_slice()[0];
            }
            return result;
        } else {
            self.into_owned()
        };

        let n = rhs.n().value().unwrap();
        let other_n = other.n().value().unwrap();
        assert_eq!(n, other_n);

        let order = rhs.order().value().unwrap();
        let other_order = other.order().value().unwrap();

        if order == other_order && n == 1 {
            let self_n = rhs.n();
            let self_order = rhs.order();

            if order == 0 {
                let value = rhs.value().clone() / other.value();
                return Differential::new_constant(value);
            }

            let mut data = rhs.polynomial_coeffs().clone();
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
                for i in 0..=order {
                    // every cicle of this loop computes a coefficient
                    data[i] /= rhs[0];
                    let c = data[i];

                    // compute the rest (A', or A'', A''' ...)
                    for j in 1..=(order - i) {
                        data[i + j] -= c * &rhs[j];
                    }
                }
            }
            Self::Output::from_polynomial_coeffs(data, self_order, self_n)
        } else {
            let value = rhs.value().clone() / other.value();
            if order == 0 {
                Self::Output::from_data(rhs.order, rhs.n, Data::from_slice(&[value]))
            } else {
                // GENERAL CASE
                let derivatives = rhs.derivatives() * &rhs.drop_one_order() - other.derivatives() * &other.drop_one_order();
                let data = std::iter::once(value)
                    .chain(derivatives.unwrap_data().make_into_iter()); // TODO <- optimize
                Self::Output::from_data(rhs.order, rhs.n, Data::from_iter(data)) // TODO <- optimize
            }
        }
    }
}

impl<Order: Dim, N: Dim, Data, Data2> Div<&Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: ConstStorage + Clone,
    Data::Owned: MutStorage<Item = Data::Item> + Clone,
    Data2: ConstStorage<Item = Data::Item, Owned = Data::Owned> + Clone,
    for <'a> Data::Item: Zero + Mul<&'a Data::Item, Output = Data::Item> + Div<&'a Data::Item, Output = Data::Item> + AddAssign + Real + MulAssign + DivAssign + SubAssign,
{
    type Output = Differential<Order, N, Data::Owned>;

    fn div(self, other: &Differential<Order, N, Data2>) -> Self::Output {
        self.div(other.clone())
    }
}

impl<Order: Dim, N: Dim, Data, Data2> DivAssign<&Differential<Order, N, Data2>> for Differential<Order, N, Data>
where
    Data: ConstStorage, // TODO use mut to avoid clone
    Data2: ConstStorage<Item = Data::Item> + Clone,
    Self: for <'a> Div<&'a Differential<Order, N, Data2>, Output = Self> + Clone, // TODO without Clone
{
    fn div_assign(&mut self, other: &Differential<Order, N, Data2>) {
        *self = self.clone() / other;
    }
}