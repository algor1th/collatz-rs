extern crate rayon;
use rayon::prelude::*;

struct Cache {
    mem: Vec<u16>,
    limit: usize
}
impl Cache {
    fn new(size: usize) -> Cache {
        let mut cache = vec![1, 1];
        cache.reserve(size);
        let cache = Cache {
                mem: cache,
                limit: size
            };
        cache
    }
}

const CACHE_LIM: usize = 256 * 1024 * 1024;
fn main() {
    let max = 2000_000_000u64;
    
    use std::cmp;
    let cache_size = cmp::min(CACHE_LIM, max as usize);
    let mut cache = Cache::new(cache_size);
    for x in 2..cache_size as u64 {
        collatz_size(&x, &mut cache);
    };

    let val_for_max = (2..max)
        .into_par_iter()
        .max_by_key(|x| collatz_thr(x, &cache))
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
fn collatz_thr(n: &u64, cache: &Cache) -> u16 {
    let mut i = *n;
    let mut count = 0;
    while i as usize >= cache.limit {
        i = match i {
            i if i % 2 == 0 => i / 2,
            i => i * 3 + 1,
        };
        count += 1;
    }
    count + cache.mem[i as usize]
}
fn collatz_size(n: &u64, cache: &mut Cache) -> u16 {
    let mut i = *n;
    let mut count = 0;
    while i >= *n {
        i = match i {
            i if i % 2 == 0 => i / 2,
            i => i * 3 + 1,
        };
        count += 1;
    }
    let r = cache.mem[i as usize] + count;
    cache.mem.insert(*n as usize, r);
    r
}
