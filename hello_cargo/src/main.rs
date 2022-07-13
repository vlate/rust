// 斐波那契数列

fn main() {
    let a = 10;
    for i in 0..a {
        println!("fib is{}, {}", i + 1, fib(i))
    }
}

fn fib(a: i32) -> i32 {
    if a <= 1 {
        a
    } else {
        fib(a - 1) + fib(a - 2)
    }
}
