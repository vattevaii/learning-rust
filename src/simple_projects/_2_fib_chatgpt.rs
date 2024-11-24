fn generate_fib_iterator(count: usize) -> Vec<i32> {
    let mut seq = vec![1, 1];
    for _ in 2..count {
        let next = seq[seq.len() - 2] + seq[seq.len() - 1];
        seq.push(next);
    }
    seq
}
pub fn fibonacci_generate() {
    let count = 10;
    for (index, value) in generate_fib_iterator(count).iter().enumerate() {
        println!("{} - {}", index + 1, value);
    }
}
