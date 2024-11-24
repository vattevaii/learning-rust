pub fn operators() {
    let num1 = 2;
    let num2 = 5;
    println!(
        "{}\n{}",
        format!("num1 = {}", num1),
        format!("num2 = {}", num2)
    );
    println!("{}", format!("sum + = {}", num1 + num2));
    println!("{}", format!("subtraction - = {}", num1 - num2));
    println!("{}", format!("product * = {}", num1 * num2));
    println!("{}", format!("division / = {}", num1 as f32 / num2 as f32));
    println!("{}", format!("and && (true && false) = {}", true && false));
    println!("{}", format!("or || (true || false) = {}", true || false));
    println!("{}", format!("not ! (!true) = {}", !true));
    println!("{}", format!("bitwise and & = {}", num1 & num2));
    println!("{}", format!("bitwise or | = {}", num1 | num2));
    println!("{}", format!("bitwise not ! (!number) = {}", !num1));
    println!("{}", format!("bitwise xor ^ = {}", num1 ^ num2));
    println!("{}", format!("bitwise shift left << = {}", num1 << num2));
    println!("{}", format!("bitwise shift right >> = {}", num1 >> num2));
}
