fn main() {
    let res = capitalize("why hello, there! babaganoush here.".to_string());
    println!("{res}");
}

fn capitalize(s: String) -> String {
    let split_str: Vec<String> = s.split(' ').map(|s| s.to_string()).collect();
    // println!("split string: {:?}", split_str);

    let mut capitalized_vec: Vec<String> = Vec::new();

    for word in split_str.iter() {
        let first_char: char = word.chars().nth(0).unwrap().to_ascii_uppercase();
        // println!("first char: {}", first_char);

        let capitalized_word = format!("{}{}", first_char, word[1..].to_string());
        // println!("cap word: {capitalized_word}");
        capitalized_vec.push(capitalized_word);
    }

    // println!("cap vec: {:?}", capitalized_vec);
    capitalized_vec.join(" ")
}
