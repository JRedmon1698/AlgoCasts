fn main() {
    fizz_buzz(30);
}

fn fizz_buzz(n: usize) -> String {
    for num in 1..n {
        match (num % 5 == 0, num % 3 == 0) {
            (true, true) => println!("fizzbuzz"),
            (true, false) => println!("buzz"),
            (false, true) => println!("fizz"),
            (false, false) => println!("{}", num),
        }
    }

    String::from("ballast")
}
