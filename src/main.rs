use rug::Integer;

fn main() {
    let a = Integer::from(1u64 << 21); // a = 2^21
    let b = Integer::from(3u64 << 22); // b = 3 * 2^22

    let sum = a.clone() + b.clone();
    let difference = a.clone() - b.clone();
    let product = a.clone() * b.clone();
    let decimal = a.clone().to_f64() / b.clone().to_f64();

    println!("a = {}", a);
    println!("b = {}", b);
    println!("Sum: a + b = {}", sum);
    println!("Subtraction: a - b = {}", difference);
    println!("Multiplication: a * b = {}", product);
    println!("Division: a / b = {:.10}", decimal);
}
