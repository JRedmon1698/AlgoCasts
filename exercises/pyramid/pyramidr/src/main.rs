fn main() {
    let res = pyramid(7);
    println!("{:?}", res);
    // let spaces = add_spaces("#######".to_string(), 3);
    // println!("{}", spaces);
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

    let start_string = base_string.clone();
    let halfway: f64 = (start_string.len() / 2) as f64;
    let start_point = (halfway.floor() as usize);
    for i in (0..=start_point).rev() {
        let display_str = add_spaces(start_string.clone(), i);
        println!("{}", display_str);
    }
}

fn add_spaces(s: String, n: usize) -> String {
    let mut spaces = String::new();
    for _ in 0..n {
        spaces.push_str(" ");
    }

    let end = s.len() - n;
    let slice = &s[n..end];
    let spaced_str: String = format!("{}{}{}", spaces, slice, spaces);

    spaced_str
}
