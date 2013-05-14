#[link(name = "prob0023", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 23,
    answer: "4179871",
    solver: solve
};

#[inline(always)]
fn is_abundant(n: uint) -> bool {
    prime::sum_of_proper_divisors(n) > n
}

pub fn solve() -> ~str {
    let max_num = 28123;

    let abundant = do vec::build_sized(max_num + 1) |push| {
        for uint::range(2, max_num + 1) |n| {
            if is_abundant(n) { push(n); }
        }
    };

    let mut sum_of_sum_abundant = 0;
    let mut is_sum_abundant = vec::from_elem(max_num + 1, false);
    for abundant.eachi |i, &a| {
        for abundant.tailn(i).each |&b| {
            let s = a + b;
            if s > max_num { break; }
            if !is_sum_abundant[s] { sum_of_sum_abundant += s; }
            is_sum_abundant[s] = true;
        }
    }

    let sum_of_all_int = (1 + max_num) * max_num / 2;

    return (sum_of_all_int - sum_of_sum_abundant).to_str();
}
