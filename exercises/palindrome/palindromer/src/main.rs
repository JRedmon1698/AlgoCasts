fn main() {
    let res_1 = palindrome("abc".to_string());
    let res_2 = palindrome("abccba".to_string());

    println!("Result 1: {} \n Result 2: {}", res_1, res_2);
}

fn palindrome(s: String) -> bool {
    let split_s: Vec<char> = s.chars().collect();
    // println!("split str: {:?}", split_s);

    let rev_vec: Vec<char> = split_s.clone().into_iter().rev().collect();
    // println!("rev vec: {:?}", rev_vec);

    let rev_string: String = rev_vec.into_iter().collect();
    // println!("reversed string: {}", rev_string);

    rev_string == s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(
            !palindrome(String::from("abc")),
            "'abc' is not a palindrome"
        );
        assert!(
            !palindrome(String::from("  aba")),
            "'  aba' is not a palindrome"
        );

        assert!(palindrome(String::from("aba")), "'aba' is a palindrome");
        assert!(palindrome(String::from("10001")), "'10001' is a palindrome");
    }
}
