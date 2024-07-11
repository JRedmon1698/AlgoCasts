fn main() {
    println!("{:?}", steps(3));
    // println!("spaces: {}", add_spaces(3));
}

fn steps(n: u32) {
    let mut step_string = String::new();
    for step in 0..n {
        step_string.push_str("#");
    }

    let print_string: String = step_string.clone();
    for (idx, char) in step_string.char_indices().rev() {
        println!("{}{}", &step_string[idx..], add_spaces(idx as u32));
    }
}

fn add_spaces(n: u32) -> String {
    let mut spaces = String::new();
    for num in 0..n {
        spaces.push_str(" ");
    }

    spaces
}
