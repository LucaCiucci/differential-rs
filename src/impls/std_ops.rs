
use num_traits::{Zero, NumCast, real::Real};

use super::*;

// TODO implementations are placed in separate files because of their
// length, maybe they will be moved here in the future after some refactoring
mod comp;
mod add;
mod sub;
mod mul;
mod div;

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