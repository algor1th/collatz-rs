fn main() {
    let mut max_len = 0;
    let max = 1000000;
    let range = 2..max as u64;
    let mut cache = vec![1,1];
    cache.reserve(max);
    let mut val_for_max = 0;
    for i in range {
        let l = collatz_size(i, &mut cache);
        if l > max_len {
            max_len = l;
            val_for_max = i;
        }
    }
    collatz_series(val_for_max, max_len);
    println!("collatz sequence length: {}", max_len);
}
fn collatz_series(n: u64, max_len: usize) {
    let mut v = Vec::with_capacity(max_len);
    let mut i = n;
    while i != 1 {
        v.push(i);
        i = match i {
            i if i % 2 == 0 => i/2,
            i => i * 3 + 1
        };
    }
    println!("{:?}", v)
}
fn collatz_size(n: u64, cache: &mut Vec<usize>) -> usize {
    let mut i = n;
    let mut count = 0;
    while i >= n {
        i = match i {
            i if i % 2 == 0 => i/2,
            i => i * 3 + 1
        };
        count += 1;
    }
    let r = cache[i as usize] + count;
    cache.insert(n as usize, r);
    r
}
