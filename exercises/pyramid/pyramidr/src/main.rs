fn main() {
    let res = pyramid(3);
    println!("{:?}", res);
    let spaces = add_spaces("#######".to_string(), 1);
    println!("{}", spaces);
}

fn pyramid(n: usize) {
    if n == 1 {
        println!("#");
    }

    let mut base_string = String::new();
    let base_length: usize = (n * 2) - 1;
    for i in 0..base_length {
        base_string.push_str("#");
    }
}

fn add_spaces(s: String, n: usize) -> String {
    let mut spaces = String::new();
    for i in 0..n {
        spaces.push_str(" ");
    }

    let length = s.len() - n;
    let slice = &s[n..length];
    let spaced_str: String = format!("{}{}{}", spaces, slice, spaces);

    spaced_str
}
