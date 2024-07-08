fn main() {
    let string_1 = "Hello There!".to_string();
    let string_2 = "threelolhe".to_string();

    let res = anagrams(string_1.clone(), string_2.clone());

    println!("{} is an anagram of {}? {}", string_1, string_2, res);
}

fn anagrams(str_1: String, str_2: String) -> bool {
    // split ea string into vec of chars
    // filter out non-alpha chars
    // map over vec and lower-case ea char
    // sort vec
    // compare

    let split_vec_1: Vec<char> = str_1.chars().collect();
    // println!("char vec: {:?}", split_vec_1);
    let split_vec_2: Vec<char> = str_2.chars().collect();

    let mut filtered_vec_1: Vec<char> = split_vec_1
        .into_iter()
        .filter(|c| c.is_alphabetic())
        .collect();
    let mut filtered_vec_2: Vec<char> = split_vec_2
        .into_iter()
        .filter(|c| c.is_alphabetic())
        .collect();
    // println!("alpha only vec: {:?}", filtered_vec_1);

    let mut lower_filtered_vec_1: Vec<char> = filtered_vec_1
        .iter_mut()
        .map(|c| c.to_ascii_lowercase())
        .collect();
    lower_filtered_vec_1.sort();

    let mut lower_filtered_vec_2: Vec<char> = filtered_vec_2
        .iter_mut()
        .map(|c| c.to_ascii_lowercase())
        .collect();
    lower_filtered_vec_2.sort();

    // println!("lower filtered vec: {:?}", lower_filtered_vec_1);
    let final_comp_string_1: String = lower_filtered_vec_1.into_iter().collect();
    let final_comp_string_2: String = lower_filtered_vec_2.into_iter().collect();

    final_comp_string_1 == final_comp_string_2
}
