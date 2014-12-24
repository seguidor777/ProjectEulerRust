#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;

use std::iter::AdditiveIterator;
use std::io::{BufferedReader, File, IoResult};

fn is_sss(nums: &mut [uint]) -> bool {
    nums.sort();

    let len = nums.len();
    let len_hd = (len + 1) / 2;
    let len_tl = len_hd - 1;
    let hd = nums[.. len_hd].iter().map(|&x| x).sum();
    let tl = nums[len - len_tl ..].iter().map(|&x| x).sum();
    if hd <= tl { return false }

    let mut sums = vec![0u];
    for &n in nums.iter() {
        let mut i = 0;
        let mut j = 0;
        let len = sums.len();
        let mut new_sums = Vec::with_capacity(len * 2);
        while i < len {
            assert!(j <= i);
            match sums[i].cmp(&(sums[j] + n)) {
                Equal   => {  return false }
                Less    => { new_sums.push(sums[i]);     i += 1; }
                Greater => { new_sums.push(sums[j] + n); j += 1; }
            }
        }
        while j < len { new_sums.push(sums[j] + n); j += 1; }
        sums = new_sums;
    }

    true
}

fn solve(file: File) -> IoResult<String> {
    let mut br = BufferedReader::new(file);

    let mut sum = 0;
    for line in br.lines() {
        let mut nums = try!(line)
            .trim()
            .split(',')
            .filter_map(StrExt::parse::<uint>)
            .collect::<Vec<_>>();

        if is_sss(nums.as_mut_slice()) {
            sum += nums.iter().map(|&x| x).sum();
        }
    }

    Ok(sum.to_string())
}

problem!("73702", "p105_sets.txt", solve);