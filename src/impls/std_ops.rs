
use std::ops::{Div, Mul, Sub};

use num_traits::Zero;

use super::*;

impl<T> PartialEq for Differential<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> PartialOrd for Differential<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> std::ops::Add for Differential<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.value + other.value,
            self.derivative + other.derivative,
        )
    }
}

impl<T> std::ops::Add<Differential<T>> for f64
where
    Differential<T>: std::ops::Add<Output = Differential<T>>,
    T: From<f64>,
{
    type Output = Differential<T>;

    fn add(self, other: Differential<T>) -> Differential<T> {
        Differential::new(self.into(), 0.0.into()) + other
    }
}

impl<T> std::ops::AddAssign for Differential<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T> std::ops::Neg for Differential<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(
            -self.value,
            -self.derivative,
        )
    }
}

impl<T> std::ops::Sub for Differential<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.value - other.value,
            self.derivative - other.derivative,
        )
    }
}

impl<T> std::ops::Sub<Differential<T>> for f64
where
    Differential<T>: std::ops::Sub<Output = Differential<T>>,
    T: From<f64>,
{
    type Output = Differential<T>;

    fn sub(self, other: Differential<T>) -> Differential<T> {
        Differential::new(self.into(), 0.0.into()) - other
    }
}

impl<T> std::ops::SubAssign for Differential<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T> std::ops::Mul for Differential<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.value * other.value,
            self.value * other.derivative + self.derivative * other.value,
        )
    }
}

impl<T> std::ops::MulAssign for Differential<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<T> std::ops::Div for Differential<T>
where
    T: Div<Output = T> + Zero + PartialEq + Mul<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.value == T::zero() && other.value == T::zero() {
            return Self::new(
                self.derivative / other.derivative,
                T::zero() // TODO correct??
            )
        }
        Self::new(
            self.value / other.value,
            (self.derivative * other.value - self.value * other.derivative) / (other.value * other.value)
        )
    }
}

impl<T> std::ops::DivAssign for Differential<T>
where
    T: Div<Output = T> + Zero + PartialEq + Mul<Output = T> + Sub<Output = T> + Copy,
{
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl<T> std::ops::Rem for Differential<T>
where
    T: std::ops::Rem<Output = T> + Div<Output = T> + Copy + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn rem(self, _other: Self) -> Self {
        // TODO to check, also is it correct for negative values?
        let rem = self.value % _other.value;
        let i_div = (self.value - rem) / _other.value;
        Self::new(
            rem,
            self.derivative - i_div * _other.derivative,
        )
    }
}

impl<T> std::ops::RemAssign for Differential<T>
where
    T: std::ops::Rem<Output = T> + Div<Output = T> + Copy + Sub<Output = T> + Mul<Output = T>,
{
    fn rem_assign(&mut self, _other: Self) {
        *self = *self % _other;
    }
}