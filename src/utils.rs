

#[inline(always)]
pub const fn number_of_elements(n: usize, order: usize) -> usize {
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
                total += number_of_elements(n, order - 1);
                n -= 1;
            }
            total
        }
    }
}

#[inline(always)]
pub fn offset_of( // TODO const fn
    index: &[usize],
    n: usize,
    order: usize,
) -> usize {
    // TODO if N = 1 (1D) and order very large, this will overflow as it is recursive
    // (same thing if N large) maybe use a loop instead
    assert!(index.iter().sum::<usize>() <= order);
    if index.len() == 0 {
        0
    } else {
        offset_of_impl(index[0], &index[1..], n, order)
    }
}

pub fn offset_of_impl( // TODO const fn
    index_0: usize,
    index_tail: &[usize],
    n: usize,
    order: usize,
) -> usize {
    // TODO if N = 1 (1D) and order very large, this will overflow as it is recursive
    // (same thing if N large) maybe use a loop instead
    if order == 0 {
        // TODO debug_assert ?
        assert!(index_0 == 0);
        assert!(index_tail.iter().all(|&i| i == 0));
    }
    if index_0 == 0 {
        if index_tail.len() == 0 {
            0
        } else {
            offset_of_impl(
                index_tail[0],
                &index_tail[1..],
                n - 1,
                order,
            )
        }
    } else {
        (1 + offset_under(n, 0, order)) + offset_of_impl(index_0 - 1, index_tail, n, order - 1)
    }
}

pub const fn offset_under(
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