/*!
Provides some differentiation utilities.
*/

use std::fmt::Debug;

use num_traits::Zero;

mod impls;

/// A (first order) differential
#[derive(Debug, Clone, Copy, Default)]
pub struct Differential<T = f64, D = T>
{
    /// The value of the function
    pub value: T,

    /// The derivative of the function
    pub derivative: D,
}

impl<T, D> Differential<T, D> {
    /// Creates a new differential with the given value and derivative
    pub fn new(value: T, derivative: D) -> Self {
        Self {
            value,
            derivative,
        }
    }
}

/*impl<T> Into<Differential<T>> for f64
where
    T: From<f64>,
{
    fn into(self) -> Differential<T> {
        Differential::new(self.into(), 0.0.into())
    }
}*/

impl<T, D> From<T> for Differential<T, D>
where
    D: Zero,
{
    fn from(x: T) -> Self {
        Differential::new(x, D::zero())
    }
}

impl<T, D> From<(T, D)> for Differential<T, D>
{
    fn from(value: (T, D)) -> Self {
        Differential::new(value.0, value.1)
    }
}


/// Computes the Jacobian matrix of a function f: R^n -> R^m
pub fn jacobian(f: impl Fn(&[Differential]) -> Vec<Differential>, params: &[f64]) -> nalgebra::DMatrix<f64> { // TODO move
    let n_params = params.len();
    let n_outputs = {
        let params: Vec<Differential> = params.iter().map(|x| (*x).into()).collect();
        f(&params).len()
    };
    let mut jacobian: nalgebra::DMatrix<f64> = nalgebra::DMatrix::zeros(n_outputs, n_params);
    for j in 0..n_params {
        let params: Vec<Differential> = params.iter().enumerate().map(|(i, x)| if i == j { Differential::new(*x, 1.0) } else { Differential::new(*x, 0.0) }).collect();
        let outputs = f(&params);
        for i in 0..n_outputs {
            jacobian[(i, j)] = outputs[i].derivative;
        }
    }
    jacobian
}

/// Converts the type into its differential form
pub trait IntoDifferentialForm {
    /// The output type
    type DifferentialForm;

    /// Converts the type into its differential form
    fn into_differential_form(self) -> Self::DifferentialForm;
}

/// Converts the type from its differential form
pub trait IntoRealForm {
    /// The output type
    type RealForm;

    /// Converts the type into its non-differential form
    fn into_real_form(self) -> Self::RealForm;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ops() {
        assert_eq!(Differential::new(1.0, 2.0) + Differential::new(3.0, 4.0), (4.0, 6.0).into());
        assert_eq!(Differential::new(1.0, 2.0) - Differential::new(3.0, 4.0), (-2.0, -2.0).into());
        assert_eq!(Differential::new(1.0, 2.0) * Differential::new(3.0, 4.0), (3.0, 10.0).into());
    }
}