#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate num;

use std::{cmp, iter};
use num::Integer;
use common::Solver;

fn count_right_at_o(x_max: uint, y_max: uint) -> uint {
    x_max * y_max
}

fn count_right_at_p(x_max: uint, y_max: uint) -> uint {
    let mut cnt = x_max * y_max; // p: (0, y0) q: (xi, y0) => xi: [1, x_max], y0: [0, y_max]

    for x in iter::range_inclusive(1, x_max) {
        for y in iter::range_inclusive(1, y_max) {
            let d = x.gcd(&y);
            let (dx, neg_dy) = (y / d, x / d);
            cnt += cmp::min(y / neg_dy, (x_max - x) / dx);
        }
    }

    cnt
}

fn compute(x_max: uint, y_max: uint) -> uint {
    count_right_at_o(x_max, y_max) + count_right_at_p(x_max, y_max) * 2
}

fn solve() -> String {
    compute(50, 50).to_string()
}

fn main() { Solver::new("14234", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn two() {
        assert_eq!(4, super::count_right_at_o(2, 2));
        assert_eq!(5, super::count_right_at_p(2, 2));
        assert_eq!(14, super::compute(2, 2));
    }
}