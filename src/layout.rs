use crate::{static_memo, DiffIndex};


#[inline(always)]
pub const fn maybenumber_of_elements(n: Option<usize>, order: Option<usize>) -> usize {
    if let (Some(n), Some(order)) = (n, order) {
        number_of_elements(n, order)
    } else {
        1
    }
}

#[inline(always)]
pub const fn number_of_elements(n: usize, order: usize) -> usize {
    let params = (n, order);
    static_memo!(number_of_elements_impl(params) -> <usize> for [
        (1, 1), (2, 1), (3, 1),
        (1, 2), (2, 2), (3, 2),
        (1, 3), (2, 3), (3, 3),
    ])
}

#[inline(always)]
pub const fn number_of_elements_impl((n, order): (usize, usize)) -> usize {
    if order == 0 {
        1
    } else if n == 1 {
        1 + order
    } else if order == 1 {
        1 + n
    } else {
        // general case
        let mut total = 1;
        if order == 0 {
            total
        } else {
            let mut n = n;
            while n > 0 {
                total += number_of_elements_impl((n, order - 1));
                n -= 1;
            }
            total
        }
    }
}

#[inline(always)]
pub fn offset_of( // TODO const fn
    index: impl DiffIndex,
    n: usize,
    order: usize,
) -> usize {
    let orders = index.into_orders();

    // TODO if N = 1 (1D) and order very large, this will overflow as it is recursive
    // (same thing if N large) maybe use a loop instead
    assert!(orders.clone().sum::<usize>() <= order);
    if orders.clone().count() == 0 {
        0
    } else {
        let mut orders = orders;
        let index_0 = orders.next().unwrap();
        offset_of_impl(index_0, orders, n, order)
    }
}

pub fn offset_of_impl( // TODO const fn
    index_0: usize,
    index_tail: impl DiffIndex,
    n: usize,
    order: usize,
) -> usize {
    let tail_orders = index_tail.into_orders();

    // TODO if N = 1 (1D) and order very large, this will overflow as it is recursive
    // (same thing if N large) maybe use a loop instead
    if order == 0 {
        // TODO debug_assert ?
        assert!(index_0 == 0);
        assert!(tail_orders.clone().all(|i| i == 0));
    }
    if index_0 == 0 {
        if tail_orders.clone().count() == 0 {
            0
        } else {
            let mut tail_orders = tail_orders;
            let index_0 = tail_orders.next().unwrap();
            offset_of_impl(
                index_0,
                tail_orders,
                n - 1,
                order,
            )
        }
    } else {
        (1 + offset_under(n, 0, order)) + offset_of_impl(index_0 - 1, tail_orders, n, order - 1)
    }
}

#[inline(always)]
pub const fn offset_under(
    n: usize,
    ii: usize,
    order: usize,
) -> usize {
    // TODO to check
    if order == 0 {
        0
    } else if order == 1 {
        n - ii - 1
    } else {
        offset_under_plain(n, ii, order)
    }
}

#[inline(always)]
pub const fn offset_under_plain(
    n: usize,
    ii: usize,
    order: usize,
) -> usize {
    let mut offset = 0;
    let mut i = 0;
    while i < (n - ii - 1) {
        i += 1;
        offset += number_of_elements(i, order - 1);
    }
    offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_under_optimizations() {
        //assert_eq!(offset_under(10, 0, 0), offset_under_impl(10, 0, 0)); TODO panics
        //assert_eq!(offset_under(10, 5, 0), offset_under_impl(10, 5, 0)); TODO panics
        assert_eq!(offset_under(10, 0, 1), offset_under_plain(10, 0, 1));
        assert_eq!(offset_under(10, 5, 1), offset_under_plain(10, 5, 1));
        assert_eq!(offset_under(13, 7, 1), offset_under_plain(13, 7, 1));
    }
}