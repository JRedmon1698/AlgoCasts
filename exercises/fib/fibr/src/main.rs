fn main() {
    let res = fib(50);
    println!("{res}");
}

// fn fib(n: usize) -> usize {
//     if n < 2 {
//         return n;
//     }

//     return fib(n - 1) + fib(n - 2);
// }

fn fib(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut current: usize = 0;
    let mut previous: usize = 1;
    let mut second_previous: usize = 0;

    for i in 2..=n {
        current = previous + second_previous;
        second_previous = previous;
        previous = current;
    }

    current
}
