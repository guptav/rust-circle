

fn mysum(n: i32) -> i32{
    let mut sum = 0;
    for i in 0..n+1 {
        sum += i;
    }
    sum
}

fn factorial(n: u64) -> u64 {
    if n <= 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn test_str() {
    for i in 0..5 {
        println!("Hello, world{}!", i);
    }
}


fn main() {
    test_str();
    println!("Sum of 1..5  is {:?}", mysum(5));
    println!("Sum of 1..10 is {:?}", mysum(10));
    println!("");
    println!("Factorial of 10 is {:?}", factorial(10));
    println!("Factorial of 100 is {:?}", factorial(100));
    println!("Factorial of 1000 is {:?}", factorial(1000));
}
