use std::io;
use std::time::SystemTime;

fn main() {
    println!("Enter the digit of the fibonacci sequence you want to see.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = input.trim().parse().expect("Please enter a number");
    let input_clone = input.clone();

    // let start = SystemTime::now();
    // let fib = fibonacci(input);
    // let end = start.elapsed().expect("Failed to get elapsed time");

    let start_fast = SystemTime::now();
    let fib_fast = fibonacci_fast(input_clone);
    let end_fast = start_fast.elapsed().expect("Failed to get elapsed time");

    // println!("The fibonacci number is: {}", fib);
    // println!("It took {} ms to calculate", end.as_millis());
    println!("The fibonacci number is: {}", fib_fast);
    println!("It took {} ms to calculate", end_fast.as_millis());
}

fn fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

// a fast fibonacci function that uses memoization
fn fibonacci_fast(n: usize) -> usize {
    let mut memo = vec![0; n + 1];
    memo[0] = 0;
    memo[1] = 1;

    for i in 2..n + 1 {
        memo[i] = memo[i - 1] + memo[i - 2];
    }

    memo[n]
}
