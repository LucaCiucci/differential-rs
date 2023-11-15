use super::*;

/// Differential over as single variable.
//pub type Diff1D<const ORDER: usize, T = f64> = Differential<Fixed<1>, Fixed<ORDER>, [T; ORDER]>;

/// First order differential over as single variable.
//pub type Diff<T = f64> = Diff1D<1, T>;
pub type Diff1<T = f64> = Differential<Fixed<1>, Fixed<1>, [T; 2]>;

/// Second order differential over as single variable.
//pub type Diff2<T = f64> = Diff1D<2, T>;
pub type Diff2<T = f64> = Differential<Fixed<1>, Fixed<2>, [T; 2]>;

/// Dynamic differential.
pub type DynDiff<T = f64> = Differential<Dynamic, Dynamic, Vec<T>>;


//impl<const ORDER: usize, T> Diff1D<ORDER, T>
//where
//    T: Clone,
//{
//    pub fn from_fixed_orders_array(orders: [T; ORDER]) -> Self {
//        Self::from_data(Fixed, Fixed, orders)
//    }
//
//    pub fn from_fixed_orders(orders: &[T]) -> Self {
//        Self::from_data(
//            Fixed,
//            Fixed,
//            std::array::from_fn(|i| orders[i].clone())
//        )
//    }
//}

impl<T> Diff1<T>
where
    T: Clone
{
    pub fn val_deriv(
        value: T,
        derivative: T
    ) -> Self {
        Self::from_data(
            Fixed,
            Fixed,
            [value, derivative]
        )
    }
}

impl<T> From<(T, T)> for Diff1<T>
where
    T: Clone
{
    fn from((value, derivative): (T, T)) -> Self {
        Self::val_deriv(value, derivative)
    }
}