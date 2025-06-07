use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub}};

use differential::{ConstStorage, Differential, Dim, Dynamic, IntoOwned};
use num_traits::{Float, Num, NumCast, One, ToPrimitive, Zero};

#[test]
fn main() {
    let mut idx = vec![0, 0, 0];
    let mut count = 0;
    iterate_derivatives(&mut idx, 3, 3, &mut |idx| {
        print_idx(idx);
        count += 1;
        println!();
    });
    println!("count: {}", count);

    let n = 2;
    let order = 5;

    let data = make_data(n, order);

    let diff = Differential::from_data(
        Dynamic(Some(order)),
        Dynamic(Some(n)),
        data,
    );
    check_data(&diff);
    let diff = diff.clone() + diff;
    check_data(&diff);
    let diff = diff.clone() * diff;
    check_data(&diff);
    check_data(&diff.drop_one_order().into_owned());
    check_data(&diff.drop_one_order());

    println!("{:?}", diff);
    println!("{:?}", diff.drop_one_order().into_owned());

    panic!("OKKKKKKK");
}

fn make_data(n: usize, order: usize) -> Vec<OVar> {
    let mut data = Vec::new();
    let mut idx = vec![0; n];
    iterate_derivatives(&mut idx, n, order, &mut |idx| {
        data.push(OVar::from_index_slice(idx));
    });
    data
}

fn check_data<S: ConstStorage<Item = OVar>>(d: &Differential<Dynamic, Dynamic, S>) {
    println!("checking differential with n = {} and order = {}", d.n().value().unwrap(), d.order().value().unwrap());
    let expected_data = make_data(d.n().value().unwrap(), d.order().value().unwrap());
    println!("expected_data: {:?}", expected_data);
    println!("data         : {:?}", d.data_slice());
    assert_eq!(d.data_slice(), expected_data.as_slice());
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct OVar {
    pub order: i32, // TODO or maybe use Option?
}

impl Debug for OVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "o({})", self.order)
    }
}

impl OVar {
    pub fn new(order: i32) -> Self {
        Self { order }
    }
    pub fn from_index_slice(index: &[usize]) -> Self {
        Self { order: index.iter().sum::<usize>() as i32 }
    }
}

impl Add for OVar {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.order != 0 && other.order != 0 {
            assert_eq!(self.order, other.order);
        }
        Self { order: self.order.max(other.order) }
    }
}

impl AddAssign for OVar {
    fn add_assign(&mut self, other: Self) {
        if self.order != 0 && other.order != 0 {
            assert_eq!(self.order, other.order);
        }
        self.order = self.order.max(other.order);
    }
}

impl Mul for OVar {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self { order: self.order + other.order }
    }
}

impl Mul<&OVar> for OVar {
    type Output = Self;
    fn mul(self, other: &Self) -> Self {
        self * *other
    }
}

impl MulAssign for OVar {
    fn mul_assign(&mut self, other: Self) {
        self.order += other.order;
    }
}

impl Zero for OVar {
    fn zero() -> Self {
        Self { order: 0 }
    }
    fn is_zero(&self) -> bool {
        self.order == 0
    }
}

impl Neg for OVar {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}

impl ToPrimitive for OVar {
    fn to_i64(&self) -> Option<i64> {
        Some(0)
    }

    fn to_u64(&self) -> Option<u64> {
        Some(0)
    }
}

impl NumCast for OVar {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        Some(Self { order: 0 })
    }
}

impl Num for OVar {
    type FromStrRadixErr = ();
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self { order: 0 })
    }
}

impl Div for OVar {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self { order: self.order - other.order }
    }
}

impl DivAssign for OVar {
    fn div_assign(&mut self, other: Self) {
        self.order -= other.order;
    }
}

impl Rem for OVar {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Self { order: self.order % other.order }
    }
}

impl Sub for OVar {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        assert_eq!(self.order, other.order);
        self
    }
}

impl One for OVar {
    fn one() -> Self {
        Self { order: 0 }
    }
}

fn iterate_derivatives(
    idx: &mut [usize],
    n: usize,
    order: usize,
    f: &mut impl FnMut(&[usize]),
) {
    f(&idx);

    if order == 0 {
        return;
    }

    for i in 0..idx.len() {
        if i >= n {
            break;
        }
        idx[i] += 1;
        iterate_derivatives(idx, i + 1, order - 1, f);
        idx[i] -= 1;
    }
}

fn print_idx(idx: &[usize]) {
    assert_eq!(idx.len(), 3);
    for _ in 0..idx[0] {
        print!("x");
    }
    for _ in 0..idx[1] {
        print!("y");
    }
    for _ in 0..idx[2] {
        print!("z");
    }
}