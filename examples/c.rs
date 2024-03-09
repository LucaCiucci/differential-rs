use std::{cell::Cell, hint::black_box, time::{Duration, Instant}};

use differential::*;
use type_map::TypeMap;


fn main() {
    let compute_time = Duration::from_secs(2);

    let mut seconds_per_iteration = Vec::new();

    for i in 1..=15 {
        let order = 2usize.pow(i);
        println!("i: {i}, order: {order}");
        let diff = Differential::from_data(
            Dynamic(Some(order)),
            Dynamic(Some(1)),
            MyStorage::from_iter([4.0, 1.0].into_iter().chain(std::iter::repeat(0.0)).take(utils::number_of_elements(1, order))),
        );
        let start = Instant::now();
        let mut iterations = 0u64;
        while start.elapsed() < compute_time {
            black_box(my_sqrt(black_box(&diff), 10));
            iterations += 1;
        }
        let s = start.elapsed().as_secs_f64() / iterations as f64;
        println!("done with {iterations} iterations");
        seconds_per_iteration.push((order, s));
        plot_time_vs_order(seconds_per_iteration.clone());
    }

    let mut seconds_per_iteration_2 = Vec::new();

    for i in 1..=20 {
        let n = 2usize.pow(i);
        println!("i: {i}, n: {n}");
        let diff = Differential::from_data(
            Dynamic(Some(1)),
            Dynamic(Some(n)),
            MyStorage::from_iter(std::iter::once(4.0).chain((0..10 * n).map(|i| i as f64)).chain(std::iter::repeat(0.0)).take(utils::number_of_elements(n, 1))),
        );
        let start = Instant::now();
        let mut iterations = 0u64;
        while start.elapsed() < compute_time {
            black_box(my_sqrt(black_box(&diff), 10));
            iterations += 1;
        }
        let s = start.elapsed().as_secs_f64() / iterations as f64;
        let allocations = ALLOCATIONS.with(|allocations| allocations.get());
        let cache_size = MyStorage::<f64>::cache_size();
        let in_use = IN_USE.with(|in_use| in_use.get());
        println!("done with {iterations} iterations {allocations} allocations {cache_size} cache_size {in_use} in_use");
        seconds_per_iteration_2.push((n, s));
        plot_time_vs_n(seconds_per_iteration_2.clone());
    }
}

fn my_sqrt<Order: Dim, N: Dim, S: MutStorage<Item = f64> + Clone + Owned>(x: &Differential<Order, N, S>, iterations: usize) -> Differential<Order, N, S>
{
    let mut diff: Differential<Order, N, S> = Differential::<Order, N, S>::new_constant(x.value().sqrt());
    for _ in 0..iterations {
        diff = (diff.clone() + x.clone() / &diff).scaled_by_inv(2.0);
    }
    diff
}

