use std::{char, collections::HashMap};

fn main() {
    println!("Max char: {}", maxchars("abca".to_string()));
    println!("Max char: {}", maxchars("abbba".to_string()));
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
    // println!("map: {:?}", char_map);

    let mut max_char: char = char_vec[0];
    let mut max_char_count: usize = 0;
    for key in char_map.keys() {
        if *char_map.get(key).unwrap() > max_char_count {
            max_char_count = *char_map.get(key).unwrap();
            max_char = *key;
        }
    }

    max_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        assert_eq!(maxchars(String::from("abbcaa")), 'a');
        assert_eq!(maxchars(String::from("12333")), '3');
    }
}
