#[link(name = "prob0048", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::uint;
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 48,
    answer: "9110846700",
    solver: solve
};

fn pow_mod(base: uint, exponent: uint, modulo: uint) -> uint {
    if base == 0 { return 0; }
    let mut acc = 1;
    for exponent.times {
        acc = (acc * base) % modulo;
    }
    return acc;
}

pub fn solve() -> ~str {
    let modulo  = 100_0000_0000;
    let mut sum = 0;
    for uint::range(1, 1000 + 1) |n| {
        sum = (sum + pow_mod(n, n, modulo)) % modulo;
    }
    return sum.to_str();
}