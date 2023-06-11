
use ::nalgebra::{Vector2, Vector3};
use num_traits::real::Real;

use super::*;

impl<T> IntoDifferentialForm for Vector2<T>
where
    T: Debug + Real + 'static,
{
    type DifferentialForm = Vector2<Differential<T>>;

    fn into_differential_form(self) -> Self::DifferentialForm {
        Vector2::new(self.x.into(), self.y.into())
    }
}

impl<T> IntoRealForm for Vector2<Differential<T>>
where
    T: Debug + Real + 'static,
{
    type RealForm = Vector2<T>;

    fn into_real_form(self) -> Self::RealForm {
        Vector2::new(self.x.value, self.y.value)
    }
}

impl<T> IntoDifferentialForm for Vector3<T>
where
    T: Debug + Real + 'static,
{
    type DifferentialForm = Vector3<Differential<T>>;

    fn into_differential_form(self) -> Self::DifferentialForm {
        Vector3::new(self.x.into(), self.y.into(), self.z.into())
    }
}

impl<T> IntoRealForm for Vector3<Differential<T>>
where
    T: Debug + Real + 'static,
{
    type RealForm = Vector3<T>;

    fn into_real_form(self) -> Self::RealForm {
        Vector3::new(self.x.value, self.y.value, self.z.value)
    }
}