fn plot_time_vs_order(
    times: Vec<(usize, f64)>,
) {
    use plotters::prelude::*;

    let root = SVGBackend::new("doc/time_vs_order.svg", (400, 300)).into_drawing_area();

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

fn plot_time_vs_n(
    times: Vec<(usize, f64)>,
) {
    use plotters::prelude::*;

    let root = SVGBackend::new("doc/time_vs_n.svg", (400, 300)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let min_time = times.iter().map(|(_, t)| *t).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_time = times.iter().map(|(_, t)| *t).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_iter = times.iter().map(|(i, _)| *i).min().unwrap();
    let max_iter = times.iter().map(|(i, _)| *i).max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Time per evaluation vs n", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d((min_iter as f64..max_iter as f64).log_scale(), (min_time..max_time).log_scale())
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("n")
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

thread_local! {
    static IN_USE: Cell<usize> = Cell::new(0);
    static ALLOCATIONS: Cell<usize> = Cell::new(0);
    static CACHE: std::cell::UnsafeCell<TypeMap> = std::cell::UnsafeCell::new(TypeMap::new());
}

struct MyStorage<T: 'static> {
    data: Vec<T>,
}

impl<T: 'static> MyStorage<T> {
    pub fn acquire_storage() -> Vec<T> {
        // increment
        IN_USE.with(|in_use| in_use.set(in_use.get() + 1));
        CACHE.with(|cache| unsafe {
            let cache_map: &mut TypeMap = &mut *cache.get();
            let cache: &mut Vec<Vec<T>> = if cache_map.contains::<Vec<Vec<T>>>() {
                cache_map.get_mut::<Vec<Vec<T>>>().unwrap()
            } else {
                cache_map.insert::<Vec<Vec<T>>>(Vec::new());
                cache_map.get_mut::<Vec<Vec<T>>>().unwrap()
            };
            cache.pop().unwrap_or_else(|| {
                if cache.len() > 0 {
                    panic!("cache is not empty")
                }
                ALLOCATIONS.with(|allocations| allocations.set(allocations.get() + 1));
                Vec::new()
            })
        })
    }

    pub fn return_storage(data: Vec<T>) {
        IN_USE.with(|in_use| in_use.set(in_use.get() - 1));
        CACHE.with(|cache| unsafe {
            let cache_map: &mut TypeMap = &mut *cache.get();
            let cache: &mut Vec<Vec<T>> = if cache_map.contains::<Vec<Vec<T>>>() {
                cache_map.get_mut::<Vec<Vec<T>>>().unwrap()
            } else {
                cache_map.insert::<Vec<Vec<T>>>(Vec::new());
                cache_map.get_mut::<Vec<Vec<T>>>().unwrap()
            };
            cache.push(data);
        })
    }

    pub fn cache_size() -> usize {
        CACHE.with(|cache| unsafe {
            let cache_map: &mut TypeMap = &mut *cache.get();
            let cache: &mut Vec<Vec<T>> = if cache_map.contains::<Vec<Vec<T>>>() {
                cache_map.get_mut::<Vec<Vec<T>>>().unwrap()
            } else {
                cache_map.insert::<Vec<Vec<T>>>(Vec::new());
                cache_map.get_mut::<Vec<Vec<T>>>().unwrap()
            };
            cache.len()
        })
    }
}

impl<T: 'static> IntoOwned for MyStorage<T> {
    type Owned = Self;
    fn into_owned(self) -> Self::Owned {
        self
    }
}

impl<T: Clone + 'static> ConstStorage for MyStorage<T> {
    type Item = T;
    fn slice(&self) -> &[Self::Item] {
        &self.data
    }
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned {
        let mut data = MyStorage::<T>::acquire_storage();
        data.clear();
        data.extend(iter);
        MyStorage { data }
    }
    fn from_order_0(order_0: Self::Item, _zeros: impl Fn() -> Self::Item) -> Self::Owned {
        let mut data = MyStorage::<T>::acquire_storage();
        data.clear();
        data.push(order_0);
        MyStorage { data }
    }
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        let mut data = MyStorage::<T>::acquire_storage();
        data.clear();
        data.extend_from_slice(slice);
        MyStorage { data }
    }
    fn is_owned(&self) -> bool {
        true
    }
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item> {
        let mut index = 0;
        std::iter::from_fn(move || {
            let i = index;
            let data = &self.data;
            if i < data.len() {
                index += 1;
                Some(data[i].clone())
            } else {
                None
            }
        })
    }
    fn map_into_owned(mut self, mut f: impl FnMut(&mut Self::Item, usize)) -> Self::Owned {
        for (i, x) in self.data.iter_mut().enumerate() {
            f(x, i);
        }
        self
    }
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        let mut data = MyStorage::<T>::acquire_storage();
        data.clear();
        data.extend((0..len).map(f));
        MyStorage { data }
    }
}

impl<T: Clone + 'static> MutStorage for MyStorage<T> {
    fn assign_iter(&mut self, offset: usize, iter: impl IntoIterator<Item = Self::Item>) {
        let data = &mut self.data;
        data.shrink_to(offset);
        data.extend(iter);
    }
    fn slice_mut(&mut self) -> &mut [Self::Item] {
        &mut self.data
    }
}

impl Clone for MyStorage<f64> {
    fn clone(&self) -> Self {
        let mut data = MyStorage::<f64>::acquire_storage();
        data.clear();
        data.extend(self.data.iter().cloned());
        MyStorage { data }
    }
}

impl<T: 'static> Drop for MyStorage<T> {
    fn drop(&mut self) {
        MyStorage::<T>::return_storage(std::mem::take(&mut self.data));
    }
}