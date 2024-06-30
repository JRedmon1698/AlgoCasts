fn main() {
    println!("reverse of 123 is {}", reverseInt(123));
    println!("reverse of -120 is {}", reverseInt(-120));
}

fn reverseInt(i: isize) -> isize {
    let int_string = i.to_string();

    let abs_vec = match i < 0 {
        true => int_string
            .chars()
            .into_iter()
            .skip(1)
            .collect::<Vec<char>>(),
        false => int_string.chars().into_iter().collect::<Vec<char>>(),
    };

    let rev_vec: Vec<char> = abs_vec.into_iter().rev().collect();

    let mut rev_string: String = rev_vec.into_iter().collect();
    if let true = i < 0 {
        rev_string = format!("-{}", rev_string)
    }

    rev_string.parse::<isize>().unwrap()
}
