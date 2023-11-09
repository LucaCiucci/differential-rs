
use std::ops::{Div, Mul, Sub};

use num_traits::Zero;

use super::*;

impl<T, D> PartialEq for Differential<T, D>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, D> PartialOrd for Differential<T, D>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, D> std::ops::Add for Differential<T, D>
where
    T: std::ops::Add<Output = T>,
    D: std::ops::Add<Output = D>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.value + other.value,
            self.derivative + other.derivative,
        )
    }
}

impl<T, D> std::ops::Add<Differential<T, D>> for f64
where
    Differential<T, D>: std::ops::Add<Output = Differential<T, D>>,
    T: From<f64>,
    D: Zero,
{
    type Output = Differential<T, D>;

    fn add(self, other: Differential<T, D>) -> Differential<T, D> {
        Differential::<T, D>::new(self.into(), D::zero()) + other
    }
}

impl<T, D> std::ops::AddAssign for Differential<T, D>
where
    T: std::ops::AddAssign,
    D: std::ops::AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
        self.derivative += other.derivative;
    }
}

impl<T, D> std::ops::Neg for Differential<T, D>
where
    T: std::ops::Neg<Output = T>,
    D: std::ops::Neg<Output = D>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(
            -self.value,
            -self.derivative,
        )
    }
}

impl<T, D> std::ops::Sub for Differential<T, D>
where
    T: std::ops::Sub<Output = T>,
    D: std::ops::Sub<Output = D>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.value - other.value,
            self.derivative - other.derivative,
        )
    }
}

impl<T, D> std::ops::Sub<Differential<T, D>> for f64
where
    Differential<T, D>: std::ops::Sub<Output = Differential<T, D>>,
    T: From<f64>,
    D: Zero,
{
    type Output = Differential<T, D>;

    fn sub(self, other: Differential<T, D>) -> Differential<T, D> {
        Differential::<T, D>::new(self.into(), D::zero()) - other
    }
}

impl<T, D> std::ops::SubAssign for Differential<T, D>
where
    T: std::ops::SubAssign,
    D: std::ops::SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
        self.derivative -= other.derivative;
    }
}

impl<T, D> std::ops::Mul for Differential<T, D>
where
    T: std::ops::Mul<Output = T> + Clone,
    D: std::ops::Mul<T, Output = D> + std::ops::Add<Output = D> + Clone,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.value.clone() * other.value.clone(),
            other.derivative * self.value + self.derivative * other.value,
        )
    }
}

impl<T, D> std::ops::Mul<T> for Differential<T, D>
where
    T: std::ops::Mul<Output = T> + Clone,
    D: std::ops::Mul<T, Output = D>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.value * rhs.clone(),
            self.derivative * rhs,
        )
    }
}

impl<T, D> std::ops::MulAssign for Differential<T, D>
where
    Self: std::ops::Mul<Output = Self> + Clone, // TODO without Clone
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T, D> std::ops::Div for Differential<T, D>
where
    T: Div<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone, // TODO without Clone
    D: Zero + Mul<T, Output = D> + Sub<Output = D> + Div<T, Output = D>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(
            self.value.clone() / other.value.clone(),
            (self.derivative * other.value.clone() - other.derivative * self.value.clone()) / (other.value.clone() * other.value)
        )
    }
}

impl<T, D> std::ops::DivAssign for Differential<T, D>
where
    Self: std::ops::Div<Output = Self> + Clone, // TODO without Clone
{
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

impl<T, D> std::ops::Rem for Differential<T, D>
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
            self.derivative - _other.derivative * i_div,
        )
    }
}

impl<T, D> std::ops::RemAssign for Differential<T, D>
where
    Self: std::ops::Rem<Output = Self> + Clone, // TODO without Clone
{
    fn rem_assign(&mut self, _other: Self) {
        *self = self.clone() % _other;
    }
}