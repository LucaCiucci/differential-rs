use approx::{AbsDiffEq, UlpsEq, RelativeEq};

use super::*;


impl<T, D> AbsDiffEq for Differential<T, D>
where
    T: AbsDiffEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_eq(&other.value, epsilon)
    }

    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_ne(&other.value, epsilon)
    }
}

impl<T, D> UlpsEq for Differential<T, D>
where
    T: UlpsEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.value.ulps_eq(&other.value, epsilon, max_ulps)
    }

    fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.value.ulps_ne(&other.value, epsilon, max_ulps)
    }
}

impl<T, D> RelativeEq for Differential<T, D>
where
    T: RelativeEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon)
            -> bool {
        self.value.relative_eq(&other.value, epsilon, max_relative)
    }

    fn relative_ne(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon)
            -> bool {
        self.value.relative_ne(&other.value, epsilon, max_relative)
    }
}