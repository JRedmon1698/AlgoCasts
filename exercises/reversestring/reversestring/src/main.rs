fn main() {
    let res = reverse("hello".to_string());
    println!("Result: {}", res);
}

fn reverse(str: String) -> String {
    let split_str: Vec<char> = str.chars().collect();
    // println!("split string: {:?}", split_str);

    let reversed_vec: Vec<char> = split_str.clone().into_iter().rev().collect();
    // println!("reversed split string: {:?}", reversed_vec);

    let res_string: String = reversed_vec.into_iter().collect();
    // println!("result: {}", res_str);

    res_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let res_1 = reverse("hello".to_string());
        assert_eq!(res_1, "olleh".to_string());

        let res_2 = reverse("  abc".to_string());
        assert_eq!(res_2, "cba  ".to_string());
    }
}
