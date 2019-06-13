fn main() {
    let mut max_len = 0;
    let mut max_sequence: Vec<u64> = vec![];
    let range = 1..1000000;
    let mut v = vec![];
    for i in range {
        v = collatz(v, i);
        if v.len() > max_len {
            max_len = v.len();
            max_sequence = v.clone();
        }
    }
    // println!("{:?}", max_sequence);
    println!("{:?}\nCollatz sequence length: {}", max_sequence, max_len)
}
fn collatz(mut v: Vec<u64>, n: u64) -> Vec<u64> {
    v.clear();
    let mut i = n;
    while i != 1 {
        if i % 2 == 0 {
            i /= 2;
        } else {
            i = 3 * i + 1;
        }
        v.push(i);
    }
    v
}
