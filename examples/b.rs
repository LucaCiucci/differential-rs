use differential::*;
use std::hint::black_box;


fn main() {
    let d = Diff::<Fixed::<4>, Fixed::<1>, &[f64; 5]>::from_data(
        Fixed,
        Fixed,
        &[2.0, 1.0, 0.0, 0.0, 0.0],
    );

    //let d: Diff<Fixed<4>, Fixed<1>, [f64; 5]> = d * &d * &d;

    println!("{:?}", d);
    //println!("{:?}", d.drop_one_order());

    let diff = Diff::from_data(
        Dynamic(9),
        Fixed::<1>,
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
    );

    println!("{:?}", diff[&[1]]);
    println!("{:?}", diff.drop_one_order());
    println!("{:?}", diff.drop_one_order() * diff.drop_one_order());
    println!("{:?}", (diff.clone() * diff.clone()).drop_one_order());
    println!("{:?}", (diff.clone() * diff.clone()));
    println!("{:?}", diff.clone().drop_one_order()[&[1]]);

    let d = Diff::<Fixed::<1>, Fixed::<1>, &[f64; 2]>::from_data(
        Fixed,
        Fixed,
        &[2.0, 1.0],
    );
    let _d2: Diff<Fixed<1>, Fixed<1>, [f64; 2]> = d * d;

    let start = std::time::Instant::now();
    let n = 1000000u64;
    for _ in 0..n {
        //let diff = d;
        black_box(black_box(diff.clone()) * black_box(diff.clone()));
    }
    println!("{}ms", start.elapsed().as_millis());
    println!("{}ns/iter", start.elapsed().as_nanos() as f64 / n as f64);
    println!("{:?}", d * d);

    let d = 1.0f64;
    let start = std::time::Instant::now();
    for _i in 0..1000000000 {
        black_box(black_box(d) * black_box(d));
    }
    println!("{}ms", start.elapsed().as_millis());
    println!("{}ns/iter", start.elapsed().as_nanos() as f64 / 1000000000u128 as f64);

    //let a: i32 = 0.0f64.into();
}