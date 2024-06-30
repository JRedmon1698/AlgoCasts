use std::{char, collections::HashMap};

fn main() {
    println!("Max char: {}", maxchars("abca".to_string()));
}

fn maxchars(s: String) -> char {
    let mut char_map: HashMap<char, usize> = HashMap::new();

    let char_vec: Vec<char> = s.chars().collect();
    // println!("char vec: {:?}", char_vec);

    for char in char_vec.iter() {
        match char_map.get(char) {
            Some(mut count) => {
                let mut new_count = count + 1;
                char_map.insert(*char, new_count);
            }
            None => {
                char_map.insert(*char, 1);
            }
        };
    }

    for val in char_map.values() {}

    'a'
}
