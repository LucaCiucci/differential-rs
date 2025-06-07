use differential::{layout::number_of_elements, *};

fn main() {
    let _diff = Differential::from_data(
        Fixed::<20>,
        Fixed::<1>,
        [4.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        //[4.0, 2.0, 0.0, 0.0, 0.0, 0.0],
    );
    let o = 3;
    let diff = Differential::from_data(
        Dynamic(Some(o)),
        Fixed::<2>,
        [4.23254903, 1.0].into_iter().chain((1..).map(|i| std::f64::consts::PI / i as f64)).take(number_of_elements(2, o)).collect::<Vec<_>>(),
    );
    let diff_size = std::mem::size_of_val(&diff);
    println!("diff_size = {:?}", diff_size / std::mem::size_of::<f64>());

    let s = babylon_sqrt(&diff, 1000);
    println!("sqrt = {:?}", s);
    let s = sqrt(&diff);
    println!("sqrt = {:?}", s);

    let s2 = sqrt(&diff.drop_one_order().into_owned());
    println!("sqrt2 = {:?}", s2);
}

fn babylon_sqrt<Order: Dim, N: Dim, S: MutStorage<Item = f64> + Clone + Owned>(x: &Differential<Order, N, S>, iterations: usize) -> Differential<Order, N, S>
{
    let mut diff: Differential<Order, N, S> = Differential::<Order, N, S>::new_constant(x.value().sqrt());
    for s in 0..iterations {
        let new = (diff.clone() + x.clone() / &diff).scaled_by_inv(2.0);
        if new.data_slice() == diff.data_slice() {
            println!("converged after {s} iterations");
            break;
        }
        diff = new;
    }
    diff
}

fn sqrt<Order: Dim, N: Dim, S: MutStorage<Item = f64> + Clone + Owned>(x: &Differential<Order, N, S>) -> Differential<Order, N, S>
{
    if x.order().value().unwrap() == 0 {
        Differential::new_constant(x.value().sqrt())
    } else {
        let sqrt_rec = sqrt(&x.drop_one_order().into_owned());

        let derivative = x.derivatives() * &(Differential::<_, _, S>::new_constant(0.5) / &sqrt_rec);
        Differential::from_data(
            x.order(),
            x.n(),
            S::from_iter(std::iter::once(*sqrt_rec.value()).chain(derivative.unwrap_inner().data.make_into_iter())),
        )
    }
}