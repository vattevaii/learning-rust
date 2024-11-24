fn sum(a: i32, b: i32) -> i32 {
    a + b
}
pub fn fibonacci() {
    let mut count = 10;
    let mut current = 1;
    let mut a = 1;
    let mut b = 1;
    loop {
        println!("{} - {}", current, a);
        let c = sum(a, b);
        a = b;
        b = c;
        count -= 1;
        current += 1;
        if count == 0 {
            break;
        }
    }
}

// 1 1 2 3 5 8 13 21 34 55
