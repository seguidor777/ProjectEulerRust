#[link(name = "data", vers = "0.0")];
#[crate_type = "lib"];

#[feature(globs)];

extern mod extra;

pub mod card;
pub mod extiter;
pub mod monoid;
pub mod union_find;
