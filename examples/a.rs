
use differential::Differential;
use nalgebra::Vector2;
use num_traits::real::Real;

fn main() {
    let d = Differential::new(1.0, Vector2::new(1.0, 1.0));

    println!("{:?}", d.sin());
}