fn main() {
    let mut max_len = 0;
    let mut max_sequence: Vec<u64> = vec![];
    for i in 1..1000000 {
        let v = collatz(i);
        if v.len() > max_len {
            max_len = v.len();
            max_sequence = v;
        }
    }
    println!("{:?}", max_sequence);
    println!("Collatz sequence length: {}", max_len)
}
fn collatz(n: u64) -> Vec<u64> {
    let mut iterations = vec![];
    let mut i = n;
    while i != 1 {
        if i % 2 == 0 {
            i /= 2;
        } else {
            i = 3 * i + 1;
        }
        iterations.push(i);
    }
    iterations
}
