extern crate rayon;
use rayon::prelude::*;

fn main() {
    let max = 200_000_000;
    let mut cache = vec![1, 1];
    cache.reserve(max);
    let val_for_max = (2..max)
        .into_par_iter()
        .max_by_key(|x| collatz_thr(x))//(x, &mut cache))
        .unwrap() as u64;
    collatz_series(val_for_max);
}
fn collatz_series(n: u64) {
    let mut v = 0;
    let mut i = n;
    while i != 1 {
        print!("{} ", i);
        v += 1;
        i = match i {
            i if i % 2 == 0 => i / 2,
            i => i * 3 + 1,
        };
    }
    println!(", length: {}", v)
}
fn collatz_thr(n: &usize) -> u32 {
    let mut i = *n;
    let mut count = 0;
    while i != 1 {
        i = match i {
            i if i % 2 == 0 => i / 2,
            i => i * 3 + 1,
        };
        count += 1;
    }
    count
}
fn collatz_size(n: &usize, cache: &mut Vec<u16>) -> u16 {
    let mut i = *n;
    let mut count = 0;
    while i >= *n {
        i = match i {
            i if i % 2 == 0 => i / 2,
            i => i * 3 + 1,
        };
        count += 1;
    }
    let r = cache[i as usize] + count;
    cache.insert(*n as usize, r);
    r
}
