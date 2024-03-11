use std::{hint::black_box, time::{Duration, Instant}};

use differential::{layout::number_of_elements, *};

fn main() {
    let compute_time = Duration::from_secs(1);

    let mut seconds_per_iteration = Vec::new();

    for i in 1..=15 {
        let order = 2usize.pow(i);
        println!("i: {i}, order: {order}");
        let diff = Differential::from_data(
            Dynamic(Some(order)),
            Fixed::<1>,
            [2.0, 1.0].into_iter().chain(std::iter::repeat(0.0)).take(number_of_elements(1, order)).collect::<Vec<_>>(),
        );
        let start = Instant::now();
        let mut iterations = 0u64;
        while start.elapsed() < compute_time {
            black_box(exp(black_box(&diff)));
            iterations += 1;
        }
        let s = start.elapsed().as_secs_f64() / iterations as f64;
        println!("done with {iterations} iterations");
        seconds_per_iteration.push((order, s));
        plot_time_vs_order(seconds_per_iteration.clone());
    }
}

fn exp<Order: Dim, N: Dim, S: MutStorage<Item = f64> + Clone + Owned>(x: &Differential<Order, N, S>) -> Differential<Order, N, S>
{
    //return x.clone() * x;
    if x.order().value().unwrap() == 0 {
        Differential::new_constant(x.value().exp())
    } else {
        let exp_rec = exp(&x.drop_one_order().into_owned());

        let derivative = x.derivatives() * &exp_rec;
        Differential::from_data(
            x.order(),
            x.n(),
            S::from_iter(std::iter::once(*exp_rec.value()).chain(derivative.data.make_into_iter())),
        )
    }
}

fn plot_time_vs_order(
    times: Vec<(usize, f64)>,
) {
    use plotters::prelude::*;

    let root = SVGBackend::new("doc/exp_time_vs_order.svg", (400, 300)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let min_time = times.iter().map(|(_, t)| *t).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_time = times.iter().map(|(_, t)| *t).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_iter = times.iter().map(|(i, _)| *i).min().unwrap();
    let max_iter = times.iter().map(|(i, _)| *i).max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Time per evaluation vs order", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d((min_iter as f64..max_iter as f64).log_scale(), (min_time..max_time).log_scale())
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("order")
        .y_desc("Time per evaluation (s)")
        .draw()
        .unwrap();

    chart.draw_series(LineSeries::new(
        times.iter().map(|(i, t)| (*i as f64, *t)),
        &RED,
    )).unwrap();

    // also draw the points
    chart.draw_series(
        times.iter().map(|(i, t)| {
            Circle::new((*i as f64, *t), 2, BLUE.filled())
        })
    ).unwrap();
}