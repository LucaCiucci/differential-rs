
use std::ops::Neg;

use num_traits::{real::Real, NumCast, ToPrimitive, Zero, One, Num, Signed, NumOps};

use super::*;



impl<T, D> One for Differential<T, D>
where
    T: One,
    D: Zero,
    Self: std::ops::Mul<Output = Self>,
{
    fn one() -> Differential<T, D> {
        Differential::new(T::one(), D::zero())
    }
}

impl<T, D> Zero for Differential<T, D>
where
    T: Zero,
    D: Zero,
    Self: std::ops::Add<Output = Self>,
{
    fn zero() -> Differential<T, D> {
        Differential::new(T::zero(), D::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero() && self.derivative.is_zero()
    }
}

impl<T, D> Num for Differential<T, D>
where
    T: Num,
    D: Zero,
    Self: NumOps,
{
    type FromStrRadixErr = FormStrRadixErr;

    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!("from_str_radix is not implemented for Differential, yet.");
    }
}

type FormStrRadixErr = ();

impl<T, D> Signed for Differential<T, D>
where
    T: Neg<Output = T> + Num + PartialOrd + Signed,
    D: Neg<Output = D> + Zero,
    Self: NumOps + Clone,
{
    fn abs(&self) -> Self {
        if self.value < T::zero() {
            -self.clone()
        } else {
            self.clone()
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        if self.value < other.value {
            Self::zero()
        } else {
            self.clone() - other.clone()
        }
    }

    fn signum(&self) -> Self {
        Self::new(self.value.signum(), D::zero())
    }

    fn is_positive(&self) -> bool {
        self.value.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.value.is_negative()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    #[test]
    fn signed_abs() {
        type D = Differential<f64>;
        assert_eq!(D::new(1.0, 2.0).abs(), D::new(1.0, 2.0));
        assert_eq!(D::new(1.0, -2.0).abs(), D::new(1.0, -2.0));
        assert_eq!(D::new(-1.0, 2.0).abs(), D::new(1.0, -2.0));
        assert_eq!(D::new(-1.0, -2.0).abs(), D::new(1.0, 2.0));
        assert_eq!(D::new(0.0, 2.0).abs(), D::new(0.0, 2.0));
        assert_eq!(D::new(0.0, -2.0).abs(), D::new(0.0, -2.0));
        assert_eq!(D::new(0.0, 0.0).abs(), D::new(0.0, 0.0));
    }

    #[cfg(test)]
    #[test]
    fn signed_abs_sub() {
        type D = Differential<f64>;
        assert_eq!(
            D::new(1.0, 3.0).abs_sub(D::new(1.0, 2.0)),
            D::new(0.0, 1.0)
        );
        assert_eq!(
            D::new(1.0, 2.0).abs_sub(D::new(1.0, 3.0)),
            D::new(0.0, -1.0)
        );
        assert_eq!(
            D::new(1.0, 2.0).abs_sub(D::new(1.0, 2.0)),
            D::new(0.0, 0.0)
        );
        assert_eq!(
            D::new(1.0, 2.0).abs_sub(D::new(2.0, 2.0)),
            D::new(0.0, 0.0)
        );
        assert_eq!(
            D::new(2.0, 2.0).abs_sub(D::new(1.0, 2.0)),
            D::new(1.0, 0.0)
        );
        // TODO check, possibly more cases
    }

    #[test]
    fn signed_signum() {
        assert_eq!((&Differential::new(1.0, 2.0)).signum(), Differential::new(1.0, 0.0));
        assert_eq!((&Differential::new(-1.0, 2.0)).signum(), Differential::new(-1.0, 0.0));
        assert_eq!((&Differential::new(0.0, 2.0)).signum(), Differential::new(1.0, 0.0)); // <----
        assert_eq!((&Differential::new(1, 2)).signum(), Differential::new(1, 0));         //     |
        assert_eq!((&Differential::new(-1, 2)).signum(), Differential::new(-1, 0));       //     |
        assert_eq!((&Differential::new(0, 2)).signum(), Differential::new(0, 0));         // <----   note that for integers, signum is 0 for 0
    }

    #[cfg(test)]
    #[test]
    fn signed_is_positive() {
        assert_eq!((&Differential::new(1.0, 2.0)).is_positive(), true);
        assert_eq!((&Differential::new(-1.0, 2.0)).is_positive(), false);
        assert_eq!((&Differential::new(0.0, 2.0)).is_positive(), true); // <----
        assert_eq!((&Differential::new(1, 2)).is_positive(), true);     //     |
        assert_eq!((&Differential::new(-1, 2)).is_positive(), false);   //     |
        assert_eq!((&Differential::new(0, 2)).is_positive(), false);    // <----   note that for integers, 0 is not positive
    }

    #[cfg(test)]
    #[test]
    fn signed_is_negative() {
        assert_eq!((&Differential::new(1.0, 2.0)).is_negative(), false);
        assert_eq!((&Differential::new(-1.0, 2.0)).is_negative(), true);
        assert_eq!((&Differential::new(0.0, 2.0)).is_negative(), false); // <----
        assert_eq!((&Differential::new(1, 2)).is_negative(), false);     //     |
        assert_eq!((&Differential::new(-1, 2)).is_negative(), true);     //     |
        assert_eq!((&Differential::new(0, 2)).is_negative(), false);     // <----   note that for integers, 0 is not negative
    }
}

impl<T, D> ToPrimitive for Differential<T, D>
where
    T: ToPrimitive,
{
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }
}

impl<T, D> NumCast for Differential<T, D>
where
    T: ToPrimitive + NumCast,
    D: Zero,
{
    fn from<T2: ToPrimitive>(n: T2) -> Option<Self> {
        let f = n.to_f64()?;
        Some(Self::new(T::from(f).unwrap(), D::zero())) // TODO correct??? we are losing derivative and precision here!
    }
}

impl<T, D> Real for Differential<T, D>
where
    T: Real + NumCast,
    D: Neg<Output = D> + Zero + Copy + std::ops::Mul<T, Output = D> + std::ops::Div<T, Output = D> + std::ops::Sub<Output = D>, // TODO remove copy
    Self: NumOps,
{
    fn min_value() -> Self {
        Self::new(T::min_value(), D::zero())
    }

    fn min_positive_value() -> Self {
        Self::new(T::min_positive_value(), D::zero())
    }

    fn epsilon() -> Self {
        Self::new(T::epsilon(), D::zero())
    }

    fn max_value() -> Self {
        Self::new(T::max_value(), D::zero())
    }

    fn floor(self) -> Self {
        Self::new(self.value.floor(), D::zero())
    }

    fn ceil(self) -> Self {
        Self::new(self.value.ceil(), D::zero())
    }

    fn round(self) -> Self {
        Self::new(self.value.round(), D::zero())
    }

    fn trunc(self) -> Self {
        Self::new(self.value.trunc(), D::zero())
    }

    fn fract(self) -> Self {
        Self::new(self.value.fract(), D::zero())
    }

    fn abs(self) -> Self {
        Self::new(self.value.abs(), D::zero())
    }

    fn signum(self) -> Self {
        Self::new(self.value.signum(), D::zero())
    }

    fn is_sign_positive(self) -> bool {
        self.value.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.value.is_sign_negative()
    }

    fn mul_add(self, _a: Self, _b: Self) -> Self {
        todo!();
    }

    fn recip(self) -> Self {
        Self::one() / self
    }

    fn powi(self, n: i32) -> Self {
        Self::new(self.value.powi(n), self.derivative * self.value.powi(n - 1) * T::from::<i32>(n).unwrap()) // TODO remove unwrap somehow
    }

    fn powf(self, _n: Self) -> Self {
        todo!();
    }

    fn sqrt(self) -> Self {
        Self::new(self.value.sqrt(), self.derivative / (T::from(2).unwrap() * self.value.sqrt())) // TODO copilot, to check
    }

    fn exp(self) -> Self {
        Self::new(self.value.exp(), self.derivative * self.value.exp()) // TODO copilot, to check
    }

    fn exp2(self) -> Self {
        todo!();
    }

    fn ln(self) -> Self {
        Self::new(self.value.ln(), self.derivative / self.value) // TODO copilot, to check
    }

    fn log(self, _base: Self) -> Self {
        todo!();
    }

    fn log2(self) -> Self {
        todo!();
    }

    fn log10(self) -> Self {
        todo!();
    }

    fn to_degrees(self) -> Self {
        todo!();
    }

    fn to_radians(self) -> Self {
        todo!();
    }

    fn max(self, other: Self) -> Self {
        if self.value > other.value {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self {
        if self.value < other.value {
            self
        } else {
            other
        }
    }

    fn abs_sub(self, other: Self) -> Self {
        if self.value < other.value {
            Self::zero()
        } else {
            self - other
        }
    }

    fn cbrt(self) -> Self {
        todo!();
    }

    fn hypot(self, _other: Self) -> Self {
        todo!();
    }

    fn sin(self) -> Self {
        Self::new(
            self.value.sin(),
            self.derivative * self.value.cos(),
        )
    }

    fn cos(self) -> Self {
        Self::new(
            self.value.cos(),
            -self.derivative * self.value.sin(),
        )
    }

    fn tan(self) -> Self {
        Self::new(
            self.value.tan(),
            self.derivative / self.value.cos().powi(2), // TODO copilot, to check
        )
    }

    fn asin(self) -> Self {
        Self::new(
            self.value.asin(),
            self.derivative / (T::one() - self.value.powi(2)).sqrt(), // TODO copilot, to check
        )
    }

    fn acos(self) -> Self {
        Self::new(
            self.value.acos(),
            -self.derivative / (T::one() - self.value.powi(2)).sqrt(), // TODO copilot, to check
        )
    }

    fn atan(self) -> Self {
        Self::new(
            self.value.atan(),
            self.derivative / (T::one() + self.value.powi(2)), // TODO copilot, to check
        )
    }

    fn atan2(self, other: Self) -> Self {
        Self::new(
            self.value.atan2(other.value),
            (self.derivative * other.value - other.derivative * self.value) / (self.value.powi(2) + other.value.powi(2)), // TODO copilot, to check
        )
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.value.sin_cos();
        (
            Self::new(sin, self.derivative * cos),
            Self::new(cos, -self.derivative * sin),
        )
    }

    fn exp_m1(self) -> Self {
        self.exp() - Self::one()
    }

    fn ln_1p(self) -> Self {
        todo!();
    }

    fn sinh(self) -> Self {
        Self::new(
            self.value.sinh(),
            self.derivative * self.value.cosh(), // TODO copilot, to check
        )
    }

    fn cosh(self) -> Self {
        Self::new(
            self.value.cosh(),
            self.derivative * self.value.sinh(), // TODO copilot, to check
        )
    }

    fn tanh(self) -> Self {
        Self::new(
            self.value.tanh(),
            self.derivative / self.value.cosh().powi(2), // TODO copilot, to check
        )
    }

    fn asinh(self) -> Self {
        Self::new(
            self.value.asinh(),
            self.derivative / (self.value.powi(2) + T::one()).sqrt(), // TODO copilot, to check
        )
    }

    fn acosh(self) -> Self {
        Self::new(
            self.value.acosh(),
            self.derivative / (self.value.powi(2) - T::one()).sqrt(), // TODO copilot, to check
        )
    }

    fn atanh(self) -> Self {
        Self::new(
            self.value.atanh(),
            self.derivative / (T::one() - self.value.powi(2)), // TODO copilot, to check
        )
    }
}