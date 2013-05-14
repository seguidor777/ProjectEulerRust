#[link(name = "prob0025", vers = "0.0")];
#[crate_type = "lib"];

extern mod std;
extern mod common;

use core::iterator::{ IteratorUtil };
use core::from_str::{ FromStr };
use std::bigint::{ BigUint };
use common::extiter::{ Fibonacci, ExtIteratorUtil };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 25,
    answer: "4782",
    solver: solve
};

pub fn solve() -> ~str {
    let limit = FromStr::from_str(str::repeat("9", 999)).get();
    let it = Fibonacci::new::<BigUint>().take_while(|&n| n <= limit);
    return (it.count_elem() + 1).to_str();
}
