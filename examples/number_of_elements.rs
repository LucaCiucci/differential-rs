use differential::utils::number_of_elements;
use num_traits::Pow;
use plotters::prelude::*;

fn naive_shape(n: usize, order: usize) -> usize {
    let mut sum = 0;
    for o in 0..=order {
        sum += n.pow(o as _);
    }
    sum
}

fn main() {
    let n = 2;
    let max_order = 30;

    let root = SVGBackend::new("doc/figs/number_of_elements.svg", (400, 300)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Number of elements vs order (N = 2)", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d((1..max_order).log_scale(), (0..n.pow(max_order) as _).log_scale())
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .max_light_lines(0)
        .x_desc("order")
        .y_desc("Number of elements")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (1..=max_order).map(|order| (order, number_of_elements(n, order as _) as _)),
            &BLUE,
        ))
        .unwrap()
        .label("Number of elements")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_series(
            (1..=max_order).map(|order| Circle::new((order, number_of_elements(n, order as _) as _), 4, BLUE),
        ))
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (1..=max_order).map(|order| (order, naive_shape(n, order as _) as _)),
            &BLACK,
        ))
        .unwrap()
        .label("Naive shape")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart
        .draw_series(
            (1..=max_order).map(|order| Circle::new((order, naive_shape(n, order as _) as _), 4, BLACK),
        ))
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (1..=max_order).map(|order| (order, n.pow(order) as _)),
            &RED,
        ))
        .unwrap()
        .label("N^order")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(
            (1..=max_order).map(|order| Circle::new((order, n.pow(order) as _), 4, RED),
        ))
        .unwrap();

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperLeft)
        .draw()
        .unwrap();
}

